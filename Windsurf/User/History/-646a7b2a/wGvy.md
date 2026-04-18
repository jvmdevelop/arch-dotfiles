<h1 align="center">mini-messenger</h1>
<p align="center" >
  <img alt="Java" src="https://img.shields.io/badge/Java-ED8B00?logo=openjdk&logoColor=white">
  <img alt="Spring Boot" src="https://img.shields.io/badge/Spring%20Boot-6DB33F?logo=spring-boot&logoColor=white">
  <img alt="PostgreSQL" src="https://img.shields.io/badge/PostgreSQL-4169E1?logo=postgresql&logoColor=white">
  <img alt="Redis" src="https://img.shields.io/badge/Redis-DC382D?logo=redis&logoColor=white">
  <img alt="Status" src="https://img.shields.io/badge/status-beta-yellow">
  <img alt="License" src="https://img.shields.io/badge/license-ISC-blue">
</p>

<br>

**mini-messenger** is a modern messaging application built with Spring Boot, featuring real-time chat, WebRTC video calling, and comprehensive user management with JWT authentication.

## features

- real-time messaging with WebSocket support
- webRTC video calling capabilities
- JWT-based authentication and authorization
- PostgreSQL database with JPA
- Redis caching for session management
- RESTful API design
- user management and chat room functionality
- message history and persistence

## installation

### prerequisites:

- Java 21
- Maven 3.6+
- PostgreSQL database
- Redis server

### from source:

```bash
git clone git@github.com:jvmdevelop/mini-messenger.git
cd mini-messenger
mvn clean install
mvn spring-boot:run
```

## usage

### configuration

configure your `application.properties`:

```properties
spring.datasource.url=jdbc:postgresql://localhost:5432/mini-messenger
spring.datasource.username=your_username
spring.datasource.password=your_password
spring.redis.host=localhost
spring.redis.port=6379
```

### running the application:

```bash
mvn spring-boot:run
```

The application will be available at `http://localhost:8080`

## api endpoints

### authentication

| endpoint | method | description |
|:---------|:------:|:------------|
| `/api/auth/register` | POST | user registration |
| `/api/auth/login` | POST | user login |
| `/api/auth/refresh` | POST | refresh JWT token |

### chat management

| endpoint | method | description |
|:---------|:------:|:------------|
| `/api/chat/create` | POST | create new chat |
| `/api/chat/add-user` | POST | add user to chat |
| `/api/chat/messages` | GET | get chat messages |
| `/api/chat/send` | POST | send message |

### user management

| endpoint | method | description |
|:---------|:------:|:------------|
| `/api/user/profile` | GET | get user profile |
| `/api/user/update` | PUT | update user profile |

## websocket endpoints

- `/ws/chat` - real-time chat messaging
- `/ws/webrtc` - webRTC video calling

## project structure

- `src/main/java/com/jvmdevelop/mini-messenger/` - main application package
- `controller/` - REST API controllers
- `service/` - business logic services
- `model/` - JPA entities
- `repo/` - repository interfaces
- `dto/` - data transfer objects
- `config/` - configuration classes
- `utils/` - utility classes

## examples

register a new user:

```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","email":"test@example.com","password":"password"}'
```

Create a chat:

```bash
curl -X POST http://localhost:8080/api/chat/create \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name":"Test Chat","type":"GROUP"}'
```

## dependencies

- Spring Boot 3.4.2
- Spring Security
- Spring Data JPA
- Spring WebSocket
- PostgreSQL Driver
- Redis
- JWT (jjwt)
- Kurento WebRTC
- Lombok

## contributing

1. fork the repository
2. create a feature branch
3. submit a pull request

## license

ISC — see [LICENSE](LICENSE) for details.

## EOF
