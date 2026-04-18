
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' 

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

if ! docker info > /dev/null 2>&1; then
    print_error "Docker is not running. Please start Docker first."
    exit 1
fi

print_status "Docker is running ✓"

# Build the Docker image
print_status "Building Docker image..."
docker build -t legis-entropy-api .

if [ $? -eq 0 ]; then
    print_success "Docker image built successfully ✓"
else
    print_error "Failed to build Docker image"
    exit 1
fi

# Stop any existing container
print_status "Stopping any existing containers..."
docker stop legis-entropy-api 2>/dev/null || true
docker rm legis-entropy-api 2>/dev/null || true

# Run the container
print_status "Starting container..."
docker run -d \
    --name legis-entropy-api \
    -p 8000:8000 \
    -v $(pwd)/data:/app/data \
    -v $(pwd)/logs:/app/logs \
    legis-entropy-api

if [ $? -eq 0 ]; then
    print_success "Container started successfully ✓"
else
    print_error "Failed to start container"
    exit 1
fi

# Wait for the container to be ready
print_status "Waiting for API to be ready..."
sleep 10

# Check container health
print_status "Checking container health..."
HEALTH=$(docker inspect --format='{{.State.Health.Status}}' legis-entropy-api 2>/dev/null || echo "unknown")

if [ "$HEALTH" = "healthy" ]; then
    print_success "Container is healthy ✓"
elif [ "$HEALTH" = "starting" ]; then
    print_warning "Container is still starting..."
    sleep 5
else
    print_warning "Container health status: $HEALTH"
fi

# Test API endpoints
print_status "Testing API endpoints..."

API_URL="http://localhost:8000"

# Test 1: Health check
print_status "Testing health check..."
HEALTH_RESPONSE=$(curl -s -w "%{http_code}" -o /tmp/health.json "$API_URL/health")
HTTP_CODE="${HEALTH_RESPONSE: -3}"

if [ "$HTTP_CODE" = "200" ]; then
    print_success "Health check passed ✓"
    cat /tmp/health.json | python3 -m json.tool | head -10
else
    print_error "Health check failed (HTTP $HTTP_CODE)"
    cat /tmp/health.json
fi

echo ""

# Test 2: Root endpoint
print_status "Testing root endpoint..."
ROOT_RESPONSE=$(curl -s -w "%{http_code}" -o /tmp/root.json "$API_URL/")
HTTP_CODE="${ROOT_RESPONSE: -3}"

if [ "$HTTP_CODE" = "200" ]; then
    print_success "Root endpoint passed ✓"
    echo "API Version: $(cat /tmp/root.json | python3 -c "import sys, json; print(json.load(sys.stdin)['version'])" 2>/dev/null || echo 'unknown')"
else
    print_error "Root endpoint failed (HTTP $HTTP_CODE)"
fi

echo ""

# Test 3: Document Analysis
print_status "Testing document analysis..."
TEST_DOCUMENT='{
    "document": {
        "id": "docker_test_001",
        "title": "Тестовый закон",
        "content": "Статья 1. Все граждане должны соблюдать закон. Статья 2. Нарушители несут ответственность. Статья 3. Государство обеспечивает правопорядок.",
        "document_type": "law",
        "source": "docker_test"
    },
    "analyze_connections": true,
    "deep_analysis": false
}'

ANALYSIS_RESPONSE=$(curl -s -w "%{http_code}" -o /tmp/analysis.json -X POST \
    -H "Content-Type: application/json" \
    -d "$TEST_DOCUMENT" \
    "$API_URL/analyze")

HTTP_CODE="${ANALYSIS_RESPONSE: -3}"

if [ "$HTTP_CODE" = "200" ]; then
    print_success "Document analysis passed ✓"
    SUCCESS=$(cat /tmp/analysis.json | python3 -c "import sys, json; print(json.load(sys.stdin)['success'])" 2>/dev/null || echo 'false')
    if [ "$SUCCESS" = "True" ]; then
        ENTROPY=$(cat /tmp/analysis.json | python3 -c "import sys, json; print(json.load(sys.stdin)['analysis']['entropy_score'])" 2>/dev/null || echo 'N/A')
        echo "Entropy Score: $ENTROPY"
        NORMS_COUNT=$(cat /tmp/analysis.json | python3 -c "import sys, json; print(len(json.load(sys.stdin)['analysis']['norm_analysis']))" 2>/dev/null || echo 'N/A')
        echo "Norms Found: $NORMS_COUNT"
    else
        print_warning "Analysis returned success=false"
    fi
else
    print_error "Document analysis failed (HTTP $HTTP_CODE)"
    cat /tmp/analysis.json
fi

echo ""

# Test 4: Monitoring Dashboard
print_status "Testing monitoring dashboard..."
DASHBOARD_RESPONSE=$(curl -s -w "%{http_code}" -o /tmp/dashboard.json "$API_URL/monitoring/dashboard")
HTTP_CODE="${DASHBOARD_RESPONSE: -3}"

if [ "$HTTP_CODE" = "200" ]; then
    print_success "Monitoring dashboard passed ✓"
    STORAGE_STATS=$(cat /tmp/dashboard.json | python3 -c "import sys, json; print(json.load(sys.stdin)['storage_statistics']['total_documents'])" 2>/dev/null || echo 'N/A')
    echo "Stored Documents: $STORAGE_STATS"
else
    print_error "Monitoring dashboard failed (HTTP $HTTP_CODE)"
fi

echo ""

# Test 5: Cache operations
print_status "Testing cache operations..."
CACHE_RESPONSE=$(curl -s -w "%{http_code}" -o /tmp/cache.json "$API_URL/cache")
HTTP_CODE="${CACHE_RESPONSE: -3}"

if [ "$HTTP_CODE" = "200" ]; then
    print_success "Cache operations passed ✓"
    CACHE_SIZE=$(cat /tmp/cache.json | python3 -c "import sys, json; print(json.load(sys.stdin)['cache_size'])" 2>/dev/null || echo 'N/A')
    echo "Cache Size: $CACHE_SIZE"
else
    print_error "Cache operations failed (HTTP $HTTP_CODE)"
fi

echo ""

# Show container logs
print_status "Recent container logs:"
docker logs --tail 10 legis-entropy-api

echo ""

# Show container status
print_status "Container status:"
docker ps --filter name=legis-entropy-api

echo ""
print_success "Docker test completed! ✓"
echo ""
echo "🌐 API is running at: http://localhost:8000"
echo "📚 Interactive docs: http://localhost:8000/docs"
echo "📊 Monitoring dashboard: http://localhost:8000/monitoring/dashboard"
echo ""
echo "To stop the container: docker stop legis-entropy-api"
echo "To view logs: docker logs -f legis-entropy-api"
echo "To access container shell: docker exec -it legis-entropy-api bash"

# Cleanup temporary files
rm -f /tmp/health.json /tmp/root.json /tmp/analysis.json /tmp/dashboard.json /tmp/cache.json
