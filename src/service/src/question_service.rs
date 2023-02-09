use repository::domain::question::QuestionModel;
use repository::repository::question_repository;
use utils::core::db::Tx;
use utils::core::db::TxAsync;
use utils::error::app_error::AppGenericError;
use utils::error::app_service_error::AppServiceError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QuestionDto {
    id: Option<i32>,
    title: Option<String>,
    content: Option<String>,
    tags: Option<Vec<String>>,
}


impl QuestionDto {
    pub fn new(id: Option::<i32>, title: Option<String>, content: Option<String>, tags: &Option<Vec<String>>) -> Self {
        QuestionDto {
            id,
            title,
            content,
            tags: tags.to_owned(),
        }
    }
}

impl QuestionDto {
    pub fn from_model(model: QuestionModel) -> Self {
        QuestionDto::new(*model.get_id(), Some((*model.get_title()).to_string()), Some((*model.get_content()).to_string()), model.get_tags())
    }
}

impl QuestionDto {
    pub fn get_id(&self) -> &Option<i32> { &self.id }
    pub fn get_title(&self) -> &Option<String> {
        &self.title
    }
    pub fn get_content(&self) -> &Option<String> {
        &self.content
    }
    pub fn get_tags(&self) -> &Option<Vec<String>> {
        &self.tags
    }
}

pub async fn get_questions() -> Result<Vec<QuestionDto>, AppGenericError> {
    let mut tx = Tx::begin().await;
    match question_repository::get_questions(&mut tx).await {
        Ok(data) => {
            Tx::commit(tx).await;
            let result: Vec<QuestionDto> = data.into_iter()
                .map(|model| QuestionDto::from_model(model))
                .collect();
            Ok(result)
        }
        Err(boxErr) => Err(boxErr)
    }
}

pub async fn post_question(question: QuestionDto) -> Result<QuestionDto, AppGenericError> {
    let mut tx = Tx::begin().await;
    let model = QuestionModel::new(*question.get_id(),
                                   question.get_title().as_ref().unwrap().to_owned(),
                                   question.get_content().as_ref().unwrap().to_owned(),
                                   question.get_tags().to_owned());
    match question_repository::post_question(&mut tx, model).await {
        Ok(data) => {
            Tx::commit(tx).await;
            Ok(QuestionDto::from_model(data))
        }
        Err(boxErr) => Err(boxErr)
    }
}

pub async fn get_question_by_id(
    id: i32
) -> Result<QuestionDto, AppGenericError> {
    let mut tx = Tx::begin().await;
    match question_repository::get_question_by_id(&mut tx, id).await {
        Ok(data) => {
            Tx::commit(tx).await;
            Ok(QuestionDto::from_model(data))
        }
        Err(boxErr) => Err(boxErr)
    }
}

pub async fn delete_question_by_id(id: i32) -> Result<bool, AppGenericError> {
    let mut tx = Tx::begin().await;

    if let Err(err) = question_repository::get_question_by_id(&mut tx, id).await {
        return Err(AppServiceError::entity_does_not_exists_for_param_value("id".to_owned(), id.to_string()));
    }

    match question_repository::delete_question_by_id(&mut tx, id).await {
        Ok(data) => {
            Tx::commit(tx).await;
            Ok(true)
        }
        Err(boxErr) => Err(boxErr)
    }
}

pub async fn patch_question(question: QuestionDto) -> Result<QuestionDto, AppGenericError> {
    let mut tx = Tx::begin().await;

    let mut stored_task = question_repository::get_question_by_id(&mut tx, question.get_id().unwrap()).await
        .map_err(|_err| AppServiceError::entity_does_not_exists_for_param_value("id".to_owned(), question.get_id().unwrap().to_string()))
        .unwrap();

    if let Some(title) = question.get_title() {
        stored_task.set_title(title.to_owned());
    }
    if let Some(content) = question.get_content() {
        stored_task.set_content(content.to_owned());
    }
    if let Some(tags) = question.get_tags() {
        stored_task.set_tags(tags.to_owned());
    }

    match question_repository::patch_question(&mut tx, stored_task).await {
        Ok(data) => {
            Tx::commit(tx).await;
            Ok(QuestionDto::from_model(data))
        }
        Err(boxErr) => Err(boxErr)
    }
}