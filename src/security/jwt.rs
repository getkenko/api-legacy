use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO: move it to a config file
const DURATION: Duration = Duration::days(21);
const ALGORITHM: Algorithm = Algorithm::ES256;
// TODO: read keys at the runtime instead of embedding them into the binary
const PRIVATE_KEY: &[u8] = include_bytes!("../../keys/private.pem");
const PUBLIC_KEY: &[u8] = include_bytes!("../../keys/public.pem");

type Result<T> = jsonwebtoken::errors::Result<T>;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub sub: Uuid,
    pub exp: i64,
    pub display_name: String,
    pub email: String,
}

impl Token {
    pub fn new(user_id: Uuid, display_name: String, email: String) -> Self {
        Self {
            sub: user_id,
            exp: (Utc::now() + DURATION).timestamp(),
            display_name,
            email,
        }
    }

    pub fn encode(&self) -> Result<String> {
        let key = EncodingKey::from_ec_pem(PRIVATE_KEY)?;
        let token = jsonwebtoken::encode(&Header::new(ALGORITHM), &self, &key)?;
        Ok(token)
    }

    pub fn decode(token: &str) -> Result<Option<Self>> {
        let key = DecodingKey::from_ec_pem(PUBLIC_KEY)?;
        let token = jsonwebtoken::decode(token, &key, &Validation::new(ALGORITHM)).ok();
        Ok(token.map(|t| t.claims))
    }
}
