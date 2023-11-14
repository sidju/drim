use super::*;

use serde::{Serialize, Deserialize};

pub async fn login_handler(
  state: &'static State,
  req: Request,
) -> Result<Response, Error> {
  // Create an authorization URL for this user
  let (authorize_url, csrf_state, nonce) = state.oidc_client
    .authorize_url(
      openidconnect::AuthenticationFlow::<openidconnect::core::CoreResponseType>::AuthorizationCode,
      openidconnect::CsrfToken::new_random,
      openidconnect::Nonce::new_random,
    )
    .add_scope(openidconnect::Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()))
    .url()
  ;
  // Save all the data we need to keep through the OIDC login process to DB
  sqlx::query!(
    "INSERT INTO Logins(StateID, Nonce, TargetPath, TargetQuery) VALUES($1, $2, $3, $4)",
    csrf_state.secret(),
    nonce.secret(),
    // Save some data about the current request, to return the user here after
    &req.uri().path(),
    req.uri().query(), // Returns Option, therefore nullable in the database
  )
    .execute(&state.db)
    .await
  ?;

  // Redirect the user to that url
  redirect(authorize_url.as_str())
}

#[derive(Deserialize, Serialize)]
pub struct PostLoginQueryData{
  code: String,
  state: String,
}
#[derive(sqlx::FromRow)]
pub struct OIDCPostLoginData {
  nonce: String,
  targetpath: String,
  targetquery: Option<String>,
}

pub async fn post_login_handler(
  state: &'static State,
  req: Request,
) -> Result<Response, Error> {
  // TODO:
  //   - Parse out the "code" and "state" fields of the parameters
  //   - use them to get an auth token from google


  // The query parameters should contain everything we need to know
  // Parse out "code" and "state" parameters
  let oidc_response: PostLoginQueryData = parse_query(&req)?;
  // Get all the data about this login from the database, also deleting it
  let oidc_data = sqlx::query_as!(OIDCPostLoginData,
    "SELECT Nonce, TargetPath, TargetQuery from Logins WHERE StateID = $1",
    oidc_response.state,
  )
    .fetch_optional(&state.db)
    .await
    // Open result to get to Option. Make Some into Ok and None into this error
    ?.ok_or(ClientError::UnknownOIDCProcess)
  ?;

  // If the state was valid we have validated againts CSRF and can request the
  // real code from our OIDC provider
  let code = openidconnect::AuthorizationCode::new(oidc_response.code);
  let token_response = state.oidc_client
    .exchange_code(code)
    .request_async(openidconnect::reqwest::async_http_client)
    .await
  ?;

  // Verify this response using the nonce from the DB
  let id_token_verifier = state.oidc_client.id_token_verifier();
  let nonce = openidconnect::Nonce::new(oidc_data.nonce);
  let id_token_claims = token_response
    .extra_fields()
    .id_token()
    .ok_or(ClientError::OIDCGaveNoToken)?
    .claims(&id_token_verifier, &nonce)? // Errors if auth chain has been tampered with
  ;
  
  // With all that done we can start using it by saving the id_token to a cookie

  json(&oidc_response.state)
}
