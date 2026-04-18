
echo "Starting Walto Backend on Debian..."

if ! command -v docker &> /dev/null; then
    echo "Docker is not installed. Installing Docker..."
    sudo apt-get update
    
    sudo apt-get install -y apt-transport-https ca-certificates curl gnupg lsb-release
    
    curl -fsSL https://download.docker.com/linux/debian/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
    
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
    
    sudo apt-get update
    sudo apt-get install -y docker-ce docker-ce-cli containerd.io
    
    sudo usermod -aG docker $USER
    
    echo "Docker installed. Please log out and log back in to use Docker without sudo."
    echo "Then run this script again."
    exit 1
fi

if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null; then
    echo "Docker Compose is not installed. Installing Docker Compose..."
    sudo apt-get update
    sudo apt-get install -y docker-compose-plugin
    
    echo "Docker Compose installed."
fi

echo "Stopping existing containers..."
docker compose down 2>/dev/null || docker-compose down 2>/dev/null || true

echo "Building and starting application..."
docker compose up --build -d

echo "Waiting for application to start..."
sleep 30

if curl -f http://localhost:8080/actuator/health &> /dev/null; then
    echo "✅ Application started successfully!"
    echo "🌐 Frontend URL: http://localhost:8080"
    echo "🗄️  Database: postgresql://localhost:5432/walto"
else
    echo "❌ Application may not be running properly. Check logs with: docker compose logs -f"
fi
