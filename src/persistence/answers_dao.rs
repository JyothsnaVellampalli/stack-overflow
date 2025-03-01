use sqlx::PgPool;
use async_trait::async_trait;

use crate::models::{Answer, AnswerDetail, DBError, postgres_error_codes};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
  pub fn new(db: PgPool) -> Self {
      Self {
          db,
      }
  }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
  async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
    let uuid = sqlx::types::Uuid::parse_str(&answer.question_uuid).map_err(|e| DBError::InvalidUUID(e.to_string()))?;

    let record = sqlx::query!(
      r#"
        INSERT INTO answers (question_uuid, content)
        VALUES ($1, $2)
        RETURNING *
      "#,
      uuid,
      answer.content
    ).fetch_one(&self.db).await.map_err(|e| {
      match e {
        sqlx::Error::Database(e) => {
          if let Some(code) = e.code() {
            if code.eq(postgres_error_codes::FOREIGN_KEY_VIOLATION) {
              return DBError::InvalidUUID(format!("Invalid question uuid {}", answer.question_uuid));
            }
          }
          DBError::Other(Box::new(e))
        },
        _ => DBError::Other(Box::new(e))
      }
    })?;

    Ok(
      AnswerDetail {
        answer_uuid: record.answer_uuid.to_string(),
        question_uuid: record.question_uuid.to_string(),
        content: record.content,
        created_at: record.created_at.to_string(),
      }
    )
  }

  async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
    let uuid = sqlx::types::Uuid::parse_str(&question_uuid).map_err(|e| DBError::InvalidUUID(e.to_string()))?;

    let records = sqlx::query!(
      r#"
        SELECT * FROM answers WHERE question_uuid = $1
      "#, uuid
    ).fetch_all(&self.db).await.map_err(|e| DBError::Other(Box::new(e)))?;

    Ok(
      records.iter().map(|r| {
        AnswerDetail {
          answer_uuid: r.answer_uuid.to_string(),
          question_uuid: r.question_uuid.to_string(),
          content: r.content.clone(),
          created_at: r.created_at.to_string(),
        }
      }).collect()
    )
  }

  async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
    let uuid = sqlx::types::Uuid::parse_str(&answer_uuid).map_err(|e| DBError::InvalidUUID(e.to_string()))?;

    let _ = sqlx::query!(
      r#"
        DELETE FROM answers
        WHERE question_uuid = $1
      "#,
      uuid
    ).execute(&self.db).await.map_err(|e| {
      DBError::Other(Box::new(e))
    })?;

    Ok(())
  }
}