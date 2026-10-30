mod router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = router::router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Starting the application at http://localhost:3000/");
    axum::serve(listener, app).await?;

    Ok(())
}
