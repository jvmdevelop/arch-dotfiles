from fastapi import FastAPI, HTTPException, BackgroundTasks
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse
import logging
import uvicorn

from models import LegalDocument, InferenceRequest, InferenceResponse
from factories.model_factory import ModelFactory
from api.routes import AnalysisRoutes
from api.monitoring_routes import MonitoringRoutes

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

app = FastAPI(
    title="Legislative Entropy Analysis API",
    description="AI system for analyzing legislative documents entropy and identifying issues with Adilet.zan.kz integration",
    version="3.0.0"
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

analysis_service = ModelFactory.create_analysis_service()
cache_service = ModelFactory.create_cache_service()
routes = AnalysisRoutes(analysis_service, cache_service)

monitoring_routes = MonitoringRoutes()

@app.get("/")
async def root():
    """Root endpoint with API information"""
    return {
        "message": "Legislative Entropy Analysis API",
        "version": "3.0.0",
        "architecture": "SOLID principles with Adilet.zan.kz integration",
        "features": [
            "Document analysis",
            "Legislative monitoring",
            "Change detection",
            "Conflict analysis",
            "PDF downloads"
        ],
        "endpoints": {
            "analysis": {
                "analyze": "/analyze",
                "analyze_batch": "/analyze-batch",
                "health": "/health",
                "cache": "/cache",
                "statistics": "/statistics"
            },
            "monitoring": {
                "scan_documents": "/monitoring/scan",
                "scan_background": "/monitoring/scan-background",
                "analyze_changes": "/monitoring/analyze-changes/{document_id}",
                "detect_conflicts": "/monitoring/detect-conflicts",
                "dashboard": "/monitoring/dashboard",
                "documents": "/monitoring/documents",
                "document_by_id": "/monitoring/documents/{document_id}",
                "documents_by_type": "/monitoring/documents/type/{document_type}",
                "recent_documents": "/monitoring/documents/recent",
                "storage_stats": "/monitoring/storage/stats"
            },
            "docs": "/docs"
        }
    }

@app.get("/health")
async def health_check():
    """Health check endpoint"""
    cache_info = cache_service.get_cache_info()
    return {
        "status": "healthy",
        "services": {
            "analysis_service": "running",
            "cache_service": "running",
            "monitoring_service": "running",
            "adilet_parser": "available"
        },
        "cache_size": cache_info["cache_size"]
    }

@app.post("/analyze", response_model=InferenceResponse)
async def analyze_document(request: InferenceRequest):
    try:
        result = await routes.analyze_document(request.dict())
        return result
    except Exception as e:
        logger.error(f"Error in analyze_document: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/analyze-batch")
async def analyze_batch(documents: List[LegalDocument]):

    try:
        result = await routes.analyze_batch(documents)
        return result
    except Exception as e:
        logger.error(f"Error in analyze_batch: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/cache")
async def get_cache_info():
    try:
        return await routes.get_cache_info()
    except Exception as e:
        logger.error(f"Error in get_cache_info: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.delete("/cache/{document_id}")
async def clear_cache_entry(document_id: str):

    try:
        return await routes.clear_cache_entry(document_id)
    except Exception as e:
        logger.error(f"Error in clear_cache_entry: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.delete("/cache")
async def clear_all_cache():

    try:
        return await routes.clear_all_cache()
    except Exception as e:
        logger.error(f"Error in clear_all_cache: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/statistics")
async def get_statistics():
    try:
        return await routes.get_statistics()
    except Exception as e:
        logger.error(f"Error in get_statistics: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/monitoring/scan")
async def scan_new_documents(
    max_documents: int = 50,
    analyze_immediately: bool = True,
    download_pdfs: bool = False
):
    try:
        return await monitoring_routes.scan_new_documents(
            max_documents=max_documents,
            analyze_immediately=analyze_immediately,
            download_pdfs=download_pdfs
        )
    except Exception as e:
        logger.error(f"Error in scan_new_documents: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/monitoring/scan-background")
async def scan_new_documents_background(
    background_tasks: BackgroundTasks,
    max_documents: int = 50,
    analyze_immediately: bool = True,
    download_pdfs: bool = False
):

    try:
        return await monitoring_routes.scan_new_documents_background(
            background_tasks=background_tasks,
            max_documents=max_documents,
            analyze_immediately=analyze_immediately,
            download_pdfs=download_pdfs
        )
    except Exception as e:
        logger.error(f"Error in scan_new_documents_background: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/monitoring/analyze-changes/{document_id}")
async def analyze_document_changes(document_id: str, compare_with_previous: bool = True):
    """Analyze changes for a specific document"""
    try:
        return await monitoring_routes.analyze_document_changes(
            document_id=document_id,
            compare_with_previous=compare_with_previous
        )
    except Exception as e:
        logger.error(f"Error in analyze_document_changes: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/monitoring/detect-conflicts")
async def detect_legislative_conflicts(document_ids: List[str]):
    """Detect conflicts and contradictions across multiple documents"""
    try:
        return await monitoring_routes.detect_legislative_conflicts(document_ids)
    except Exception as e:
        logger.error(f"Error in detect_legislative_conflicts: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/monitoring/dashboard")
async def get_monitoring_dashboard():
    """Get monitoring dashboard data"""
    try:
        return await monitoring_routes.get_monitoring_dashboard()
    except Exception as e:
        logger.error(f"Error in get_monitoring_dashboard: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/monitoring/documents")
async def get_stored_documents():
    """Get list of all stored documents"""
    try:
        return await monitoring_routes.get_stored_documents()
    except Exception as e:
        logger.error(f"Error in get_stored_documents: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/monitoring/documents/{document_id}")
async def get_document_by_id(document_id: str):
    """Get specific document by ID"""
    try:
        return await monitoring_routes.get_document_by_id(document_id)
    except Exception as e:
        logger.error(f"Error in get_document_by_id: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/monitoring/documents/type/{document_type}")
async def get_documents_by_type(document_type: str):
    """Get documents by type"""
    try:
        return await monitoring_routes.get_documents_by_type(document_type)
    except Exception as e:
        logger.error(f"Error in get_documents_by_type: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/monitoring/documents/recent")
async def get_recent_documents(days: int = 30):
    """Get recent documents"""
    try:
        return await monitoring_routes.get_recent_documents(days)
    except Exception as e:
        logger.error(f"Error in get_recent_documents: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.delete("/monitoring/documents/{document_id}")
async def delete_document(document_id: str):
    """Delete a document"""
    try:
        return await monitoring_routes.delete_document(document_id)
    except Exception as e:
        logger.error(f"Error in delete_document: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/monitoring/storage/stats")
async def get_storage_statistics():
    """Get storage statistics"""
    try:
        return await monitoring_routes.get_storage_statistics()
    except Exception as e:
        logger.error(f"Error in get_storage_statistics: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))

@app.exception_handler(Exception)
async def global_exception_handler(request, exc):
    """Global exception handler"""
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
