import requests
import json
from datetime import datetime

# Example document content for testing (Russian)
sample_document = {
    "id": "law_001",
    "title": "Закон об охране окружающей среды",
    "content": """
    Статья 1. Все граждане должны защищать окружающую среду и природные ресурсы.
    Статья 2. Промышленные объекты обязаны получить экологические разрешения перед началом эксплуатации.
    Статья 3. Нарушение экологических норм влечет за собой штрафные санкции.
    Статья 4. Министерство охраны окружающей среды несет ответственность за надзор.
    Статья 5. Предприятия обязаны ежеквартально отчитываться о выбросах.
    Статья 6. Незаконная сброс отходов запрещается и карается штрафами.
    Статья 7. Экологическая экспертиза требуется для новых проектов.
    Статья 8. Граждане имеют право на доступ к экологической информации.
    Статья 9. Правительство должно способствовать устойчивому развитию.
    Статья 10. Меры по контролю за загрязнением должны внедряться всеми отраслями.
    """,
    "document_type": "law",
    "version": "2.1",
    "adoption_date": datetime.now().isoformat(),
    "source": "adilet.zan.kz"
}

def test_analysis():
    """Test the document analysis endpoint"""
    url = "http://localhost:8000/analyze"
    
    payload = {
        "document": sample_document,
        "analyze_connections": True,
        "deep_analysis": True
    }
    
    try:
        response = requests.post(url, json=payload)
        response.raise_for_status()
        
        result = response.json()
        print("Analysis Results:")
        print(f"Success: {result['success']}")
        print(f"Processing Time: {result['processing_time']:.2f}s")
        print(f"Message: {result['message']}")
        
        if result['success'] and result['analysis']:
            analysis = result['analysis']
            print(f"\nDocument ID: {analysis['document_id']}")
            print(f"Entropy Score: {analysis['entropy_score']}")
            print(f"Summary: {analysis['summary']}")
            
            print(f"\nNorms Found: {len(analysis['norm_analysis'])}")
            for norm in analysis['norm_analysis'][:3]:  # Show first 3
                print(f"  - {norm['norm_id']}: {norm['text'][:50]}...")
                print(f"    Importance: {norm['importance_score']}, Category: {norm['category']}")
            
            print(f"\nConnections Found: {len(analysis['connections'])}")
            for conn in analysis['connections']:
                print(f"  - {conn['source_doc_id']} -> {conn['target_doc_id']}")
                print(f"    Type: {conn['connection_type']}, Strength: {conn['strength']}")
            
            print(f"\nIssues Found: {len(analysis['issues'])}")
            for issue in analysis['issues']:
                print(f"  - {issue['type']}: {issue['severity']} severity")
                print(f"    {issue['description']}")
        
        return result
        
    except requests.exceptions.ConnectionError:
        print("Error: Could not connect to the API. Make sure the server is running on localhost:8000")
        return None
    except requests.exceptions.RequestException as e:
        print(f"Error: {e}")
        return None

def test_health():
    """Test the health check endpoint"""
    url = "http://localhost:8000/health"
    
    try:
        response = requests.get(url)
        response.raise_for_status()
        
        result = response.json()
        print("Health Check:")
        print(json.dumps(result, indent=2))
        
        return result
        
    except requests.exceptions.ConnectionError:
        print("Error: Could not connect to the API")
        return None
    except requests.exceptions.RequestException as e:
        print(f"Error: {e}")
        return None

def test_statistics():
    """Test the statistics endpoint"""
    url = "http://localhost:8000/statistics"
    
    try:
        response = requests.get(url)
        response.raise_for_status()
        
        result = response.json()
        print("Statistics:")
        print(json.dumps(result, indent=2))
        
        return result
        
    except requests.exceptions.ConnectionError:
        print("Error: Could not connect to the API")
        return None
    except requests.exceptions.RequestException as e:
        print(f"Error: {e}")
        return None

def test_batch_analysis():
    """Test batch analysis with multiple documents"""
    url = "http://localhost:8000/analyze-batch"
    
    doc2 = {
        "id": "regulation_001",
        "title": "Environmental Regulation",
        "content": """
        Section 1. All businesses must register with environmental authorities.
        Section 2. Waste disposal requires special permits.
        Section 3. Water quality standards must be maintained.
        Section 4. Air pollution monitoring is mandatory.
        Section 5. Non-compliance results in suspension of operations.
        """,
        "document_type": "regulation",
        "version": "1.0",
        "source": "adilet.zan.kz"
    }
    
    documents = [sample_document, doc2]
    
    try:
        response = requests.post(url, json=documents)
        response.raise_for_status()
        
        result = response.json()
        print("Batch Analysis Results:")
        print(f"Success: {result['success']}")
        print(f"Message: {result['message']}")
        print(f"Total Processing Time: {result['total_processing_time']:.2f}s")
        print(f"Documents Processed: {len(result['results'])}")
        
        return result
        
    except requests.exceptions.ConnectionError:
        print("Error: Could not connect to the API")
        return None
    except requests.exceptions.RequestException as e:
        print(f"Error: {e}")
        return None

if __name__ == "__main__":
    print("Testing Legislative Entropy Analysis API")
    print("=" * 50)
    
    print("\n1. Testing Health Check...")
    test_health()
    
    print("\n2. Testing Single Document Analysis...")
    test_analysis()
    
    print("\n3. Testing Batch Analysis...")
    test_batch_analysis()
    
    print("\n4. Testing Statistics...")
    test_statistics()
    
    print("\nTesting completed!")
