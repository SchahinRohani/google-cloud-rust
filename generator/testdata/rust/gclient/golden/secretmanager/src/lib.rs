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

use gax::error::{Error, HttpError};
use google_cloud_auth::{Credential, CredentialConfig};
use std::sync::Arc;

pub mod client;

const DEFAULT_HOST: &str = "https://secretmanager.googleapis.com/";

/// A `Result` alias where the `Err` case is an [Error].
pub type Result<T> = std::result::Result<T, Error>;

struct InnerClient {
    http_client: reqwest::Client,
    cred: Credential,
    endpoint: String,
}

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

#[derive(serde::Serialize)]
#[allow(dead_code)]
struct NoBody {}

/// Secret Manager Service
///
/// Manages secrets and operations using those secrets. Implements a REST
/// model with the following objects:
///
/// * [Secret][google.cloud.secretmanager.v1.Secret]
/// * [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]
#[derive(Clone)]
pub struct SecretManagerServiceClient {
    inner: Arc<InnerClient>,
}

impl SecretManagerServiceClient {
    pub async fn new() -> Result<Self> {
        Self::new_with_config(ConfigBuilder::new()).await
    }

    pub async fn new_with_config(conf: ConfigBuilder) -> Result<Self> {
        let inner = InnerClient {
            http_client: conf.client.unwrap_or(ConfigBuilder::default_client()),
            cred: conf
                .cred
                .unwrap_or(ConfigBuilder::default_credential().await?),
            endpoint: conf.endpoint.unwrap_or(DEFAULT_HOST.to_string()),
        };
        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    /// Lists [Secrets][google.cloud.secretmanager.v1.Secret].
    pub async fn list_secrets(
        &self,
        req: crate::model::ListSecretsRequest,
    ) -> Result<crate::model::ListSecretsResponse> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .get(format!(
                "{}/v1/{}/secrets",
                inner_client.endpoint, req.parent,
            ))
            .query(&[("alt", "json")]);
        let builder =
            gax::query_parameter::add(builder, "pageSize", &req.page_size).map_err(Error::other)?;
        let builder = gax::query_parameter::add(builder, "pageToken", &req.page_token)
            .map_err(Error::other)?;
        let builder =
            gax::query_parameter::add(builder, "filter", &req.filter).map_err(Error::other)?;
        self.execute(builder, None::<NoBody>).await
    }

