use poem::{Route, Server, get, handler, listener::TcpListener, web::Path};

#[handler]
async fn index(Path(name): Path<String>) -> String {
    format!("Server running")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/", get(index));

    println!("Serving at http://localhost:3000");
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
