from fastapi import FastAPI, HTTPException, BackgroundTasks
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse
import time
import logging
from typing import List, Dict, Optional
import uvicorn

from models import (
    LegalDocument, InferenceRequest, InferenceResponse,
    EntropyAnalysis, NormAnalysis, DocumentConnection, Issue
)
from legislative_entropy_model import LegislativeEntropyModel

app = FastAPI(
    title="Legislative Entropy Analysis API",
    description="AI system for analyzing legislative documents entropy and identifying issues",
    version="1.0.0"
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

model = LegislativeEntropyModel()

analysis_cache: Dict[str, EntropyAnalysis] = {}

@app.get("/")
async def root():
    return {
        "message": "Legislative Entropy Analysis API",
        "version": "1.0.0",
        "endpoints": {
            "analyze": "/analyze",
            "health": "/health",
            "cache": "/cache",
            "docs": "/docs"
        }
    }

@app.get("/health")
async def health_check():
    return {
        "status": "healthy",
        "model_loaded": True,
        "cache_size": len(analysis_cache)
    }

@app.post("/analyze", response_model=InferenceResponse)
async def analyze_document(request: InferenceRequest):
    start_time = time.time()
    
    try:
        logger.info(f"Starting analysis for document: {request.document.id}")
        
        if request.document.id in analysis_cache:
            logger.info(f"Returning cached analysis for document: {request.document.id}")
            cached_analysis = analysis_cache[request.document.id]
            processing_time = time.time() - start_time
            return InferenceResponse(
                success=True,
                analysis=cached_analysis,
                message="Analysis retrieved from cache",
                processing_time=processing_time
            )
        
        analysis_result = model.analyze_document(
            request.document.content,
            request.document.id
        )
        
        norm_analysis = [
            NormAnalysis(
                norm_id=norm['norm_id'],
                text=norm['text'],
                importance_score=norm['importance_score'],
                category=norm['category'],
                keywords=norm['keywords']
            ) for norm in analysis_result['norm_analysis']
        ]
        
        connections = [
            DocumentConnection(
                source_doc_id=conn['source_norm_id'],
                target_doc_id=conn['target_norm_id'],
                connection_type=conn['connection_type'],
                strength=conn['strength'],
                explanation=conn['explanation']
            ) for conn in analysis_result['connections']
        ]
        
        issues = [
            Issue(
                issue_id=issue['issue_id'],
                type=issue['type'],
                severity=issue['severity'],
                description=issue['description'],
                affected_norms=issue['affected_norms'],
                explanation=issue['explanation'],
                recommendation=issue['recommendation']
            ) for issue in analysis_result['issues']
        ]
        
        entropy_analysis = EntropyAnalysis(
            document_id=analysis_result['document_id'],
            entropy_score=analysis_result['entropy_score'],
            norm_analysis=norm_analysis,
            connections=connections,
            issues=issues,
            summary=analysis_result['summary'],
            visualization_data=analysis_result['visualization_data']
        )
        
        analysis_cache[request.document.id] = entropy_analysis
        
        processing_time = time.time() - start_time
        logger.info(f"Analysis completed for document: {request.document.id} in {processing_time:.2f}s")
        
        return InferenceResponse(
            success=True,
            analysis=entropy_analysis,
            message="Analysis completed successfully",
            processing_time=processing_time
        )
        
    except Exception as e:
        logger.error(f"Error analyzing document {request.document.id}: {str(e)}")
        processing_time = time.time() - start_time
        return InferenceResponse(
            success=False,
            analysis=None,
            message=f"Analysis failed: {str(e)}",
            processing_time=processing_time
        )

@app.post("/analyze-batch")
async def analyze_batch(documents: List[LegalDocument]):
    start_time = time.time()
    results = []
    
    try:
        for doc in documents:
            request = InferenceRequest(document=doc)
            result = await analyze_document(request)
            results.append(result)
        
        processing_time = time.time() - start_time
        return {
            "success": True,
            "message": f"Batch analysis completed for {len(documents)} documents",
            "results": results,
            "total_processing_time": processing_time
        }
        
    except Exception as e:
        logger.error(f"Error in batch analysis: {str(e)}")
        raise HTTPException(status_code=500, detail=f"Batch analysis failed: {str(e)}")

@app.get("/cache")
async def get_cache_info():
    return {
        "cache_size": len(analysis_cache),
        "cached_documents": list(analysis_cache.keys()),
        "memory_usage_estimate": len(str(analysis_cache)) / 1024 / 1024  # MB
    }

@app.delete("/cache/{document_id}")
async def clear_cache_entry(document_id: str):
    if document_id in analysis_cache:
        del analysis_cache[document_id]
        return {"message": f"Cache entry for {document_id} cleared"}
    else:
        raise HTTPException(status_code=404, detail="Document not found in cache")

@app.delete("/cache")
async def clear_all_cache():
    analysis_cache.clear()
    return {"message": "All cache cleared"}

@app.get("/statistics")
async def get_statistics():
    if not analysis_cache:
        return {"message": "No analyses in cache"}
    
    entropy_scores = [analysis.entropy_score for analysis in analysis_cache.values()]
    total_norms = sum(len(analysis.norm_analysis) for analysis in analysis_cache.values())
    total_issues = sum(len(analysis.issues) for analysis in analysis_cache.values())
    
    issue_types = {}
    for analysis in analysis_cache.values():
        for issue in analysis.issues:
            issue_types[issue.type] = issue_types.get(issue.type, 0) + 1
    
    return {
        "total_documents": len(analysis_cache),
        "average_entropy_score": sum(entropy_scores) / len(entropy_scores),
        "min_entropy_score": min(entropy_scores),
        "max_entropy_score": max(entropy_scores),
        "total_norms_analyzed": total_norms,
        "total_issues_found": total_issues,
        "issue_types": issue_types,
        "average_norms_per_document": total_norms / len(analysis_cache),
        "average_issues_per_document": total_issues / len(analysis_cache)
    }

@app.exception_handler(Exception)
async def global_exception_handler(request, exc):
    logger.error(f"Global exception: {str(exc)}")
    return JSONResponse(
        status_code=500,
        content={"message": "Internal server error", "detail": str(exc)}
    )

if __name__ == "__main__":
    uvicorn.run(
        "main:app",
        host="0.0.0.0",
        port=8000,
        reload=True,
        log_level="info"
    )
