# MySqlRust
# Actix Web Application

This is a simple Actix web application that demonstrates the usage of Actix web framework for building a web server in Rust.

## Features

- Manages a list of managers and employees.
- Utilizes Actix web for handling HTTP requests.
- Implements a basic in-memory database.

## Prerequisites

Before running the application, ensure you have the following installed:

- Rust: [Installation Guide](https://www.rust-lang.org/tools/install)
- Cargo: Rust's package manager, included with the Rust installation.

## Getting Started

1. Clone the repository:

    ```bash
    git clone git@github.com:saiprasad-coditas/MySqlRust.git
    ```

2. Change into the project directory:

    ```bash
    cd employeePortal
    ```

3. Build and run the application:

    ```bash
    cargo run
    ```

    The server will start on `http://127.0.0.1:8080/`.

## Usage

- The API is accessible at `http://127.0.0.1:8080/api`.
- The application initializes an in-memory database and creates tables on startup.

## Endpoints

- `/api/managers`: Retrieve a list of managers.
- `/api/employees`: Retrieve a list of employees.

## Dependencies

- [Actix web](https://actix.rs/): A powerful, pragmatic, and extremely fast web framework for Rust.
- [SQLite](https://www.sqlite.org/): A C library that provides a lightweight disk-based database.

1. Configure MySQL Connection:
   
    Modify the connection parameters in `Database::new()` method of `database.rs` with your MySQL server details.

2. Build and run the application:

    ```bash
    cargo run
    ```

    The server will start on `http://127.0.0.1:8080/`.

## Database Initialization

- The application initializes a MySQL database and creates tables on startup.
- If the specified database (`employeeprotal`) does not exist, it will be created.

## Database Schema

The application uses two tables:

1. **Manager Table:**
   - `id` (INT): Primary key with auto-increment.
   - `name` (VARCHAR): Manager's name.
   - `role` (VARCHAR): Manager's role.

2. **Employee Table:**
   - `id` (INT): Primary key with auto-increment.
   - `name` (VARCHAR): Employee's name.
   - `role` (VARCHAR): Employee's role.
   - `manager_id` (INT): Foreign key referencing the `id` column of the Manager table.

## API Endpoints

- `/api/managers`: Retrieve a list of managers.
- `/api/employees`: Retrieve a list of employees.

# Actix Web Application for Employee Management

This Rust Actix web application provides RESTful API endpoints to manage managers and employees. It uses in-memory storage for simplicity.

## Features

- CRUD operations for managers and employees.
- Utilizes Actix web for handling HTTP requests.
- Demonstrates basic usage of asynchronous Rust.

### Manager Endpoints

- **Add Manager:** `POST /manager`
- **Get Managers:** `GET /manager`
- **Update Manager:** `PUT /manager/{id}`
- **Delete Manager:** `DELETE /manager/{id}`

### Employee Endpoints

- **Add Employee:** `POST /employee`
- **Get Employees:** `GET /employee`
- **Update Employee:** `PUT /employee/{id}`
- **Delete Employee:** `DELETE /employee/{id}`

## Data Structures

### Manager

```json
{
  "id": 1,
  "name": "John Doe",
  "role": "Manager"
}

{
  "id": 1,
  "name": "Jane Doe",
  "role": "Developer",
  "manager_id": 1
}
```

