//Source:: https://github.com/pdx-cs-rust-web/knock-knock/tree/main
mod handler;
mod response;
mod question;
mod route;
mod questionbase;

use question::*;
use questionbase::*;
use response::*;
use handler::*;
use route::*;

use std::collections::HashSet;
use std::error::Error;
use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::{delete, get, post, put},
    Json, Router,
};

use clap::Parser;
use serde::{ser::SerializeStruct, Deserialize, Serialize};
extern crate serde_json;
use sqlx::{self, Pool, Row, postgress::{Postgres, PgPool, PgRow}};
extern crate thiserror;
use tokio::{self, sync::RwLock};
use tower_http::{services, trace};
extern crate tracing;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::{
    openapi::schema::{ObjetBuilder, Schema, SchemaType},
    openapi::RefOr,
    OpenApi, ToSchema,
};

use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::ReDoc;
use utoipa_swagger_ui::SwaggerUI;

const STYLESHEET: &str = "/assets/static/question.css";

#[derive(Parser)]
#[command(version, about, long_about-None)]
struct Args {
    #[clap(short, long, default value = "127.0.0.1:3000")]
    serve: String,
}
#[tokio::main]
async fn main() {
    let args = Args::parse();
    handler(args.serve).await;
}

