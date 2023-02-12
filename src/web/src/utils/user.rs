use tracing::info;
use utils::core::cache;
use core::option::Option;

const USER_ID: &str = "user_id ";
const TOKEN_ID: &str = "token ";

pub async fn cache_user_data(id: u32, token: &str) {
    // 1. check current token in cache
    info!("cache_user_data {} {}", id, &token);
    let key = &format!("{}{}", USER_ID, id);

    match cache::get_key(&key[..]).await {
        Some(stored_token) => {
            if stored_token.ne(&token[..]) {
                store_data(id, &token[..]).await;
            }
        },
        None => {
            store_data(id, &token[..]).await;
        }
    }
}

async fn store_data(id: u32, token: &str) {
    let prepared_id = &format!("{}{}", USER_ID, id);
    let prepared_token = &format!("{}{}", TOKEN_ID, token);

    cache::set_key_value(&prepared_id, &prepared_token).await;
    cache::set_key_value(&prepared_token, &prepared_id).await;
}

pub async fn get_cached_user_data_by_id(id: u32) -> Option<String> {
    let key = &format!("{}{}", USER_ID, id);
    let token_opt:Option<String> =  match cache::get_key(key).await {
        Some(data) => {
            for (index, word) in data.split_whitespace().enumerate() {
                if index == 1 {
                    return Some(word.to_owned());
                }
            }
            return None;
        },
        None => None
    };
    token_opt
}

// pub async fn get_cached_user_data_by_token(token: &str) -> String {
//     let key = &format!("{}{}", TOKEN_ID, token);
//     let value = cache::get_key(key).await;
//     for (index, word) in value.split_whitespace().enumerate() {
//         if index == 1 {
//             return word.to_owned()
//         }
//     }
//     return String::new()
// }