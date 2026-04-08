use crate::openapi::Client;
use uuid::Uuid;

const DEFAULT_BASE_URL: &str = "https://api.freestyle.sh";

/// Authentication method for the Freestyle API.
pub enum Auth {
    /// Bearer token from an API key.
    ApiKey(String),
    /// Identity access token.
    AccessToken(String),
}

/// High-level Freestyle client wrapping the generated API client.
///
/// Handles authentication and provides convenience accessors.
///
/// # Examples
///
/// ```no_run
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // From environment variable FREESTYLE_API_KEY:
/// let fs = freestyle_sandboxes::Freestyle::from_env()?;
///
/// // Explicit API key:
/// let fs = freestyle_sandboxes::Freestyle::new("your-api-key");
///
/// // Check auth:
/// let who = fs.client().handle_whoami().await?;
/// # Ok(())
/// # }
/// ```
pub struct Freestyle {
    client: Client,
}

impl Freestyle {
    /// Create a new client with an API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_auth(Auth::ApiKey(api_key.into()), None)
    }

    /// Create a new client from the `FREESTYLE_API_KEY` environment variable.
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let key = std::env::var("FREESTYLE_API_KEY")?;
        Ok(Self::new(key))
    }

    /// Create a new client with explicit auth and optional base URL override.
    pub fn with_auth(auth: Auth, base_url: Option<&str>) -> Self {
        let base_url = base_url
            .map(String::from)
            .or_else(|| std::env::var("FREESTYLE_API_URL").ok())
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
        let base_url = base_url.trim_end_matches('/').to_string();

        let mut headers = reqwest::header::HeaderMap::new();
        match &auth {
            Auth::ApiKey(key) => {
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    format!("Bearer {key}").parse().expect("valid header value"),
                );
            }
            Auth::AccessToken(token) => {
                headers.insert(
                    reqwest::header::HeaderName::from_static("x-freestyle-identity-access-token"),
                    token.parse().expect("valid header value"),
                );
            }
        }

        let reqwest_client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .connect_timeout(std::time::Duration::from_secs(15))
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .expect("failed to build reqwest client");

        Self {
            client: Client::new_with_client(&base_url, reqwest_client),
        }
    }

    /// Access the underlying generated API client for direct endpoint calls.
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a VM handle for convenience methods (fs, exec, systemd).
    pub fn vm(&self, vm_id: impl Into<String>) -> crate::VmHandle {
        crate::VmHandle::new(self.client.clone(), vm_id)
    }

    /// Get a deployment handle for the fetch proxy.
    pub fn deployment(&self, deployment_id: Uuid) -> DeploymentHandle {
        DeploymentHandle {
            client: self.client.clone(),
            deployment_id,
        }
    }
}

/// Handle for a specific deployment, providing the fetch proxy method.
pub struct DeploymentHandle {
    client: Client,
    deployment_id: Uuid,
}

impl DeploymentHandle {
    /// Proxy an HTTP request through the Freestyle API to this deployment.
    ///
    /// Forwards the request to the deployment's server using the
    /// `X-Freestyle-Request-Url` header to specify the target path.
    pub async fn fetch(
        &self,
        method: reqwest::Method,
        url: &str,
        body: Option<reqwest::Body>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let request_url = match reqwest::Url::parse(url) {
            Ok(parsed) => {
                let mut path = parsed.path().to_string();
                if let Some(q) = parsed.query() {
                    path.push('?');
                    path.push_str(q);
                }
                path
            }
            Err(_) => url.to_string(),
        };

        let api_url = format!(
            "{}/web/v1/deployments/{}/fetch",
            self.client.baseurl, self.deployment_id,
        );

        let mut req = self
            .client
            .client
            .request(method, &api_url)
            .header("X-Freestyle-Request-Url", &request_url);

        if let Some(body) = body {
            req = req.body(body);
        }

        req.send().await
    }
}
