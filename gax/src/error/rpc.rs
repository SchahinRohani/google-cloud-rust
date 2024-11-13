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

/// Describes the cause of the error with structured details.
///
/// Example of an error when contacting the "pubsub.googleapis.com" API when it
/// is not enabled:
///
///     { "reason": "API_DISABLED"
///       "domain": "googleapis.com"
///       "metadata": {
///         "resource": "projects/123",
///         "service": "pubsub.googleapis.com"
///       }
///     }
///
/// This response indicates that the pubsub.googleapis.com API is not enabled.
///
/// Example of an error that is returned when attempting to create a Spanner
/// instance in a region that is out of stock:
///
///     { "reason": "STOCKOUT"
///       "domain": "spanner.googleapis.com",
///       "metadata": {
///         "availableRegions": "us-central1,us-east2"
///       }
///     }
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ErrorInfo {
    /// The reason of the error. This is a constant value that identifies the
    /// proximate cause of the error. Error reasons are unique within a particular
    /// domain of errors. This should be at most 63 characters and match a
    /// regular expression of `[A-Z][A-Z0-9_]+[A-Z0-9]`, which represents
    /// UPPER_SNAKE_CASE.
    pub reason: String,

    /// The logical grouping to which the "reason" belongs. The error domain
    /// is typically the registered service name of the tool or product that
    /// generates the error. Example: "pubsub.googleapis.com". If the error is
    /// generated by some common infrastructure, the error domain must be a
    /// globally unique value that identifies the infrastructure. For Google API
    /// infrastructure, the error domain is "googleapis.com".
    pub domain: String,

    /// Additional structured details about this error.
    ///
    /// Keys should match /[a-zA-Z0-9-_]/ and be limited to 64 characters in
    /// length. When identifying the current value of an exceeded limit, the units
    /// should be contained in the key, not the value.  For example, rather than
    /// {"instanceLimit": "100/request"}, should be returned as,
    /// {"instanceLimitPerRequest": "100"}, if the client exceeds the number of
    /// instances that can be created in a single (batch) request.
    pub metadata: std::collections::HashMap<String, String>,
}

/// Describes when the clients can retry a failed request. Clients could ignore
/// the recommendation here or retry when this information is missing from error
/// responses.
///
/// It's always recommended that clients should use exponential backoff when
/// retrying.
///
/// Clients should wait until `retry_delay` amount of time has passed since
/// receiving the error response before retrying.  If retrying requests also
/// fail, clients should use an exponential backoff scheme to gradually increase
/// the delay between retries based on `retry_delay`, until either a maximum
/// number of retries have been reached or a maximum retry delay cap has been
/// reached.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RetryInfo {
    /// Clients should wait at least this long between retrying the same request.
    pub retry_delay: Option<types::Duration>,
}

/// Describes additional debugging info.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DebugInfo {
    /// The stack trace entries indicating where the error occurred.
    pub stack_entries: Vec<String>,

    /// Additional debugging information provided by the server.
    pub detail: String,
}

/// Describes how a quota check failed.
///
/// For example if a daily limit was exceeded for the calling project,
/// a service could respond with a QuotaFailure detail containing the project
/// id and the description of the quota limit that was exceeded.  If the
/// calling project hasn't enabled the service in the developer console, then
/// a service could respond with the project id and set `service_disabled`
/// to true.
///
/// Also see RetryInfo and Help types for other details about handling a
/// quota failure.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct QuotaFailure {
    /// Describes all quota violations.
    pub violations: Vec<quota_failure::Violation>,
}

/// Defines additional types related to QuotaFailure
pub mod quota_failure {

    /// A message type used to describe a single quota violation.  For example, a
    /// daily quota or a custom quota that was exceeded.
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct Violation {
        /// The subject on which the quota check failed.
        /// For example, "clientip:<ip address of client>" or "project:<Google
        /// developer project id>".
        pub subject: String,

        /// A description of how the quota check failed. Clients can use this
        /// description to find more about the quota configuration in the service's
        /// public documentation, or find the relevant quota limit to adjust through
        /// developer console.
        ///
        /// For example: "Service disabled" or "Daily Limit for read operations
        /// exceeded".
        pub description: String,
    }
}

/// Describes what preconditions have failed.
///
/// For example, if an RPC failed because it required the Terms of Service to be
/// acknowledged, it could list the terms of service violation in the
/// PreconditionFailure message.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PreconditionFailure {
    /// Describes all precondition violations.
    pub violations: Vec<precondition_failure::Violation>,
}

/// Defines additional types related to PreconditionFailure
pub mod precondition_failure {

