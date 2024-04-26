mod api;
mod joke;
mod jokebase;
mod web;

use std::net::SocketAddr;
/*
use api::*;
use joke::*;
use jokebase::*;
use web::*;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{ErrorKind, Seek, Write};
use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post, put},
    Json, Router,
};
*/

#[tokio::main]
async fn main() {
    let ip = SocketAddr::new([127, 0, 0, 1].into(), 3000);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    tracing::debug!("serving {}", listener.local_addr().unwrap());
    axum::serve(listener, web::app()).await.unwrap();
}
