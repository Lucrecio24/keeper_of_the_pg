use axum::{
    routing::get,
    Router,
};
mod templates;

async fn start_webserver() {
    // build our application with a single route
    let app = Router::new()
    .route("/", get(templates::root))
    .route("/archivo", get(templates::get_archivo));
    /* SENDS MESSAGE USING BOT
    .route("/bot", get(templates::get_bot).post(templates::post_bot))*/
    ;

    let app = app.fallback(templates::fallback);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"192.168.1.169:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
