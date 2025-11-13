use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Book struct
#[derive(Serialize, Deserialize, Clone)]
struct Book {
    id: u32,
    title: String,
    author: String,
    year: u32,
}

// App state to hold books in memory
struct AppState {
    books: Mutex<Vec<Book>>,
}

// POST: create a book
async fn create_book(book: web::Json<Book>, data: web::Data<AppState>) -> impl Responder {
    let mut books = data.books.lock().unwrap();
    books.push(book.into_inner());
    HttpResponse::Created().json(books.last())
}

// GET: get all books
async fn get_books(data: web::Data<AppState>) -> impl Responder {
    let books = data.books.lock().unwrap();
    HttpResponse::Ok().json(&*books)
}

// GET: get book by id
async fn get_book(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let books = data.books.lock().unwrap();
    let id = *path; // copy the u32
    if let Some(book) = books.iter().find(|b| b.id == id) {
        HttpResponse::Ok().json(book)
    } else {
        HttpResponse::NotFound().body("Book not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        books: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/books", web::post().to(create_book))
            .route("/books", web::get().to(get_books))
            .route("/books/{id}", web::get().to(get_book))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
