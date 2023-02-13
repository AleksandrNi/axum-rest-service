use repository::domain::user::UserModel;
use utils::core::hash::{hash_password, verify_password};
use utils::core::jwt::jwt_create;
use utils::error::app_error::AppGenericError;
use utils::error::app_service_error::AppServiceError;
use crate::user::user_dto::UserDto;
use repository::repository::user_repository;
use utils::core::db::Tx;
use utils::core::db::TxAsync;

const PASSWORD_PARAM: &str = "password";
const EMAIL_PARAM: &str = "email";

pub async fn user_create(mut user_dto: UserDto) -> Result<UserDto, AppGenericError> {
    let hash = if let Some(password) = user_dto.get_password() {
        hash_password(&password[..]).map_err(|err| AppServiceError::general_error(err))?
    } else {
        return Err(AppServiceError::parameter_must_be_provided(PASSWORD_PARAM.to_owned()));
    };

    user_dto.password = Some(hash);
    user_dto.deleted_at = None;

    let mut tx = Tx::begin().await;
    match user_repository::user_create(&mut tx, UserModel::from(user_dto)).await {
        Ok(data) => {
            Tx::commit(tx).await;
            Ok(UserDto::from(data))
        }
        Err(err) => Err(err)
    }
}

pub async fn user_login(user_dto: UserDto) -> Result<UserDto, AppGenericError> {
    let passed_password = if let Some(passed_password) = &user_dto.password {
        passed_password.to_owned()
    } else {
        return Err(AppServiceError::parameter_must_be_provided(PASSWORD_PARAM.to_owned()));
    };

    if let None = &user_dto.email {
        return Err(AppServiceError::parameter_must_be_provided(EMAIL_PARAM.to_owned()));
    }

    let mut tx = Tx::begin().await;
    let mut loaded_user_dto = match user_repository::user_load_by_email(&mut tx, UserModel::from(user_dto)).await {
        Ok(data) => {
            // Tx::commit(tx).await;
            Ok(UserDto::from(data))
        }
        Err(err) => Err(err)
    }.unwrap();

    let stored_password_hash = &loaded_user_dto.password.unwrap()[..];

    if !verify_password(passed_password.clone(), stored_password_hash).unwrap() {
        return Err(AppServiceError::relation_between_2_parameters_does_not_exist(EMAIL_PARAM.to_owned(), PASSWORD_PARAM.to_owned()));
    }

    loaded_user_dto.password = Some(stored_password_hash.to_owned());
    let updated_user = match user_repository::user_update(&mut tx, UserModel::from(loaded_user_dto)).await {
        Ok(data) => {
            Tx::commit(tx).await;
            Ok(UserDto::from(data))
        }
        Err(err) => Err(err)
    }.unwrap();

    Ok(updated_user)
}

pub async fn user_load_by_email(email: String) -> Result<UserDto, AppGenericError> {
    let mut tx = Tx::begin().await;
    let user_dto = UserDto {
        id: None,
        name: None,
        email: Some(email),
        password: None,
        deleted_at: None,
    };
    match user_repository::user_load_by_email(&mut tx, UserModel::from(user_dto)).await {
        Ok(data) => {
            Tx::commit(tx).await;
            Ok(UserDto::from(data))
        }
        Err(err) => Err(err)
    }
}

pub async fn user_load_by_id(id: u32) -> Result<UserDto, AppGenericError> {
    let mut tx = Tx::begin().await;
    let user_dto = UserDto {
        id: Some(id),
        name: None,
        email: None,
        password: None,
        deleted_at: None,
    };
    match user_repository::user_load_by_id(&mut tx, UserModel::from(user_dto)).await {
        Ok(data) => {
            Tx::commit(tx).await;
            Ok(UserDto::from(data))
        }
        Err(err) => Err(err)
    }
}