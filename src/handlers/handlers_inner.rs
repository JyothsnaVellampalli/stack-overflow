use crate::{
    models::{Answer, AnswerDetail, AnswerId, DBError, Question, QuestionDetail, QuestionId},
    persistence::{answers_dao::AnswersDao, questions_dao::QuestionsDao},
};

#[derive(Debug)]
pub enum HandlerError {
  BadRequest(String),
  InternalError(String),
}

impl HandlerError {
  pub fn default_internal_error() -> Self {
    HandlerError::InternalError("Something went wrong! Please try again.".to_owned())
  }
}

pub async fn create_question(
  question: Question,
  questions_dao: &(dyn QuestionsDao + Send + Sync),
) -> Result<QuestionDetail, HandlerError> {
  let question = questions_dao.create_question(question).await.map_err(|e| {
    match e {
      DBError::InvalidUUID(e) => HandlerError::BadRequest(e),
      DBError::Other(e) => HandlerError::InternalError(e.to_string()),
    }
  })?;

  Ok(question)
}

pub async fn read_questions(questions_dao: &(dyn QuestionsDao + Send + Sync)) -> Result<Vec<QuestionDetail>, HandlerError> {
  let questions = questions_dao.get_questions().await.map_err(|e| {
    match e {
      DBError::InvalidUUID(e) => HandlerError::BadRequest(e),
      DBError::Other(e) => HandlerError::InternalError(e.to_string()),
    }
  })?;

  Ok(questions)
}

pub async fn delete_question(
  question_id: QuestionId,
  questions_dao: &(dyn QuestionsDao + Send + Sync),
) -> Result<(), HandlerError> {
  questions_dao.delete_question(question_id.question_uuid).await.map_err(|e| {
    match e {
      DBError::InvalidUUID(e) => HandlerError::BadRequest(e),
      DBError::Other(e) => HandlerError::InternalError(e.to_string()),
    }
  })?;

  Ok(())
}

pub async fn create_answer(
  answer: Answer,
  answers_dao: &(dyn AnswersDao + Sync + Send),
) -> Result<AnswerDetail, HandlerError> {
  let answer = answers_dao.create_answer(answer).await.map_err(|e| {
    match e {
      DBError::InvalidUUID(e) => HandlerError::BadRequest(e),
      DBError::Other(e) => HandlerError::InternalError(e.to_string()),
    }
  })?;

  Ok(answer)
}

pub async fn read_answers(
  question_id: QuestionId,
  answers_dao: &(dyn AnswersDao + Sync + Send),
) -> Result<Vec<AnswerDetail>, HandlerError> {
  let answers = answers_dao.get_answers(question_id.question_uuid).await.map_err(|e| {
    match e {
      DBError::InvalidUUID(e) => HandlerError::BadRequest(e),
      DBError::Other(e) => HandlerError::InternalError(e.to_string()),
    }
  })?;

  Ok(answers)
}

pub async fn delete_answer(answer_id: AnswerId, answers_dao: &(dyn AnswersDao + Sync + Send)) -> Result<(), HandlerError> {
  answers_dao.delete_answer(answer_id.answer_uuid).await.map_err(|e| {
    match e {
      DBError::InvalidUUID(e) => HandlerError::BadRequest(e),
      DBError::Other(e) => HandlerError::InternalError(e.to_string()),
    }
  })?;

  Ok(())
}