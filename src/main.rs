use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use self::models::File;
use axum::{
    extract::{DefaultBodyLimit, Multipart, Query},
    http::{header, HeaderName, StatusCode},
    response::Html,
    routing::{get, post},
    Json, Router,
};
use flashies::*;
use diesel::prelude::*;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let connection = &mut establish_connection();
    run_migrations(connection).unwrap();

    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(|| async { "OK" }))
        .route("/check", get(check_file_hash))
        .route("/get", get(get_file))
        .route("/all", get(get_all))
        .route("/post", post(post_file))
        .layer(CorsLayer::permissive())
        .layer(DefaultBodyLimit::max(20971520));

    // run it with hyper on localhost:3000
    println!("Ready.");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

// checks if a file with the given hash exists in the db
async fn check_file_hash(Query(sha): Query<HashMap<String, String>>) -> (StatusCode, &'static str) {
    use self::schema::files::dsl::*;

    let connection = &mut establish_connection();
    let r = files
        .filter(hash.eq(sha.values().next()))
        .limit(1)
        .select(File::as_select())
        .first(connection);

    if r.is_ok() {
        (StatusCode::OK, "in db")
    } else {
        (StatusCode::NOT_FOUND, "not in db")
    }
}

// returns the file with the given hash in the db
async fn get_file(
    Query(sha): Query<HashMap<String, String>>,
) -> Result<(StatusCode, [(HeaderName, String); 1], Vec<u8>), StatusCode> {
    use self::schema::files::dsl::*;

    let connection = &mut establish_connection();
    let r = files
        .filter(hash.eq(sha.values().next()))
        .limit(1)
        .select(File::as_select())
        .first(connection);

    r.map(|x| {
        Ok((
            StatusCode::OK,
            [(
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", x.name.unwrap()),
            )],
            x.data.to_owned().unwrap().clone(),
        ))
    })
    .unwrap_or(Err(StatusCode::NOT_FOUND))
}

// posts a file to the db, returns the hash
async fn post_file(mut multipart: Multipart) -> StatusCode {
    let connection = &mut establish_connection();

    let (mut name, mut hash, mut data) = Default::default();
    while let Some(f) = multipart.next_field().await.unwrap() {
        match f.name().unwrap() {
            "name" => name = f.text().await.unwrap(),
            "hash" => hash = f.text().await.unwrap(),
            "data" => data = f.bytes().await.unwrap().to_vec(),
            _ => (),
        }
    }

    // assume the sha has been checked
    match create_file(connection, &name, &hash, &data) {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

#[derive(Deserialize, Serialize)]
struct UploadFile {
    name: String,
    hash: String,
    data: Vec<u8>,
}

async fn get_all() -> Result<Json<(usize, Vec<AllFile>)>, StatusCode> {
    use self::schema::files::dsl::*;

    let connection = &mut establish_connection();

    let r = files.load::<File>(connection);

    match r {
        Ok(x) => Ok(Json((
            x.len(),
            x.iter()
                .map(|f| AllFile {
                    name: f.name.to_owned().unwrap(),
                    hash: f.hash.to_owned().unwrap(),
                })
                .collect(),
        ))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Serialize)]
struct AllFile {
    name: String,
    hash: String,
}
