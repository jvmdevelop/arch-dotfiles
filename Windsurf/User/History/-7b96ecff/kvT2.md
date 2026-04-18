<h1 align="center">anidromvost</h1>
<p align="center" >
  <img alt="Java" src="https://img.shields.io/badge/Java-ED8B00?logo=openjdk&logoColor=white">
  <img alt="Spring Boot" src="https://img.shields.io/badge/Spring%20Boot-6DB33F?logo=spring-boot&logoColor=white">
  <img alt="PostgreSQL" src="https://img.shields.io/badge/PostgreSQL-4169E1?logo=postgresql&logoColor=white">
  <img alt="Redis" src="https://img.shields.io/badge/Redis-DC382D?logo=redis&logoColor=white">
  <img alt="Status" src="https://img.shields.io/badge/status-beta-yellow">
  <img alt="License" src="https://img.shields.io/badge/license-ISC-blue">
</p>

<br>

**anidromvost** is a modern web application built with Spring Boot, featuring user authentication, news management, and real-time WebSocket communication capabilities.

## Features

- JWT-based authentication system with Spring Security
- User registration and login functionality
- News management system with CRUD operations
- Real-time WebSocket support
- PostgreSQL database with JPA
- Redis for session management and caching
- RESTful API design
- Spring Modulith for modular architecture
- Maven build system

## Installation

### Prerequisites:

- Java 17 
- Maven 3.6+
- PostgreSQL database
- Redis server
- Docker & Docker Compose (optional)

### From source:

```bash
git clone git@github.com:jvmdevelop/anidromvost.git
cd anidromvost
mvn clean install
mvn spring-boot:run
```

### With Docker Compose:

```bash
cd anidromvost
docker-compose up
```

## Usage

### Configuration

Configure your `application.properties`:

```properties
spring.datasource.url=jdbc:postgresql://localhost:5432/anidromvost
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

- `src/main/java/com/jvmd/anidromvost/` - Main package
  - `AnidromvostApplication.java` - Main application class
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

- Spring Boot 3.4.5
- Spring Security
- Spring Data JPA
- Spring WebSocket
- Spring Modulith
- PostgreSQL Driver
- Redis
- JWT (jjwt)
- Lombok
- Maven

## Security

The application uses:
- JWT tokens for authentication
- Spring Security for authorization
- Role-based access control
- HttpOnly cookies for token storage

## Contributing

1. fork the repository
2. create a feature branch
3. submit a pull request

## License

ISC — see [LICENSE](LICENSE) for details.

## EOF
