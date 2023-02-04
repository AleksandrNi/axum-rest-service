use repository::domain::question::Question;
use repository::repository::question;
use utils::core::db::Tx;
use utils::core::db::TxAsync;
use utils::error::app_service_error::AppServiceError;

pub async fn get_questions() -> Result<Vec<Question>, AppServiceError> {
    let mut tx = Tx::begin().await;
    let questions = question::get_questions(&mut tx).await.unwrap();
        // .unwrap_or_else(|err| AppError::Service(AppServiceError::general_error(&err.to_string()[..])))
    Tx::commit(tx);
    Ok(questions)
}
