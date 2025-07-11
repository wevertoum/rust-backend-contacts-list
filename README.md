# 📞 Rust Backend: Contacts & Users API

A RESTful backend written in **Rust** using [`axum`](https://www.google.com/search?q=%5Bhttps://github.com/tokio-rs/axum%5D\(https://github.com/tokio-rs/axum\)), **SeaORM** for ORM, and **PostgreSQL** for database persistence.

[![wakatime](https://wakatime.com/badge/user/23a27b6b-815f-4cef-8a6c-21bdf1f9c3ed/project/e5404b78-1a50-47d8-a1a1-c8c03287850e.svg)](https://wakatime.com/badge/user/23a27b6b-815f-4cef-8a6c-21bdf1f9c3ed/project/e5404b78-1a50-47d8-a1a1-c8c03287850e)

-----

## 📁 Project Structure

```text
rust-backend-contacts-list/
├── .env                  # Environment variables (DB credentials, etc.)
├── .gitignore            # Git ignored files
├── Cargo.toml            # Rust dependencies
├── Cargo.lock            # Dependency lock file
├── client_test.http      # REST Client requests for testing
├── schema.sql            # SQL schema for PostgreSQL (users + contacts)
├── src/                  # Application source code
│   ├── main.rs           # App entry point
│   ├── db.rs             # DB connection logic
│   ├── handlers/         # Business logic
│   │   ├── contacts.rs
│   │   ├── users.rs
│   │   └── mod.rs
│   ├── models/           # Data models (entities / DTOs)
│   │   ├── contact.rs
│   │   ├── user.rs
│   │   └── mod.rs
│   └── routes/           # HTTP routes for Axum
│       ├── contacts.rs
│       ├── users.rs
│       └── mod.rs
```

-----

## 🧪 API Endpoints

### **Users (`/users`)**

Endpoints to manage the main resource: **Users**. Creating a user also creates their associated contact.

#### ➕ Create a User and their Contact

```http
POST http://localhost:3000/users
Content-Type: application/json

{
  "name": "Weverton Rodrigues",
  "genre": "Male",
  "email": "wevertondev@outlook.com"
}
```

#### 📄 List all Users

```http
GET http://localhost:3000/users
Accept: application/json
```

#### 🔍 Get a specific User (with their Contact)

```http
GET http://localhost:3000/users/{id}
Accept: application/json
```

#### ✏️ Update a User (and/or their Contact)

```http
PUT http://localhost:3000/users/{id}
Content-Type: application/json

{
  "name": "Weverton R. Silva",
  "email": "weverton.new@outlook.com"
}
```

#### ❌ Delete a User

```http
DELETE http://localhost:3000/users/{id}
Accept: application/json
```

-----

### **Contacts (`/contacts`)**

Secondary endpoints for direct **Contact** management.

#### 📄 List all Contacts

```http
GET http://localhost:3000/contacts
Accept: application/json
```

#### 🔍 Get a specific Contact by ID

```http
GET http://localhost:3000/contacts/{id}
Accept: application/json
```

#### ✏️ Update a Contact's Email

```http
PUT http://localhost:3000/contacts/{id}
Content-Type: application/json

{
  "email": "johndoe.updated@testmail.com"
}
```

#### ❌ Delete a specific Contact

```http
DELETE http://localhost:3000/contacts/{id}
Accept: application/json
```

-----

## 🔗 Relationship: User & Contact

  * The API is modeled with a **1-to-1 relationship** between `User` and `Contact`.
  * A `Contact` **must belong** to a `User` and cannot exist independently.
  * **`ON DELETE CASCADE`**: Thanks to this rule in the database, when deleting a `User` (using `DELETE /users/{id}`), their associated `Contact` is **automatically removed**, ensuring data consistency.

-----

## 🛠 Technologies

  * **Rust** with [axum](https://github.com/tokio-rs/axum)
  * [SeaORM](https://www.sea-ql.org/SeaORM/) – async ORM
  * **PostgreSQL**
  * [tokio](https://tokio.rs/) – async runtime
  * [tower-http](https://docs.rs/tower-http/latest/tower_http/) – middleware (CORS, etc.)
  * `.env` + [dotenvy](https://docs.rs/dotenvy/) – environment management

-----

## 🚀 Getting Started

1.  Clone the repo

2.  Create a `.env` file with your DB URL (example below)

3.  Run PostgreSQL and create the DB

4.  Run the SQL schema:

    ```bash
    psql postgres://{usr}:{pwd}@localhost:5432/contacts_db -f schema.sql
    ```

5.  Start the server:

    ```bash
    cargo run
    ```

-----

## 📄 Example `.env`

```env
DATABASE_URL=postgres://{usr}:{pwd}@localhost:5432/contacts_db
```

-----

## ✨ Author

[Weverton Rodrigues](https://github.com/wevertoum)