//use axum::extract::Form;
use axum::{
    extract::{State , Form},
    http::StatusCode,
    response::{IntoResponse , Html},
};
use hyper::Response;
use tokio_util::io::ReaderStream;
use tokio::fs::File;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FormData {
    channel_id: u64,
    message: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadForm {
    filename: String
}

pub async fn root() -> Html<&'static str> {Html(std::include_str!("index.html"))}

////BOT PAGES
pub async fn get_bot() -> Html<&'static str> {Html(std::include_str!("bot.html"))}
pub async fn get_test() -> Html<&'static str> {Html(std::include_str!("test.html"))}
pub async fn get_download() -> Html<&'static str> {Html(std::include_str!("download.html"))}

#[axum_macros::debug_handler]
pub async fn post_download(State(_context): State<Arc<serenity::http::Http>> , form: Form<DownloadForm>) -> impl IntoResponse {
    // Getting data from form received from post
    let data = form.0;

    // Process the form data
    let file_name: String = data.filename;
    let file = match File::open(format!("download/{}" , file_name)).await {
        Ok(file) => file,
        Err(err) => {
            log::debug!("Error at finding file\n{}" , err);
            return Err((StatusCode::NOT_FOUND, format!("File {} found: {}", file_name , err)))
        }
    };
    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = axum::body::Body::from_stream(stream);

    let response = Response::builder()
        .header("Content_type", "application/x-download")
        .header("Content-Disposition", format!("attachment; filename=\"{}\"", file_name))
        .body(body)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to build response".to_string()))?;

    Ok(response)
}

#[axum_macros::debug_handler]
pub async fn send_message_to_channel_handler(State(context): State<Arc<serenity::http::Http>> , form: Form<FormData>) -> impl IntoResponse {
    // Getting data from form received from post
    let data = form.0;

    // Process the form data
    let channel_id: u64 = data.channel_id;
    let message = &data.message;

    //Sending message via the entered channel id, if channel id is valid
    if let Err(_) = serenity::model::id::ChannelId::from(channel_id).say(context , format!("{}" , message)).await{
        println!("Couldn't send message to channel_id = {}" , channel_id);
    }

    Html(format!(
        "<h1>Form Data Received</h1><p>Channel ID: {}</p><p>Message: {}</p>",
        channel_id, message
    ))
}
pub async fn fallback() -> &'static str {"ERROR 404: Page not found"}