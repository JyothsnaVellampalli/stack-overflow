use sqlx::PgPool;
use async_trait::async_trait;

use crate::models::{Question, QuestionDetail, DBError};

#[async_trait]
pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self {
            db,
        }
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
  async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
    let record = sqlx::query!(
       r#"
        INSERT INTO questions (title, description)
        VALUES ($1, $2)
        RETURNING *
        "#,
        question.title,
        question.description
    ).fetch_one(&self.db).await.map_err(|e| DBError::Other(Box::new(e)))?;

     Ok(QuestionDetail {
      question_uuid: record.question_uuid.to_string(),
      title: record.title,
      description: record.description,
      created_at: record.created_at.to_string(),
    })
  }

  async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
    let records = sqlx::query!(
      r#"
      SELECT * FROM questions
      "#
    ).fetch_all(&self.db).await.map_err(|e| DBError::Other(Box::new(e)))?;

    Ok(
      records.iter().map(|r| {
        QuestionDetail {
          question_uuid: r.question_uuid.to_string(),
          title: r.title.clone(),
          description: r.description.clone(),
          created_at: r.created_at.to_string(),
        }
      }).collect()
    )
  }

  async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
    let uuid: sqlx::types::Uuid = sqlx::types::Uuid::parse_str(&question_uuid).map_err(|e| DBError::InvalidUUID(format!("{} {}", question_uuid, e.to_string())))?;
    let _ = sqlx::query!(
      r#"
      DELETE FROM questions
      WHERE question_uuid = $1
      "#,
      uuid
    ).execute(&self.db).await.map_err(|e| DBError::Other(Box::new(e)))?;

    Ok(())
  }
}