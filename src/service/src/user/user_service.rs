use repository::domain::user::UserModel;
use utils::core::hash::hash_password;
use utils::core::jwt::jwt_create;
use utils::error::app_error::AppGenericError;
use utils::error::app_service_error::AppServiceError;
use crate::user::user_dto::UserDto;
use repository::repository::user_repository;
use utils::core::db::Tx;
use utils::core::db::TxAsync;

const PASSWORD_PARAM: &str = "password";

pub async fn user_create(mut user_dto: UserDto) -> Result<UserDto, AppGenericError> {

    let hash = if let Some(password ) = user_dto.get_password(){
        hash_password(&password[..]).map_err(|err| AppServiceError::general_error(err))?
    } else {
        return Err(AppServiceError::parameter_must_be_provided(PASSWORD_PARAM.to_owned()));
    };

    let jwt_token = jwt_create();

    user_dto.password = Some(hash);
    user_dto.token = Some(jwt_token);
    user_dto.deleted_at = None;

    let mut tx = Tx::begin().await;
    match user_repository::user_create(&mut tx, UserModel::from(user_dto)).await {
        Ok(data) => {
            Tx::commit(tx).await;
            Ok(UserDto::from(data))
        },
        Err(err) => Err(err)
    }
}