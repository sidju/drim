// Needed imports
use crate::Reply;
use hyper::header::HeaderValue;
use hyper::{
  Body,
  Request,
  Response,
  StatusCode,
};
use serde::Serialize;
// Public errors to wrap
use hyper::header::ToStrError as UnreadableHeaderError;
use serde_json::Error as JsonError;
use serde_urlencoded::de::Error as UrlEncodingError;
use std::num::ParseIntError;
// Private errors to wrap
use hyper::Error as ConnectionError;
use hyper::header::InvalidHeaderValue;
use sqlx::error::Error as SqlxError;
use openidconnect::ClaimsVerificationError as OIDCClaimsVerificationError;

type OIDCRequestError = openidconnect::RequestTokenError<
  openidconnect::reqwest::Error<reqwest::Error>,
  openidconnect::StandardErrorResponse<openidconnect::core::CoreErrorResponseType>
>;

// Error representation for internal errors
// Prints to stderr and returns a http 500 internal error
#[derive(Debug)]
pub enum InternalError {
  Connection(ConnectionError),
  InvalidHeader(InvalidHeaderValue),
  Db(SqlxError),
  OIDCRequestError(OIDCRequestError),
}
impl Reply for InternalError {
  fn into_response(self) -> Response<Body> {
    eprintln!("{:?}", &self);
    // By using a constant instance of ClientError formatting is consistent
    ClientError::InternalError.into_response()
  }
}

#[derive(Debug, Serialize)]
pub enum ClientError {
  InternalError,

  // Routing errors
  PathNotFound(String),
  MethodNotFound(String),
  Unauthorized,
  Forbidden,

  // Parsing errors
  PathDataBeforeRoot(String),
  UnreadableHeader(String),
  InvalidContentLength(String),
  InvalidContentType(String),
  InvalidJson(String),
  InvalidUrlEncoding(String),
  InvalidIndexPath(String),

  // Non-parsing user errors
  UnknownOIDCProcess, // Post-login OIDC handler did not find the OIDC login in DB
  OIDCGaveNoToken, // Unlikely, would probably be error in OIDC provider
  TamperedOIDCLogin, // Validation using the Nonce after getting token failed
}
impl Reply for ClientError {
  fn into_response(self) -> Response<Body> {
    let mut re = Response::new(
      serde_json::to_string(&self)
        .unwrap() // Only errors if self cannot be represented as json
        .into()
    );
    *re.status_mut() = match self {
      Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,

      Self::PathNotFound(_) => StatusCode::NOT_FOUND,
      Self::MethodNotFound(_) => StatusCode::METHOD_NOT_ALLOWED,
      Self::Unauthorized => StatusCode::UNAUTHORIZED,
      Self::Forbidden => StatusCode::FORBIDDEN,

      // All the remaining should be bad request
      _ => StatusCode::BAD_REQUEST
    };
    re.headers_mut().insert(
      "Content-Type",
      HeaderValue::from_static("application/json; charset=utf-8"),
    );
    re
  }
}

// Enum over both internal and client error
// allows us treating both consistently
#[derive(Debug)]
pub enum Error {
  InternalError(InternalError),
  ClientError(ClientError),
}
// Utility constructors
// We use .into() to convert ClientError into Error
impl Error {
  pub fn path_data_before_root(data: String) -> Self {
    ClientError::PathDataBeforeRoot(data).into()
  }
  pub fn path_not_found(req: &Request<Body>) -> Self {
    ClientError::PathNotFound(req.uri().path().to_owned()).into()
  }
  pub fn method_not_found(req: &Request<Body>) -> Self {
    ClientError::MethodNotFound(req.method().to_string()).into()
  }
  pub fn unauthorized() -> Self {
    ClientError::Unauthorized.into()
  }
  pub fn forbidden() -> Self {
    ClientError::Forbidden.into()
  }
  // Where multiple parsing errors give the same error type
  // we need to use a function for one of the cases
  pub fn unreadable_header(e: UnreadableHeaderError, header: &str) -> Self {
    ClientError::UnreadableHeader(format!(
      "Error reading header {}: {}",
      header, e,
    )).into()
  }

  pub fn content_length_missing() -> Self {
    ClientError::InvalidContentLength(
      "No content length given".to_string()
    ).into()
  }
  pub fn content_length_not_int(err: ParseIntError) -> Self {
    ClientError::InvalidContentLength(format!(
      "Invalid unsigned int: {}",
      err,
    )).into()
  }
  pub fn content_length_too_large(parsed: usize, max: usize) -> Self {
    ClientError::InvalidContentLength(format!(
      "Too large. Maximum allowed is {}, received {}",
      max, parsed,
    )).into()
  }
  pub fn content_length_mismatch(given: usize, promised: usize) -> Self {
    let at_least = if given > promised {" at least"} else {""};
    ClientError::InvalidContentLength(format!(
      "Mismatch. Header is {}, received {} {}",
      promised, at_least, given,
    )).into()
  }
  pub fn invalid_content_type(parsed: &str, expected: &str) -> Self {
    ClientError::InvalidContentType(format!(
      "Expected {}, received {}",
      parsed, expected
    )).into()
  }
}

// Implementing Reply on this error type enables rust to convert any error into
// the correct response to the client (with a print to stderr for internal).
impl Reply for Error {
  fn into_response(self) -> Response<Body> {
    match self {
      Self::InternalError(e) => e.into_response(),
      Self::ClientError(e) => e.into_response(),
    }
  }
}

// Implementing these allows '?' and .into() to convert them all into our Error
impl From<InternalError> for Error {
  fn from(e: InternalError) -> Self {
    Self::InternalError(e)
  }
}
impl From<ClientError> for Error {
  fn from(e: ClientError) -> Self {
    Self::ClientError(e)
  }
}

impl From<JsonError> for Error {
  fn from(e: JsonError) -> Self {
    ClientError::InvalidJson(format!("{}", e)).into()
  }
}
impl From<UrlEncodingError> for Error {
  fn from(e: UrlEncodingError) -> Self {
    ClientError::InvalidUrlEncoding(format!("{}", e)).into()
  }
}
impl From<ParseIntError> for Error {
  fn from(e: ParseIntError) -> Self {
    ClientError::InvalidIndexPath(format!("{}", e)).into()
  }
}
impl From<SqlxError> for Error {
  fn from(e: SqlxError) -> Self {
    InternalError::Db(e).into()
  }
}
impl From<ConnectionError> for Error {
  fn from(e: ConnectionError) -> Self {
    InternalError::Connection(e).into()
  }
}
// most likely created by an invalid redirect
impl From<InvalidHeaderValue> for Error {
  fn from(e: InvalidHeaderValue) -> Self {
    InternalError::InvalidHeader(e).into()
  }
}
impl From<OIDCClaimsVerificationError> for Error {
  fn from(e: OIDCClaimsVerificationError) -> Self {
    ClientError::TamperedOIDCLogin.into()
  }
}
impl From<OIDCRequestError> for Error {
  fn from(e: OIDCRequestError) -> Self {
    InternalError::OIDCRequestError(e).into()
  }
}