    /// Creates a new [Secret][google.cloud.secretmanager.v1.Secret] containing no
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion].
    pub async fn create_secret(
        &self,
        req: crate::model::CreateSecretRequest,
    ) -> Result<crate::model::Secret> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .post(format!(
                "{}/v1/{}/secrets",
                inner_client.endpoint, req.parent,
            ))
            .query(&[("alt", "json")]);
        let builder =
            gax::query_parameter::add(builder, "secretId", &req.secret_id).map_err(Error::other)?;
        self.execute(builder, Some(req.secret)).await
    }

    /// Creates a new [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]
    /// containing secret data and attaches it to an existing
    /// [Secret][google.cloud.secretmanager.v1.Secret].
    pub async fn add_secret_version(
        &self,
        req: crate::model::AddSecretVersionRequest,
    ) -> Result<crate::model::SecretVersion> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .post(format!(
                "{}/v1/{}:addVersion",
                inner_client.endpoint, req.parent,
            ))
            .query(&[("alt", "json")]);
        self.execute(builder, Some(req)).await
    }

    /// Gets metadata for a given [Secret][google.cloud.secretmanager.v1.Secret].
    pub async fn get_secret(
        &self,
        req: crate::model::GetSecretRequest,
    ) -> Result<crate::model::Secret> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .get(format!("{}/v1/{}", inner_client.endpoint, req.name,))
            .query(&[("alt", "json")]);
        self.execute(builder, None::<NoBody>).await
    }

    /// Updates metadata of an existing
    /// [Secret][google.cloud.secretmanager.v1.Secret].
    pub async fn update_secret(
        &self,
        req: crate::model::UpdateSecretRequest,
    ) -> Result<crate::model::Secret> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .patch(format!(
                "{}/v1/{}",
                inner_client.endpoint,
                gax::path_parameter::PathParameter::required(&req.secret, "secret")
                    .map_err(Error::other)?
                    .name,
            ))
            .query(&[("alt", "json")]);
        let builder = gax::query_parameter::add(
            builder,
            "updateMask",
            &serde_json::to_value(&req.update_mask).map_err(Error::serde)?,
        )
        .map_err(Error::other)?;
        self.execute(builder, Some(req.secret)).await
    }

    /// Deletes a [Secret][google.cloud.secretmanager.v1.Secret].
    pub async fn delete_secret(
        &self,
        req: crate::model::DeleteSecretRequest,
    ) -> Result<wkt::Empty> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .delete(format!("{}/v1/{}", inner_client.endpoint, req.name,))
            .query(&[("alt", "json")]);
        let builder =
            gax::query_parameter::add(builder, "etag", &req.etag).map_err(Error::other)?;
        self.execute(builder, None::<NoBody>).await
    }

    /// Lists [SecretVersions][google.cloud.secretmanager.v1.SecretVersion]. This
    /// call does not return secret data.
    pub async fn list_secret_versions(
        &self,
        req: crate::model::ListSecretVersionsRequest,
    ) -> Result<crate::model::ListSecretVersionsResponse> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .get(format!(
                "{}/v1/{}/versions",
                inner_client.endpoint, req.parent,
            ))
            .query(&[("alt", "json")]);
        let builder =
            gax::query_parameter::add(builder, "pageSize", &req.page_size).map_err(Error::other)?;
        let builder = gax::query_parameter::add(builder, "pageToken", &req.page_token)
            .map_err(Error::other)?;
        let builder =
            gax::query_parameter::add(builder, "filter", &req.filter).map_err(Error::other)?;
        self.execute(builder, None::<NoBody>).await
    }

    /// Gets metadata for a
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// `projects/*/secrets/*/versions/latest` is an alias to the most recently
    /// created [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    pub async fn get_secret_version(
        &self,
        req: crate::model::GetSecretVersionRequest,
    ) -> Result<crate::model::SecretVersion> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .get(format!("{}/v1/{}", inner_client.endpoint, req.name,))
            .query(&[("alt", "json")]);
        self.execute(builder, None::<NoBody>).await
    }

    /// Accesses a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    /// This call returns the secret data.
    ///
    /// `projects/*/secrets/*/versions/latest` is an alias to the most recently
    /// created [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    pub async fn access_secret_version(
        &self,
        req: crate::model::AccessSecretVersionRequest,
    ) -> Result<crate::model::AccessSecretVersionResponse> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .get(format!("{}/v1/{}:access", inner_client.endpoint, req.name,))
            .query(&[("alt", "json")]);
        self.execute(builder, None::<NoBody>).await
    }

    /// Disables a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to
    /// [DISABLED][google.cloud.secretmanager.v1.SecretVersion.State.DISABLED].
    pub async fn disable_secret_version(
        &self,
        req: crate::model::DisableSecretVersionRequest,
    ) -> Result<crate::model::SecretVersion> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .post(format!("{}/v1/{}:disable", inner_client.endpoint, req.name,))
            .query(&[("alt", "json")]);
        self.execute(builder, Some(req)).await
    }

    /// Enables a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to
    /// [ENABLED][google.cloud.secretmanager.v1.SecretVersion.State.ENABLED].
    pub async fn enable_secret_version(
        &self,
        req: crate::model::EnableSecretVersionRequest,
    ) -> Result<crate::model::SecretVersion> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .post(format!("{}/v1/{}:enable", inner_client.endpoint, req.name,))
            .query(&[("alt", "json")]);
        self.execute(builder, Some(req)).await
    }

    /// Destroys a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to
    /// [DESTROYED][google.cloud.secretmanager.v1.SecretVersion.State.DESTROYED]
    /// and irrevocably destroys the secret data.
    pub async fn destroy_secret_version(
        &self,
        req: crate::model::DestroySecretVersionRequest,
    ) -> Result<crate::model::SecretVersion> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .post(format!("{}/v1/{}:destroy", inner_client.endpoint, req.name,))
            .query(&[("alt", "json")]);
        self.execute(builder, Some(req)).await
    }

    /// Sets the access control policy on the specified secret. Replaces any
    /// existing policy.
    ///
    /// Permissions on
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] are enforced
    /// according to the policy set on the associated
    /// [Secret][google.cloud.secretmanager.v1.Secret].
    pub async fn set_iam_policy(
        &self,
        req: iam::model::SetIamPolicyRequest,
    ) -> Result<iam::model::Policy> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .post(format!(
                "{}/v1/{}:setIamPolicy",
                inner_client.endpoint, req.resource,
            ))
            .query(&[("alt", "json")]);
        self.execute(builder, Some(req)).await
    }

    /// Gets the access control policy for a secret.
    /// Returns empty policy if the secret exists and does not have a policy set.
    pub async fn get_iam_policy(
        &self,
        req: iam::model::GetIamPolicyRequest,
    ) -> Result<iam::model::Policy> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .get(format!(
                "{}/v1/{}:getIamPolicy",
                inner_client.endpoint, req.resource,
            ))
            .query(&[("alt", "json")]);
        let builder = gax::query_parameter::add(
            builder,
            "options",
            &serde_json::to_value(&req.options).map_err(Error::serde)?,
        )
        .map_err(Error::other)?;
        self.execute(builder, None::<NoBody>).await
    }

    /// Returns permissions that a caller has for the specified secret.
    /// If the secret does not exist, this call returns an empty set of
    /// permissions, not a NOT_FOUND error.
    ///
    /// Note: This operation is designed to be used for building permission-aware
    /// UIs and command-line tools, not for authorization checking. This operation
    /// may "fail open" without warning.
    pub async fn test_iam_permissions(
        &self,
        req: iam::model::TestIamPermissionsRequest,
    ) -> Result<iam::model::TestIamPermissionsResponse> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .post(format!(
                "{}/v1/{}:testIamPermissions",
                inner_client.endpoint, req.resource,
            ))
            .query(&[("alt", "json")]);
        self.execute(builder, Some(req)).await
    }

    async fn execute<I: serde::ser::Serialize, O: serde::de::DeserializeOwned>(
        &self,
        mut builder: reqwest::RequestBuilder,
        body: Option<I>,
    ) -> Result<O> {
        let inner_client = self.inner.clone();
        builder = builder.bearer_auth(
            &inner_client
                .cred
                .access_token()
                .await
                .map_err(Error::authentication)?
                .value,
        );
        if let Some(body) = body {
            builder = builder.json(&body);
        }
        let resp = builder.send().await.map_err(Error::io)?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let headers = gax::error::convert_headers(resp.headers());
            let body = resp.bytes().await.map_err(Error::io)?;
            return Err(HttpError::new(status, headers, Some(body)).into());
        }
        let response = resp.json::<O>().await.map_err(Error::serde)?;
        Ok(response)
    }
}

