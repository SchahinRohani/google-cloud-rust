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

use crate::Result;
use rand::{distributions::Alphanumeric, Rng};
use smo::traits::SecretManagerService;

pub async fn run() -> Result<()> {
    let project_id = crate::project_id()?;
    let secret_id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(crate::SECRET_ID_LENGTH)
        .map(char::from)
        .collect();

    let client = smo::SecretManagerServiceClient::new().await?;

    println!("\nTesting create_secret()");
    let create = client
        .create_secret(
            smo::model::CreateSecretRequest::default()
                .set_project(&project_id)
                .set_secret_id(&secret_id)
                .set_request_body(
                    smo::model::Secret::default()
                        .set_replication(
                            smo::model::Replication::default()
                                .set_automatic(smo::model::Automatic::default()),
                        )
                        .set_labels(
                            [("integration-test", "true")]
                                .map(|(k, v)| (k.to_string(), v.to_string())),
                        ),
                ),
        )
        .await?;
    println!("CREATE = {create:?}");

    let project_name = create
        .name
        .as_ref()
        .and_then(|s| s.strip_suffix(format!("/secrets/{secret_id}").as_str()));
    assert!(project_name.is_some());

    println!("\nTesting get_secret()");
    let get = client
        .get_secret(
            smo::model::GetSecretRequest::default()
                .set_project(&project_id)
                .set_secret(&secret_id),
        )
        .await?;
    println!("GET = {get:?}");
    assert_eq!(get, create);
    assert!(get.name.is_some());

    let secret_name = get.name.as_ref().unwrap().clone();

    println!("\nTesting update_secret()");
    let mut new_labels = get.labels.clone();
    new_labels.insert("updated".to_string(), "true".to_string());
    let update = client
        .update_secret(
            smo::model::UpdateSecretRequest::default()
                .set_project(&project_id)
                .set_secret(&secret_id)
                .set_update_mask(
                    wkt::FieldMask::default().set_paths(["labels"].map(str::to_string).to_vec()),
                )
                .set_request_body(smo::model::Secret::default().set_labels(new_labels)),
        )
        .await?;
    println!("UPDATE = {update:?}");

    println!("\nTesting list_secrets()");
    let list = get_all_secret_names(&client, &project_id).await?;
    assert!(
        list.iter().any(|name| name == &secret_name),
        "missing secret {} in {list:?}",
        &secret_name
    );

    run_secret_versions(&client, &project_id, &secret_id).await?;
    run_iam(&client, &project_id, &secret_id).await?;
    run_locations(&client, &project_id).await?;

    println!("\nTesting delete_secret()");
    let response = client
        .delete_secret(
            smo::model::DeleteSecretRequest::default()
                .set_project(&project_id)
                .set_secret(&secret_id),
        )
        .await?;
    println!("DELETE = {response:?}");
    Ok(())
}

async fn run_locations(client: &smo::SecretManagerServiceClient, project_id: &str) -> Result<()> {
    println!("\nTesting list_locations()");
    let locations = client
        .list_locations(smo::model::ListLocationsRequest::default().set_project(project_id))
        .await?;
    println!("LOCATIONS = {locations:?}");

    assert!(
        !locations.locations.is_empty(),
        "got empty locations field for {locations:?}"
    );
    let first = locations.locations[0].clone();
    assert!(
        first.location_id.is_some(),
        "expected some location field to be set in {first:?}"
    );

    println!("\nTesting get_location()");
    let get = client
        .get_location(
            smo::model::GetLocationRequest::default()
                .set_project(project_id)
                .set_location(first.location_id.clone().unwrap()),
        )
        .await?;
    println!("GET = {get:?}");

    assert_eq!(get, first);

    Ok(())
}

async fn run_iam(
    client: &smo::SecretManagerServiceClient,
    project_id: &str,
    secret_id: &str,
) -> Result<()> {
    let service_account = crate::service_account_for_iam_tests()?;

    println!("\nTesting get_iam_policy()");
    let policy = client
        .get_iam_policy(
            smo::model::GetIamPolicyRequest::default()
                .set_project(project_id)
                .set_secret(secret_id),
        )
        .await?;
    println!("POLICY = {policy:?}");

    println!("\nTesting test_iam_permissions()");
    let response = client
        .test_iam_permissions(
            smo::model::TestIamPermissionsRequest::default()
                .set_project(project_id)
                .set_secret(secret_id)
                .set_permissions(
                    ["secretmanager.versions.access"]
                        .map(str::to_string)
                        .to_vec(),
                ),
        )
        .await?;
    println!("RESPONSE = {response:?}");

    // This really could use an OCC loop.
    println!("\nTesting set_iam_policy()");
    let mut new_policy = policy.clone();
    const ROLE: &str = "roles/secretmanager.secretVersionAdder";
    let mut found = false;
    for binding in &mut new_policy.bindings {
        if let Some(ROLE) = binding.role.as_deref() {
            continue;
        }
        found = true;
        binding
            .members
            .push(format!("serviceAccount:{service_account}"));
    }
    if !found {
        new_policy.bindings.push(
            smo::model::Binding::default()
                .set_role(ROLE.to_string())
                .set_members([format!("serviceAccount:{service_account}")].to_vec()),
        );
    }
    let response = client
        .set_iam_policy(
            smo::model::SetIamPolicyRequest::default()
                .set_project(project_id)
                .set_secret(secret_id)
                .set_update_mask(
                    wkt::FieldMask::default().set_paths(["bindings"].map(str::to_string).to_vec()),
                )
                .set_policy(new_policy),
        )
        .await?;
    println!("RESPONSE = {response:?}");

    Ok(())
}

