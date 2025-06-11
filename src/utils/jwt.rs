use chrono::{Duration, Utc};
use jsonwebtoken::{errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

// TODO: use asymmetric keys for encode/decode
// TODO: better way of encoding & decoding because the code is duplicated

const SECRET: &str = "chuj123";

pub trait AuthToken: Sized + Serialize + DeserializeOwned {
    const DURATION: Duration;

    fn encode(&self) -> Result<String, jsonwebtoken::errors::Error>;
    fn decode(token: &str) -> Result<Option<Self>, jsonwebtoken::errors::Error>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AccessToken {
    pub sub: Uuid,
    pub display_name: String,
    pub exp: i64,
}

impl AccessToken {
    pub fn new(user_id: &Uuid, display_name: &str) -> Self {
        Self {
            sub: *user_id,
            display_name: display_name.to_string(),
            exp: (Utc::now() + Self::DURATION).timestamp(),
        }
    }
}

impl AuthToken for AccessToken {
    const DURATION: Duration = Duration::minutes(5);
    
    fn encode(&self) -> Result<String, jsonwebtoken::errors::Error> {
        let key = EncodingKey::from_secret(SECRET.as_bytes());
        let token_str = jsonwebtoken::encode(&Header::default(), &self, &key)?;
        Ok(token_str)
    }
    
    fn decode(token: &str) -> Result<Option<Self>, jsonwebtoken::errors::Error> {
        let key = DecodingKey::from_secret(SECRET.as_bytes());
        let token = jsonwebtoken::decode(token, &key, &Validation::default());
        let claims = match token {
            Ok(t) => t.claims,
            Err(why) => {
                match why.kind() {
                    ErrorKind::MissingAlgorithm | ErrorKind::Base64(_) |
                    ErrorKind::Crypto(_) | ErrorKind::Utf8(_) |
                    ErrorKind::Json(_) => {
                        return Err(why);
                    }
                    _ => return Ok(None)
                }
            }
        };

        Ok(claims)
    }
}

#[derive(Serialize, Deserialize)]
pub struct RefreshToken {
    pub sub: Uuid,
    pub exp: i64,

}

impl RefreshToken {
    pub fn new(user_id: &Uuid) -> Self {
        Self {
            sub: *user_id,
            exp: (Utc::now() + Self::DURATION).timestamp(),
        }
    }
}

impl AuthToken for RefreshToken {
    const DURATION: Duration = Duration::days(7);
    
    fn encode(&self) -> Result<String, jsonwebtoken::errors::Error> {
        let key = EncodingKey::from_secret(SECRET.as_bytes());
        let token_str = jsonwebtoken::encode(&Header::default(), &self, &key)?;
        Ok(token_str)
    }
    
    fn decode(token: &str) -> Result<Option<Self>, jsonwebtoken::errors::Error> {
        let key = DecodingKey::from_secret(SECRET.as_bytes());
        let token = jsonwebtoken::decode(token, &key, &Validation::default());
        let claims = match token {
            Ok(t) => t.claims,
            Err(why) => {
                match why.kind() {
                    ErrorKind::MissingAlgorithm | ErrorKind::Base64(_) |
                    ErrorKind::Crypto(_) | ErrorKind::Utf8(_) |
                    ErrorKind::Json(_) => {
                        return Err(why);
                    }
                    _ => return Ok(None)
                }
            }
        };

        Ok(claims)
    }
}