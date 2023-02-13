use utils::core::cache;
use core::option::Option;
use tracing::info;

const USER_ID: &str = "user_id ";
const USER_TOKEN: &str = "user_token ";

pub async fn cache_refresh_user_data(id: u32, token: &str) {
    // 1. check current token in cache
    let user_id = &format!("{}{}", USER_ID, id);
    match cache::get_key(&user_id[..]).await {
        Some(stored_token) => {
            if stored_token.ne(&token[..]) {
                // 2. delete old values and set ne values
                let stored_token = &format!("{}{}", USER_TOKEN, stored_token);
                assert_eq!(1, cache_delete_data(user_id).await.expect("Deleted 1 record"));
                assert_eq!(1, cache_delete_data(&stored_token).await.expect("Deleted 1 record"));
                // 3. set new values
                cache_set_data(id, &token[..]).await;
            }
        },
        None => {
            cache_set_data(id, &token[..]).await;
        }
    }
}

async fn cache_set_data(id: u32, token: &str) {
    let prepared_id = &format!("{}{}", USER_ID, id);
    let prepared_token = &format!("{}{}", USER_TOKEN, token);

    cache::set_key_value(&prepared_id, &token).await;
    cache::set_key_value(&prepared_token, &id.to_string()[..]).await;
}

pub async fn get_cached_user_token_by_id(id: u32) -> Option<String> {
    let key = &format!("{}{}", USER_ID, id);
    get_cached_value_by_key(&key[..]).await
}

pub async fn get_cached_user_id_by_token(token: &str) -> Option<u32> {
    let key = &format!("{}{}", USER_TOKEN, token);
    match get_cached_value_by_key(&key[..]).await {
        Some(data) => {
            Some(data.parse::<u32>().unwrap())
        },
        None => None
    }
}

async fn get_cached_value_by_key(key: &str) -> Option<String> {
    cache::get_key(key).await
    // let token_opt:Option<String> =  match cache::get_key(key).await {
    //     Some(data) => {
    //         for (index, word) in data.split_whitespace().enumerate() {
    //             if index == 1 {
    //                 return Some(word.to_owned());
    //             }
    //         }
    //         return None;
    //     },
    //     None => None
    // };
    // token_opt
}

async fn cache_delete_data(key: &str) -> Option<i32> {
     cache::del_key(key).await
}