//use axum::extract::Form;
use axum::{
    body::StreamBody,
    http::{header, StatusCode},
    response::{IntoResponse , Html},
};
use hyper::Response;
use tokio_util::io::ReaderStream;
use tokio::fs::File;


pub async fn root() -> Html<&'static str> {Html(std::include_str!("index.html"))}

////BOT PAGES
pub async fn get_bot() -> Html<&'static str> {Html(std::include_str!("bot.html"))}
//POST (DEPRECATED, SENDS MESSAGE VIA REDIS WITH INTENDED BOT MESSAGE.)
/*pub async fn post_bot(form: Form<FormData>) -> impl IntoResponse {
    let data = form.0;

    // Process the form data
    let channel_id: &u64 = &data.channel_id;
    let message = &data.message;

    let data_pack = to_vec(&data).unwrap();
    // Connect to Redis
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await.unwrap();

    // Publish the message to the Redis channel
    let _: () = con.publish("send_message", data_pack).await.unwrap();

    // Create an HTML response with the processed data
    Html(format!(
        "<h1>Form Data Received</h1><p>Channel ID: {}</p><p>Message: {}</p>",
        channel_id, message
    ))
}*/
pub async fn fallback() -> &'static str {"ERROR 404: Toty not found"}


pub async fn get_archivo() -> Result<Response<StreamBody<ReaderStream<File>>> , (StatusCode, String)> {
    // `File` implements `AsyncRead`
    let file = match File::open("wakfu_ui_skins.zip").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    let header = [
        (header::CONTENT_TYPE , "application/zip"),
        (header::CONTENT_DISPOSITION , format!("attachment; filename=\"{}\"", "wakfu_ui_skins.zip").as_str()),
    ];

    let response = Response::builder()
        .header("content_type", "application/zip")
        .header(
            "content_disposition",
            format!("attachment; filename=\"{}\"", "wakfu_ui_skins.zip"),
        )
        .body(body)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to build response".to_string()))?;
        
    Ok(response)
}