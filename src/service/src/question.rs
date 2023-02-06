use repository::domain::question::Question;
use repository::repository::question;
use utils::core::db::Tx;
use utils::core::db::TxAsync;
use utils::error::app_service_error::AppServiceError;

pub async fn get_questions() -> Result<Vec<Question>, AppServiceError> {
    let mut tx = Tx::begin().await;
    let questions = question::get_questions(&mut tx).await.unwrap();
    Tx::commit(tx);
    Ok(questions)
}

pub async fn post_question(question: Question) -> Result<Question, AppServiceError> {
    let mut tx = Tx::begin().await;
    let stored_question = question::post_question(&mut tx, question).await.unwrap();
    Tx::commit(tx);
    println!("stored_question = {:?}", stored_question);
    Ok(stored_question)
}

pub async fn get_question_by_id(
    id: i32
) -> Result<Question, AppServiceError> {
    let mut tx = Tx::begin().await;
    let question = question::get_question_by_id(&mut tx, id).await.unwrap();
    Tx::commit(tx);
    Ok(question)
}
