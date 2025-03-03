// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.

/// The messages and enums that are part of this client library.
pub mod model;

use gax::error::Error;
use google_cloud_auth::{Credential, CredentialConfig};

pub mod traits;
pub(crate) mod transport;

const DEFAULT_HOST: &str = "https://cloud.googleapis.com/";

/// A `Result` alias where the `Err` case is an [Error].
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct ConfigBuilder {
    pub(crate) endpoint: Option<String>,
    pub(crate) client: Option<reqwest::Client>,
    pub(crate) cred: Option<Credential>,
}

impl ConfigBuilder {
    /// Returns a default [ConfigBuilder].
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets an endpoint that overrides the default endpoint for a service.
    pub fn set_endpoint<T: Into<String>>(mut self, v: T) -> Self {
        self.endpoint = Some(v.into());
        self
    }

    pub(crate) fn default_client() -> reqwest::Client {
        reqwest::Client::builder().build().unwrap()
    }

    pub(crate) async fn default_credential() -> Result<Credential> {
        let cc = CredentialConfig::builder()
            .scopes(vec![
                "https://www.googleapis.com/auth/cloud-platform".to_string()
            ])
            .build()
            .map_err(Error::authentication)?;
        Credential::find_default(cc)
            .await
            .map_err(Error::authentication)
    }
}

pub type LocationsClient = crate::transport::Locations;
