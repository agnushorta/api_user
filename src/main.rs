
use async_graphql::{Schema, EmptyMutation, EmptySubscription};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{Router,routing::{post}};
use crate::db::DB;
use crate::query_engine::Query;

mod db;
mod user_service;
mod query_engine;

async fn graphql_handler(graphql_request: GraphQLRequest) -> GraphQLResponse {
    let query = Query { db: DB };

    let schema = Schema::new(
        query,
        EmptyMutation,
        EmptySubscription
    );

    let res = schema.execute(graphql_request
        .into_inner())
        .await;

    res.into()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/gql", post(graphql_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000") 
        .await
        .unwrap();

    println!("Listening ...!");
    axum::serve(listener, app)
        .await
        .unwrap()
}
