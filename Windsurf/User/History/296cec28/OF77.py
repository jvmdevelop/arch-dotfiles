"""
Модуль для детекции противоречий в юридических нормах
"""

import re
from typing import List, Dict, Any, Tuple, Set
import spacy
from sentence_transformers import SentenceTransformer
import numpy as np
import logging

from ..core.legal_analyzer import LegalNorm

logger = logging.getLogger(__name__)


class ConflictDetector:
    
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
        
        self.conflict_patterns = {
            'prohibition_permission': [
                (r'запрещается|не допускается', r'разрешается|допускается|вправе'),
                (r'не разрешается', r'может|разрешается'),
                (r'запрет', r'право|разрешение')
            ],
            'obligation_negation': [
                (r'обязан|должен|необходимо', r'не обязан|не должен|не является обязательным'),
                (r'требуется', r'не требуется')
            ],
            'contradictory_conditions': [
                (r'во всех случаях|всегда', r'в отдельных случаях|иногда'),
                (r'каждый|все', r'некоторые|отдельные'),
                (r'обязательно', r'необязательно|факультативно')
            ],
            'temporal_conflicts': [
                (r'незамедлительно|немедленно', r'в течение.*дней|в разумный срок'),
                (r'постоянно|бессрочно', r'временно|на срок')
            ],
            'quantitative_conflicts': [
                (r'не менее (\d+)', r'менее (\d+)'),
                (r'не более (\d+)', r'более (\d+)'),
                (r'ровно (\d+)', r'не (\d+)')
            ]
        }
        
        self.conflict_keywords = {
            'direct_contradiction': ['противоречит', 'не соответствует', 'нарушает'],
            'implicit_contradiction': ['несмотря на', 'вопреки', 'несмотря на то что'],
            'legal_hierarchy': ['уступает', 'имеет преимущественную силу', 'имеет большую юридическую силу']
        }
    
    def detect_conflicts(self, norms: List[LegalNorm], relationships: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        logger.info(f"Начало детекции противоречий для {len(norms)} норм")
        
        conflicts = []
        
        # 1. Детекция прямых противоречий по паттернам
        pattern_conflicts = self._detect_pattern_conflicts(norms)
        conflicts.extend(pattern_conflicts)
        
        # 2. Детекция семантических противоречий
        semantic_conflicts = self._detect_semantic_conflicts(norms)
        conflicts.extend(semantic_conflicts)
        
        # 3. Детекция противоречий через связи
        relationship_conflicts = self._detect_relationship_conflicts(norms, relationships)
        conflicts.extend(relationship_conflicts)
        
        # 4. Детекция иерархических противоречий
        hierarchical_conflicts = self._detect_hierarchical_conflicts(norms, relationships)
        conflicts.extend(hierarchical_conflicts)
        
        # 5. Детекция временных противоречий
        temporal_conflicts = self._detect_temporal_conflicts(norms)
        conflicts.extend(temporal_conflicts)
        
        # Удаление дубликатов и сортировка по уверенности
        unique_conflicts = self._remove_duplicate_conflicts(conflicts)
        unique_conflicts.sort(key=lambda x: x['confidence'], reverse=True)
        
        logger.info(f"Обнаружено {len(unique_conflicts)} уникальных противоречий")
        return unique_conflicts
    
    def detect_cross_document_conflicts(self, all_norms: List[LegalNorm], 
                                      results: List[Any]) -> Dict[int, List[Dict[str, Any]]]:
        conflicts_by_document = {}
        
        # Группировка норм по документам
        norms_by_document = {}
        for norm in all_norms:
            if norm.document_id not in norms_by_document:
                norms_by_document[norm.document_id] = []
            norms_by_document[norm.document_id].append(norm)
        
        # Поиск противоречий между документами
        for doc_id1, norms1 in norms_by_document.items():
            for doc_id2, norms2 in norms_by_document.items():
                if doc_id1 >= doc_id2:  # Избегаем дублирования
                    continue
                
                cross_conflicts = self._find_cross_document_conflicts(norms1, norms2)
                
                # Распределение конфликтов по документам
                for conflict in cross_conflicts:
                    doc_idx1 = next(i for i, r in enumerate(results) if r.document.id == doc_id1)
                    doc_idx2 = next(i for i, r in enumerate(results) if r.document.id == doc_id2)
                    
                    if doc_idx1 not in conflicts_by_document:
                        conflicts_by_document[doc_idx1] = []
                    if doc_idx2 not in conflicts_by_document:
                        conflicts_by_document[doc_idx2] = []
                    
                    conflicts_by_document[doc_idx1].append(conflict)
                    conflicts_by_document[doc_idx2].append(conflict)
        
        return conflicts_by_document
    
    def _detect_pattern_conflicts(self, norms: List[LegalNorm]) -> List[Dict[str, Any]]:
        conflicts = []
        
        for i, norm1 in enumerate(norms):
            for norm2 in norms[i+1:]:
                for conflict_type, pattern_pairs in self.conflict_patterns.items():
                    for pattern1, pattern2 in pattern_pairs:
                        if (re.search(pattern1, norm1.content, re.IGNORECASE) and 
                            re.search(pattern2, norm2.content, re.IGNORECASE)):
                            
                            conflict = {
                                'type': conflict_type,
                                'norm1_id': norm1.id,
                                'norm2_id': norm2.id,
                                'confidence': 0.8,
                                'evidence': f"Паттерн: {pattern1} vs {pattern2}",
                                'description': f"Обнаружено противоречие типа '{conflict_type}'",
                                'norm1_fragment': self._extract_matching_fragment(norm1.content, pattern1),
                                'norm2_fragment': self._extract_matching_fragment(norm2.content, pattern2)
                            }
                            conflicts.append(conflict)
        
        return conflicts
    
    def _detect_semantic_conflicts(self, norms: List[LegalNorm]) -> List[Dict[str, Any]]:
        conflicts = []
        
        if not self.sentence_model:
            return conflicts
        
        # Получение эмбеддингов для всех норм
        embeddings = self.sentence_model.encode([norm.content for norm in norms])
        
        # Поиск семантически схожих норм с противоречивым содержанием
        for i, norm1 in enumerate(norms):
            for j, norm2 in enumerate(norms[i+1:], i+1):
                similarity = np.dot(embeddings[i], embeddings[j])
                
                # Если нормы семантически схожи, но可能有 противоречия
                if similarity > 0.7:  # Порог схожести
                    conflict_score = self._calculate_semantic_conflict_score(norm1, norm2)
                    
                    if conflict_score > 0.6:
                        conflict = {
                            'type': 'semantic_contradiction',
                            'norm1_id': norm1.id,
                            'norm2_id': norm2.id,
                            'confidence': conflict_score,
                            'evidence': f'Семантическая схожесть: {similarity:.2f}',
                            'description': 'Семантически схожие нормы с потенциальным противоречием',
                            'similarity_score': similarity
                        }
                        conflicts.append(conflict)
        
        return conflicts
    
    def _detect_relationship_conflicts(self, norms: List[LegalNorm], 
                                     relationships: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        conflicts = []
        
        # Создание словаря норм для быстрого доступа
        norm_dict = {norm.id: norm for norm in norms}
        
        for rel in relationships:
            if rel['relationship_type'] == 'reference':
                source_norm = norm_dict.get(rel['source_norm_id'])
                target_norm = norm_dict.get(rel['target_norm_id'])
                
                if source_norm and target_norm:
                    # Проверка на противоречие между ссылающейся и ссылочной нормой
                    conflict_score = self._calculate_conflict_score(source_norm, target_norm)
                    
                    if conflict_score > 0.5:
                        conflict = {
                            'type': 'reference_conflict',
                            'norm1_id': source_norm.id,
                            'norm2_id': target_norm.id,
                            'confidence': conflict_score,
                            'evidence': f"Противоречие в ссылке: {rel['evidence']}",
                            'description': 'Противоречие между ссылающейся и ссылочной нормой'
                        }
                        conflicts.append(conflict)
        
        return conflicts
    
    def _detect_hierarchical_conflicts(self, norms: List[LegalNorm], 
                                     relationships: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        conflicts = []
        
        # Поиск связей с иерархическим типом
        hierarchical_rels = [rel for rel in relationships 
                           if rel['relationship_type'] == 'hierarchical']
        
        norm_dict = {norm.id: norm for norm in norms}
        
        for rel in hierarchical_rels:
            source_norm = norm_dict.get(rel['source_norm_id'])
            target_norm = norm_dict.get(rel['target_norm_id'])
            
            if source_norm and target_norm:
                # Проверка на нарушение иерархии
                if self._violates_hierarchy(source_norm, target_norm, rel):
                    conflict = {
                        'type': 'hierarchical_conflict',
                        'norm1_id': source_norm.id,
                        'norm2_id': target_norm.id,
                        'confidence': 0.9,
                        'evidence': f"Нарушение иерархии: {rel['evidence']}",
                        'description': 'Нарушение юридической иерархии'
                    }
                    conflicts.append(conflict)
        
        return conflicts
    
    def _detect_temporal_conflicts(self, norms: List[LegalNorm]) -> List[Dict[str, Any]]:
        conflicts = []
        
        for i, norm1 in enumerate(norms):
            for norm2 in norms[i+1:]:
                temporal_conflict = self._check_temporal_conflict(norm1, norm2)
                
                if temporal_conflict:
                    conflicts.append(temporal_conflict)
        
        return conflicts
    
    def _calculate_conflict_score(self, norm1: LegalNorm, norm2: LegalNorm) -> float:
        score = 0.0
        
        # Проверка на противоречивые ключевые слова
        for conflict_type, keywords in self.conflict_keywords.items():
            for keyword in keywords:
                if keyword in norm1.content.lower() and keyword in norm2.content.lower():
                    score += 0.3
        
        # Проверка на противоположные модальные глаголы
        modal_opposites = [
            ('должен', 'не должен'),
            ('обязан', 'не обязан'),
            ('может', 'не может'),
            ('разрешается', 'запрещается'),
            ('допускается', 'не допускается')
        ]
        
        for positive, negative in modal_opposites:
            if positive in norm1.content.lower() and negative in norm2.content.lower():
                score += 0.4
            elif negative in norm1.content.lower() and positive in norm2.content.lower():
                score += 0.4
        
        # Нормализация оценки
        return min(score, 1.0)
    
    def _calculate_semantic_conflict_score(self, norm1: LegalNorm, norm2: LegalNorm) -> float:
        # Извлечение ключевых слов
        doc1 = self.nlp(norm1.content)
        doc2 = self.nlp(norm2.content)
        
        keywords1 = {token.lemma_.lower() for token in doc1 
                    if not token.is_stop and token.pos_ in ['VERB', 'ADJ']}
        keywords2 = {token.lemma_.lower() for token in doc2 
                    if not token.is_stop and token.pos_ in ['VERB', 'ADJ']}
        
        # Проверка на противоположные понятия
        opposites = [
            ('разрешать', 'запрещать'),
            ('обязывать', 'освобождать'),
            ('включать', 'исключать'),
            ('повышать', 'понижать'),
            ('увеличивать', 'уменьшать')
        ]
        
        conflict_score = 0.0
        for positive, negative in opposites:
            if (positive in keywords1 and negative in keywords2) or \
               (negative in keywords1 and positive in keywords2):
                conflict_score += 0.5
        
        return min(conflict_score, 1.0)
    
    def _extract_matching_fragment(self, text: str, pattern: str) -> str:
        match = re.search(pattern, text, re.IGNORECASE)
        if match:
            start = max(0, match.start() - 50)
            end = min(len(text), match.end() + 50)
            return text[start:end].strip()
        return ""
    
    def _violates_hierarchy(self, source_norm: LegalNorm, target_norm: LegalNorm, 
                           relationship: Dict[str, Any]) -> bool:
        # Простая эвристика: если норма низкого уровня противоречит норме высокого уровня
        metadata = relationship.get('metadata', {})
        source_level = metadata.get('current_level', 5)
        target_level = metadata.get('hierarchy_level', 5)
        
        # Если источник ссылается на документ более высокого уровня и противоречит ему
        if target_level < source_level:
            conflict_score = self._calculate_conflict_score(source_norm, target_norm)
            return conflict_score > 0.5
        
        return False
    
    def _check_temporal_conflict(self, norm1: LegalNorm, norm2: LegalNorm) -> Dict[str, Any]:
        # Извлечение временных маркеров
        time_patterns = [
            (r'незамедлительно|немедленно', 'immediate'),
            (r'в течение (\d+) (дня|дней|месяца|месяцев|года|лет)', 'delayed'),
            (r'постоянно|бессрочно', 'permanent'),
            (r'временно|на срок', 'temporary')
        ]
        
        norm1_time = None
        norm2_time = None
        
        for pattern, time_type in time_patterns:
            if re.search(pattern, norm1.content, re.IGNORECASE):
                norm1_time = time_type
            if re.search(pattern, norm2.content, re.IGNORECASE):
                norm2_time = time_type
        
        # Проверка на противоречивые временные требования
        conflicting_pairs = [
            ('immediate', 'delayed'),
            ('permanent', 'temporary')
        ]
        
        if norm1_time and norm2_time:
            for time1, time2 in conflicting_pairs:
                if ((norm1_time == time1 and norm2_time == time2) or 
                    (norm1_time == time2 and norm2_time == time1)):
                    
                    return {
                        'type': 'temporal_conflict',
                        'norm1_id': norm1.id,
                        'norm2_id': norm2.id,
                        'confidence': 0.7,
                        'evidence': f'Временное противоречие: {time1} vs {time2}',
                        'description': 'Противоречие во временных требованиях'
                    }
        
        return None
    
    def _find_cross_document_conflicts(self, norms1: List[LegalNorm], 
                                     norms2: List[LegalNorm]) -> List[Dict[str, Any]]:
        all_norms = norms1 + norms2
        relationships = []  # Для междокументного анализа связи не учитываем
        
        return self.detect_conflicts(all_norms, relationships)
    
    def _remove_duplicate_conflicts(self, conflicts: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        unique_conflicts = []
        seen_pairs = set()
        
        for conflict in conflicts:
            pair = tuple(sorted([conflict['norm1_id'], conflict['norm2_id']]))
            
            if pair not in seen_pairs:
                seen_pairs.add(pair)
                unique_conflicts.append(conflict)
        
        return unique_conflicts
