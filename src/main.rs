use axum::{
    routing::{get, post, delete},
    Router
};
use dotenvy::dotenv;
use pretty_env_logger;
use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;

mod models;
mod handlers;
mod persistence;

use persistence::{
    questions_dao::{QuestionsDaoImpl, QuestionsDao},
    answers_dao::{AnswersDaoImpl, AnswersDao},
};

use handlers::*;

#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao + Sync + Send>,
    pub answers_dao: Arc<dyn AnswersDao + Sync + Send>,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    pretty_env_logger::init();
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5) // Set max connection pool size
        .connect(&std::env::var("DATABASE_URL").expect("Invalid database URL"))
        .await.expect("Unable to create postgres connection pool");

    let questions_dao = Arc::new(QuestionsDaoImpl::new(pool.clone()));
    let answers_dao = Arc::new(AnswersDaoImpl::new(pool));

    let app_state = AppState{
        questions_dao,
        answers_dao,
    };

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}