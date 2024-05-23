use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
mod templates;

pub async fn start_webserver(context: Arc<serenity::http::Http>) -> Result<() , String>{
    // build our application with a single route
    let app = Router::new()
    .route("/", get(templates::root))
    
    /* DOWNLOAD FUNCTIONALITY
    .route("/archivo", get(templates::get_archivo))
     */
    // SENDS MESSAGE USING BOT
    .route("/sendmessage", get(templates::get_bot).post(templates::send_message_to_channel_handler))
    .route("/test", get(templates::get_test))
    ;

    let app = app.fallback(templates::fallback);

    let app = app.with_state(context);
    let listener = tokio::net::TcpListener::bind("192.168.1.169:3000").await.unwrap();

    if let Err(x) = axum::serve(listener, app).await{
        return Err(x.to_string())
    }
    println!("Hi");
    return Ok(())
}
