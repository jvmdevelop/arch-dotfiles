<h1 align="center">Transaction Checker Service</h1>
<p align="center" >
  <img alt="Java" src="https://img.shields.io/badge/Java-ED8B00?logo=openjdk&logoColor=white">
  <img alt="Spring Boot" src="https://img.shields.io/badge/Spring%20Boot-6DB33F?logo=spring-boot&logoColor=white">
  <img alt="PostgreSQL" src="https://img.shields.io/badge/PostgreSQL-4169E1?logo=postgresql&logoColor=white">
  <img alt="Redis" src="https://img.shields.io/badge/Redis-DC382D?logo=redis&logoColor=white">
  <img alt="Docker" src="https://img.shields.io/badge/Docker-2496ED?logo=docker&logoColor=white">
  <img alt="Status" src="https://img.shields.io/badge/status-beta-yellow">
  <img alt="License" src="https://img.shields.io/badge/license-ISC-blue">
</p>

<br>

**Transaction Checker Service** is an intelligent fraud detection system powered by Spring Boot and AI, featuring real-time transaction analysis, rule-based detection, and comprehensive monitoring capabilities.

## features

- ai-powered fraud detection with machine learning models
- real-time transaction processing and analysis
- configurable rule-based detection system
- comprehensive monitoring with prometheus and grafana
- centralized logging with graylog, elasticsearch, and mongodb
- redis caching for high-performance operations
- restful api with openapi documentation
- websocket support for real-time notifications
- email and webhook notifications
- docker compose support for easy deployment

## installation

### prerequisites:

- java 21 
- gradle 7.0+
- postgresql database
- redis
- docker & docker compose 

### from source:

```bash
git clone git@github.com:jvmdevelop/transaction-checker-service.git
cd transaction-checker-service
./gradlew build
./gradlew bootRun
```

### with docker compose:

```bash
cd transaction-checker-service
docker-compose up
```

## usage

### configuration

configure your `application.properties`:

```properties
spring.datasource.url=jdbc:postgresql://localhost:5432/fraud_detection
spring.datasource.username=your_username
spring.datasource.password=your_password
spring.redis.host=localhost
spring.redis.port=6379
```

### Running the application:

```bash
./gradlew bootRun
```

The application will be available at `http://localhost:8080` 

### Accessing Services:

- **main application**: `http://localhost:8080`
- **frontend**: `http://localhost:3000`
- **grafana dashboard**: `http://localhost:3001` (admin/admin)
- **prometheus**: `http://localhost:9090`
- **graylog**: `http://localhost:9000` (admin/admin)
- **api documentation**: `http://localhost:8080/swagger-ui.html`

## API Endpoints

### Transactions

| Endpoint | Method | Description |
|:---------|:------:|:------------|
| `/api/transactions` | GET | get all transactions |
| `/api/transactions` | POST | create new transaction |
| `/api/transactions/{id}` | GET | get transaction details |
| `/api/transactions/search` | POST | search transactions |

### Rules Management

| Endpoint | Method | Description |
|:---------|:------:|:------------|
| `/api/rules` | GET | get all rules |
| `/api/rules` | POST | create new rule |
| `/api/rules/{id}` | PUT | update rule |
| `/api/rules/{id}` | DELETE | delete rule |
| `/api/rules/{id}/history` | GET | get rule change history |

### Admin & Monitoring

| Endpoint | Method | Description |
|:---------|:------:|:------------|
| `/api/admin/dashboard` | GET | get dashboard statistics |
| `/api/admin/metrics` | GET | get application metrics |
| `/api/admin/notifications` | GET | get notification logs |

### Notifications

| Endpoint | Method | Description |
|:---------|:------:|:------------|
| `/api/notifications/config` | GET | get notification config |
| `/api/notifications/config` | PUT | update notification config |
| `/api/notifications/test` | POST | test notification |

## Project Structure

- `src/main/java/com/jvmd/transationapp/` - main package
  - `controller/` - rest api controllers
  - `service/` - business logic services
  - `model/` - jpa entities
  - `repository/` - repository interfaces
  - `dto/` - data transfer objects
  - `config/` - configuration classes
- `fraud-detection-frontend/` - frontend application
- `monitoring/` - monitoring and logging configurations
- `ml-model/` - machine learning models

## Examples

Create a new transaction:

```bash
curl -X POST http://localhost:8080/api/transactions \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 1500.00,
    "description": "Online purchase",
    "accountId": "ACC123456",
    "merchantId": "MER789012",
    "timestamp": "2024-01-15T10:30:00Z"
  }'
```

Create a new fraud detection rule:

```bash
curl -X POST http://localhost:8080/api/rules \
  -H "Content-Type: application/json" \
  -d '{
    "name": "High Amount Alert",
    "type": "AMOUNT_THRESHOLD",
    "condition": "amount > 5000",
    "action": "ALERT",
    "priority": "HIGH",
    "active": true
  }'
```

## Dependencies

- spring boot 3.5.6
- spring data jpa
- spring security
- spring boot actuator
- postgresql driver
- redis
- prometheus & grafana
- graylog, elasticsearch & mongodb
- spring ai with ollama
- djl (deep java library)
- lombok
- docker compose

## ai integration

The application integrates with AI models for:
- Fraud pattern detection
- Anomaly detection in transactions
- Risk scoring and assessment
- Natural language processing for transaction descriptions

## monitoring & logging

- **prometheus**: metrics collection and storage
- **grafana**: visualization and dashboards
- **graylog**: centralized log management
- **elasticsearch**: log indexing and search
- **mongodb**: log data storage

## contributing

1. fork the repository
2. create a feature branch
3. submit a pull request

## license

ISC — see [LICENSE](LICENSE) for details.

## EOF