/// Manages location-related information with an API service.
#[derive(Clone)]
pub struct LocationsClient {
    inner: Arc<InnerClient>,
}

impl LocationsClient {
    pub async fn new() -> Result<Self> {
        Self::new_with_config(ConfigBuilder::new()).await
    }

    pub async fn new_with_config(conf: ConfigBuilder) -> Result<Self> {
        let inner = InnerClient {
            http_client: conf.client.unwrap_or(ConfigBuilder::default_client()),
            cred: conf
                .cred
                .unwrap_or(ConfigBuilder::default_credential().await?),
            endpoint: conf.endpoint.unwrap_or(DEFAULT_HOST.to_string()),
        };
        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    /// Lists information about the supported locations for this service.
    pub async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
    ) -> Result<location::model::ListLocationsResponse> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .get(format!(
                "{}/v1/{}/locations",
                inner_client.endpoint, req.name,
            ))
            .query(&[("alt", "json")]);
        let builder =
            gax::query_parameter::add(builder, "filter", &req.filter).map_err(Error::other)?;
        let builder =
            gax::query_parameter::add(builder, "pageSize", &req.page_size).map_err(Error::other)?;
        let builder = gax::query_parameter::add(builder, "pageToken", &req.page_token)
            .map_err(Error::other)?;
        self.execute(builder, None::<NoBody>).await
    }

    /// Gets information about a location.
    pub async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
    ) -> Result<location::model::Location> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .get(format!("{}/v1/{}", inner_client.endpoint, req.name,))
            .query(&[("alt", "json")]);
        self.execute(builder, None::<NoBody>).await
    }

    async fn execute<I: serde::ser::Serialize, O: serde::de::DeserializeOwned>(
        &self,
        mut builder: reqwest::RequestBuilder,
        body: Option<I>,
    ) -> Result<O> {
        let inner_client = self.inner.clone();
        builder = builder.bearer_auth(
            &inner_client
                .cred
                .access_token()
                .await
                .map_err(Error::authentication)?
                .value,
        );
        if let Some(body) = body {
            builder = builder.json(&body);
        }
        let resp = builder.send().await.map_err(Error::io)?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let headers = gax::error::convert_headers(resp.headers());
            let body = resp.bytes().await.map_err(Error::io)?;
            return Err(HttpError::new(status, headers, Some(body)).into());
        }
        let response = resp.json::<O>().await.map_err(Error::serde)?;
        Ok(response)
    }
}
