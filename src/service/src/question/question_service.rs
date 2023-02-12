use repository::domain::question::QuestionModel;
use repository::repository::question_repository;
use utils::core::db::Tx;
use utils::core::db::TxAsync;
use utils::error::app_error::AppGenericError;
use utils::error::app_service_error::AppServiceError;
use crate::question::question_dto::QuestionDto;


const ID_PARAM: &str = "id";
const TITLE_PARAM: &str = "title";
const CONTENT_PARAM: &str = "content";

pub async fn get_questions() -> Result<Vec<QuestionDto>, AppGenericError> {
    let mut tx = Tx::begin().await;
    match question_repository::get_questions(&mut tx).await {
        Ok(data) => {
            Tx::commit(tx).await;
            let result: Vec<QuestionDto> = data.into_iter()
                .map(|model| QuestionDto::from(model))
                .collect();
            Ok(result)
        }
        Err(err) => Err(err)
    }
}

pub async fn post_question(question: QuestionDto) -> Result<QuestionDto, AppGenericError> {
    let mut tx = Tx::begin().await;
    let model = QuestionModel::from(question);
    match question_repository::post_question(&mut tx, model).await {
        Ok(data) => {
            Tx::commit(tx).await;
            Ok(QuestionDto::from(data))
        }
        Err(err) => Err(err)
    }
}

pub async fn get_question_by_id(
    id: i32
) -> Result<QuestionDto, AppGenericError> {
    let mut tx = Tx::begin().await;
    match question_repository::get_question_by_id(&mut tx, id).await {
        Ok(data) => {
            Tx::commit(tx).await;
            Ok(QuestionDto::from(data))
        }
        Err(err) => Err(err)
    }
}

pub async fn delete_question_by_id(id: i32) -> Result<bool, AppGenericError> {
    let mut tx = Tx::begin().await;

    if let Err(_err) = question_repository::get_question_by_id(&mut tx, id).await {
        return Err(AppServiceError::entity_does_not_exists_for_param_value(ID_PARAM.to_owned(), id.to_string()));
    }

    match question_repository::delete_question_by_id(&mut tx, id).await {
        Ok(_data) => {
            Tx::commit(tx).await;
            Ok(true)
        }
        Err(err) => Err(err)
    }
}

pub async fn patch_question(question: QuestionDto) -> Result<QuestionDto, AppGenericError> {
    let mut tx = Tx::begin().await;

    if let None = question.get_id() {
        return Err(AppServiceError::parameter_must_be_provided(ID_PARAM.to_owned()))
    }

    let mut stored_task = question_repository::get_question_by_id(&mut tx, question.get_id().unwrap().unwrap()).await
        .map_err(|_err| AppServiceError::entity_does_not_exists_for_param_value(ID_PARAM.to_owned(), question.get_id().unwrap().unwrap().to_string()))?;

    if let Some(title) = question.get_title() {
        if let None = title {
            return Err(AppServiceError::parameter_must_be_provided(TITLE_PARAM.to_owned()));
        }
        stored_task.set_title(title.as_ref().unwrap().to_owned());
    }
    if let Some(content) = question.get_content() {
        if let None = content {
            return Err(AppServiceError::parameter_must_be_provided(CONTENT_PARAM.to_owned()));
        }
        stored_task.set_content((content.as_ref().unwrap()).to_owned());
    }
    if let Some(tags) = question.get_tags() {
        stored_task.set_tags(tags.to_owned());
    }

    match question_repository::patch_question(&mut tx, stored_task).await {
        Ok(data) => {
            Tx::commit(tx).await;
            Ok(QuestionDto::from(data))
        }
        Err(err) => Err(err)
    }
}