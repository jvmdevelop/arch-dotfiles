import re
from typing import List, Dict, Any, Tuple
import spacy
import logging
from ..core.legal_analyzer import LegalNorm

logger = logging.getLogger(__name__)


class NormExtractor:
    
    def __init__(self):
        try:
            self.nlp = spacy.load("ru_core_news_lg")
        except OSError:
            self.nlp = spacy.load("xx_ent_wiki_sm")
            logger.warning("Используется многоязычная модель вместо русской")
        
        self.patterns = {
            'article': r'статья\s+(\d+[.-]?\d*)\s*\.?\s*([^.\n]*?)(?=\n|статья|пункт|раздел|$)',
            'paragraph': r'пункт\s+(\d+[.-]?\d*)\s*\.?\s*([^.\n]*?)(?=\n|пункт|подпункт|статья|$)',
            'subparagraph': r'подпункт\s+(\d+[.-]?\d*)\s*\.?\s*([^.\n]*?)(?=\n|подпункт|пункт|$)',
            'part': r'часть\s+(\d+[.-]?\d*)\s*\.?\s*([^.\n]*?)(?=\n|часть|пункт|$)',
            'section': r'раздел\s+(\d+[.-]?\d*)\s*\.?\s*([^.\n]*?)(?=\n|раздел|глава|$)',
            'chapter': r'глава\s+(\d+[.-]?\d*)\s*\.?\s*([^.\n]*?)(?=\n|глава|раздел|$)'
        }
        
        self.norm_keywords = {
            'prohibition': ['запрещается', 'не допускается', 'не разрешается', 'запрет'],
            'obligation': ['обязан', 'должен', 'необходимо', 'требуется', 'обязывает'],
            'permission': ['вправе', 'может', 'разрешается', 'допускается'],
            'definition': ['понимается', 'определяется как', 'это', 'подразумевает'],
            'procedure': ['порядок', 'процедура', 'механизм', 'алгоритм'],
            'responsibility': ['несет ответственность', 'влечет ответственность', 'наказание'],
            'rights': ['имеет право', 'право на', 'вправе требовать']
        }
        
        self.legal_terms = [
            'закон', 'кодекс', 'указ', 'постановление', 'приказ', 'распоряжение',
            'договор', 'соглашение', 'конвенция', 'протокол', 'регламент',
            'инструкция', 'правила', 'положение', 'устав', 'статут'
        ]
    
    def extract_norms(self, document_id: str, text: str, document_type: str) -> List[LegalNorm]:
        logger.info(f"Извлечение норм из документа {document_id}")
        
        norms = []
        
        pattern_norms = self._extract_by_patterns(document_id, text)
        norms.extend(pattern_norms)
        
        nlp_norms = self._extract_by_nlp(document_id, text)
        norms.extend(nlp_norms)
        
        complex_norms = self._extract_complex_norms(document_id, text, norms)
        norms.extend(complex_norms)
        
        unique_norms = self._remove_duplicates(norms)
        
        logger.info(f"Извлечено {len(unique_norms)} уникальных норм")
        return unique_norms
    
    def _extract_by_patterns(self, document_id: str, text: str) -> List[LegalNorm]:
        norms = []
        
        for norm_type, pattern in self.patterns.items():
            matches = re.finditer(pattern, text, re.IGNORECASE | re.MULTILINE)
            
            for match in matches:
                number = match.group(1).strip()
                content = match.group(2).strip()
                
                if len(content) > 10:
                    keywords, entities = self._extract_keywords_and_entities(content)
                    
                    norm = LegalNorm(
                        id=f"{document_id}_{norm_type}_{number}",
                        document_id=document_id,
                        content=content,
                        norm_type=norm_type,
                        number=number,
                        keywords=keywords,
                        entities=entities
                    )
                    norms.append(norm)
        
        return norms
    
    def _extract_by_nlp(self, document_id: str, text: str) -> List[LegalNorm]:
        norms = []
        
        doc = self.nlp(text)
        
        sentences = list(doc.sents)
        
        for i, sent in enumerate(sentences):
            if self._is_legal_norm(sent.text):
                norm_type = self._classify_norm_type(sent.text)
                
                keywords, entities = self._extract_keywords_and_entities(sent.text)
                
                norm = LegalNorm(
                    id=f"{document_id}_sent_{i}",
                    document_id=document_id,
                    content=sent.text.strip(),
                    norm_type=norm_type,
                    number=str(i + 1),
                    keywords=keywords,
                    entities=entities
                )
                norms.append(norm)
        
        return norms
    
    def _extract_complex_norms(self, document_id: str, text: str, existing_norms: List[LegalNorm]) -> List[LegalNorm]:
        complex_norms = []
        
        doc = self.nlp(text)
        sentences = list(doc.sents)
        
        i = 0
        while i < len(sentences):
            current_sent = sentences[i].text.strip()
            
            if self._is_complex_norm_start(current_sent):
                complex_content = current_sent
                j = i + 1
                
                while j < len(sentences) and self._is_part_of_complex_norm(sentences[j].text, complex_content):
                    complex_content += " " + sentences[j].text.strip()
                    j += 1
                
                if j > i + 1:
                    keywords, entities = self._extract_keywords_and_entities(complex_content)
                    
                    norm = LegalNorm(
                        id=f"{document_id}_complex_{i}",
                        document_id=document_id,
                        content=complex_content,
                        norm_type="complex",
                        number=str(i + 1),
                        keywords=keywords,
                        entities=entities
                    )
                    complex_norms.append(norm)
                
                i = j
            else:
                i += 1
        
        return complex_norms
    
    def _is_legal_norm(self, text: str) -> bool:
        text_lower = text.lower()
        
        # Проверка наличия ключевых слов
        for category, keywords in self.norm_keywords.items():
            for keyword in keywords:
                if keyword in text_lower:
                    return True
        
        # Проверка наличия юридических терминов
        for term in self.legal_terms:
            if term in text_lower:
                return True
        
        # Проверка структуры (наличие модальных глаголов, долженствования и т.д.)
        modal_verbs = ['должен', 'обязан', 'вправе', 'может', 'запрещается', 'необходимо']
        for verb in modal_verbs:
            if verb in text_lower:
                return True
        
        return False
    
    def _classify_norm_type(self, text: str) -> str:
        text_lower = text.lower()
        
        for category, keywords in self.norm_keywords.items():
            for keyword in keywords:
                if keyword in text_lower:
                    return category
        
        return 'general'
    
    def _extract_keywords_and_entities(self, text: str) -> Tuple[List[str], List[str]]:
        doc = self.nlp(text)
        
        keywords = []
        for token in doc:
            if (not token.is_stop and 
                not token.is_punct and 
                not token.is_space and 
                len(token.lemma_) > 2 and
                token.pos_ in ['NOUN', 'VERB', 'ADJ']):
                keywords.append(token.lemma_.lower())
        
        entities = []
        for ent in doc.ents:
            if ent.label_ in ['PERSON', 'ORG', 'GPE', 'LAW']:
                entities.append(ent.text)
        
        keywords = list(set(keywords))
        entities = list(set(entities))
        
        return keywords[:10], entities[:5]
    
    def _is_complex_norm_start(self, text: str) -> bool:
        complex_starters = [
            'в соответствии с', 'согласно', 'на основании', 'порядок',
            'процедура', 'механизм', 'условия', 'требования'
        ]
        
        text_lower = text.lower()
        for starter in complex_starters:
            if text_lower.startswith(starter):
                return True
        
        return False
    
    def _is_part_of_complex_norm(self, text: str, complex_content: str) -> bool:
        if len(text.strip()) < 100:
            return True
        
        doc1 = self.nlp(complex_content)
        doc2 = self.nlp(text)
        
        words1 = [token.lemma_.lower() for token in doc1 if not token.is_stop]
        words2 = [token.lemma_.lower() for token in doc2 if not token.is_stop]
        
        common_words = set(words1) & set(words2)
        
        return len(common_words) > 2
    
    def _remove_duplicates(self, norms: List[LegalNorm]) -> List[LegalNorm]:
        unique_norms = []
        seen_contents = set()
        
        for norm in norms:
            normalized_content = re.sub(r'\s+', ' ', norm.content.lower().strip())
            
            if normalized_content not in seen_contents:
                seen_contents.add(normalized_content)
                unique_norms.append(norm)
        
        return unique_norms
