use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct Question {
  pub title: String,
  pub description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QuestionDetail {
  pub question_uuid: String,
  pub title: String,
  pub description: String,
  pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct QuestionId {
  pub question_uuid: String,
} 

#[derive(Serialize, Deserialize)]
pub struct Answer {
  pub question_uuid: String,
  pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct AnswerDetail {
  pub answer_uuid: String,
  pub question_uuid: String,
  pub content: String,
  pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct AnswerId {
  pub answer_uuid: String,
}

#[derive(Debug, Error)]
pub enum DBError {
  #[error("Invalid UUID: {0}")]
  InvalidUUID(String),
  #[error("Postgres error")]
  Other(#[from] Box<dyn std::error::Error>),
}

// source: https://www.postgresql.org/docs/current/errcodes-appendix.html
pub mod postgres_error_codes {
    pub const FOREIGN_KEY_VIOLATION: &str = "23503";
    pub const UNIQUE_KEY_VIOLATION: &str = "23505";
}
