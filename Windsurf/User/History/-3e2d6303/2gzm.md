<h1 align="center">CRM Service</h1>
<p align="center" >
  <img alt="Java" src="https://img.shields.io/badge/Java-ED8B00?logo=openjdk&logoColor=white">
  <img alt="Spring Boot" src="https://img.shields.io/badge/Spring%20Boot-6DB33F?logo=spring-boot&logoColor=white">
  <img alt="PostgreSQL" src="https://img.shields.io/badge/PostgreSQL-4169E1?logo=postgresql&logoColor=white">
  <img alt="React" src="https://img.shields.io/badge/React-61DAFB?logo=react&logoColor=white">
  <img alt="TypeScript" src="https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=white">
  <img alt="Status" src="https://img.shields.io/badge/status-beta-yellow">
  <img alt="License" src="https://img.shields.io/badge/license-ISC-blue">
</p>

<br>

**CRM Service** is a comprehensive customer relationship management platform powered by Spring Boot and React, featuring project management, task tracking, team collaboration, and secure authentication.

## Features

- User authentication and authorization with JWT
- Project and task management system
- Team collaboration and member management
- Real-time chat functionality
- Password reset and email notifications
- PostgreSQL database with JPA
- Spring Security integration
- RESTful API design
- React frontend with TypeScript
- Ant Design UI components
- Docker Compose support

## Installation

### Prerequisites:

- Java 17 
- Maven 3.6+
- PostgreSQL database
- Node.js 18+
- Docker & Docker Compose 

### Backend Setup:

```bash
git clone git@github.com:jvmdevelop/crm-service.git
cd crm-service/backend
mvn clean install
mvn spring-boot:run
```

### Frontend Setup:

```bash
cd crm-service/frontend
npm install
npm run dev
```

### With Docker Compose:

```bash
cd crm-service
docker-compose up
```

## Usage

### Configuration

Configure your `application.properties`:

```properties
spring.datasource.url=jdbc:postgresql://localhost:5432/crm_service
spring.datasource.username=your_username
spring.datasource.password=your_password
spring.jpa.hibernate.ddl-auto=update
```

### Running the application:

**Backend:**
```bash
cd backend
mvn spring-boot:run
```

**Frontend:**
```bash
cd frontend
npm run dev
```

The application will be available at:
- Backend API: `http://localhost:8080`
- Frontend: `http://localhost:5173`

## API Endpoints

### Authentication

| Endpoint | Method | Description |
|:---------|:------:|:------------|
| `/api/auth/register` | POST | Register new user |
| `/api/auth/login` | POST | User login |
| `/api/auth/me` | GET | Get current user |
| `/api/auth/reset-password/initiate` | POST | Initiate password reset |
| `/api/auth/reset-password` | POST | Reset password |

### Project Management

| Endpoint | Method | Description |
|:---------|:------:|:------------|
| `/api/projects` | GET | get all projects |
| `/api/projects` | POST | create new project |
| `/api/projects/{id}` | PUT | update project |
| `/api/projects/{id}` | DELETE | delete project |

### Task Management

| Endpoint | Method | Description |
|:---------|:------:|:------------|
| `/api/tasks` | GET | get all tasks |
| `/api/tasks` | POST | create new task |
| `/api/tasks/{id}` | PUT | update task |
| `/api/tasks/{id}` | DELETE | delete task |

## Project Structure

- `backend/` - Spring Boot backend application
  - `src/main/java/com/mono/` - Main package
  - `controllers/` - REST API controllers
  - `service/` - Business logic services
  - `models/` - JPA entities
  - `repository/` - Repository interfaces
  - `security/` - Security configuration
  - `dto/` - Data transfer objects
- `frontend/` - React frontend application
  - `src/` - Source code
  - `src/components/` - React components
  - `src/pages/` - Page components
  - `src/store/` - Redux store

## Examples

Register a new user:

```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username": "john_doe", "email": "john@example.com", "password": "securePassword"}'
```

Create a new project:

```bash
curl -X POST http://localhost:8080/api/projects \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{"name": "New Project", "description": "Project description"}'
```

Create a new task:

```bash
curl -X POST http://localhost:8080/api/tasks \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{"title": "Complete setup", "description": "Initial setup task", "projectId": 1}'
```

## Dependencies

### Backend:
- Spring Boot 3.3.4
- Spring Security
- Spring Data JPA
- Spring Mail
- PostgreSQL Driver
- JWT (jjwt)
- Lombok
- SpringDoc OpenAPI
- Swagger UI

### Frontend:
- React 18.3.1
- TypeScript
- Ant Design
- Redux Toolkit
- React Router
- Axios
- Tailwind CSS
- Vite

## Security Features

- JWT-based authentication
- Password encryption
- Role-based access control
- Email verification
- Password reset functionality
- CORS configuration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Submit a pull request

## License

ISC — see [LICENSE](LICENSE) for details.

## EOF
