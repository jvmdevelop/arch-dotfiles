

import re
import spacy
from typing import List, Dict, Any
from datetime import datetime
import logging

logger = logging.getLogger(__name__)


class TextProcessor:

    def __init__(self, language: str = "ru"):
        
        self.language = language
        
        try:
            if language == "ru":
                self.nlp = spacy.load("ru_core_news_lg")
            elif language == "kk":
                self.nlp = spacy.load("xx_ent_wiki_sm")
            else:
                self.nlp = spacy.load("xx_ent_wiki_sm")
        except OSError:
            logger.warning(f"Модель для языка {language} не найдена, используется базовая модель")
            self.nlp = spacy.load("xx_ent_wiki_sm")
        
        self.cleanup_patterns = [
            r'\s+',  
            r'\n+',  
            r'[^\w\s\.\,\;\:\!\?\-\(\)\[\]\"\'\/\#\@\$\%\&\*\+\=\~\`]',  
        ]
        
        self.legal_abbreviations = {
            'ст': 'статья',
            'п': 'пункт',
            'подп': 'подпункт',
            'ч': 'часть',
            'абз': 'абзац',
            'РК': 'Республика Казахстан',
            'НПА': 'нормативно-правовой акт',
            'ГК': 'Гражданский кодекс',
            'УК': 'Уголовный кодекс',
            'КоАП': 'Кодекс об административных правонарушениях',
            'ЗРК': 'Закон Республики Казахстан',
            'Указ': 'Указ Президента',
            'Постановление': 'Постановление Правительства'
        }
    
    def process(self, text: str) -> str:
        
        cleaned_text = self._basic_cleanup(text)
        
        expanded_text = self._expand_abbreviations(cleaned_text)
        
        normalized_text = self._normalize_with_spacy(expanded_text)
        
        self._extract_metadata(normalized_text)
        self._extract_metadata(normalized_text)
        
        return normalized_text
    
    def _basic_cleanup(self, text: str) -> str:
        
        text = re.sub(r'\s+', ' ', text)
        text = re.sub(r'\n+', '\n', text)
        
        text = re.sub(r'Страница \d+ из \d+', '', text)
        
        text = re.sub(r'©.*?\d{4}', '', text)
        
        text = text.replace('"', '"').replace('"', '"')
        text = text.replace(''', "'").replace(''', "'")
        
        return text.strip()
    
    def _expand_abbreviations(self, text: str) -> str:
        for abbr, full_form in self.legal_abbreviations.items():
            pattern = r'\b' + re.escape(abbr) + r'\b'
            text = re.sub(pattern, full_form, text, flags=re.IGNORECASE)
        
        return text
    
    def _normalize_with_spacy(self, text: str) -> str:
        doc = self.nlp(text)
        
        normalized_tokens = []
        for token in doc:
            if token.is_punct and token.text in '.;:!?':
                normalized_tokens.append(token.text)
            elif not token.is_stop and not token.is_space:
                normalized_tokens.append(token.lemma_.lower())
        
        return ' '.join(normalized_tokens)
    
    def _extract_metadata(self, text: str) -> Dict[str, Any]:
        metadata = {}
        
        date_pattern = r'\d{1,2}\.\d{1,2}\.\d{4}|\d{4}\-\d{2}\-\d{2}'
        dates = re.findall(date_pattern, text)
        metadata['dates'] = dates
        
        doc_number_pattern = r'№\s*\d+[-\w]*'
        doc_numbers = re.findall(doc_number_pattern, text)
        metadata['document_numbers'] = doc_numbers
        
        reference_pattern = r'(?:согласно|в соответствии с|в соответствии|согласно|пунктом|статьей|частью)\s+[^.]*'
        reference_pattern = r'(?:согласно|в соответствии с|в соответствии|согласно|пунктом|статьей|частью)\s+[^.]*'
        references = re.findall(reference_pattern, text, re.IGNORECASE)
        metadata['references'] = references
        
        return metadata
    
    def extract_sentences(self, text: str) -> List[str]:
        doc = self.nlp(text)
        return [sent.text.strip() for sent in doc.sents]
    
    def extract_entities(self, text: str) -> Dict[str, List[str]]:

        doc = self.nlp(text)
        
        entities = {
            'PERSON': [],
            'ORG': [],
            'GPE': [], 
            'LAW': [],
            'DATE': [],
            'MONEY': [],
            'CARDINAL': []
        }
        
        for ent in doc.ents:
            if ent.label_ in entities:
                entities[ent.label_].append(ent.text)
        
        for key in entities:
            entities[key] = list(set(entities[key]))
        
        return entities
    
    def detect_language(self, text: str) -> str:
        
        ru_chars = set('абвгдеёжзийклмнопрстуфхцчшщъыьэюя')
        kk_chars = set('әғқңөұһі')
        
        text_lower = text.lower()
        
        ru_count = sum(1 for char in text_lower if char in ru_chars)
        kk_count = sum(1 for char in text_lower if char in kk_chars)
        
        if kk_count > ru_count * 0.1:
            return 'kk'
        elif ru_count > 0:
            return 'ru'
        else:
            return 'en'
    
    def preprocess_document_batch(self, texts: List[str]) -> List[str]:
        
        processed_texts = []
        
        for i, text in enumerate(texts):
            try:
                processed_text = self.process(text)
                processed_texts.append(processed_text)
                logger.info(f"Обработан документ {i+1}/{len(texts)}")
            except Exception as e:
                logger.error(f"Ошибка при обработке документа {i+1}: {str(e)}")
                processed_texts.append(text)  # Возвращаем исходный текст в случае ошибки
        
        return processed_texts
