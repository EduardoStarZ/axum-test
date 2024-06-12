use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation, decode};
use axum::{http::{StatusCode, HeaderMap}, Json};
use crate::model::{Claims, LoginInfo, LoginResponse};

pub async fn login_handler(Json(login_info) : Json<LoginInfo>) -> Result<Json<LoginResponse>, StatusCode> {                               
    let username : &String = &login_info.username;

    let password : &String = &login_info.password;

    if is_valid(username, password) {
        let claims : Claims = Claims { sub : username.clone(), expire: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize};
        let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())) {                               
            
            Ok(token) => token,
            Err(e) => { 
                eprintln!("Error generating token: {e}");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        return Ok::<Json<LoginResponse>, StatusCode>(Json(LoginResponse{token}));
    };
    return Err(StatusCode::UNAUTHORIZED);
}

pub async fn get_info_handler(header_map : HeaderMap) -> Result<Json<String>, StatusCode> {
    if let Some(auth_header) = header_map.get("Authorization") {
        if let Ok(auth_header_str) = auth_header.to_str() {
            if auth_header_str.starts_with("Bearer ") {
               let token : String = auth_header_str.trim_start_matches("Bearer ").to_string(); 
                
               match decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()) {
                    Ok(_) => {
                        let info : String = String::from("You are valid, here is the information");
                        return Ok(Json(info));
                    },
                    Err(e) => {
                        eprintln!("Error generating token: {e}");
                        return Err::<Json<String>, StatusCode>(StatusCode::UNAUTHORIZED);
                    }
               }
            }
        }
    }

    return Err(StatusCode::UNAUTHORIZED);
}

fn is_valid(username : &str, password : &str) -> bool {
    username != "" && password != ""
}
