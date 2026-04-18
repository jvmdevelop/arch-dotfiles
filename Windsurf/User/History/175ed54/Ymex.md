<h1 align="center">mh</h1>
<p align="center" >
  <img alt="Java" src="https://img.shields.io/badge/Java-ED8B00?logo=openjdk&logoColor=white">
  <img alt="Spring Boot" src="https://img.shields.io/badge/Spring%20Boot-6DB33F?logo=spring-boot&logoColor=white">
  <img alt="PostgreSQL" src="https://img.shields.io/badge/PostgreSQL-4169E1?logo=postgresql&logoColor=white">
  <img alt="Elasticsearch" src="https://img.shields.io/badge/Elasticsearch-005571?logo=elasticsearch&logoColor=white">
  <img alt="Status" src="https://img.shields.io/badge/status-beta-yellow">
  <img alt="License" src="https://img.shields.io/badge/license-ISC-blue">
</p>

<br>

**mh** is an intelligent educational platform powered by Spring Boot and AI, featuring RAG (Retrieval-Augmented Generation) capabilities for mathematical problem solving, task management, and scheduling.

## Features

- AI-powered mathematical problem solving with RAG
- Real-time chat interface with WebSocket support
- Task management and scheduling system
- Document ingestion and retrieval
- PostgreSQL database with JPA
- Elasticsearch for semantic search
- Spring Security authentication
- RESTful API design
- Docker Compose support

## Installation

### Prerequisites:

- Java 21 
- Gradle 7.0+
- PostgreSQL database
- Elasticsearch
- Docker & Docker Compose 

### From source:

```bash
git clone git@github.com:jvmdevelop/mh.git
cd mh/mh-backend
./gradlew build
./gradlew bootRun
```

### With Docker Compose:

```bash
cd mh/mh-backend
docker-compose up
```

## Usage

### Configuration

Configure your `application.properties`:

```properties
spring.datasource.url=jdbc:postgresql://localhost:5432/mh
spring.datasource.username=your_username
spring.datasource.password=your_password
spring.elasticsearch.uris=http://localhost:9200
```

### Running the application:

```bash
./gradlew bootRun
```

The application will be available at `http://localhost:8080`

## API Endpoints

### Chat & AI

| Endpoint | Method | Description |
|:---------|:------:|:------------|
| `/api/chat/send` | POST | send message to AI |
| `/api/chat/history` | GET | get chat history |
| `/api/rag/ingest` | POST | ingest documents |

### Task Management

| Endpoint | Method | Description |
|:---------|:------:|:------------|
| `/api/tasks` | GET | get all tasks |
| `/api/tasks` | POST | create new task |
| `/api/tasks/{id}` | PUT | update task |
| `/api/tasks/{id}` | DELETE | delete task |

### Scheduling

| Endpoint | Method | Description |
|:---------|:------:|:------------|
| `/api/schedule` | GET | get schedule |
| `/api/schedule` | POST | create schedule entry |
| `/api/schedule/{id}` | PUT | update schedule |

## WebSocket Endpoints

- `/ws/chat` - real-time AI chat interface

## Project Structure

- `mh-backend/` - Spring Boot backend application
  - `src/main/java/com/jvmd/mh_backend/` - Main package
  - `controller/` - REST API controllers
  - `service/` - Business logic services
  - `service/rag/` - RAG and AI services
  - `model/` - JPA entities
  - `repo/` - Repository interfaces
  - `config/` - Configuration classes
- `mh-frontend/` - Frontend application

## Examples

Send a math problem to AI:

```bash
curl -X POST http://localhost:8080/api/chat/send \
  -H "Content-Type: application/json" \
  -d '{"message": "Solve: 2x + 5 = 15"}'
```

Create a new task:

```bash
curl -X POST http://localhost:8080/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Complete homework", "description": "Math chapter 5", "dueDate": "2024-01-15"}'
```

## Dependencies

- Spring Boot 3.5.9
- Spring Security
- Spring Data JPA
- Spring WebSocket
- PostgreSQL Driver
- Elasticsearch
- Spring AI
- Lombok
- Docker Compose

## AI Integration

The application integrates with AI models for:
- Mathematical problem solving
- Step-by-step explanations
- Graph and visualization generation
- Natural language processing

## Contributing

1. Fork the repository
2. Create a feature branch
3. Submit a pull request

## License

ISC — see [LICENSE](LICENSE) for details.

## EOF