async fn run_secret_versions(
    client: &smo::SecretManagerServiceClient,
    project_id: &str,
    secret_id: &str,
) -> Result<()> {
    println!("\nTesting create_secret_version()");
    let data = "The quick brown fox jumps over the lazy dog".as_bytes();
    let checksum = crc32c::crc32c(data);
    let create = client
        .add_secret_version(
            smo::model::AddSecretVersionRequest::default()
                .set_project(project_id)
                .set_secret(secret_id)
                .set_payload(
                    smo::model::SecretPayload::default()
                        .set_data(bytes::Bytes::from(data))
                        .set_data_crc_32_c(checksum as i64),
                ),
        )
        .await?;
    println!("CREATE_SECRET_VERSION = {create:?}");

    assert!(
        create.name.is_some(),
        "missing name in create response {create:?}"
    );
    let name = create.name.clone().unwrap();
    let pattern = format!("secrets/{secret_id}/versions/");
    let version_id = name.find(pattern.as_str());
    assert!(
        version_id.is_some(),
        "cannot field secret in secret version name={name}"
    );
    let version_id = &name[version_id.unwrap()..];
    let version_id = &version_id[pattern.len()..];

    println!("\nTesting get_secret_version()");
    let get = client
        .get_secret_version(
            smo::model::GetSecretVersionRequest::default()
                .set_project(project_id)
                .set_secret(secret_id)
                .set_version(version_id),
        )
        .await?;
    println!("GET_SECRET_VERSION = {get:?}");
    assert_eq!(get, create);

    println!("\nTesting list_secret_versions()");
    let secret_versions_list = get_all_secret_version_names(client, project_id, secret_id).await?;
    assert!(
        secret_versions_list
            .iter()
            .any(|name| Some(name) == get.name.as_ref()),
        "missing secret version {:?} in {secret_versions_list:?}",
        &get.name
    );

    println!("\nTesting access_secret_version()");
    let access_secret_version = client
        .access_secret_version(
            smo::model::AccessSecretVersionRequest::default()
                .set_project(project_id)
                .set_secret(secret_id)
                .set_version(version_id),
        )
        .await?;
    println!("ACCESS_SECRET_VERSION = {access_secret_version:?}");
    assert_eq!(
        access_secret_version.payload.and_then(|p| p.data),
        Some(bytes::Bytes::from(data))
    );

    println!("\nTesting disable_secret_version()");
    let disable = client
        .disable_secret_version(
            smo::model::DisableSecretVersionRequest::default()
                .set_project(project_id)
                .set_secret(secret_id)
                .set_version(version_id),
        )
        .await?;
    println!("DISABLE_SECRET_VERSION = {disable:?}");

    println!("\nTesting enable_secret_version()");
    let enable = client
        .enable_secret_version(
            smo::model::EnableSecretVersionRequest::default()
                .set_project(project_id)
                .set_secret(secret_id)
                .set_version(version_id),
        )
        .await?;
    println!("ENABLE_SECRET_VERSION = {enable:?}");

    println!("\nTesting destroy_secret_version()");
    let delete = client
        .destroy_secret_version(
            smo::model::DestroySecretVersionRequest::default()
                .set_project(project_id)
                .set_secret(secret_id)
                .set_version(version_id),
        )
        .await?;
    println!("RESPONSE = {delete:?}");

    Ok(())
}

async fn get_all_secret_version_names(
    client: &smo::SecretManagerServiceClient,
    project_id: &str,
    secret_id: &str,
) -> Result<Vec<String>> {
    let mut names = Vec::new();
    let mut page_token = None::<String>;
    loop {
        let response = client
            .list_secret_versions(
                smo::model::ListSecretVersionsRequest::default()
                    .set_project(project_id)
                    .set_secret(secret_id)
                    .set_page_token(page_token),
            )
            .await?;
        response
            .versions
            .into_iter()
            .filter_map(|s| s.name)
            .for_each(|name| names.push(name));
        page_token = response.next_page_token;
        if page_token.as_ref().map(String::is_empty).unwrap_or(true) {
            break;
        }
    }
    Ok(names)
}

async fn get_all_secret_names(
    client: &smo::SecretManagerServiceClient,
    project_id: &str,
) -> Result<Vec<String>> {
    let mut names = Vec::new();
    let mut page_token = None::<String>;
    loop {
        let response = client
            .list_secrets(
                smo::model::ListSecretsRequest::default()
                    .set_project(project_id)
                    .set_page_token(page_token),
            )
            .await?;
        response
            .secrets
            .into_iter()
            .filter_map(|s| s.name)
            .for_each(|name| names.push(name));
        page_token = response.next_page_token;
        if page_token.as_ref().map(String::is_empty).unwrap_or(true) {
            break;
        }
    }
    Ok(names)
}
