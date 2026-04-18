<h1 align="center">amv</h1>
<p align="center" >
  <img alt="Java" src="https://img.shields.io/badge/Java-ED8B00?logo=openjdk&logoColor=white">
  <img alt="Spring Boot" src="https://img.shields.io/badge/Spring%20Boot-6DB33F?logo=spring-boot&logoColor=white">
  <img alt="PostgreSQL" src="https://img.shields.io/badge/PostgreSQL-4169E1?logo=postgresql&logoColor=white">
  <img alt="Redis" src="https://img.shields.io/badge/Redis-DC382D?logo=redis&logoColor=white">
  <img alt="Status" src="https://img.shields.io/badge/status-beta-yellow">
  <img alt="License" src="https://img.shields.io/badge/license-ISC-blue">
</p>

<br>

**amv** is a modern web application built with Spring Boot, featuring user authentication, news management, and real-time WebSocket communication capabilities.

## Features

- jwt-based authentication system with Spring Security
- user registration and login functionality
- news management system with CRUD operations
- real-time websocket support
- postgresql database with JPA
- redis for session management and caching
- restful API design
- spring modulith for modular architecture
- maven build system

## Installation

### Prerequisites:

- Java 17 
- Maven 3.6+
- PostgreSQL database
- Redis server
- Docker & Docker Compose (optional)

### From source:

```bash
git clone git@github.com:jvmdevelop/amv.git
cd amv
mvn clean install
mvn spring-boot:run
```

### With Docker Compose:

```bash
cd amv
docker-compose up
```

## Usage

### Configuration

Configure your `application.properties`:

```properties
spring.datasource.url=jdbc:postgresql://localhost:5432/amv
spring.datasource.username=your_username
spring.datasource.password=your_password
spring.data.redis.host=localhost
spring.data.redis.port=6379
```

### Running the application:

```bash
mvn spring-boot:run
```

The application will be available at `http://localhost:8080` 

## API Endpoints

### Authentication

| Endpoint | Method | Description |
|:---------|:------:|:------------|
| `/api/v1/public/auth/login` | POST | user login |
| `/api/v1/public/auth/auth` | POST | user registration |
| `/api/v1/public/auth/me` | GET | get current user info |

### News Management

| Endpoint | Method | Description |
|:---------|:------:|:------------|
| `/api/news` | GET | get all news |
| `/api/news` | POST | create news article |
| `/api/news/{id}` | PUT | update news article |
| `/api/news/{id}` | DELETE | delete news article |

## WebSocket Endpoints

- `/ws` - real-time communication interface

## Project Structure

- `src/main/java/com/jvmd/amv/` - Main package
  - `AmvApplication.java` - Main application class
  - `controllers/` - REST API controllers
  - `service/` - Business logic services
  - `model/` - JPA entities and models
  - `repository/` - Repository interfaces
  - `config/` - Configuration classes
  - `filters/` - Security filters
  - `util/` - Utility classes
- `src/main/resources/` - Configuration files
- `src/test/` - Test classes

## Examples

Login to the application:

```bash
curl -X POST http://localhost:8080/api/v1/public/auth/login \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=your_username&password=your_password"
```

Register a new user:

```bash
curl -X POST http://localhost:8080/api/v1/public/auth/auth \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=newuser&password=password123&email=user@example.com"
```

Create a news article:

```bash
curl -X POST http://localhost:8080/api/news \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{"title": "Breaking News", "content": "News content here"}'
```

## Dependencies

- spring boot 3.4.5
- spring security
- spring data jpa
- spring websocket
- spring modulith
- postgresql driver
- redis
- jwt 
- lombok
- maven

## Security

The application uses:
- jwt tokens for authentication
- spring security for authorization
- role-based access control
- httpOnly cookies for token storage

## Contributing

1. fork the repository
2. create a feature branch
3. submit a pull request

## License

ISC — see [LICENSE](LICENSE) for details.

## EOF
