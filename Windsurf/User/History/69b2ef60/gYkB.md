# Legislative Entropy Analysis API - Adilet.zan.kz Integration

## Overview

This project is a comprehensive AI system for analyzing legislative documents entropy and identifying legal issues, with full integration for parsing and monitoring documents from [Adilet.zan.kz](https://adilet.zan.kz/rus/index/docs) - the database of regulatory legal acts of the Republic of Kazakhstan.

## Features

### 🔍 Document Analysis
- **Norm Extraction**: Automatically identifies legal norms from document text
- **Importance Scoring**: Calculates significance scores for each norm
- **Categorization**: Classifies norms into legal categories (procedural, substantive, penalty, etc.)
- **Connection Analysis**: Finds references, contradictions, and duplications between norms
- **Issue Detection**: Identifies contradictions, duplications, and outdated norms
- **Entropy Calculation**: Computes overall document complexity score
- **Visualization**: Generates network graphs for document relationships

### 🌐 Adilet.zan.kz Integration
- **Web Parsing**: Automatically scrapes documents from Adilet.zan.kz
- **PDF Downloads**: Downloads official PDF versions of documents
- **Change Detection**: Monitors document updates and changes
- **Version Comparison**: Compares different versions of the same law
- **Cross-Document Analysis**: Detects conflicts across multiple documents
- **Storage Management**: Stores and manages parsed documents locally

### 📊 Monitoring & Analytics
- **Real-time Monitoring**: Continuous monitoring of legislative changes
- **Background Processing**: Asynchronous document scanning and analysis
- **Dashboard**: Comprehensive monitoring dashboard
- **Conflict Detection**: Identifies contradictions across the legal framework
- **Historical Analysis**: Tracks changes over time

## Architecture

The system follows SOLID principles with clean, maintainable architecture:

```
legis-entropy/
├── core/
│   ├── interfaces.py      # All interface definitions
│   └── entities.py        # Domain entities and enums
├── services/
│   ├── text_processor.py          # Text processing
│   ├── embedding_provider.py      # Embedding generation
│   ├── norm_extractor.py          # Norm extraction
│   ├── connection_analyzer.py     # Connection analysis
│   ├── issue_detector.py          # Issue detection
│   ├── entropy_calculator.py      # Entropy calculation
│   ├── visualization_generator.py # Visualization
│   ├── document_analyzer.py       # Main analyzer
│   ├── analysis_service.py        # Analysis orchestration
│   ├── cache_service.py           # Cache management
│   ├── adilet_parser.py           # Adilet.zan.kz parser
│   ├── document_comparator.py     # Version comparison
│   ├── document_storage.py        # Document storage
│   └── legislative_monitoring_service.py # Monitoring service
├── factories/
│   └── model_factory.py           # Dependency injection
├── api/
│   ├── routes.py                  # Analysis API routes
│   └── monitoring_routes.py       # Monitoring API routes
├── models.py                      # Pydantic models
├── main.py                        # Main application
├── example_monitoring.py          # Usage examples
└── README.md                      # This documentation
```

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd legis-entropy
```

2. Install dependencies:
```bash
pip install -r requirements.txt
```

3. Run the application:
```bash
python main.py
```

The API will be available at `http://localhost:8000`

## API Endpoints

### 📊 Analysis Endpoints

#### Analyze Document
```http
POST /analyze
Content-Type: application/json

{
  "document": {
    "id": "law_001",
    "title": "Закон об охране окружающей среды",
    "content": "Полный текст документа...",
    "document_type": "law",
    "source": "adilet.zan.kz"
  },
  "analyze_connections": true,
  "deep_analysis": false
}
```

#### Batch Analysis
```http
POST /analyze-batch
Content-Type: application/json

[
  {
    "id": "doc_001",
    "title": "First Law",
    "content": "...",
    "document_type": "law",
    "source": "adilet.zan.kz"
  }
]
```

### 🌐 Monitoring Endpoints

#### Scan New Documents
```http
POST /monitoring/scan
Content-Type: application/json

{
  "max_documents": 50,
  "analyze_immediately": true,
  "download_pdfs": false
}
```

#### Background Scanning
```http
POST /monitoring/scan-background
```

#### Analyze Document Changes
```http
GET /monitoring/analyze-changes/{document_id}?compare_with_previous=true
```

#### Detect Conflicts
```http
POST /monitoring/detect-conflicts
Content-Type: application/json

["doc_001", "doc_002", "doc_003"]
```

#### Monitoring Dashboard
```http
GET /monitoring/dashboard
```

#### Document Management
```http
GET /monitoring/documents                    # Get all documents
GET /monitoring/documents/{id}               # Get specific document
GET /monitoring/documents/type/{type}       # Get by type
GET /monitoring/documents/recent?days=30    # Get recent documents
DELETE /monitoring/documents/{id}            # Delete document
GET /monitoring/storage/stats               # Storage statistics
```

### 🔧 Utility Endpoints

```http
GET /health                                  # Health check
GET /cache                                   # Cache info
DELETE /cache/{id}                          # Clear cache entry
DELETE /cache                               # Clear all cache
GET /statistics                             # Analysis statistics
GET /docs                                   # Interactive API docs
```

## Usage Examples

### Basic Document Analysis
```python
import requests

document = {
    "document": {
        "id": "env_law",
        "title": "Закон об охране окружающей среды",
        "content": "Статья 1. Все граждане должны защищать окружающую среду...",
        "document_type": "law",
        "source": "adilet.zan.kz"
    }
}

response = requests.post("http://localhost:8000/analyze", json=document)
result = response.json()

if result['success']:
    analysis = result['analysis']
    print(f"Entropy Score: {analysis['entropy_score']}")
    print(f"Issues Found: {len(analysis['issues'])}")
```

### Monitoring Adilet.zan.kz
```python
import requests

# Scan for new documents
response = requests.post("http://localhost:8000/monitoring/scan", json={
    "max_documents": 20,
    "analyze_immediately": True,
    "download_pdfs": True
})

scan_result = response.json()
print(f"New documents: {scan_result['new_documents_count']}")
print(f"Updated documents: {scan_result['updated_documents_count']}")

# Get monitoring dashboard
dashboard = requests.get("http://localhost:8000/monitoring/dashboard").json()
print(f"Total documents: {dashboard['storage_statistics']['total_documents']}")
```

### Running Tests
```bash
# Test monitoring functionality
python example_monitoring.py

# Test basic analysis
python example_usage.py
```

## Adilet.zan.kz Integration

### Document Parsing
The system automatically:
- Scrapes document listings from `https://adilet.zan.kz/rus/index/docs`
- Extracts document metadata (title, type, dates)
- Parses document content and structure
- Downloads PDF versions using URLs like `https://adilet.zan.kz/rus/docs/K950001000_/download`

### Change Detection
- Monitors document updates by comparing last modified dates
- Identifies added, removed, and modified norms
- Tracks version changes over time
- Detects contradictions between document versions

### Cross-Document Analysis
- Finds contradictions across different laws
- Identifies duplications in the legal framework
- Detects outdated norms that need revision
- Provides recommendations for conflict resolution

## Analysis Features

### Entropy Score Interpretation
- **0-2**: Low entropy - Well-structured, clear document
- **2-5**: Medium entropy - Some complexity, may require attention  
- **5+**: High entropy - Significant complexity or potential issues

### Issue Types
- **Contradictions**: Conflicting legal norms
- **Duplications**: Redundant or duplicate content
- **Outdated**: Norms that are no longer relevant
- **Quality**: Low importance or poorly structured norms

### Connection Types
- **Reference**: Internal or external references
- **Contradiction**: Conflicting norms
- **Duplication**: Duplicate content
- **Similarity**: Semantically similar norms

## Russian Language Support

The system is optimized for Russian legal documents:
- Russian legal terminology and keywords
- Russian stop words and text processing
- Multilingual embedding models
- Russian document structure recognition
- Russian explanations and recommendations

## Performance & Scalability

### Caching
- Intelligent caching of analysis results
- Document storage with metadata
- Configurable cache policies

### Background Processing
- Asynchronous document scanning
- Background analysis tasks
- Non-blocking API operations

### Storage
- Local document storage with JSON format
- PDF download capabilities
- Metadata indexing and search
- Storage statistics and management

## API Documentation

Interactive API documentation is available at:
- **Swagger UI**: `http://localhost:8000/docs`
- **ReDoc**: `http://localhost:8000/redoc`

## Configuration

### Environment Variables
```bash
# Server configuration
HOST=0.0.0.0
PORT=8000

# Storage configuration
STORAGE_DIR=data/documents
PDF_DIR=data/documents/pdfs

# Parsing configuration
MAX_DOCUMENTS_PER_SCAN=100
SCAN_DELAY=1.0
DOWNLOAD_PDFS=false
```

### Customization
- Add new legal keywords in `services/norm_extractor.py`
- Modify document type detection in `services/adilet_parser.py`
- Extend analysis algorithms by implementing new interfaces
- Add new visualization generators

## Troubleshooting

### Common Issues

1. **Parser Errors**: Check internet connection and Adilet.zan.kz availability
2. **Analysis Failures**: Verify sentence-transformers models are downloaded
3. **Storage Issues**: Ensure write permissions for data directory
4. **Memory Issues**: Reduce `max_documents` parameter for scanning

### Logging
All operations are logged with appropriate levels:
```python
import logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)
```

## Contributing

1. Follow SOLID principles and clean code practices
2. Add comprehensive tests for new features
3. Update documentation for API changes
4. Use type hints and proper error handling

## License

[License information]

## Support

For issues and questions:
- Create an issue in the repository
- Check API documentation at `/docs`
- Review example usage scripts

---

**Version**: 3.0.0  
**Last Updated**: 2024  
**Features**: SOLID Architecture, Adilet.zan.kz Integration, Russian Language Support, Real-time Monitoring
