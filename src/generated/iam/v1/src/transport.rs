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

use crate::{Credential, Result};
use gax::error::{Error, HttpError};
use std::sync::Arc;

// Shared implementation across clients.
struct InnerClient {
    http_client: reqwest::Client,
    cred: Credential,
    endpoint: String,
}

#[derive(serde::Serialize)]
#[allow(dead_code)]
struct NoBody {}

/// API Overview
///
/// Manages Identity and Access Management (IAM) policies.
///
/// Any implementation of an API that offers access control features
/// implements the google.iam.v1.IAMPolicy interface.
///
/// ## Data model
///
/// Access control is applied when a principal (user or service account), takes
/// some action on a resource exposed by a service. Resources, identified by
/// URI-like names, are the unit of access control specification. Service
/// implementations can choose the granularity of access control and the
/// supported permissions for their resources.
/// For example one database service may allow access control to be
/// specified only at the Table level, whereas another might allow access control
/// to also be specified at the Column level.
///
/// ## Policy Structure
///
/// See google.iam.v1.Policy
///
/// This is intentionally not a CRUD style API because access control policies
/// are created and deleted implicitly with the resources to which they are
/// attached.
#[derive(Clone)]
pub struct Iampolicy {
    inner: Arc<InnerClient>,
}

impl std::fmt::Debug for Iampolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Iampolicy[{}]", self.inner.endpoint)
    }
}

impl Iampolicy {
    pub async fn new() -> Result<Self> {
        Self::new_with_config(crate::ConfigBuilder::default()).await
    }

    pub async fn new_with_config(conf: crate::ConfigBuilder) -> Result<Self> {
        let inner = InnerClient {
            http_client: conf
                .client
                .unwrap_or(crate::ConfigBuilder::default_client()),
            cred: conf
                .cred
                .unwrap_or(crate::ConfigBuilder::default_credential().await?),
            endpoint: conf.endpoint.unwrap_or(crate::DEFAULT_HOST.to_string()),
        };
        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    async fn fetch_token(&self) -> Result<String> {
        let tok = self
            .inner
            .cred
            .access_token()
            .await
            .map_err(Error::authentication)?;
        Ok(tok.value)
    }

    async fn execute<I: serde::ser::Serialize, O: serde::de::DeserializeOwned>(
        access_token: String,
        mut builder: reqwest::RequestBuilder,
        body: Option<I>,
    ) -> Result<O> {
        builder = builder.bearer_auth(access_token);
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

impl crate::traits::Iampolicy for Iampolicy {
    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    async fn set_iam_policy(
        &self,
        req: crate::model::SetIamPolicyRequest,
    ) -> Result<crate::model::Policy> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .post(format!(
                "{}/v1/{}:setIamPolicy",
                inner_client.endpoint, req.resource,
            ))
            .query(&[("alt", "json")]);
        let access_token = self.fetch_token().await?;
        Self::execute(access_token, builder, Some(req)).await
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    async fn get_iam_policy(
        &self,
        req: crate::model::GetIamPolicyRequest,
    ) -> Result<crate::model::Policy> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .post(format!(
                "{}/v1/{}:getIamPolicy",
                inner_client.endpoint, req.resource,
            ))
            .query(&[("alt", "json")]);
        let access_token = self.fetch_token().await?;
        Self::execute(access_token, builder, Some(req)).await
    }

    /// Returns permissions that a caller has on the specified resource. If the
    /// resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building
    /// permission-aware UIs and command-line tools, not for authorization
    /// checking. This operation may "fail open" without warning.
    async fn test_iam_permissions(
        &self,
        req: crate::model::TestIamPermissionsRequest,
    ) -> Result<crate::model::TestIamPermissionsResponse> {
        let inner_client = self.inner.clone();
        let builder = inner_client
            .http_client
            .post(format!(
                "{}/v1/{}:testIamPermissions",
                inner_client.endpoint, req.resource,
            ))
            .query(&[("alt", "json")]);
        let access_token = self.fetch_token().await?;
        Self::execute(access_token, builder, Some(req)).await
    }
}
