

from typing import List, Dict, Any, Optional
from dataclasses import dataclass
from datetime import datetime
import logging

from ..nlp.norm_extractor import NormExtractor
from ..nlp.text_processor import TextProcessor
from ..analysis.conflict_detector import ConflictDetector
from ..analysis.duplicate_detector import DuplicateDetector
from ..analysis.relationship_analyzer import RelationshipAnalyzer
from ..graph.legal_graph import LegalGraph
from ..explainability.explainer import LegalExplainer

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
    explanations: List[Dict[str, Any]]
    graph_data: Dict[str, Any]


class LegalAnalyzer:
    
    def __init__(self, config: Optional[Dict[str, Any]] = None):
        self.config = config or {}
        self.text_processor = TextProcessor()
        self.norm_extractor = NormExtractor()
        self.conflict_detector = ConflictDetector()
        self.duplicate_detector = DuplicateDetector()
        self.relationship_analyzer = RelationshipAnalyzer()
        self.legal_graph = LegalGraph()
        self.explainer = LegalExplainer()
        
        logger.info("LegalAnalyzer инициализирован")
    
    def analyze_document(self, document: LegalDocument) -> AnalysisResult:
        """
        Полный анализ документа
        
        Args:
            document: Юридический документ для анализа
            
        Returns:
            AnalysisResult: Результаты анализа
        """
        logger.info(f"Начало анализа документа: {document.title}")
        
        # 1. Предобработка текста
        processed_text = self.text_processor.process(document.content)
        
        # 2. Извлечение юридических норм
        norms = self.norm_extractor.extract_norms(
            document.id, processed_text, document.document_type
        )
        
        # 3. Анализ связей между документами
        relationships = self.relationship_analyzer.analyze_relationships(
            document, norms
        )
        
        # 4. Детекция противоречий
        conflicts = self.conflict_detector.detect_conflicts(norms, relationships)
        
        # 5. Детекция дублирования
        duplicates = self.duplicate_detector.detect_duplicates(norms)
        
        # 6. Построение графа связей
        graph_data = self.legal_graph.build_graph(document, norms, relationships)
        
        # 7. Генерация объяснений
        explanations = self.explainer.generate_explanations(
            conflicts, duplicates, relationships
        )
        
        result = AnalysisResult(
            document=document,
            norms=norms,
            conflicts=conflicts,
            duplicates=duplicates,
            relationships=relationships,
            explanations=explanations,
            graph_data=graph_data
        )
        
        logger.info(f"Анализ документа завершен: {len(norms)} норм, "
                   f"{len(conflicts)} противоречий, {len(duplicates)} дубликатов")
        
        return result
    
    def analyze_multiple_documents(self, documents: List[LegalDocument]) -> List[AnalysisResult]:
        """
        Анализ множества документов с учетом взаимосвязей
        
        Args:
            documents: Список документов для анализа
            
        Returns:
            List[AnalysisResult]: Результаты анализа всех документов
        """
        logger.info(f"Начало анализа {len(documents)} документов")
        
        results = []
        
        # Сначала анализируем каждый документ отдельно
        for document in documents:
            result = self.analyze_document(document)
            results.append(result)
        
        # Затем проводим междокументный анализ
        all_norms = []
        for result in results:
            all_norms.extend(result.norms)
        
        # Междокументная детекция противоречий
        cross_document_conflicts = self.conflict_detector.detect_cross_document_conflicts(
            all_norms, results
        )
        
        # Междокументная детекция дублирования
        cross_document_duplicates = self.duplicate_detector.detect_cross_document_duplicates(
            all_norms, results
        )
        
        # Обновляем результаты
        for i, result in enumerate(results):
            result.conflicts.extend(cross_document_conflicts.get(i, []))
            result.duplicates.extend(cross_document_duplicates.get(i, []))
        
        logger.info("Анализ множества документов завершен")
        return results
    
    def get_document_statistics(self, results: List[AnalysisResult]) -> Dict[str, Any]:
        """
        Получение статистики по проанализированным документам
        
        Args:
            results: Результаты анализа
            
        Returns:
            Dict[str, Any]: Статистика
        """
        total_norms = sum(len(r.norms) for r in results)
        total_conflicts = sum(len(r.conflicts) for r in results)
        total_duplicates = sum(len(r.duplicates) for r in results)
        total_relationships = sum(len(r.relationships) for r in results)
        
        document_types = {}
        for result in results:
            doc_type = result.document.document_type
            document_types[doc_type] = document_types.get(doc_type, 0) + 1
        
        return {
            "total_documents": len(results),
            "total_norms": total_norms,
            "total_conflicts": total_conflicts,
            "total_duplicates": total_duplicates,
            "total_relationships": total_relationships,
            "document_types": document_types,
            "average_norms_per_document": total_norms / len(results) if results else 0,
            "conflict_rate": total_conflicts / total_norms if total_norms > 0 else 0,
            "duplicate_rate": total_duplicates / total_norms if total_norms > 0 else 0
        }
