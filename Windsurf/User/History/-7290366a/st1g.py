from pydantic import BaseModel, Field
from typing import List, Dict, Optional, Any
from datetime import datetime

class LegalDocument(BaseModel):
    id: str
    title: str
    content: str
    document_type: str = Field(..., description="Type of legal document (law, regulation, etc.)")
    version: str = Field(default="1.0", description="Document version")
    adoption_date: Optional[datetime] = None
    source: str = Field(..., description="Source database or URL")

class NormAnalysis(BaseModel):
    norm_id: str
    text: str
    importance_score: float = Field(..., ge=0, le=1, description="Importance score 0-1")
    category: str = Field(..., description="Category of the norm")
    keywords: List[str] = []

class DocumentConnection(BaseModel):
    source_doc_id: str
    target_doc_id: str
    connection_type: str = Field(..., description="Type of connection: reference, contradiction, duplication")
    strength: float = Field(..., ge=0, le=1, description="Connection strength 0-1")
    explanation: str

class Issue(BaseModel):
    issue_id: str
    type: str = Field(..., description="Type: contradiction, duplication, outdated")
    severity: str = Field(..., description="Severity: low, medium, high, critical")
    description: str
    affected_norms: List[str] = []
    explanation: str
    recommendation: str

class EntropyAnalysis(BaseModel):
    document_id: str
    entropy_score: float = Field(..., ge=0, description="Overall entropy score")
    norm_analysis: List[NormAnalysis] = []
    connections: List[DocumentConnection] = []
    issues: List[Issue] = []
    summary: str
    visualization_data: Optional[Dict[str, Any]] = None

class InferenceRequest(BaseModel):
    document: LegalDocument
    compare_with_versions: Optional[List[str]] = []
    analyze_connections: bool = True
    deep_analysis: bool = False

class InferenceResponse(BaseModel):
    success: bool
    analysis: Optional[EntropyAnalysis] = None
    message: str
    processing_time: float
