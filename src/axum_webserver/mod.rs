use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
mod templates;

pub async fn start_webserver(context: Arc<serenity::http::Http>) -> Result<() , String>{
    // build our application with a single route
    let app = Router::new()
    .route("/", get(templates::root)).with_state(context)
    
    /* DOWNLOAD FUNCTIONALITY
    .route("/archivo", get(templates::get_archivo))
     */
    /* SENDS MESSAGE USING BOT
    .route("/bot", get(templates::get_bot).post(templates::post_bot))
    */
    ;

    let app = app.fallback(templates::fallback);

    // run it with hyper on localhost:3000
    if let Err(x) = axum::Server::bind(&"192.168.1.169:3000".parse().unwrap()).serve(app.into_make_service()).await {
        return Err(format!("{:?}" , x))
    } else {
        return Ok(())
    }

}
