<h1 align="center">mh-backend</h1>
<p align="center" >
  <img alt="Java" src="https://img.shields.io/badge/Java-ED8B00?logo=openjdk&logoColor=white">
  <img alt="Spring Boot" src="https://img.shields.io/badge/Spring%20Boot-6DB33F?logo=spring-boot&logoColor=white">
  <img alt="PostgreSQL" src="https://img.shields.io/badge/PostgreSQL-4169E1?logo=postgresql&logoColor=white">
  <img alt="Elasticsearch" src="https://img.shields.io/badge/Elasticsearch-005571?logo=elasticsearch&logoColor=white">
  <img alt="Status" src="https://img.shields.io/badge/status-beta-yellow">
  <img alt="License" src="https://img.shields.io/badge/license-ISC-blue">
</p>

<br>

**mh-backend** is a powerful Spring Boot backend service for mathematical assistance, featuring AI-powered problem solving, RAG (Retrieval-Augmented Generation) capabilities, and comprehensive task management system.

## features

- ai-powered mathematical problem solving with rag
- real-time chat interface with websocket support
- task management and scheduling system
- document ingestion and retrieval
- postgresql database with jpa
- elasticsearch for semantic search
- spring security authentication
- restful api design
- docker compose support

## installation

### prerequisites:

- java 21
- gradle 7.0+
- postgresql database
- elasticsearch
- docker & docker compose

### from source:

```bash
git clone git@github.com:jvmdevelop/mh-backend.git
cd mh-backend
./gradlew build
./gradlew bootRun
```

### with docker compose:

```bash
cd mh-backend
docker compose up -d
./gradlew bootRun
```

## usage

### configuration

Configure your `application.yml`:

```yaml
spring:
  datasource:
    url: jdbc:postgresql://localhost:5432/mh
    username: your_username
    password: your_password
  elasticsearch:
    uris: http://localhost:9200
  ai:
    openai:
      api-key: ${OPENAI_API_KEY}
```

### running the application:

```bash
./gradlew bootRun
```

The application will be available at `http://localhost:8080`

## api endpoints

### chat & ai

| endpoint | method | description |
|:---------|:------:|:------------|
| `/api/chat/send` | POST | send message to ai |
| `/api/chat/history` | GET | get chat history |
| `/api/rag/ingest` | POST | ingest documents |

### task management

| endpoint | method | description |
|:---------|:------:|:------------|
| `/api/tasks` | GET | get all tasks |
| `/api/tasks` | POST | create new task |
| `/api/tasks/{id}` | PUT | update task |
| `/api/tasks/{id}` | DELETE | delete task |

### scheduling

| endpoint | method | description |
|:---------|:------:|:------------|
| `/api/schedule` | GET | get schedule |
| `/api/schedule` | POST | create schedule entry |
| `/api/schedule/{id}` | PUT | update schedule |

## websocket endpoints

- `/ws/chat` - real-time ai chat interface

## project structure

- `src/main/java/com/jvmd/mh_backend/` - main package
  - `controller/` - rest api controllers
  - `service/` - business logic services
  - `service/rag/` - rag and ai services
  - `model/` - jpa entities
  - `repo/` - repository interfaces
  - `config/` - configuration classes

## examples

send a math problem to ai:

```bash
curl -X POST http://localhost:8080/api/chat/send \
  -H "Content-Type: application/json" \
  -d '{"message": "Solve: 2x + 5 = 15"}'
```

create a new task:

```bash
curl -X POST http://localhost:8080/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"name": "Complete homework", "context": "Math chapter 5"}'
```

## dependencies

- spring boot 3.5.9
- spring security
- spring data jpa
- spring websocket
- postgresql driver
- elasticsearch
- spring ai
- lombok
- docker compose

## ai integration

the application integrates with ai models for:
- mathematical problem solving
- step-by-step explanations
- graph and visualization generation
- natural language processing

## docker compose

the project includes docker compose configuration for:
- postgresql 16 database
- elasticsearch 8.10.2 search engine
- automatic container orchestration

## contributing

1. fork the repository
2. create a feature branch
3. submit a pull request

## license

ISC — see [LICENSE](LICENSE) for details.

## eof
