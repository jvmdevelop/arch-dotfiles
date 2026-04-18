from transformers import AutoTokenizer, AutoModel
from sentence_transformers import SentenceTransformer
import torch
import networkx as nx
import pandas as pd
import numpy as np
from typing import List, Dict, Any, Optional
from dataclasses import dataclass
from datetime import datetime
import re
import json
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

@dataclass
class LegalDocument:
    id: str
    title: str
    content: str
    document_type: str
    adoption_date: Optional[datetime] = None
    version: str = "1.0"
    source: str = ""

@dataclass 
class LegalNorm:
    id: str
    document_id: str
    content: str
    norm_type: str
    number: str
    keywords: List[str]
    entities: List[str]

@dataclass
class AnalysisResult:
    document: LegalDocument
    norms: List[LegalNorm]
    conflicts: List[Dict[str, Any]]
    duplicates: List[Dict[str, Any]]
    relationships: List[Dict[str, Any]]

class LegalAnalyzer:
    def __init__(self):
        logger.info("Инициализация LegalAnalyzer с готовыми моделями")
        
        self.sentence_model = SentenceTransformer('paraphrase-multilingual-MiniLM-L12-v2')
        self.tokenizer = AutoTokenizer.from_pretrained('bert-base-multilingual-cased')
        self.text_model = AutoModel.from_pretrained('bert-base-multilingual-cased')
        
        self.norm_patterns = [
            r'статья\s+(\d+[.-]?\d*)\s*\.?\s*([^.\n]*?)(?=\n|статья|пункт|раздел|$)',
            r'пункт\s+(\d+[.-]?\d*)\s*\.?\s*([^.\n]*?)(?=\n|пункт|подпункт|статья|$)',
            r'раздел\s+(\d+[.-]?\d*)\s*\.?\s*([^.\n]*?)(?=\n|раздел|глава|$)'
        ]
        
        self.legal_keywords = {
            'prohibition': ['запрещается', 'не допускается', 'не разрешается'],
            'obligation': ['обязан', 'должен', 'необходимо', 'требуется'],
            'permission': ['вправе', 'может', 'разрешается', 'допускается'],
            'rights': ['имеет право', 'право на', 'вправе требовать']
        }
        
        self.conflict_patterns = {
            'prohibition_permission': [
                (r'запрещается|не допускается', r'разрешается|допускается|вправе'),
                (r'не разрешается', r'может|разрешается')
            ],
            'obligation_negation': [
                (r'обязан|должен|необходимо', r'не обязан|не должен|не является обязательным'),
                (r'требуется', r'не требуется')
            ]
        }
        
        logger.info("Модели успешно загружены")
    
    def analyze_document(self, document: LegalDocument) -> AnalysisResult:
        logger.info(f"Анализ документа: {document.title}")
        
        norms = self._extract_norms(document)
        
        conflicts = self._detect_conflicts(norms)
        
        duplicates = self._detect_duplicates(norms)
        
        relationships = self._analyze_relationships(norms)
        
        return AnalysisResult(
            document=document,
            norms=norms,
            conflicts=conflicts,
            duplicates=duplicates,
            relationships=relationships
        )
    
    def analyze_multiple_documents(self, documents: List[LegalDocument]) -> List[AnalysisResult]:
        results = []
        
        for document in documents:
            result = self.analyze_document(document)
            results.append(result)
        
        self._cross_document_analysis(results)
        
        return results
    
    def _extract_norms(self, document: LegalDocument) -> List[LegalNorm]:
        norms = []
        
        for pattern in self.norm_patterns:
            matches = re.finditer(pattern, document.content, re.IGNORECASE | re.MULTILINE)
            
            for match in matches:
                number = match.group(1).strip()
                content = match.group(2).strip()
                
                if len(content) > 20:
                    norm_type = self._classify_norm_type(content)
                    keywords = self._extract_keywords(content)
                    entities = self._extract_entities(content)
                    
                    norm = LegalNorm(
                        id=f"{document.id}_norm_{len(norms)}",
                        document_id=document.id,
                        content=content,
                        norm_type=norm_type,
                        number=number,
                        keywords=keywords,
                        entities=entities
                    )
                    norms.append(norm)
        
        return norms
    
    def _classify_norm_type(self, text: str) -> str:
        text_lower = text.lower()
        
        for category, keywords in self.legal_keywords.items():
            for keyword in keywords:
                if keyword in text_lower:
                    return category
        
        return 'general'
    
    def _extract_keywords(self, text: str) -> List[str]:
        try:
            words = re.findall(r'\b\w+\b', text.lower())
            words = [word for word in words if len(word) > 3 and word.isalpha()]
            
            stop_words = {'этот', 'тот', 'который', 'когда', 'где', 'как', 'что', 'для', 'с', 'в', 'на', 'по', 'и', 'а', 'но', 'или', 'если', 'то'}
            words = [word for word in words if word not in stop_words]
            
            word_freq = {}
            for word in words:
                word_freq[word] = word_freq.get(word, 0) + 1
            
            return sorted(word_freq.keys(), key=lambda x: word_freq[x], reverse=True)[:10]
        except:
            return []
    
    def _extract_entities(self, text: str) -> List[str]:
        entities = []
        
        dates = re.findall(r'\d{1,2}\.\d{1,2}\.\d{4}', text)
        entities.extend(dates)
        
        doc_numbers = re.findall(r'№\s*\d+[-\w]*', text)
        entities.extend(doc_numbers)
        
        amounts = re.findall(r'\d+\s*(?:тыс\.?|млн\.?)?\s*(?:тенге|тг|kzt)', text, re.IGNORECASE)
        entities.extend(amounts)
        
        return entities[:5]
    
    def _detect_conflicts(self, norms: List[LegalNorm]) -> List[Dict[str, Any]]:
        conflicts = []
        
        for i, norm1 in enumerate(norms):
            for norm2 in norms[i+1:]:
                # Проверка паттернов противоречий
                for conflict_type, pattern_pairs in self.conflict_patterns.items():
                    for pattern1, pattern2 in pattern_pairs:
                        if (re.search(pattern1, norm1.content, re.IGNORECASE) and 
                            re.search(pattern2, norm2.content, re.IGNORECASE)):
                            
                            conflict = {
                                'type': conflict_type,
                                'norm1_id': norm1.id,
                                'norm2_id': norm2.id,
                                'confidence': 0.8,
                                'description': f'Противоречие типа {conflict_type}',
                                'norm1_fragment': norm1.content[:100] + '...',
                                'norm2_fragment': norm2.content[:100] + '...'
                            }
                            conflicts.append(conflict)
        
        return conflicts
    
    def _detect_duplicates(self, norms: List[LegalNorm]) -> List[Dict[str, Any]]:
        duplicates = []
        
        if len(norms) > 1:
            try:
                embeddings = self.sentence_model.encode([norm.content for norm in norms])
                
                for i, norm1 in enumerate(norms):
                    for j, norm2 in enumerate(norms[i+1:], i+1):
                        similarity = np.dot(embeddings[i], embeddings[j])
                        
                        if similarity > 0.8:  # Порог схожести
                            duplicate = {
                                'type': 'semantic_duplicate',
                                'norm1_id': norm1.id,
                                'norm2_id': norm2.id,
                                'similarity_score': float(similarity),
                                'description': 'Семантическое дублирование',
                                'norm1_fragment': norm1.content[:100] + '...',
                                'norm2_fragment': norm2.content[:100] + '...'
                            }
                            duplicates.append(duplicate)
            except Exception as e:
                logger.warning(f"Ошибка при детекции дубликатов: {e}")
        
        return duplicates
    
    def _analyze_relationships(self, norms: List[LegalNorm]) -> List[Dict[str, Any]]:
        relationships = []
        
        for i, norm1 in enumerate(norms):
            for norm2 in norms[i+1:]:
                # Поиск ссылок на другие документы
                if re.search(r'(?:согласно|в соответствии с|пунктом|статьей|частью)\s+[^.]*', norm1.content, re.IGNORECASE):
                    if any(word in norm2.content.lower() for word in ['закон', 'кодекс', 'указ']):
                        relationship = {
                            'type': 'reference',
                            'source_norm_id': norm1.id,
                            'target_norm_id': norm2.id,
                            'confidence': 0.6,
                            'description': 'Ссылка на другой документ'
                        }
                        relationships.append(relationship)
        
        return relationships
    
    def _cross_document_analysis(self, results: List[AnalysisResult]):
        # Междокументный анализ конфликтов и дубликатов
        all_norms = []
        for result in results:
            all_norms.extend(result.norms)
        
        # Детекция междокументных проблем
        cross_conflicts = self._detect_conflicts(all_norms)
        cross_duplicates = self._detect_duplicates(all_norms)
        
        # Распределение проблем по документам
        for result in results:
            result.conflicts.extend([c for c in cross_conflicts 
                                   if c['norm1_id'] in [n.id for n in result.norms] or 
                                      c['norm2_id'] in [n.id for n in result.norms]])
            result.duplicates.extend([d for d in cross_duplicates 
                                    if d['norm1_id'] in [n.id for n in result.norms] or 
                                       d['norm2_id'] in [n.id for n in result.norms]])
    
    def get_statistics(self, results: List[AnalysisResult]) -> Dict[str, Any]:
        total_docs = len(results)
        total_norms = sum(len(r.norms) for r in results)
        total_conflicts = sum(len(r.conflicts) for r in results)
        total_duplicates = sum(len(r.duplicates) for r in results)
        total_relationships = sum(len(r.relationships) for r in results)
        
        return {
            'total_documents': total_docs,
            'total_norms': total_norms,
            'total_conflicts': total_conflicts,
            'total_duplicates': total_duplicates,
            'total_relationships': total_relationships,
            'conflict_rate': total_conflicts / max(total_norms, 1),
            'duplicate_rate': total_duplicates / max(total_norms, 1),
            'average_norms_per_document': total_norms / max(total_docs, 1)
        }
    
    def visualize_graph(self, results: List[AnalysisResult]) -> Dict[str, Any]:
        G = nx.Graph()
        
        # Добавление узлов документов
        for result in results:
            G.add_node(result.document.id, type='document', title=result.document.title)
            
            # Добавление узлов норм
            for norm in result.norms:
                G.add_node(norm.id, type='norm', content=norm.content[:50] + '...')
                G.add_edge(result.document.id, norm.id, type='contains')
        
        # Добавление связей конфликтов
        for result in results:
            for conflict in result.conflicts:
                if conflict['norm1_id'] in G.nodes and conflict['norm2_id'] in G.nodes:
                    G.add_edge(conflict['norm1_id'], conflict['norm2_id'], 
                              type='conflict', weight=conflict['confidence'])
        
        # Добавление связей дубликатов
        for result in results:
            for duplicate in result.duplicates:
                if duplicate['norm1_id'] in G.nodes and duplicate['norm2_id'] in G.nodes:
                    G.add_edge(duplicate['norm1_id'], duplicate['norm2_id'], 
                              type='duplicate', weight=duplicate['similarity_score'])
        
        return {
            'nodes_count': G.number_of_nodes(),
            'edges_count': G.number_of_edges(),
            'density': nx.density(G),
            'is_connected': nx.is_connected(G),
            'graph_data': nx.node_link_data(G)
        }
