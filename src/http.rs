use reqwest::{Client, Method, RequestBuilder, Response, header::HeaderMap};
use serde::Serialize;

use super::errors::NetworkError;

/// Minimal REST client wrapper built on top of `reqwest::Client`.
///
/// `base_url` and endpoint paths are normalized so callers can pass values with
/// or without leading/trailing slashes.
pub struct RestClient {
  pub base_url: String,
  pub client: Client,
}

impl RestClient {
  /// Creates a new client with a configured user-agent string.
  pub fn new(base_url: String, user_agent: String) -> Self {
    Self {
      base_url,
      client: Client::builder()
        .user_agent(user_agent)
        .build()
        .unwrap_or(Client::new()),
    }
  }

  fn request_with_headers(&self, request: RequestBuilder, headers: Option<HeaderMap>) -> RequestBuilder {
    if let Some(headers) = headers {
      request.headers(headers)
    } else {
      request
    }
  }

  fn build_url(&self, endpoint: &str) -> String {
    format!(
      "{base_url}/{endpoint}",
      base_url = self.base_url.trim_end_matches('/'),
      endpoint = endpoint.trim_start_matches('/')
    )
  }

  async fn send_request(
    &self,
    method: Method,
    endpoint: &str,
    headers: Option<HeaderMap>,
  ) -> Result<Response, NetworkError> {
    let url = self.build_url(endpoint);
    let request = self.request_with_headers(self.client.request(method, url), headers);

    Ok(request.send().await?)
  }

  async fn send_request_with_payload(
    &self,
    method: Method,
    endpoint: &str,
    headers: Option<HeaderMap>,
    payload: impl Serialize,
  ) -> Result<Response, NetworkError> {
    let url = self.build_url(endpoint);
    let request = self.request_with_headers(self.client.request(method, url).json(&payload), headers);

    Ok(request.send().await?)
  }

  /// Sends a `POST` request with a JSON payload.
  pub async fn post(
    &self,
    endpoint: &str,
    headers: Option<HeaderMap>,
    payload: impl Serialize,
  ) -> Result<Response, NetworkError> {
    self
      .send_request_with_payload(Method::POST, endpoint, headers, payload)
      .await
  }

  /// Sends a `PATCH` request with a JSON payload.
  pub async fn patch(
    &self,
    endpoint: &str,
    headers: Option<HeaderMap>,
    payload: impl Serialize,
  ) -> Result<Response, NetworkError> {
    self
      .send_request_with_payload(Method::PATCH, endpoint, headers, payload)
      .await
  }

  /// Sends a `PUT` request with a JSON payload.
  pub async fn put(
    &self,
    endpoint: &str,
    headers: Option<HeaderMap>,
    payload: impl Serialize,
  ) -> Result<Response, NetworkError> {
    self
      .send_request_with_payload(Method::PUT, endpoint, headers, payload)
      .await
  }

  /// Sends a `GET` request.
  pub async fn get(&self, endpoint: &str, headers: Option<HeaderMap>) -> Result<Response, NetworkError> {
    self.send_request(Method::GET, endpoint, headers).await
  }
}

#[cfg(test)]
mod tests {
  use super::RestClient;
  use httpmock::prelude::*;
  use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
  use serde_json::json;

  #[test]
  fn build_url_normalizes_leading_and_trailing_slashes() {
    let cases = [
      ("http://localhost:3000", "users", "http://localhost:3000/users"),
      ("http://localhost:3000/", "users", "http://localhost:3000/users"),
      ("http://localhost:3000", "/users", "http://localhost:3000/users"),
      ("http://localhost:3000/", "/users", "http://localhost:3000/users"),
      (
        "http://localhost:3000/",
        "users?limit=10",
        "http://localhost:3000/users?limit=10",
      ),
      ("http://localhost:3000", "", "http://localhost:3000/"),
    ];

    for (base_url, endpoint, expected) in cases {
      let client = RestClient::new(base_url.to_string(), "network-tests".to_string());
      assert_eq!(client.build_url(endpoint), expected);
    }
  }

  #[tokio::test]
  async fn post_sends_headers_and_json_body() {
    let server = MockServer::start();
    let post_mock = server.mock(|when, then| {
      when
        .method(POST)
        .path("/v1/items")
        .header("x-request-id", "abc-123")
        .json_body_obj(&json!({ "name": "widget" }));
      then.status(200);
    });

    let mut headers = HeaderMap::new();
    headers.insert(
      HeaderName::from_static("x-request-id"),
      HeaderValue::from_static("abc-123"),
    );

    let client = RestClient::new(server.base_url(), "network-tests".to_string());
    let response = client
      .post("/v1/items", Some(headers), json!({ "name": "widget" }))
      .await
      .expect("post request should succeed");

    assert_eq!(response.status(), 200);
    post_mock.assert();
  }

  #[tokio::test]
  async fn patch_sends_json_body() {
    let server = MockServer::start();
    let patch_mock = server.mock(|when, then| {
      when
        .method("PATCH")
        .path("/v1/items/1")
        .json_body_obj(&json!({ "name": "updated" }));
      then.status(200);
    });

    let client = RestClient::new(server.base_url(), "network-tests".to_string());
    let response = client
      .patch("v1/items/1", None, json!({ "name": "updated" }))
      .await
      .expect("patch request should succeed");

    assert_eq!(response.status(), 200);
    patch_mock.assert();
  }

  #[tokio::test]
  async fn put_sends_json_body() {
    let server = MockServer::start();
    let put_mock = server.mock(|when, then| {
      when
        .method(PUT)
        .path("/v1/items/2")
        .json_body_obj(&json!({ "name": "replacement" }));
      then.status(200);
    });

    let client = RestClient::new(format!("{}/", server.base_url()), "network-tests".to_string());
    let response = client
      .put("/v1/items/2", None, json!({ "name": "replacement" }))
      .await
      .expect("put request should succeed");

    assert_eq!(response.status(), 200);
    put_mock.assert();
  }

  #[tokio::test]
  async fn get_sends_query_parameters() {
    let server = MockServer::start();
    let get_mock = server.mock(|when, then| {
      when.method(GET).path("/v1/items").query_param("limit", "5");
      then.status(200);
    });

    let client = RestClient::new(server.base_url(), "network-tests".to_string());
    let response = client
      .get("v1/items?limit=5", None)
      .await
      .expect("get request should succeed");

    assert_eq!(response.status(), 200);
    get_mock.assert();
  }
}
