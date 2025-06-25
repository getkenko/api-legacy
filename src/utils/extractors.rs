// use axum::{extract::FromRequestParts, http::request::Parts, response::Response};
// use sqlx::PgPool;

// use crate::routes::AppState;

// struct Database(PgPool);

// impl FromRequestParts<AppState> for Database {
//     type Rejection = Response;

//     async fn from_request_parts(_parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
//         let db = state.db.clone();
//         todo!()
//     }
// }