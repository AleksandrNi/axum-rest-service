use repository::domain::question::Question;
use repository::repository::question_repository;
use utils::core::db::Tx;
use utils::core::db::TxAsync;
use utils::error::app_error::AppGenericError;
use utils::error::app_repository_error::AppRepositoryError;

pub async fn get_questions() -> Result<Vec<Question>, AppGenericError> {
    let mut tx = Tx::begin().await;
    match question_repository::get_questions(&mut tx).await {
        Ok(data) => {
            Tx::commit(tx);
            Ok(data)
        }
        Err(boxErr) => Err(boxErr)
    }
}

pub async fn post_question(question: Question) -> Result<Question, AppGenericError> {
    let mut tx = Tx::begin().await;
    match question_repository::post_question(&mut tx, question).await {
        Ok(data) => {
            Tx::commit(tx);
            Ok(data)
        }
        Err(boxErr) => Err(boxErr)
    }
}

pub async fn get_question_by_id(
    id: i32
) -> Result<Question, AppGenericError> {
    let mut tx = Tx::begin().await;
    match question_repository::get_question_by_id(&mut tx, id).await {
        Ok(data) => {
            Tx::commit(tx);
            Ok(data)
        },
        Err(boxErr) => Err(boxErr)
    }
}