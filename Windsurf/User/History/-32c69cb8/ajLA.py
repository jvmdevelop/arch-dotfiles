import requests
from bs4 import BeautifulSoup
import time
import logging
from typing import List, Dict, Optional, Any
from urllib.parse import urljoin, urlparse
import re
from datetime import datetime
from dataclasses import dataclass
import os

logger = logging.getLogger(__name__)


@dataclass
class ParsedDocument:
    """Represents a parsed legal document"""
    id: str
    title: str
    content: str
    document_type: str
    version: str
    adoption_date: Optional[datetime]
    source_url: str
    download_url: str
    last_modified: Optional[datetime]


class AdiletParser:
    """Parser for Adilet.zan.kz legal documents website"""
    
    def __init__(self, base_url: str = "https://adilet.zan.kz/rus/index/docs"):
        self.base_url = base_url
        self.session = requests.Session()
        self.session.headers.update({
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
        })
        self.download_base_url = "https://adilet.zan.kz/rus/docs"
        
    def get_document_list(self, max_documents: int = 100) -> List[Dict[str, Any]]:
        """Get list of documents from the main page"""
        try:
            response = self.session.get(self.base_url)
            response.raise_for_status()
            
            soup = BeautifulSoup(response.content, 'html.parser')
            documents = []
            
            doc_links = soup.find_all('a', href=re.compile(r'/docs/'))
            
            for i, link in enumerate(doc_links[:max_documents]):
                doc_url = urljoin(self.base_url, link.get('href'))
                doc_id = self._extract_document_id(doc_url)
                title = link.get_text(strip=True)
                
                documents.append({
                    'id': doc_id,
                    'title': title,
                    'url': doc_url,
                    'download_url': f"{self.download_base_url}/{doc_id}_/download"
                })
                
                logger.info(f"Found document: {doc_id} - {title}")
            
            return documents
            
        except Exception as e:
            logger.error(f"Error getting document list: {str(e)}")
            return []
    
    def _extract_document_id(self, url: str) -> str:
        """Extract document ID from URL"""
        match = re.search(r'/docs/([^_/]+)', url)
        return match.group(1) if match else ""
    
    def parse_document(self, doc_url: str, download_url: str) -> Optional[ParsedDocument]:
        """Parse a single document page"""
        try:
            response = self.session.get(doc_url)
            response.raise_for_status()
            
            soup = BeautifulSoup(response.content, 'html.parser')
            
            title = self._extract_title(soup)
            content = self._extract_content(soup)
            doc_type = self._extract_document_type(soup, title)
            adoption_date = self._extract_adoption_date(soup)
            last_modified = self._extract_last_modified(soup)
            doc_id = self._extract_document_id(doc_url)
            
            return ParsedDocument(
                id=doc_id,
                title=title,
                content=content,
                document_type=doc_type,
                version="1.0",
                adoption_date=adoption_date,
                source_url=doc_url,
                download_url=download_url,
                last_modified=last_modified
            )
            
        except Exception as e:
            logger.error(f"Error parsing document {doc_url}: {str(e)}")
            return None
    
    def _extract_title(self, soup: BeautifulSoup) -> str:
        """Extract document title"""
        title_selectors = [
            'h1',
            '.document-title',
            '.title',
            'title'
        ]
        
        for selector in title_selectors:
            title_element = soup.select_one(selector)
            if title_element:
                return title_element.get_text(strip=True)
        
        return "Untitled Document"
    
    def _extract_content(self, soup: BeautifulSoup) -> str:
        """Extract document content"""
        content_selectors = [
            '.document-content',
            '.content',
            '.text',
            'main',
            'article'
        ]
        
        for selector in content_selectors:
            content_element = soup.select_one(selector)
            if content_element:
                content = content_element.get_text(separator='\n', strip=True)
                return self._clean_content(content)
        
        body = soup.find('body')
        if body:
            content = body.get_text(separator='\n', strip=True)
            return self._clean_content(content)
        
        return ""
    
    def _clean_content(self, content: str) -> str:
        """Clean and format document content"""
        content = re.sub(r'\s+', ' ', content)
        
        content = re.sub(r'&nbsp;', ' ', content)
        content = re.sub(r'&amp;', '&', content)
        content = re.sub(r'&lt;', '<', content)
        content = re.sub(r'&gt;', '>', content)
        
        lines = content.split('\n')
        cleaned_lines = []
        
        skip_patterns = [
            r'^\s*$',
            r'^\s*Главная\s*$',
            r'^\s*Навигация\s*$',
            r'^\s*Меню\s*$',
            r'^\s*Footer\s*$',
            r'^\s*©\s*\d{4}',
        ]
        
        for line in lines:
            line = line.strip()
            if not any(re.match(pattern, line, re.IGNORECASE) for pattern in skip_patterns):
                cleaned_lines.append(line)
        
        return '\n'.join(cleaned_lines)
    
    def _extract_document_type(self, soup: BeautifulSoup, title: str) -> str:
        """Extract document type from metadata or title"""
        type_selectors = [
            '.document-type',
            '.type',
            '[data-type]'
        ]
        
        for selector in type_selectors:
            type_element = soup.select_one(selector)
            if type_element:
                doc_type = type_element.get_text(strip=True)
                if doc_type:
                    return doc_type
        
        title_lower = title.lower()
        if 'закон' in title_lower:
            return 'law'
        elif 'указ' in title_lower:
            return 'decree'
        elif 'постановление' in title_lower:
            return 'resolution'
        elif 'кодекс' in title_lower:
            return 'code'
        elif 'конституция' in title_lower:
            return 'constitution'
        else:
            return 'document'
    
    def _extract_adoption_date(self, soup: BeautifulSoup) -> Optional[datetime]:
        """Extract document adoption date"""
        date_selectors = [
            '.adoption-date',
            '.date',
            '[data-date]',
            'time'
        ]
        
        for selector in date_selectors:
            date_element = soup.select_one(selector)
            if date_element:
                date_text = date_element.get_text(strip=True) or date_element.get('datetime', '')
                return self._parse_date(date_text)
        
        return None
    
    def _extract_last_modified(self, soup: BeautifulSoup) -> Optional[datetime]:
        """Extract last modified date"""
        modified_selectors = [
            '.last-modified',
            '.modified',
            '.updated'
        ]
        
        for selector in modified_selectors:
            modified_element = soup.select_one(selector)
            if modified_element:
                date_text = modified_element.get_text(strip=True)
                return self._parse_date(date_text)
        
        return None
    
    def _parse_date(self, date_text: str) -> Optional[datetime]:
        """Parse date from text"""
        date_patterns = [
            r'(\d{2})\.(\d{2})\.(\d{4})',  # DD.MM.YYYY
            r'(\d{4})-(\d{2})-(\d{2})',   # YYYY-MM-DD
            r'(\d{1,2})\s+(января|февраля|марта|апреля|мая|июня|июля|августа|сентября|октября|ноября|декабря)\s+(\d{4})'
        ]
        
        for pattern in date_patterns:
            match = re.search(pattern, date_text)
            if match:
                try:
                    if pattern == date_patterns[0]:  # DD.MM.YYYY
                        day, month, year = match.groups()
                        return datetime(int(year), int(month), int(day))
                    elif pattern == date_patterns[1]:  # YYYY-MM-DD
                        year, month, day = match.groups()
                        return datetime(int(year), int(month), int(day))
                    # Add more date parsing as needed
                except ValueError:
                    continue
        
        return None
    
    def download_pdf(self, download_url: str, save_path: str) -> bool:
        """Download PDF document"""
        try:
            response = self.session.get(download_url)
            response.raise_for_status()
            
            with open(save_path, 'wb') as f:
                f.write(response.content)
            
            logger.info(f"Downloaded PDF: {save_path}")
            return True
            
        except Exception as e:
            logger.error(f"Error downloading PDF from {download_url}: {str(e)}")
            return False
    
    def parse_multiple_documents(self, max_documents: int = 50, delay: float = 1.0) -> List[ParsedDocument]:
        """Parse multiple documents from the website"""
        documents = []
        
        # Get document list
        doc_list = self.get_document_list(max_documents)
        
        for doc_info in doc_list:
            logger.info(f"Parsing document: {doc_info['id']}")
            
            # Parse document
            parsed_doc = self.parse_document(doc_info['url'], doc_info['download_url'])
            if parsed_doc:
                documents.append(parsed_doc)
            
            # Add delay to avoid overwhelming the server
            time.sleep(delay)
        
        logger.info(f"Successfully parsed {len(documents)} documents")
        return documents
