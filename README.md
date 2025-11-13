# Book Server

A simple HTTP server built with **Rust** and **Actix Web** to manage books.  
Supports creating books via POST requests and retrieving books via GET requests. Data is stored **in-memory**, making this a great project to learn Rust web development.

## Features

- **POST /books**: Create a new book  
  - JSON payload: `id`, `title`, `author`, `year`
- **GET /books**: Get all books  
- **GET /books/{id}**: Get a specific book by ID

---

## Technologies Used

- **Rust** (programming language)  
- **Actix Web** (web framework)  
- **Serde** (for JSON serialization/deserialization)  

---

## Getting Started

### Prerequisites

- Rust and Cargo installed ([Rust installation guide](https://www.rust-lang.org/tools/install))  
- WSL (if on Windows) or Linux/Mac environment  

---

### Installation

1. Clone the repository:

```
git clone https://github.com/stackedbyMwita/Book-Server.git
cd book_server
```
Build the project:
```
cargo build
```
Run the server:
```
cargo run
```
Server runs at http://127.0.0.1:8080

### Example Requests
**Create a book:**
**POST** /books
**Content-Type:** application/json
```
{
  "id": 1,
  "title": "The Rust Book",
  "author": "Steve",
  "year": 2023
}

```
**Get all books:**
**GET** /books
**Get a book by ID:**

**GET** /books/1

### Notes
- Data is stored in memory (Vec<Book>), so all books are lost when the server stops.
- For persistent storage, you can later integrate a JSON file or a database.

### License
- This project is open-source and free to use.
