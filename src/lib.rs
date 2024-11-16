//! # Portkey Rust SDK
//!
//! The `portkey` SDK is a lightweight wrapper around `async-openai` designed to provide
//! a Portkey-compatible OpenAI client. This SDK simplifies the process of creating custom
//! clients with the required headers and configurations for Portkey's API.
//!
//! ## Features
//! - Integrates with `async-openai` for OpenAI API compatibility.
//! - Configures custom headers for Portkey-specific requirements.
//! - Simplifies initialization with `api_key` and `virtual_key`.
//!
//! ## License
//! This library is distributed under the MIT License. See the `LICENSE` file for details.

use async_openai::{config::OpenAIConfig, Client as OpenAIClient};
use reqwest::{header::HeaderMap, Client as ReqwestClient};

/// Base URL for the Portkey AI API.
const BASE_URL: &str = "https://api.portkey.ai/v1";

/// A client for interacting with the Portkey AI API.
///
/// The `Client` struct is a lightweight wrapper around `async-openai`, providing
/// custom configurations and headers to make it compatible with Portkey's API.
///
/// # Examples
///
/// ```rust
/// use portkey::Client;
///
/// let api_key = "your-portkey-api-key";
/// let virtual_key = "your-portkey-virtual-key";
///
/// // Create a new Portkey client
/// let client = Client::new(api_key, virtual_key);
///
/// // Access the OpenAI client
/// let openai_client = client.openai();
/// ```
pub struct Client {
    /// OpenAI client configured for Portkey.
    openai: OpenAIClient<OpenAIConfig>,
    /// Base URL for the API.
    base_url: String,
    /// The Portkey virtual key used for authentication.
    virtual_key: String,
    /// The Portkey API key used for authentication.
    api_key: String,
}

impl Client {
    /// Creates a new instance of the `Client`.
    ///
    /// This method sets up the required headers and configures the OpenAI client
    /// to work with the Portkey API.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Portkey API key.
    /// * `virtual_key` - Your Portkey virtual key.
    ///
    /// # Returns
    ///
    /// A configured instance of the `Client`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use portkey::Client;
    ///
    /// let api_key = "your-portkey-api-key";
    /// let virtual_key = "your-portkey-virtual-key";
    ///
    /// let client = Client::new(api_key, virtual_key);
    /// ```
    pub fn new(api_key: &str, virtual_key: &str) -> Self {
        let mut reqwest_headers = HeaderMap::new();
        reqwest_headers.insert(
            "x-portkey-virtual-key",
            virtual_key.parse().expect("Failed to parse virtual key"),
        );
        let reqwest_client = ReqwestClient::builder()
            .default_headers(reqwest_headers)
            .build()
            .expect("Failed to build reqwest client");

        let openai_config = OpenAIConfig::new()
            .with_api_base(BASE_URL)
            .with_api_key(api_key);

        let openai = OpenAIClient::with_config(openai_config).with_http_client(reqwest_client);

        Self {
            openai,
            base_url: BASE_URL.to_string(),
            virtual_key: virtual_key.to_string(),
            api_key: api_key.to_string(),
        }
    }

    /// Returns the underlying OpenAI client configured for Portkey.
    ///
    /// This allows you to use all features provided by the `async_openai` library
    /// with Portkey's API compatibility.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use portkey::Client;
    ///
    /// let api_key = "your-portkey-api-key";
    /// let virtual_key = "your-portkey-virtual-key";
    /// let client = Client::new(api_key, virtual_key);
    ///
    /// let openai_client = client.openai();
    /// ```
    pub fn openai(self) -> OpenAIClient<OpenAIConfig> {
        self.openai
    }
}
