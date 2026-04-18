"""
Модуль для детекции дублирования в юридических нормах
"""

import re
from typing import List, Dict, Any, Tuple, Set
import spacy
from sentence_transformers import SentenceTransformer
import numpy as np
from difflib import SequenceMatcher
import logging

from ..core.legal_analyzer import LegalNorm

logger = logging.getLogger(__name__)


class DuplicateDetector:
    
    def __init__(self):
        try:
            self.nlp = spacy.load("ru_core_news_lg")
        except OSError:
            self.nlp = spacy.load("xx_ent_wiki_sm")
            logger.warning("Используется многоязычная модель вместо русской")
        
        # Загрузка модели для семантической схожести
        try:
            self.sentence_model = SentenceTransformer('paraphrase-multilingual-MiniLM-L12-v2')
        except Exception as e:
            logger.warning(f"Не удалось загрузить модель семантической схожести: {e}")
            self.sentence_model = None
        
        self.similarity_thresholds = {
            'exact': 1.0,           # Точное совпадение
            'high': 0.9,            # Высокая схожесть
            'medium': 0.7,          # Средняя схожесть
            'low': 0.5              # Низкая схожесть
        }
        
        self.duplicate_types = {
            'exact_duplicate': 'Точное дублирование',
            'near_duplicate': 'Почти дублирование',
            'semantic_duplicate': 'Семантическое дублирование',
            'partial_duplicate': 'Частичное дублирование',
            'structural_duplicate': 'Структурное дублирование'
        }
    
    def detect_duplicates(self, norms: List[LegalNorm]) -> List[Dict[str, Any]]:
        logger.info(f"Начало детекции дублирования для {len(norms)} норм")
        
        duplicates = []
        
        # 1. Детекция точных дубликатов
        exact_duplicates = self._detect_exact_duplicates(norms)
        duplicates.extend(exact_duplicates)
        
        # 2. Детекция почти дубликатов
        near_duplicates = self._detect_near_duplicates(norms)
        duplicates.extend(near_duplicates)
        
        # 3. Детекция семантических дубликатов
        semantic_duplicates = self._detect_semantic_duplicates(norms)
        duplicates.extend(semantic_duplicates)
        
        # 4. Детекция частичных дубликатов
        partial_duplicates = self._detect_partial_duplicates(norms)
        duplicates.extend(partial_duplicates)
        
        # 5. Детекция структурных дубликатов
        structural_duplicates = self._detect_structural_duplicates(norms)
        duplicates.extend(structural_duplicates)
        
        # Удаление дубликатов и сортировка по схожести
        unique_duplicates = self._remove_duplicate_entries(duplicates)
        unique_duplicates.sort(key=lambda x: x['similarity_score'], reverse=True)
        
        logger.info(f"Обнаружено {len(unique_duplicates)} дубликатов")
        return unique_duplicates
    
    def detect_cross_document_duplicates(self, all_norms: List[LegalNorm], 
                                       results: List[Any]) -> Dict[int, List[Dict[str, Any]]]:
        duplicates_by_document = {}
        
        # Группировка норм по документам
        norms_by_document = {}
        for norm in all_norms:
            if norm.document_id not in norms_by_document:
                norms_by_document[norm.document_id] = []
            norms_by_document[norm.document_id].append(norm)
        
        # Поиск дубликатов между документами
        for doc_id1, norms1 in norms_by_document.items():
            for doc_id2, norms2 in norms_by_document.items():
                if doc_id1 >= doc_id2:  # Избегаем дублирования
                    continue
                
                cross_duplicates = self._find_cross_document_duplicates(norms1, norms2)
                
                # Распределение дубликатов по документам
                for duplicate in cross_duplicates:
                    doc_idx1 = next(i for i, r in enumerate(results) if r.document.id == doc_id1)
                    doc_idx2 = next(i for i, r in enumerate(results) if r.document.id == doc_id2)
                    
                    if doc_idx1 not in duplicates_by_document:
                        duplicates_by_document[doc_idx1] = []
                    if doc_idx2 not in duplicates_by_document:
                        duplicates_by_document[doc_idx2] = []
                    
                    duplicates_by_document[doc_idx1].append(duplicate)
                    duplicates_by_document[doc_idx2].append(duplicate)
        
        return duplicates_by_document
    
    def _detect_exact_duplicates(self, norms: List[LegalNorm]) -> List[Dict[str, Any]]:
        duplicates = []
        seen_texts = {}
        
        for norm in norms:
            # Нормализация текста для сравнения
            normalized_text = self._normalize_text_for_comparison(norm.content)
            
            if normalized_text in seen_texts:
                duplicate = {
                    'type': 'exact_duplicate',
                    'norm1_id': seen_texts[normalized_text],
                    'norm2_id': norm.id,
                    'similarity_score': 1.0,
                    'evidence': 'Точное совпадение текста',
                    'description': 'Нормы полностью идентичны'
                }
                duplicates.append(duplicate)
            else:
                seen_texts[normalized_text] = norm.id
        
        return duplicates
    
    def _detect_near_duplicates(self, norms: List[LegalNorm]) -> List[Dict[str, Any]]:
        duplicates = []
        
        for i, norm1 in enumerate(norms):
            for norm2 in norms[i+1:]:
                # Расчет текстовой схожести
                similarity = SequenceMatcher(None, norm1.content, norm2.content).ratio()
                
                if similarity >= self.similarity_thresholds['high']:
                    duplicate = {
                        'type': 'near_duplicate',
                        'norm1_id': norm1.id,
                        'norm2_id': norm2.id,
                        'similarity_score': similarity,
                        'evidence': f'Текстовая схожесть: {similarity:.3f}',
                        'description': 'Нормы очень похожи, но имеют незначительные отличия'
                    }
                    duplicates.append(duplicate)
        
        return duplicates
    
    def _detect_semantic_duplicates(self, norms: List[LegalNorm]) -> List[Dict[str, Any]]:
        duplicates = []
        
        if not self.sentence_model:
            return duplicates
        
        # Получение эмбеддингов для всех норм
        embeddings = self.sentence_model.encode([norm.content for norm in norms])
        
        # Поиск семантически схожих норм
        for i, norm1 in enumerate(norms):
            for j, norm2 in enumerate(norms[i+1:], i+1):
                similarity = np.dot(embeddings[i], embeddings[j])
                
                if similarity >= self.similarity_thresholds['medium']:
                    # Дополнительная проверка на ключевые слова
                    keyword_similarity = self._calculate_keyword_similarity(norm1, norm2)
                    
                    if keyword_similarity >= 0.6:
                        duplicate = {
                            'type': 'semantic_duplicate',
                            'norm1_id': norm1.id,
                            'norm2_id': norm2.id,
                            'similarity_score': similarity,
                            'evidence': f'Семантическая схожесть: {similarity:.3f}',
                            'description': 'Нормы имеют одинаковое значение, но разную формулировку',
                            'keyword_similarity': keyword_similarity
                        }
                        duplicates.append(duplicate)
        
        return duplicates
    
    def _detect_partial_duplicates(self, norms: List[LegalNorm]) -> List[Dict[str, Any]]:
        duplicates = []
        
        for i, norm1 in enumerate(norms):
            for norm2 in norms[i+1:]:
                # Поиск общих подстрок
                common_substrings = self._find_common_substrings(norm1.content, norm2.content)
                
                if common_substrings:
                    max_common_length = max(len(s) for s in common_substrings)
                    min_length = min(len(norm1.content), len(norm2.content))
                    
                    if max_common_length / min_length >= 0.3:  # Порог частичного дублирования
                        duplicate = {
                            'type': 'partial_duplicate',
                            'norm1_id': norm1.id,
                            'norm2_id': norm2.id,
                            'similarity_score': max_common_length / min_length,
                            'evidence': f'Общая подстрока длиной {max_common_length} символов',
                            'description': 'Нормы содержат общие фрагменты',
                            'common_substrings': common_substrings[:3]  # Топ-3 общие подстроки
                        }
                        duplicates.append(duplicate)
        
        return duplicates
    
    def _detect_structural_duplicates(self, norms: List[LegalNorm]) -> List[Dict[str, Any]]:
        duplicates = []
        
        for i, norm1 in enumerate(norms):
            for norm2 in norms[i+1:]:
                # Анализ структуры норм
                structure1 = self._analyze_structure(norm1.content)
                structure2 = self._analyze_structure(norm2.content)
                
                # Сравнение структур
                structure_similarity = self._compare_structures(structure1, structure2)
                
                if structure_similarity >= 0.8:
                    duplicate = {
                        'type': 'structural_duplicate',
                        'norm1_id': norm1.id,
                        'norm2_id': norm2.id,
                        'similarity_score': structure_similarity,
                        'evidence': f'Структурная схожесть: {structure_similarity:.3f}',
                        'description': 'Нормы имеют одинаковую структуру',
                        'structure1': structure1,
                        'structure2': structure2
                    }
                    duplicates.append(duplicate)
        
        return duplicates
    
    def _normalize_text_for_comparison(self, text: str) -> str:
        # Приведение к нижнему регистру
        text = text.lower()
        
        # Удаление лишних пробелов и переносов строк
        text = re.sub(r'\s+', ' ', text)
        
        # Удаление пунктуации
        text = re.sub(r'[^\w\s]', '', text)
        
        # Удаление стоп-слов
        try:
            doc = self.nlp(text)
            tokens = [token.lemma_ for token in doc if not token.is_stop and not token.is_punct]
            text = ' '.join(tokens)
        except:
            pass
        
        return text.strip()
    
    def _calculate_keyword_similarity(self, norm1: LegalNorm, norm2: LegalNorm) -> float:
        keywords1 = set(norm1.keywords)
        keywords2 = set(norm2.keywords)
        
        if not keywords1 or not keywords2:
            return 0.0
        
        # Коэффициент Жаккара
        intersection = len(keywords1 & keywords2)
        union = len(keywords1 | keywords2)
        
        return intersection / union if union > 0 else 0.0
    
    def _find_common_substrings(self, text1: str, text2: str, min_length: int = 20) -> List[str]:
        common_substrings = []
        
        # Используем SequenceMatcher для поиска общих подстрок
        matcher = SequenceMatcher(None, text1, text2)
        
        for match in matcher.get_matching_blocks():
            if match.size >= min_length:
                substring = text1[match.a:match.a + match.size]
                common_substrings.append(substring)
        
        return common_substrings
    
    def _analyze_structure(self, text: str) -> Dict[str, Any]:
        doc = self.nlp(text)
        
        structure = {
            'sentence_count': len(list(doc.sents)),
            'word_count': len([token for token in doc if not token.is_space]),
            'verb_count': len([token for token in doc if token.pos_ == 'VERB']),
            'noun_count': len([token for token in doc if token.pos_ == 'NOUN']),
            'has_conditions': any(token.text.lower() in ['если', 'в случае', 'при условии'] for token in doc),
            'has_obligations': any(token.text.lower() in ['должен', 'обязан', 'необходимо'] for token in doc),
            'has_permissions': any(token.text.lower() in ['может', 'вправе', 'разрешается'] for token in doc),
            'has_prohibitions': any(token.text.lower() in ['запрещается', 'не допускается'] for token in doc),
            'has_numbers': bool(re.search(r'\d+', text)),
            'has_dates': bool(re.search(r'\d{1,2}\.\d{1,2}\.\d{4}', text))
        }
        
        return structure
    
    def _compare_structures(self, structure1: Dict[str, Any], structure2: Dict[str, Any]) -> float:
        similarity_scores = []
        
        # Сравнение числовых характеристик
        numeric_keys = ['sentence_count', 'word_count', 'verb_count', 'noun_count']
        for key in numeric_keys:
            val1, val2 = structure1.get(key, 0), structure2.get(key, 0)
            if val1 + val2 > 0:
                similarity = 1 - abs(val1 - val2) / max(val1, val2)
                similarity_scores.append(similarity)
        
        # Сравнение бинарных характеристик
        binary_keys = ['has_conditions', 'has_obligations', 'has_permissions', 
                      'has_prohibitions', 'has_numbers', 'has_dates']
        for key in binary_keys:
            val1, val2 = structure1.get(key, False), structure2.get(key, False)
            similarity = 1.0 if val1 == val2 else 0.0
            similarity_scores.append(similarity)
        
        return np.mean(similarity_scores) if similarity_scores else 0.0
    
    def _find_cross_document_duplicates(self, norms1: List[LegalNorm], 
                                      norms2: List[LegalNorm]) -> List[Dict[str, Any]]:
        all_norms = norms1 + norms2
        
        return self.detect_duplicates(all_norms)
    
    def _remove_duplicate_entries(self, duplicates: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        unique_duplicates = []
        seen_pairs = set()
        
        for duplicate in duplicates:
            pair = tuple(sorted([duplicate['norm1_id'], duplicate['norm2_id']]))
            
            if pair not in seen_pairs:
                seen_pairs.add(pair)
                unique_duplicates.append(duplicate)
        
        return unique_duplicates
    
    def get_duplicate_statistics(self, duplicates: List[Dict[str, Any]]) -> Dict[str, Any]:
        if not duplicates:
            return {
                'total_duplicates': 0,
                'by_type': {},
                'average_similarity': 0.0,
                'max_similarity': 0.0,
                'min_similarity': 0.0
            }
        
        # Группировка по типам
        by_type = {}
        for duplicate in duplicates:
            dup_type = duplicate['type']
            by_type[dup_type] = by_type.get(dup_type, 0) + 1
        
        # Статистика схожести
        similarities = [d['similarity_score'] for d in duplicates]
        
        return {
            'total_duplicates': len(duplicates),
            'by_type': by_type,
            'average_similarity': np.mean(similarities),
            'max_similarity': max(similarities),
            'min_similarity': min(similarities),
            'similarity_distribution': {
                'exact': sum(1 for s in similarities if s == 1.0),
                'high': sum(1 for s in similarities if 0.9 <= s < 1.0),
                'medium': sum(1 for s in similarities if 0.7 <= s < 0.9),
                'low': sum(1 for s in similarities if 0.5 <= s < 0.7)
            }
        }
