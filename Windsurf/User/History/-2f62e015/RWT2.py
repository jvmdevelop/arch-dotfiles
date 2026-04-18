

import re
import spacy
from typing import List, Dict, Any
from datetime import datetime
import logging

logger = logging.getLogger(__name__)


class TextProcessor:
    """
    Класс для предобработки и очистки текстов юридических документов
    """
    
    def __init__(self, language: str = "ru"):
        """
        Инициализация процессора текста
        
        Args:
            language: Язык обработки (ru - русский, kk - казахский)
        """
        self.language = language
        
        # Загрузка spaCy модели
        try:
            if language == "ru":
                self.nlp = spacy.load("ru_core_news_lg")
            elif language == "kk":
                # Для казахского языка используем многоязычную модель
                self.nlp = spacy.load("xx_ent_wiki_sm")
            else:
                self.nlp = spacy.load("xx_ent_wiki_sm")
        except OSError:
            logger.warning(f"Модель для языка {language} не найдена, используется базовая модель")
            self.nlp = spacy.load("xx_ent_wiki_sm")
        
        # Паттерны для очистки текста
        self.cleanup_patterns = [
            r'\s+',  # Множественные пробелы
            r'\n+',  # Множественные переносы строк
            r'[^\w\s\.\,\;\:\!\?\-\(\)\[\]\"\'\/\#\@\$\%\&\*\+\=\~\`]',  # Специальные символы
        ]
        
        # Юридические термины и сокращения
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
        """
        Основная функция предобработки текста
        
        Args:
            text: Исходный текст документа
            
        Returns:
            str: Очищенный и обработанный текст
        """
        # 1. Базовая очистка
        cleaned_text = self._basic_cleanup(text)
        
        # 2. Расширение сокращений
        expanded_text = self._expand_abbreviations(cleaned_text)
        
        # 3. Нормализация с помощью spaCy
        normalized_text = self._normalize_with_spacy(expanded_text)
        
        # 4. Извлечение и сохранение метаданных
        self._extract_metadata(normalized_text)
        
        return normalized_text
    
    def _basic_cleanup(self, text: str) -> str:
        """Базовая очистка текста"""
        # Удаление лишних пробелов и переносов строк
        text = re.sub(r'\s+', ' ', text)
        text = re.sub(r'\n+', '\n', text)
        
        # Удаление номеров страниц
        text = re.sub(r'Страница \d+ из \d+', '', text)
        
        # Удаление колонтитулов
        text = re.sub(r'©.*?\d{4}', '', text)
        
        # Нормализация кавычек
        text = text.replace('"', '"').replace('"', '"')
        text = text.replace(''', "'").replace(''', "'")
        
        return text.strip()
    
    def _expand_abbreviations(self, text: str) -> str:
        """Расширение юридических сокращений"""
        for abbr, full_form in self.legal_abbreviations.items():
            # Используем регулярные выражения для точного совпадения
            pattern = r'\b' + re.escape(abbr) + r'\b'
            text = re.sub(pattern, full_form, text, flags=re.IGNORECASE)
        
        return text
    
    def _normalize_with_spacy(self, text: str) -> str:
        """Нормализация текста с помощью spaCy"""
        doc = self.nlp(text)
        
        # Лемматизация и удаление стоп-слов
        normalized_tokens = []
        for token in doc:
            # Сохраняем важную пунктуацию
            if token.is_punct and token.text in '.;:!?':
                normalized_tokens.append(token.text)
            # Пропускаем стоп-слова и пробелы
            elif not token.is_stop and not token.is_space:
                normalized_tokens.append(token.lemma_.lower())
        
        return ' '.join(normalized_tokens)
    
    def _extract_metadata(self, text: str) -> Dict[str, Any]:
        """Извлечение метаданных из текста"""
        metadata = {}
        
        # Извлечение дат
        date_pattern = r'\d{1,2}\.\d{1,2}\.\d{4}|\d{4}\-\d{2}\-\d{2}'
        dates = re.findall(date_pattern, text)
        metadata['dates'] = dates
        
        # Извлечение номеров документов
        doc_number_pattern = r'№\s*\d+[-\w]*'
        doc_numbers = re.findall(doc_number_pattern, text)
        metadata['document_numbers'] = doc_numbers
        
        # Извлечение ссылок на документы
        reference_pattern = r'(?:согласно|в соответствии с|в соответствии|согласно|пунктом|статьей|частью)\s+[^.]*'
        references = re.findall(reference_pattern, text, re.IGNORECASE)
        metadata['references'] = references
        
        return metadata
    
    def extract_sentences(self, text: str) -> List[str]:
        """
        Разделение текста на предложения
        
        Args:
            text: Текст для разделения
            
        Returns:
            List[str]: Список предложений
        """
        doc = self.nlp(text)
        return [sent.text.strip() for sent in doc.sents]
    
    def extract_entities(self, text: str) -> Dict[str, List[str]]:
        """
        Извлечение именованных сущностей из текста
        
        Args:
            text: Текст для анализа
            
        Returns:
            Dict[str, List[str]]: Словарь сущностей по типам
        """
        doc = self.nlp(text)
        
        entities = {
            'PERSON': [],
            'ORG': [],
            'GPE': [],  # Geopolitical Entity
            'LAW': [],
            'DATE': [],
            'MONEY': [],
            'CARDINAL': []
        }
        
        for ent in doc.ents:
            if ent.label_ in entities:
                entities[ent.label_].append(ent.text)
        
        # Удаление дубликатов
        for key in entities:
            entities[key] = list(set(entities[key]))
        
        return entities
    
    def detect_language(self, text: str) -> str:
        """
        Определение языка текста
        
        Args:
            text: Текст для анализа
            
        Returns:
            str: Код языка (ru, kk, en)
        """
        # Простая эвристика на основе символов
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
        """
        Пакетная обработка документов
        
        Args:
            texts: Список текстов для обработки
            
        Returns:
            List[str]: Список обработанных текстов
        """
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
