use crate::auth::Auth;
use crate::error::{ApiErrorBody, YookassaError};
use reqwest::Method;
use serde::Serialize;
use serde::de::DeserializeOwned;
use uuid::Uuid;

const DEFAULT_BASE_URL: &str = "https://api.yookassa.ru/v3";

#[derive(Debug, Clone)]
pub struct YookassaClient {
    http: reqwest::Client,
    auth: Auth,
    base_url: String,
}

#[derive(Debug, Clone)]
pub struct YookassaClientBuilder {
    auth: Auth,
    base_url: String,
    timeout: Option<std::time::Duration>,
    user_agent: Option<String>,
}

impl YookassaClientBuilder {
    pub fn new(auth: Auth) -> Self {
        Self {
            auth,
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout: None,
            user_agent: Some(format!("yookasa-sdk/{}", env!("CARGO_PKG_VERSION"))),
        }
    }

    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    pub fn build(self) -> Result<YookassaClient, YookassaError> {
        let mut builder = reqwest::Client::builder();
        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }
        if let Some(user_agent) = self.user_agent {
            builder = builder.user_agent(user_agent);
        }

        Ok(YookassaClient {
            http: builder.build()?,
            auth: self.auth,
            base_url: self.base_url.trim_end_matches('/').to_string(),
        })
    }
}

impl YookassaClient {
    pub fn builder(auth: Auth) -> YookassaClientBuilder {
        YookassaClientBuilder::new(auth)
    }

    pub fn new(auth: Auth) -> Result<Self, YookassaError> {
        Self::builder(auth).build()
    }

    pub(crate) fn uses_oauth(&self) -> bool {
        self.auth.is_oauth()
    }

    fn with_auth(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match &self.auth {
            Auth::Basic {
                shop_id,
                secret_key,
            } => request.basic_auth(shop_id, Some(secret_key)),
            Auth::OAuth { token } => request.bearer_auth(token),
        }
    }

    fn endpoint(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path.trim_start_matches('/'))
    }

    fn idempotence_key(idempotence_key: Option<&str>) -> Result<String, YookassaError> {
        match idempotence_key {
            Some(key)
                if !key.is_empty()
                    && key.len() <= 64
                    && key.chars().all(|c| {
                        c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '_' | '.')
                    }) =>
            {
                Ok(key.to_string())
            }
            Some(_) => Err(YookassaError::InvalidIdempotenceKey),
            None => Ok(Uuid::new_v4().to_string()),
        }
    }

    async fn send<T, Q, B>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
        idempotence_key: Option<&str>,
    ) -> Result<T, YookassaError>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        let mut request = self.http.request(method.clone(), self.endpoint(path));
        request = self.with_auth(request);

        if let Some(query) = query {
            request = request.query(query);
        }
        if let Some(body) = body {
            request = request.json(body);
        }
        if matches!(method, Method::POST | Method::DELETE) {
            let key = Self::idempotence_key(idempotence_key)?;
            request = request.header("Idempotence-Key", key);
        }

        let response = request.send().await?;
        Self::parse_response(response).await
    }

    async fn send_empty<Q>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        idempotence_key: Option<&str>,
    ) -> Result<(), YookassaError>
    where
        Q: Serialize + ?Sized,
    {
        let mut request = self.http.request(method.clone(), self.endpoint(path));
        request = self.with_auth(request);
        if let Some(query) = query {
            request = request.query(query);
        }
        if matches!(method, Method::POST | Method::DELETE) {
            let key = Self::idempotence_key(idempotence_key)?;
            request = request.header("Idempotence-Key", key);
        }

        let response = request.send().await?;
        let status = response.status();
        let body = response.text().await?;
        if status.is_success() {
            return Ok(());
        }

        if let Ok(api) = serde_json::from_str::<ApiErrorBody>(&body) {
            return Err(YookassaError::Api { status, body: api });
        }

        Err(YookassaError::UnexpectedStatus { status, body })
    }

    async fn parse_response<T>(response: reqwest::Response) -> Result<T, YookassaError>
    where
        T: DeserializeOwned,
    {
        let status = response.status();
        let body = response.text().await?;
        if status.is_success() {
            return Ok(serde_json::from_str(&body)?);
        }
        if let Ok(api) = serde_json::from_str::<ApiErrorBody>(&body) {
            return Err(YookassaError::Api { status, body: api });
        }
        Err(YookassaError::UnexpectedStatus { status, body })
    }

    pub async fn get<T, Q>(&self, path: &str, query: Option<&Q>) -> Result<T, YookassaError>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        self.send(Method::GET, path, query, Option::<&()>::None, None)
            .await
    }

    pub async fn post<T, B>(
        &self,
        path: &str,
        body: &B,
        idempotence_key: Option<&str>,
    ) -> Result<T, YookassaError>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        self.send(
            Method::POST,
            path,
            Option::<&()>::None,
            Some(body),
            idempotence_key,
        )
        .await
    }

    pub async fn post_without_body<T>(
        &self,
        path: &str,
        idempotence_key: Option<&str>,
    ) -> Result<T, YookassaError>
    where
        T: DeserializeOwned,
    {
        self.send(
            Method::POST,
            path,
            Option::<&()>::None,
            Option::<&()>::None,
            idempotence_key,
        )
        .await
    }

    pub async fn delete_empty(
        &self,
        path: &str,
        idempotence_key: Option<&str>,
    ) -> Result<(), YookassaError> {
        self.send_empty(Method::DELETE, path, Option::<&()>::None, idempotence_key)
            .await
    }
}
