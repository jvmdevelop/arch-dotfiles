import re
import numpy as np
import pandas as pd
from typing import List, Dict, Tuple, Optional
from collections import defaultdict, Counter
import networkx as nx
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.metrics.pairwise import cosine_similarity
from sklearn.cluster import KMeans
from transformers import AutoTokenizer, AutoModel
from sentence_transformers import SentenceTransformer
import torch

class LegislativeEntropyModel:
    def __init__(self):
        # Using multilingual sentence transformer for Russian support
        self.sentence_model = SentenceTransformer('paraphrase-multilingual-MiniLM-L12-v2')
        
        self.vectorizer = TfidfVectorizer(
            max_features=1000,
            stop_words=['и', 'в', 'во', 'не', 'что', 'он', 'на', 'я', 'с', 'со', 'как', 'а', 'то', 'все', 'она', 'так', 'его', 'но', 'да', 'ты', 'к', 'у', 'же', 'вы', 'за', 'бы', 'по', 'только', 'ее', 'мне', 'было', 'вот', 'от', 'меня', 'еще', 'нет', 'о', 'из', 'ему', 'теперь', 'когда', 'даже', 'ну', 'вдруг', 'ли', 'если', 'уже', 'или', 'ни', 'быть', 'был', 'него', 'до', 'вас', 'нибудь', 'опять', 'уж', 'вам', 'ведь', 'там', 'потом', 'себя', 'ничего', 'ей', 'может', 'они', 'тут', 'где', 'есть', 'надо', 'ней', 'для', 'мы', 'тебя', 'их', 'чем', 'была', 'сам', 'чтоб', 'без', 'будто', 'чего', 'раз', 'тоже', 'себе', 'под', 'будет', 'ж', 'тогда', 'кто', 'этот', 'того', 'потому', 'этого', 'какой', 'совсем', 'ним', 'здесь', 'этом', 'один', 'почти', 'мой', 'тем', 'чтобы', 'нее', 'сейчас', 'были', 'куда', 'зачем', 'всех', 'никогда', 'можно', 'при', 'наконец', 'два', 'об', 'другой', 'хоть', 'после', 'над', 'больше', 'тот', 'через', 'эти', 'нас', 'про', 'всего', 'них', 'какая', 'много', 'разве', 'три', 'эту', 'моя', 'впрочем', 'хорошо', 'свою', 'этой', 'перед', 'иногда', 'лучше', 'чуть', 'том', 'нельзя', 'такой', 'им', 'более', 'всегда', 'конечно', 'всю', 'между'],
            ngram_range=(1, 2),
            min_df=2
        )
        
        # Russian legal keywords
        self.legal_keywords = {
            'contradiction': [
                'противоречит', 'конфликтует', 'несоответствует', 'противоречие', 
                'конфликт', 'нарушает', 'противоречит', 'несогласованность'
            ],
            'duplication': [
                'дублирует', 'повторяет', 'избыточно', 'дублирование', 'повторение',
                'избыточность', 'аналогично', 'идентично', 'сходно'
            ],
            'outdated': [
                'устарел', 'утратил силу', 'не применяется', 'аннулирован',
                'отменен', 'замещен', 'пересмотрен', 'не действует'
            ],
            'reference': [
                'ссылается', 'согласно', 'в соответствии с', 'пункта', 'статьи',
                'на основании', 'со ссылкой на', 'в соответствии', 'основываясь на'
            ],
            # Russian importance indicators
            'importance': [
                'должен', 'обязан', 'необходимо', 'запрещено', 'нарушение',
                'ответственность', 'штраф', 'наказание', 'лишение свободы', 'несет'
            ]
        }
    
    def preprocess_text(self, text: str) -> str:
        text = re.sub(r'\s+', ' ', text)
        text = re.sub(r'[^\w\s\.\,\;\:\!\?]', '', text)
        return text.strip().lower()
    
    def extract_norms(self, content: str) -> List[Dict]:
        # Split into sentences using basic punctuation (works for Russian too)
        sentences = re.split(r'[.!?]+', content)
        sentences = [s.strip() for s in sentences if len(s.strip()) > 10]
        
        norms = []
        
        for i, sentence in enumerate(sentences):
            # Use multilingual sentence transformer for embedding-based importance
            embedding = self.sentence_model.encode([sentence])[0]
            
            # Calculate importance based on embedding magnitude and Russian legal keywords
            importance = self._calculate_importance_with_embedding(sentence, embedding)
            category = self._categorize_norm_with_bert(sentence)
            keywords = self._extract_keywords_with_transformers(sentence)
            
            norms.append({
                'norm_id': f'norm_{i}',
                'text': sentence,
                'importance_score': importance,
                'category': category,
                'keywords': keywords,
                'embedding': embedding
            })
        
        return norms
    
    def _calculate_importance_with_embedding(self, sentence: str, embedding: np.ndarray) -> float:
        # Russian legal importance indicators
        importance_indicators = self.legal_keywords['importance']
        
        words = sentence.lower().split()
        indicator_count = sum(1 for word in words if any(indicator in word for indicator in importance_indicators))
        
        # Base score from embedding magnitude (normalized)
        embedding_score = np.linalg.norm(embedding) / np.linalg.norm(embedding)
        
        # Legal keyword score
        keyword_score = indicator_count * 0.15
        
        # Length factor (longer sentences often more important)
        length_score = min(0.3, len(sentence.split()) / 50)
        
        total_score = embedding_score * 0.4 + keyword_score + length_score
        
        return min(1.0, total_score)
    
    def _categorize_norm_with_bert(self, sentence: str) -> str:
        # Russian legal categories
        categories = {
            'процессуальный': ['процедура', 'процесс', 'порядок', 'метод', 'алгоритм'],
            'существенный': ['право', 'обязанность', 'долг', 'запрет', 'позволение'],
            'штрафной': ['штраф', 'санкция', 'наказание', 'ответственность', 'взыскание'],
            'административный': ['администрация', 'управление', 'надзор', 'контроль'],
            'технический': ['технический', 'стандарт', 'спецификация', 'требование'],
            'процедурный': ['процедура', 'регламент', 'инструкция', 'порядок']
        }
        
        # Keyword-based categorization for Russian
        words = sentence.lower().split()
        category_scores = {}
        
        for category, keywords in categories.items():
            score = sum(1 for word in words if any(keyword in word for keyword in keywords))
            category_scores[category] = score
        
        if not category_scores or max(category_scores.values()) == 0:
            return 'общий'
        
        return max(category_scores, key=category_scores.get)
    
    def _extract_keywords_with_transformers(self, sentence: str) -> List[str]:
        # Use sentence transformer embeddings to find important words (Russian)
        words = sentence.lower().split()
        
        # Russian stop words are already in TF-IDF, but add some additional ones
        additional_stop_words = {'этот', 'тот', 'который', 'которая', 'которое', 'которые', 'такой', 'такая', 'такое', 'такие', 'весь', 'вся', 'всё', 'все', 'каждый', 'каждая', 'каждое', 'каждые', 'любой', 'любая', 'любое', 'любые', 'другой', 'другая', 'другое', 'другие', 'сам', 'сама', 'само', 'сами', 'свой', 'своя', 'своё', 'свои'}
        
        # Combine with existing stop words from vectorizer
        all_stop_words = set(self.vectorizer.stop_words_) if hasattr(self.vectorizer, 'stop_words_') else set()
        all_stop_words.update(additional_stop_words)
        
        keywords = [word for word in words if word.isalpha() and word not in all_stop_words and len(word) > 3]
        
        return list(set(keywords))[:5]
    
    def find_connections(self, doc1_norms: List[Dict], doc2_norms: List[Dict] = None) -> List[Dict]:
        connections = []
        
        if doc2_norms:
            connections.extend(self._compare_documents_with_transformers(doc1_norms, doc2_norms))
        
        connections.extend(self._find_internal_connections(doc1_norms))
        
        return connections
    
    def _compare_documents_with_transformers(self, doc1_norms: List[Dict], doc2_norms: List[Dict]) -> List[Dict]:
        connections = []
        
        # Extract embeddings from norms
        embeddings1 = np.array([norm['embedding'] for norm in doc1_norms])
        embeddings2 = np.array([norm['embedding'] for norm in doc2_norms])
        
        # Calculate cosine similarity between all pairs
        similarities = cosine_similarity(embeddings1, embeddings2)
        
        for i, norm1 in enumerate(doc1_norms):
            for j, norm2 in enumerate(doc2_norms):
                similarity = similarities[i][j]
                
                if similarity > 0.6:  # Lower threshold for semantic similarity
                    connection_type = self._determine_connection_type_with_semantics(norm1['text'], norm2['text'], similarity)
                    connections.append({
                        'source_norm_id': norm1['norm_id'],
                        'target_norm_id': norm2['norm_id'],
                        'connection_type': connection_type,
                        'strength': similarity,
                        'explanation': f"Semantic similarity ({similarity:.2f}) suggests {connection_type}"
                    })
        
        return connections
    
    def _find_internal_connections(self, norms: List[Dict]) -> List[Dict]:
        connections = []
        
        for i, norm1 in enumerate(norms):
            for j, norm2 in enumerate(norms[i+1:], i+1):
                text1 = norm1['text'].lower()
                text2 = norm2['text'].lower()
                
                # Check for references using Russian keywords and semantic similarity
                for keyword_list in self.legal_keywords['reference']:
                    if keyword_list in text1 and any(word in text2 for word in ['пункт', 'статья', 'часть', 'раздел']):
                        connections.append({
                            'source_norm_id': norm1['norm_id'],
                            'target_norm_id': norm2['norm_id'],
                            'connection_type': 'reference',
                            'strength': 0.8,
                            'explanation': "Обнаружена внутренняя ссылка"
                        })
                        break
        
        return connections
    
    def _determine_connection_type_with_semantics(self, text1: str, text2: str, similarity: float) -> str:
        text1_lower = text1.lower()
        text2_lower = text2.lower()
        
        # Check for Russian contradiction keywords
        for keyword_list in self.legal_keywords['contradiction']:
            if keyword_list in text1_lower or keyword_list in text2_lower:
                return 'contradiction'
        
        # Check for Russian duplication keywords
        for keyword_list in self.legal_keywords['duplication']:
            if keyword_list in text1_lower or keyword_list in text2_lower:
                return 'duplication'
        
        # Use similarity threshold for determining type
        if similarity > 0.85:
            return 'duplication'
        elif similarity > 0.75:
            return 'contradiction'
        else:
            return 'similarity'
    
    def identify_issues(self, norms: List[Dict], connections: List[Dict]) -> List[Dict]:
        issues = []
        
        # Find contradictions
        contradictions = [c for c in connections if c['connection_type'] == 'contradiction']
        for conn in contradictions:
            issues.append({
                'issue_id': f'contradiction_{len(issues)}',
                'type': 'contradiction',
                'severity': 'high',
                'description': f"Contradiction between {conn['source_norm_id']} and {conn['target_norm_id']}",
                'affected_norms': [conn['source_norm_id'], conn['target_norm_id']],
                'explanation': conn['explanation'],
                'recommendation': 'Review and reconcile contradictory norms'
            })
        
        # Find duplications
        duplications = [c for c in connections if c['connection_type'] == 'duplication']
        for conn in duplications:
            issues.append({
                'issue_id': f'duplication_{len(issues)}',
                'type': 'duplication',
                'severity': 'medium',
                'description': f"Duplicate content between {conn['source_norm_id']} and {conn['target_norm_id']}",
                'affected_norms': [conn['source_norm_id'], conn['target_norm_id']],
                'explanation': conn['explanation'],
                'recommendation': 'Consider consolidating duplicate norms'
            })
        
        # Check for low importance norms
        low_importance = [n for n in norms if n['importance_score'] < 0.2]
        if len(low_importance) > len(norms) * 0.3:
            issues.append({
                'issue_id': f'low_importance_{len(issues)}',
                'type': 'quality',
                'severity': 'low',
                'description': 'High proportion of low-importance norms',
                'affected_norms': [n['norm_id'] for n in low_importance],
                'explanation': 'Many norms appear to have low legal significance',
                'recommendation': 'Review document structure and normative content'
            })
        
        return issues
    
    def calculate_entropy_score(self, norms: List[Dict], connections: List[Dict], issues: List[Dict]) -> float:
        base_entropy = len(norms) * 0.1
        
        connection_entropy = len(connections) * 0.05
        contradiction_penalty = sum(1 for c in connections if c['connection_type'] == 'contradiction') * 0.3
        duplication_penalty = sum(1 for c in connections if c['connection_type'] == 'duplication') * 0.2
        
        issue_severity_weights = {'low': 0.1, 'medium': 0.2, 'high': 0.3, 'critical': 0.5}
        issue_penalty = sum(issue_severity_weights.get(issue['severity'], 0.1) for issue in issues)
        
        total_entropy = base_entropy + connection_entropy + contradiction_penalty + duplication_penalty + issue_penalty
        
        return round(total_entropy, 2)
    
    def generate_visualization_data(self, norms: List[Dict], connections: List[Dict]) -> Dict:
        G = nx.Graph()
        
        for norm in norms:
            G.add_node(norm['norm_id'], 
                      text=norm['text'][:100] + '...',
                      importance=norm['importance_score'],
                      category=norm['category'])
        
        for conn in connections:
            G.add_edge(conn['source_norm_id'], 
                      conn['target_norm_id'],
                      weight=conn['strength'],
                      type=conn['connection_type'])
        
        pos = nx.spring_layout(G)
        
        nodes = []
        for node_id, node_data in G.nodes(data=True):
            x, y = pos[node_id]
            nodes.append({
                'id': node_id,
                'x': float(x),
                'y': float(y),
                'importance': node_data['importance'],
                'category': node_data['category'],
                'text': node_data['text']
            })
        
        edges = []
        for source, target, edge_data in G.edges(data=True):
            edges.append({
                'source': source,
                'target': target,
                'strength': edge_data['weight'],
                'type': edge_data['type']
            })
        
        return {
            'nodes': nodes,
            'edges': edges,
            'stats': {
                'total_nodes': len(nodes),
                'total_edges': len(edges),
                'density': nx.density(G),
                'connected_components': nx.number_connected_components(G)
            }
        }
    
    def analyze_document(self, document_content: str, document_id: str) -> Dict:
        norms = self.extract_norms(document_content)
        connections = self.find_connections(norms)
        issues = self.identify_issues(norms, connections)
        entropy_score = self.calculate_entropy_score(norms, connections, issues)
        visualization_data = self.generate_visualization_data(norms, connections)
        
        summary = self._generate_summary(norms, connections, issues, entropy_score)
        
        return {
            'document_id': document_id,
            'entropy_score': entropy_score,
            'norm_analysis': norms,
            'connections': connections,
            'issues': issues,
            'summary': summary,
            'visualization_data': visualization_data
        }
    
    def _generate_summary(self, norms: List[Dict], connections: List[Dict], issues: List[Dict], entropy_score: float) -> str:
        summary_parts = []
        
        summary_parts.append(f"Document contains {len(norms)} identified norms with an entropy score of {entropy_score}.")
        
        if issues:
            severity_counts = Counter(issue['severity'] for issue in issues)
            summary_parts.append(f"Found {len(issues)} issues: {dict(severity_counts)}.")
        
        connection_types = Counter(conn['connection_type'] for conn in connections)
        if connection_types:
            summary_parts.append(f"Identified {dict(connection_types)} connections between norms.")
        
        if entropy_score > 5:
            summary_parts.append("High entropy suggests significant complexity or potential issues.")
        elif entropy_score > 2:
            summary_parts.append("Moderate entropy indicates some complexity that may require attention.")
        else:
            summary_parts.append("Low entropy suggests a well-structured document.")
        
        return " ".join(summary_parts)