    /// A message type used to describe a single precondition failure.
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct Violation {
        /// The type of PreconditionFailure. We recommend using a service-specific
        /// enum type to define the supported precondition violation subjects. For
        /// example, "TOS" for "Terms of Service violation".
        pub r#type: String,

        /// The subject, relative to the type, that failed.
        /// For example, "google.com/cloud" relative to the "TOS" type would indicate
        /// which terms of service is being referenced.
        pub subject: String,

        /// A description of how the precondition failed. Developers can use this
        /// description to understand how to fix the failure.
        ///
        /// For example: "Terms of service not accepted".
        pub description: String,
    }
}

/// Describes violations in a client request. This error type focuses on the
/// syntactic aspects of the request.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BadRequest {
    /// Describes all violations in a client request.
    pub field_violations: Vec<bad_request::FieldViolation>,
}

/// Defines additional types related to BadRequest
pub mod bad_request {

    /// A message type used to describe a single bad request field.
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct FieldViolation {
        /// A path that leads to a field in the request body. The value will be a
        /// sequence of dot-separated identifiers that identify a protocol buffer
        /// field.
        ///
        /// Consider the following:
        ///
        ///    message CreateContactRequest {
        ///      message EmailAddress {
        ///        enum Type {
        ///          TYPE_UNSPECIFIED = 0;
        ///          HOME = 1;
        ///          WORK = 2;
        ///        }
        ///
        ///        optional string email = 1;
        ///        repeated EmailType type = 2;
        ///      }
        ///
        ///      string full_name = 1;
        ///      repeated EmailAddress email_addresses = 2;
        ///    }
        ///
        /// In this example, in proto `field` could take one of the following values:
        ///
        /// * `full_name` for a violation in the `full_name` value
        /// * `email_addresses[1].email` for a violation in the `email` field of the
        ///  first `email_addresses` message
        /// * `email_addresses[3].type[2]` for a violation in the second `type`
        ///  value in the third `email_addresses` message.
        ///
        /// In JSON, the same values are represented as:
        ///
        /// * `fullName` for a violation in the `fullName` value
        /// * `emailAddresses[1].email` for a violation in the `email` field of the
        ///  first `emailAddresses` message
        /// * `emailAddresses[3].type[2]` for a violation in the second `type`
        ///  value in the third `emailAddresses` message.
        pub field: String,

        /// A description of why the request element is bad.
        pub description: String,
    }
}

/// Contains metadata about the request that clients can attach when filing a bug
/// or providing other forms of feedback.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RequestInfo {
    /// An opaque string that should only be interpreted by the service generating
    /// it. For example, it can be used to identify requests in the service's logs.
    pub request_id: String,

    /// Any data that was used to serve this request. For example, an encrypted
    /// stack trace that can be sent back to the service provider for debugging.
    pub serving_data: String,
}

/// Describes the resource that is being accessed.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ResourceInfo {
    /// A name for the type of resource being accessed, e.g. "sql table",
    /// "cloud storage bucket", "file", "Google calendar"; or the type URL
    /// of the resource: e.g. "type.googleapis.com/google.pubsub.v1.Topic".
    pub resource_type: String,

    /// The name of the resource being accessed.  For example, a shared calendar
    /// name: "example.com_4fghdhgsrgh@group.calendar.google.com", if the current
    /// error is
    /// [google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED].
    pub resource_name: String,

    /// The owner of the resource (optional).
    /// For example, "user:<owner email>" or "project:<Google developer project
    /// id>".
    pub owner: String,

    /// Describes what error is encountered when accessing this resource.
    /// For example, updating a cloud project may require the `writer` permission
    /// on the developer console project.
    pub description: String,
}

/// Provides links to documentation or for performing an out of band action.
///
/// For example, if a quota check failed with an error indicating the calling
/// project hasn't enabled the accessed service, this can contain a URL pointing
/// directly to the right place in the developer console to flip the bit.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Help {
    /// URL(s) pointing to additional information on handling the current error.
    pub links: Vec<help::Link>,
}

/// Defines additional types related to Help
pub mod help {

    /// Describes a URL link.
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct Link {
        /// Describes what the link offers.
        pub description: String,

        /// The URL of the link.
        pub url: String,
    }
}

/// Provides a localized error message that is safe to return to the user
/// which can be attached to an RPC error.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LocalizedMessage {
    /// The locale used following the specification defined at
    /// https://www.rfc-editor.org/rfc/bcp/bcp47.txt.
    /// Examples are: "en-US", "fr-CH", "es-MX"
    pub locale: String,

    /// The localized error message in the above locale.
    pub message: String,
}
