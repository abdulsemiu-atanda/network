use reqwest::{Client, RequestBuilder, Response, header::HeaderMap};
use serde::Serialize;

use super::errors::NetworkError;

pub struct RestClient {
  pub base_url: String,
  pub client: Client,
}

impl RestClient {
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

  pub async fn post(
    &self,
    endpoint: &str,
    headers: Option<HeaderMap>,
    payload: impl Serialize,
  ) -> Result<Response, NetworkError> {
    let url = format!("{base_url}/{endpoint}", base_url = self.base_url);
    let request = self.request_with_headers(self.client.post(url).json(&payload), headers);

    Ok(request.send().await?)
  }

  pub async fn patch(
    &self,
    endpoint: &str,
    headers: Option<HeaderMap>,
    payload: impl Serialize,
  ) -> Result<Response, NetworkError> {
    let url = format!("{base_url}/{endpoint}", base_url = self.base_url);
    let request = self.request_with_headers(self.client.patch(url).json(&payload), headers);

    Ok(request.send().await?)
  }

  pub async fn put(
    &self,
    endpoint: &str,
    headers: Option<HeaderMap>,
    payload: impl Serialize,
  ) -> Result<Response, NetworkError> {
    let url = format!("{base_url}/{endpoint}", base_url = self.base_url);
    let request = self.request_with_headers(self.client.put(url).json(&payload), headers);

    Ok(request.send().await?)
  }

  pub async fn get(&self, endpoint: &str, headers: Option<HeaderMap>) -> Result<Response, NetworkError> {
    let url = format!("{base_url}/{endpoint}", base_url = self.base_url);
    let request = self.request_with_headers(self.client.get(url), headers);

    Ok(request.send().await?)
  }
}
