# \ActionsApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**actions_slash_add_custom_labels_to_self_hosted_runner_for_org**](ActionsApi.md#actions_slash_add_custom_labels_to_self_hosted_runner_for_org) | **POST** /orgs/{org}/actions/runners/{runner_id}/labels | Add custom labels to a self-hosted runner for an organization
[**actions_slash_add_custom_labels_to_self_hosted_runner_for_repo**](ActionsApi.md#actions_slash_add_custom_labels_to_self_hosted_runner_for_repo) | **POST** /repos/{owner}/{repo}/actions/runners/{runner_id}/labels | Add custom labels to a self-hosted runner for a repository
[**actions_slash_add_repo_access_to_self_hosted_runner_group_in_org**](ActionsApi.md#actions_slash_add_repo_access_to_self_hosted_runner_group_in_org) | **PUT** /orgs/{org}/actions/runner-groups/{runner_group_id}/repositories/{repository_id} | Add repository access to a self-hosted runner group in an organization
[**actions_slash_add_selected_repo_to_org_secret**](ActionsApi.md#actions_slash_add_selected_repo_to_org_secret) | **PUT** /orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id} | Add selected repository to an organization secret
[**actions_slash_add_selected_repo_to_org_variable**](ActionsApi.md#actions_slash_add_selected_repo_to_org_variable) | **PUT** /orgs/{org}/actions/variables/{name}/repositories/{repository_id} | Add selected repository to an organization variable
[**actions_slash_add_self_hosted_runner_to_group_for_org**](ActionsApi.md#actions_slash_add_self_hosted_runner_to_group_for_org) | **PUT** /orgs/{org}/actions/runner-groups/{runner_group_id}/runners/{runner_id} | Add a self-hosted runner to a group for an organization
[**actions_slash_approve_workflow_run**](ActionsApi.md#actions_slash_approve_workflow_run) | **POST** /repos/{owner}/{repo}/actions/runs/{run_id}/approve | Approve a workflow run for a fork pull request
[**actions_slash_cancel_workflow_run**](ActionsApi.md#actions_slash_cancel_workflow_run) | **POST** /repos/{owner}/{repo}/actions/runs/{run_id}/cancel | Cancel a workflow run
[**actions_slash_create_environment_variable**](ActionsApi.md#actions_slash_create_environment_variable) | **POST** /repos/{owner}/{repo}/environments/{environment_name}/variables | Create an environment variable
[**actions_slash_create_or_update_environment_secret**](ActionsApi.md#actions_slash_create_or_update_environment_secret) | **PUT** /repos/{owner}/{repo}/environments/{environment_name}/secrets/{secret_name} | Create or update an environment secret
[**actions_slash_create_or_update_org_secret**](ActionsApi.md#actions_slash_create_or_update_org_secret) | **PUT** /orgs/{org}/actions/secrets/{secret_name} | Create or update an organization secret
[**actions_slash_create_or_update_repo_secret**](ActionsApi.md#actions_slash_create_or_update_repo_secret) | **PUT** /repos/{owner}/{repo}/actions/secrets/{secret_name} | Create or update a repository secret
[**actions_slash_create_org_variable**](ActionsApi.md#actions_slash_create_org_variable) | **POST** /orgs/{org}/actions/variables | Create an organization variable
[**actions_slash_create_registration_token_for_org**](ActionsApi.md#actions_slash_create_registration_token_for_org) | **POST** /orgs/{org}/actions/runners/registration-token | Create a registration token for an organization
[**actions_slash_create_registration_token_for_repo**](ActionsApi.md#actions_slash_create_registration_token_for_repo) | **POST** /repos/{owner}/{repo}/actions/runners/registration-token | Create a registration token for a repository
[**actions_slash_create_remove_token_for_org**](ActionsApi.md#actions_slash_create_remove_token_for_org) | **POST** /orgs/{org}/actions/runners/remove-token | Create a remove token for an organization
[**actions_slash_create_remove_token_for_repo**](ActionsApi.md#actions_slash_create_remove_token_for_repo) | **POST** /repos/{owner}/{repo}/actions/runners/remove-token | Create a remove token for a repository
[**actions_slash_create_repo_variable**](ActionsApi.md#actions_slash_create_repo_variable) | **POST** /repos/{owner}/{repo}/actions/variables | Create a repository variable
[**actions_slash_create_self_hosted_runner_group_for_org**](ActionsApi.md#actions_slash_create_self_hosted_runner_group_for_org) | **POST** /orgs/{org}/actions/runner-groups | Create a self-hosted runner group for an organization
[**actions_slash_create_workflow_dispatch**](ActionsApi.md#actions_slash_create_workflow_dispatch) | **POST** /repos/{owner}/{repo}/actions/workflows/{workflow_id}/dispatches | Create a workflow dispatch event
[**actions_slash_delete_actions_cache_by_id**](ActionsApi.md#actions_slash_delete_actions_cache_by_id) | **DELETE** /repos/{owner}/{repo}/actions/caches/{cache_id} | Delete a GitHub Actions cache for a repository (using a cache ID)
[**actions_slash_delete_actions_cache_by_key**](ActionsApi.md#actions_slash_delete_actions_cache_by_key) | **DELETE** /repos/{owner}/{repo}/actions/caches | Delete GitHub Actions caches for a repository (using a cache key)
[**actions_slash_delete_artifact**](ActionsApi.md#actions_slash_delete_artifact) | **DELETE** /repos/{owner}/{repo}/actions/artifacts/{artifact_id} | Delete an artifact
[**actions_slash_delete_environment_secret**](ActionsApi.md#actions_slash_delete_environment_secret) | **DELETE** /repos/{owner}/{repo}/environments/{environment_name}/secrets/{secret_name} | Delete an environment secret
[**actions_slash_delete_environment_variable**](ActionsApi.md#actions_slash_delete_environment_variable) | **DELETE** /repos/{owner}/{repo}/environments/{environment_name}/variables/{name} | Delete an environment variable
[**actions_slash_delete_org_secret**](ActionsApi.md#actions_slash_delete_org_secret) | **DELETE** /orgs/{org}/actions/secrets/{secret_name} | Delete an organization secret
[**actions_slash_delete_org_variable**](ActionsApi.md#actions_slash_delete_org_variable) | **DELETE** /orgs/{org}/actions/variables/{name} | Delete an organization variable
[**actions_slash_delete_repo_secret**](ActionsApi.md#actions_slash_delete_repo_secret) | **DELETE** /repos/{owner}/{repo}/actions/secrets/{secret_name} | Delete a repository secret
[**actions_slash_delete_repo_variable**](ActionsApi.md#actions_slash_delete_repo_variable) | **DELETE** /repos/{owner}/{repo}/actions/variables/{name} | Delete a repository variable
[**actions_slash_delete_self_hosted_runner_from_org**](ActionsApi.md#actions_slash_delete_self_hosted_runner_from_org) | **DELETE** /orgs/{org}/actions/runners/{runner_id} | Delete a self-hosted runner from an organization
[**actions_slash_delete_self_hosted_runner_from_repo**](ActionsApi.md#actions_slash_delete_self_hosted_runner_from_repo) | **DELETE** /repos/{owner}/{repo}/actions/runners/{runner_id} | Delete a self-hosted runner from a repository
[**actions_slash_delete_self_hosted_runner_group_from_org**](ActionsApi.md#actions_slash_delete_self_hosted_runner_group_from_org) | **DELETE** /orgs/{org}/actions/runner-groups/{runner_group_id} | Delete a self-hosted runner group from an organization
[**actions_slash_delete_workflow_run**](ActionsApi.md#actions_slash_delete_workflow_run) | **DELETE** /repos/{owner}/{repo}/actions/runs/{run_id} | Delete a workflow run
[**actions_slash_delete_workflow_run_logs**](ActionsApi.md#actions_slash_delete_workflow_run_logs) | **DELETE** /repos/{owner}/{repo}/actions/runs/{run_id}/logs | Delete workflow run logs
[**actions_slash_disable_selected_repository_github_actions_organization**](ActionsApi.md#actions_slash_disable_selected_repository_github_actions_organization) | **DELETE** /orgs/{org}/actions/permissions/repositories/{repository_id} | Disable a selected repository for GitHub Actions in an organization
[**actions_slash_disable_workflow**](ActionsApi.md#actions_slash_disable_workflow) | **PUT** /repos/{owner}/{repo}/actions/workflows/{workflow_id}/disable | Disable a workflow
[**actions_slash_download_artifact**](ActionsApi.md#actions_slash_download_artifact) | **GET** /repos/{owner}/{repo}/actions/artifacts/{artifact_id}/{archive_format} | Download an artifact
[**actions_slash_download_job_logs_for_workflow_run**](ActionsApi.md#actions_slash_download_job_logs_for_workflow_run) | **GET** /repos/{owner}/{repo}/actions/jobs/{job_id}/logs | Download job logs for a workflow run
[**actions_slash_download_workflow_run_attempt_logs**](ActionsApi.md#actions_slash_download_workflow_run_attempt_logs) | **GET** /repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}/logs | Download workflow run attempt logs
[**actions_slash_download_workflow_run_logs**](ActionsApi.md#actions_slash_download_workflow_run_logs) | **GET** /repos/{owner}/{repo}/actions/runs/{run_id}/logs | Download workflow run logs
[**actions_slash_enable_selected_repository_github_actions_organization**](ActionsApi.md#actions_slash_enable_selected_repository_github_actions_organization) | **PUT** /orgs/{org}/actions/permissions/repositories/{repository_id} | Enable a selected repository for GitHub Actions in an organization
[**actions_slash_enable_workflow**](ActionsApi.md#actions_slash_enable_workflow) | **PUT** /repos/{owner}/{repo}/actions/workflows/{workflow_id}/enable | Enable a workflow
[**actions_slash_force_cancel_workflow_run**](ActionsApi.md#actions_slash_force_cancel_workflow_run) | **POST** /repos/{owner}/{repo}/actions/runs/{run_id}/force-cancel | Force cancel a workflow run
[**actions_slash_generate_runner_jitconfig_for_org**](ActionsApi.md#actions_slash_generate_runner_jitconfig_for_org) | **POST** /orgs/{org}/actions/runners/generate-jitconfig | Create configuration for a just-in-time runner for an organization
[**actions_slash_generate_runner_jitconfig_for_repo**](ActionsApi.md#actions_slash_generate_runner_jitconfig_for_repo) | **POST** /repos/{owner}/{repo}/actions/runners/generate-jitconfig | Create configuration for a just-in-time runner for a repository
[**actions_slash_get_actions_cache_list**](ActionsApi.md#actions_slash_get_actions_cache_list) | **GET** /repos/{owner}/{repo}/actions/caches | List GitHub Actions caches for a repository
[**actions_slash_get_actions_cache_usage**](ActionsApi.md#actions_slash_get_actions_cache_usage) | **GET** /repos/{owner}/{repo}/actions/cache/usage | Get GitHub Actions cache usage for a repository
[**actions_slash_get_actions_cache_usage_by_repo_for_org**](ActionsApi.md#actions_slash_get_actions_cache_usage_by_repo_for_org) | **GET** /orgs/{org}/actions/cache/usage-by-repository | List repositories with GitHub Actions cache usage for an organization
[**actions_slash_get_actions_cache_usage_for_org**](ActionsApi.md#actions_slash_get_actions_cache_usage_for_org) | **GET** /orgs/{org}/actions/cache/usage | Get GitHub Actions cache usage for an organization
[**actions_slash_get_allowed_actions_organization**](ActionsApi.md#actions_slash_get_allowed_actions_organization) | **GET** /orgs/{org}/actions/permissions/selected-actions | Get allowed actions and reusable workflows for an organization
[**actions_slash_get_allowed_actions_repository**](ActionsApi.md#actions_slash_get_allowed_actions_repository) | **GET** /repos/{owner}/{repo}/actions/permissions/selected-actions | Get allowed actions and reusable workflows for a repository
[**actions_slash_get_artifact**](ActionsApi.md#actions_slash_get_artifact) | **GET** /repos/{owner}/{repo}/actions/artifacts/{artifact_id} | Get an artifact
[**actions_slash_get_custom_oidc_sub_claim_for_repo**](ActionsApi.md#actions_slash_get_custom_oidc_sub_claim_for_repo) | **GET** /repos/{owner}/{repo}/actions/oidc/customization/sub | Get the customization template for an OIDC subject claim for a repository
[**actions_slash_get_environment_public_key**](ActionsApi.md#actions_slash_get_environment_public_key) | **GET** /repos/{owner}/{repo}/environments/{environment_name}/secrets/public-key | Get an environment public key
[**actions_slash_get_environment_secret**](ActionsApi.md#actions_slash_get_environment_secret) | **GET** /repos/{owner}/{repo}/environments/{environment_name}/secrets/{secret_name} | Get an environment secret
[**actions_slash_get_environment_variable**](ActionsApi.md#actions_slash_get_environment_variable) | **GET** /repos/{owner}/{repo}/environments/{environment_name}/variables/{name} | Get an environment variable
[**actions_slash_get_github_actions_default_workflow_permissions_organization**](ActionsApi.md#actions_slash_get_github_actions_default_workflow_permissions_organization) | **GET** /orgs/{org}/actions/permissions/workflow | Get default workflow permissions for an organization
[**actions_slash_get_github_actions_default_workflow_permissions_repository**](ActionsApi.md#actions_slash_get_github_actions_default_workflow_permissions_repository) | **GET** /repos/{owner}/{repo}/actions/permissions/workflow | Get default workflow permissions for a repository
[**actions_slash_get_github_actions_permissions_organization**](ActionsApi.md#actions_slash_get_github_actions_permissions_organization) | **GET** /orgs/{org}/actions/permissions | Get GitHub Actions permissions for an organization
[**actions_slash_get_github_actions_permissions_repository**](ActionsApi.md#actions_slash_get_github_actions_permissions_repository) | **GET** /repos/{owner}/{repo}/actions/permissions | Get GitHub Actions permissions for a repository
[**actions_slash_get_job_for_workflow_run**](ActionsApi.md#actions_slash_get_job_for_workflow_run) | **GET** /repos/{owner}/{repo}/actions/jobs/{job_id} | Get a job for a workflow run
[**actions_slash_get_org_public_key**](ActionsApi.md#actions_slash_get_org_public_key) | **GET** /orgs/{org}/actions/secrets/public-key | Get an organization public key
[**actions_slash_get_org_secret**](ActionsApi.md#actions_slash_get_org_secret) | **GET** /orgs/{org}/actions/secrets/{secret_name} | Get an organization secret
[**actions_slash_get_org_variable**](ActionsApi.md#actions_slash_get_org_variable) | **GET** /orgs/{org}/actions/variables/{name} | Get an organization variable
[**actions_slash_get_pending_deployments_for_run**](ActionsApi.md#actions_slash_get_pending_deployments_for_run) | **GET** /repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments | Get pending deployments for a workflow run
[**actions_slash_get_repo_public_key**](ActionsApi.md#actions_slash_get_repo_public_key) | **GET** /repos/{owner}/{repo}/actions/secrets/public-key | Get a repository public key
[**actions_slash_get_repo_secret**](ActionsApi.md#actions_slash_get_repo_secret) | **GET** /repos/{owner}/{repo}/actions/secrets/{secret_name} | Get a repository secret
[**actions_slash_get_repo_variable**](ActionsApi.md#actions_slash_get_repo_variable) | **GET** /repos/{owner}/{repo}/actions/variables/{name} | Get a repository variable
[**actions_slash_get_reviews_for_run**](ActionsApi.md#actions_slash_get_reviews_for_run) | **GET** /repos/{owner}/{repo}/actions/runs/{run_id}/approvals | Get the review history for a workflow run
[**actions_slash_get_self_hosted_runner_for_org**](ActionsApi.md#actions_slash_get_self_hosted_runner_for_org) | **GET** /orgs/{org}/actions/runners/{runner_id} | Get a self-hosted runner for an organization
[**actions_slash_get_self_hosted_runner_for_repo**](ActionsApi.md#actions_slash_get_self_hosted_runner_for_repo) | **GET** /repos/{owner}/{repo}/actions/runners/{runner_id} | Get a self-hosted runner for a repository
[**actions_slash_get_self_hosted_runner_group_for_org**](ActionsApi.md#actions_slash_get_self_hosted_runner_group_for_org) | **GET** /orgs/{org}/actions/runner-groups/{runner_group_id} | Get a self-hosted runner group for an organization
[**actions_slash_get_workflow**](ActionsApi.md#actions_slash_get_workflow) | **GET** /repos/{owner}/{repo}/actions/workflows/{workflow_id} | Get a workflow
[**actions_slash_get_workflow_access_to_repository**](ActionsApi.md#actions_slash_get_workflow_access_to_repository) | **GET** /repos/{owner}/{repo}/actions/permissions/access | Get the level of access for workflows outside of the repository
[**actions_slash_get_workflow_run**](ActionsApi.md#actions_slash_get_workflow_run) | **GET** /repos/{owner}/{repo}/actions/runs/{run_id} | Get a workflow run
[**actions_slash_get_workflow_run_attempt**](ActionsApi.md#actions_slash_get_workflow_run_attempt) | **GET** /repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number} | Get a workflow run attempt
[**actions_slash_get_workflow_run_usage**](ActionsApi.md#actions_slash_get_workflow_run_usage) | **GET** /repos/{owner}/{repo}/actions/runs/{run_id}/timing | Get workflow run usage
[**actions_slash_get_workflow_usage**](ActionsApi.md#actions_slash_get_workflow_usage) | **GET** /repos/{owner}/{repo}/actions/workflows/{workflow_id}/timing | Get workflow usage
[**actions_slash_list_artifacts_for_repo**](ActionsApi.md#actions_slash_list_artifacts_for_repo) | **GET** /repos/{owner}/{repo}/actions/artifacts | List artifacts for a repository
[**actions_slash_list_environment_secrets**](ActionsApi.md#actions_slash_list_environment_secrets) | **GET** /repos/{owner}/{repo}/environments/{environment_name}/secrets | List environment secrets
[**actions_slash_list_environment_variables**](ActionsApi.md#actions_slash_list_environment_variables) | **GET** /repos/{owner}/{repo}/environments/{environment_name}/variables | List environment variables
[**actions_slash_list_jobs_for_workflow_run**](ActionsApi.md#actions_slash_list_jobs_for_workflow_run) | **GET** /repos/{owner}/{repo}/actions/runs/{run_id}/jobs | List jobs for a workflow run
[**actions_slash_list_jobs_for_workflow_run_attempt**](ActionsApi.md#actions_slash_list_jobs_for_workflow_run_attempt) | **GET** /repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}/jobs | List jobs for a workflow run attempt
[**actions_slash_list_labels_for_self_hosted_runner_for_org**](ActionsApi.md#actions_slash_list_labels_for_self_hosted_runner_for_org) | **GET** /orgs/{org}/actions/runners/{runner_id}/labels | List labels for a self-hosted runner for an organization
[**actions_slash_list_labels_for_self_hosted_runner_for_repo**](ActionsApi.md#actions_slash_list_labels_for_self_hosted_runner_for_repo) | **GET** /repos/{owner}/{repo}/actions/runners/{runner_id}/labels | List labels for a self-hosted runner for a repository
[**actions_slash_list_org_secrets**](ActionsApi.md#actions_slash_list_org_secrets) | **GET** /orgs/{org}/actions/secrets | List organization secrets
[**actions_slash_list_org_variables**](ActionsApi.md#actions_slash_list_org_variables) | **GET** /orgs/{org}/actions/variables | List organization variables
[**actions_slash_list_repo_access_to_self_hosted_runner_group_in_org**](ActionsApi.md#actions_slash_list_repo_access_to_self_hosted_runner_group_in_org) | **GET** /orgs/{org}/actions/runner-groups/{runner_group_id}/repositories | List repository access to a self-hosted runner group in an organization
[**actions_slash_list_repo_organization_secrets**](ActionsApi.md#actions_slash_list_repo_organization_secrets) | **GET** /repos/{owner}/{repo}/actions/organization-secrets | List repository organization secrets
[**actions_slash_list_repo_organization_variables**](ActionsApi.md#actions_slash_list_repo_organization_variables) | **GET** /repos/{owner}/{repo}/actions/organization-variables | List repository organization variables
[**actions_slash_list_repo_secrets**](ActionsApi.md#actions_slash_list_repo_secrets) | **GET** /repos/{owner}/{repo}/actions/secrets | List repository secrets
[**actions_slash_list_repo_variables**](ActionsApi.md#actions_slash_list_repo_variables) | **GET** /repos/{owner}/{repo}/actions/variables | List repository variables
[**actions_slash_list_repo_workflows**](ActionsApi.md#actions_slash_list_repo_workflows) | **GET** /repos/{owner}/{repo}/actions/workflows | List repository workflows
[**actions_slash_list_runner_applications_for_org**](ActionsApi.md#actions_slash_list_runner_applications_for_org) | **GET** /orgs/{org}/actions/runners/downloads | List runner applications for an organization
[**actions_slash_list_runner_applications_for_repo**](ActionsApi.md#actions_slash_list_runner_applications_for_repo) | **GET** /repos/{owner}/{repo}/actions/runners/downloads | List runner applications for a repository
[**actions_slash_list_selected_repos_for_org_secret**](ActionsApi.md#actions_slash_list_selected_repos_for_org_secret) | **GET** /orgs/{org}/actions/secrets/{secret_name}/repositories | List selected repositories for an organization secret
[**actions_slash_list_selected_repos_for_org_variable**](ActionsApi.md#actions_slash_list_selected_repos_for_org_variable) | **GET** /orgs/{org}/actions/variables/{name}/repositories | List selected repositories for an organization variable
[**actions_slash_list_selected_repositories_enabled_github_actions_organization**](ActionsApi.md#actions_slash_list_selected_repositories_enabled_github_actions_organization) | **GET** /orgs/{org}/actions/permissions/repositories | List selected repositories enabled for GitHub Actions in an organization
[**actions_slash_list_self_hosted_runner_groups_for_org**](ActionsApi.md#actions_slash_list_self_hosted_runner_groups_for_org) | **GET** /orgs/{org}/actions/runner-groups | List self-hosted runner groups for an organization
[**actions_slash_list_self_hosted_runners_for_org**](ActionsApi.md#actions_slash_list_self_hosted_runners_for_org) | **GET** /orgs/{org}/actions/runners | List self-hosted runners for an organization
[**actions_slash_list_self_hosted_runners_for_repo**](ActionsApi.md#actions_slash_list_self_hosted_runners_for_repo) | **GET** /repos/{owner}/{repo}/actions/runners | List self-hosted runners for a repository
[**actions_slash_list_self_hosted_runners_in_group_for_org**](ActionsApi.md#actions_slash_list_self_hosted_runners_in_group_for_org) | **GET** /orgs/{org}/actions/runner-groups/{runner_group_id}/runners | List self-hosted runners in a group for an organization
[**actions_slash_list_workflow_run_artifacts**](ActionsApi.md#actions_slash_list_workflow_run_artifacts) | **GET** /repos/{owner}/{repo}/actions/runs/{run_id}/artifacts | List workflow run artifacts
[**actions_slash_list_workflow_runs**](ActionsApi.md#actions_slash_list_workflow_runs) | **GET** /repos/{owner}/{repo}/actions/workflows/{workflow_id}/runs | List workflow runs for a workflow
[**actions_slash_list_workflow_runs_for_repo**](ActionsApi.md#actions_slash_list_workflow_runs_for_repo) | **GET** /repos/{owner}/{repo}/actions/runs | List workflow runs for a repository
[**actions_slash_re_run_job_for_workflow_run**](ActionsApi.md#actions_slash_re_run_job_for_workflow_run) | **POST** /repos/{owner}/{repo}/actions/jobs/{job_id}/rerun | Re-run a job from a workflow run
[**actions_slash_re_run_workflow**](ActionsApi.md#actions_slash_re_run_workflow) | **POST** /repos/{owner}/{repo}/actions/runs/{run_id}/rerun | Re-run a workflow
[**actions_slash_re_run_workflow_failed_jobs**](ActionsApi.md#actions_slash_re_run_workflow_failed_jobs) | **POST** /repos/{owner}/{repo}/actions/runs/{run_id}/rerun-failed-jobs | Re-run failed jobs from a workflow run
[**actions_slash_remove_all_custom_labels_from_self_hosted_runner_for_org**](ActionsApi.md#actions_slash_remove_all_custom_labels_from_self_hosted_runner_for_org) | **DELETE** /orgs/{org}/actions/runners/{runner_id}/labels | Remove all custom labels from a self-hosted runner for an organization
[**actions_slash_remove_all_custom_labels_from_self_hosted_runner_for_repo**](ActionsApi.md#actions_slash_remove_all_custom_labels_from_self_hosted_runner_for_repo) | **DELETE** /repos/{owner}/{repo}/actions/runners/{runner_id}/labels | Remove all custom labels from a self-hosted runner for a repository
[**actions_slash_remove_custom_label_from_self_hosted_runner_for_org**](ActionsApi.md#actions_slash_remove_custom_label_from_self_hosted_runner_for_org) | **DELETE** /orgs/{org}/actions/runners/{runner_id}/labels/{name} | Remove a custom label from a self-hosted runner for an organization
[**actions_slash_remove_custom_label_from_self_hosted_runner_for_repo**](ActionsApi.md#actions_slash_remove_custom_label_from_self_hosted_runner_for_repo) | **DELETE** /repos/{owner}/{repo}/actions/runners/{runner_id}/labels/{name} | Remove a custom label from a self-hosted runner for a repository
[**actions_slash_remove_repo_access_to_self_hosted_runner_group_in_org**](ActionsApi.md#actions_slash_remove_repo_access_to_self_hosted_runner_group_in_org) | **DELETE** /orgs/{org}/actions/runner-groups/{runner_group_id}/repositories/{repository_id} | Remove repository access to a self-hosted runner group in an organization
[**actions_slash_remove_selected_repo_from_org_secret**](ActionsApi.md#actions_slash_remove_selected_repo_from_org_secret) | **DELETE** /orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id} | Remove selected repository from an organization secret
[**actions_slash_remove_selected_repo_from_org_variable**](ActionsApi.md#actions_slash_remove_selected_repo_from_org_variable) | **DELETE** /orgs/{org}/actions/variables/{name}/repositories/{repository_id} | Remove selected repository from an organization variable
[**actions_slash_remove_self_hosted_runner_from_group_for_org**](ActionsApi.md#actions_slash_remove_self_hosted_runner_from_group_for_org) | **DELETE** /orgs/{org}/actions/runner-groups/{runner_group_id}/runners/{runner_id} | Remove a self-hosted runner from a group for an organization
[**actions_slash_review_custom_gates_for_run**](ActionsApi.md#actions_slash_review_custom_gates_for_run) | **POST** /repos/{owner}/{repo}/actions/runs/{run_id}/deployment_protection_rule | Review custom deployment protection rules for a workflow run
[**actions_slash_review_pending_deployments_for_run**](ActionsApi.md#actions_slash_review_pending_deployments_for_run) | **POST** /repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments | Review pending deployments for a workflow run
[**actions_slash_set_allowed_actions_organization**](ActionsApi.md#actions_slash_set_allowed_actions_organization) | **PUT** /orgs/{org}/actions/permissions/selected-actions | Set allowed actions and reusable workflows for an organization
[**actions_slash_set_allowed_actions_repository**](ActionsApi.md#actions_slash_set_allowed_actions_repository) | **PUT** /repos/{owner}/{repo}/actions/permissions/selected-actions | Set allowed actions and reusable workflows for a repository
[**actions_slash_set_custom_labels_for_self_hosted_runner_for_org**](ActionsApi.md#actions_slash_set_custom_labels_for_self_hosted_runner_for_org) | **PUT** /orgs/{org}/actions/runners/{runner_id}/labels | Set custom labels for a self-hosted runner for an organization
[**actions_slash_set_custom_labels_for_self_hosted_runner_for_repo**](ActionsApi.md#actions_slash_set_custom_labels_for_self_hosted_runner_for_repo) | **PUT** /repos/{owner}/{repo}/actions/runners/{runner_id}/labels | Set custom labels for a self-hosted runner for a repository
[**actions_slash_set_custom_oidc_sub_claim_for_repo**](ActionsApi.md#actions_slash_set_custom_oidc_sub_claim_for_repo) | **PUT** /repos/{owner}/{repo}/actions/oidc/customization/sub | Set the customization template for an OIDC subject claim for a repository
[**actions_slash_set_github_actions_default_workflow_permissions_organization**](ActionsApi.md#actions_slash_set_github_actions_default_workflow_permissions_organization) | **PUT** /orgs/{org}/actions/permissions/workflow | Set default workflow permissions for an organization
[**actions_slash_set_github_actions_default_workflow_permissions_repository**](ActionsApi.md#actions_slash_set_github_actions_default_workflow_permissions_repository) | **PUT** /repos/{owner}/{repo}/actions/permissions/workflow | Set default workflow permissions for a repository
[**actions_slash_set_github_actions_permissions_organization**](ActionsApi.md#actions_slash_set_github_actions_permissions_organization) | **PUT** /orgs/{org}/actions/permissions | Set GitHub Actions permissions for an organization
[**actions_slash_set_github_actions_permissions_repository**](ActionsApi.md#actions_slash_set_github_actions_permissions_repository) | **PUT** /repos/{owner}/{repo}/actions/permissions | Set GitHub Actions permissions for a repository
[**actions_slash_set_repo_access_to_self_hosted_runner_group_in_org**](ActionsApi.md#actions_slash_set_repo_access_to_self_hosted_runner_group_in_org) | **PUT** /orgs/{org}/actions/runner-groups/{runner_group_id}/repositories | Set repository access for a self-hosted runner group in an organization
[**actions_slash_set_selected_repos_for_org_secret**](ActionsApi.md#actions_slash_set_selected_repos_for_org_secret) | **PUT** /orgs/{org}/actions/secrets/{secret_name}/repositories | Set selected repositories for an organization secret
[**actions_slash_set_selected_repos_for_org_variable**](ActionsApi.md#actions_slash_set_selected_repos_for_org_variable) | **PUT** /orgs/{org}/actions/variables/{name}/repositories | Set selected repositories for an organization variable
[**actions_slash_set_selected_repositories_enabled_github_actions_organization**](ActionsApi.md#actions_slash_set_selected_repositories_enabled_github_actions_organization) | **PUT** /orgs/{org}/actions/permissions/repositories | Set selected repositories enabled for GitHub Actions in an organization
[**actions_slash_set_self_hosted_runners_in_group_for_org**](ActionsApi.md#actions_slash_set_self_hosted_runners_in_group_for_org) | **PUT** /orgs/{org}/actions/runner-groups/{runner_group_id}/runners | Set self-hosted runners in a group for an organization
[**actions_slash_set_workflow_access_to_repository**](ActionsApi.md#actions_slash_set_workflow_access_to_repository) | **PUT** /repos/{owner}/{repo}/actions/permissions/access | Set the level of access for workflows outside of the repository
[**actions_slash_update_environment_variable**](ActionsApi.md#actions_slash_update_environment_variable) | **PATCH** /repos/{owner}/{repo}/environments/{environment_name}/variables/{name} | Update an environment variable
[**actions_slash_update_org_variable**](ActionsApi.md#actions_slash_update_org_variable) | **PATCH** /orgs/{org}/actions/variables/{name} | Update an organization variable
[**actions_slash_update_repo_variable**](ActionsApi.md#actions_slash_update_repo_variable) | **PATCH** /repos/{owner}/{repo}/actions/variables/{name} | Update a repository variable
[**actions_slash_update_self_hosted_runner_group_for_org**](ActionsApi.md#actions_slash_update_self_hosted_runner_group_for_org) | **PATCH** /orgs/{org}/actions/runner-groups/{runner_group_id} | Update a self-hosted runner group for an organization



## actions_slash_add_custom_labels_to_self_hosted_runner_for_org

> models::ActionsListLabelsForSelfHostedRunnerForOrg200Response actions_slash_add_custom_labels_to_self_hosted_runner_for_org(org, runner_id, actions_add_custom_labels_to_self_hosted_runner_for_org_request)
Add custom labels to a self-hosted runner for an organization

Adds custom labels to a self-hosted runner configured in an organization.  Authenticated users must have admin access to the organization to use this endpoint.  OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**runner_id** | **i32** | Unique identifier of the self-hosted runner. | [required] |
**actions_add_custom_labels_to_self_hosted_runner_for_org_request** | [**ActionsAddCustomLabelsToSelfHostedRunnerForOrgRequest**](ActionsAddCustomLabelsToSelfHostedRunnerForOrgRequest.md) |  | [required] |

### Return type

[**models::ActionsListLabelsForSelfHostedRunnerForOrg200Response**](actions_list_labels_for_self_hosted_runner_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_add_custom_labels_to_self_hosted_runner_for_repo

> models::ActionsListLabelsForSelfHostedRunnerForOrg200Response actions_slash_add_custom_labels_to_self_hosted_runner_for_repo(owner, repo, runner_id, actions_add_custom_labels_to_self_hosted_runner_for_org_request)
Add custom labels to a self-hosted runner for a repository

Adds custom labels to a self-hosted runner configured in a repository.  Authenticated users must have admin access to the organization to use this endpoint.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**runner_id** | **i32** | Unique identifier of the self-hosted runner. | [required] |
**actions_add_custom_labels_to_self_hosted_runner_for_org_request** | [**ActionsAddCustomLabelsToSelfHostedRunnerForOrgRequest**](ActionsAddCustomLabelsToSelfHostedRunnerForOrgRequest.md) |  | [required] |

### Return type

[**models::ActionsListLabelsForSelfHostedRunnerForOrg200Response**](actions_list_labels_for_self_hosted_runner_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_add_repo_access_to_self_hosted_runner_group_in_org

> actions_slash_add_repo_access_to_self_hosted_runner_group_in_org(org, runner_group_id, repository_id)
Add repository access to a self-hosted runner group in an organization

Adds a repository to the list of repositories that can access a self-hosted runner group. The runner group must have `visibility` set to `selected`. For more information, see \"[Create a self-hosted runner group for an organization](#create-a-self-hosted-runner-group-for-an-organization).\"  OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**runner_group_id** | **i32** | Unique identifier of the self-hosted runner group. | [required] |
**repository_id** | **i32** | The unique identifier of the repository. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_add_selected_repo_to_org_secret

> actions_slash_add_selected_repo_to_org_secret(org, secret_name, repository_id)
Add selected repository to an organization secret

Adds a repository to an organization secret when the `visibility` for repository access is set to `selected`. For more information about setting the visibility, see [Create or update an organization secret](https://docs.github.com/rest/actions/secrets#create-or-update-an-organization-secret).  Authenticated users must have collaborator access to a repository to create, update, or read secrets.  OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |
**repository_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_add_selected_repo_to_org_variable

> actions_slash_add_selected_repo_to_org_variable(org, name, repository_id)
Add selected repository to an organization variable

Adds a repository to an organization variable that is available to selected repositories. Organization variables that are available to selected repositories have their `visibility` field set to `selected`.  Authenticated users must have collaborator access to a repository to create, update, or read secrets.  OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**name** | **String** | The name of the variable. | [required] |
**repository_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_add_self_hosted_runner_to_group_for_org

> actions_slash_add_self_hosted_runner_to_group_for_org(org, runner_group_id, runner_id)
Add a self-hosted runner to a group for an organization

Adds a self-hosted runner to a runner group configured in an organization.  OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**runner_group_id** | **i32** | Unique identifier of the self-hosted runner group. | [required] |
**runner_id** | **i32** | Unique identifier of the self-hosted runner. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_approve_workflow_run

> serde_json::Value actions_slash_approve_workflow_run(owner, repo, run_id)
Approve a workflow run for a fork pull request

Approves a workflow run for a pull request from a public fork of a first time contributor. For more information, see [\"Approving workflow runs from public forks](https://docs.github.com/actions/managing-workflow-runs/approving-workflow-runs-from-public-forks).\"  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_cancel_workflow_run

> serde_json::Value actions_slash_cancel_workflow_run(owner, repo, run_id)
Cancel a workflow run

Cancels a workflow run using its `id`.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_create_environment_variable

> serde_json::Value actions_slash_create_environment_variable(owner, repo, environment_name, actions_create_repo_variable_request)
Create an environment variable

Create an environment variable that you can reference in a GitHub Actions workflow.  Authenticated users must have collaborator access to a repository to create, update, or read variables.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**actions_create_repo_variable_request** | [**ActionsCreateRepoVariableRequest**](ActionsCreateRepoVariableRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_create_or_update_environment_secret

> serde_json::Value actions_slash_create_or_update_environment_secret(owner, repo, environment_name, secret_name, actions_create_or_update_environment_secret_request)
Create or update an environment secret

Creates or updates an environment secret with an encrypted value. Encrypt your secret using [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see \"[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api).\"  Authenticated users must have collaborator access to a repository to create, update, or read secrets.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |
**actions_create_or_update_environment_secret_request** | [**ActionsCreateOrUpdateEnvironmentSecretRequest**](ActionsCreateOrUpdateEnvironmentSecretRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_create_or_update_org_secret

> serde_json::Value actions_slash_create_or_update_org_secret(org, secret_name, actions_create_or_update_org_secret_request)
Create or update an organization secret

Creates or updates an organization secret with an encrypted value. Encrypt your secret using [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see \"[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api).\"  Authenticated users must have collaborator access to a repository to create, update, or read secrets.  OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |
**actions_create_or_update_org_secret_request** | [**ActionsCreateOrUpdateOrgSecretRequest**](ActionsCreateOrUpdateOrgSecretRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_create_or_update_repo_secret

> serde_json::Value actions_slash_create_or_update_repo_secret(owner, repo, secret_name, actions_create_or_update_repo_secret_request)
Create or update a repository secret

Creates or updates a repository secret with an encrypted value. Encrypt your secret using [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see \"[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api).\"  Authenticated users must have collaborator access to a repository to create, update, or read secrets.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |
**actions_create_or_update_repo_secret_request** | [**ActionsCreateOrUpdateRepoSecretRequest**](ActionsCreateOrUpdateRepoSecretRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_create_org_variable

> serde_json::Value actions_slash_create_org_variable(org, actions_create_org_variable_request)
Create an organization variable

Creates an organization variable that you can reference in a GitHub Actions workflow.  Authenticated users must have collaborator access to a repository to create, update, or read variables.  OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**actions_create_org_variable_request** | [**ActionsCreateOrgVariableRequest**](ActionsCreateOrgVariableRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_create_registration_token_for_org

> models::AuthenticationToken actions_slash_create_registration_token_for_org(org)
Create a registration token for an organization

Returns a token that you can pass to the `config` script. The token expires after one hour.  For example, you can replace `TOKEN` in the following example with the registration token provided by this endpoint to configure your self-hosted runner:  ``` ./config.sh --url https://github.com/octo-org --token TOKEN ```  Authenticated users must have admin access to the organization to use this endpoint.  OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::AuthenticationToken**](authentication-token.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_create_registration_token_for_repo

> models::AuthenticationToken actions_slash_create_registration_token_for_repo(owner, repo)
Create a registration token for a repository

Returns a token that you can pass to the `config` script. The token expires after one hour.  For example, you can replace `TOKEN` in the following example with the registration token provided by this endpoint to configure your self-hosted runner:  ``` ./config.sh --url https://github.com/octo-org --token TOKEN ```  Authenticated users must have admin access to the repository to use this endpoint.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::AuthenticationToken**](authentication-token.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_create_remove_token_for_org

> models::AuthenticationToken actions_slash_create_remove_token_for_org(org)
Create a remove token for an organization

Returns a token that you can pass to the `config` script to remove a self-hosted runner from an organization. The token expires after one hour.  For example, you can replace `TOKEN` in the following example with the registration token provided by this endpoint to remove your self-hosted runner from an organization:  ``` ./config.sh remove --token TOKEN ```  Authenticated users must have admin access to the organization to use this endpoint.  OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::AuthenticationToken**](authentication-token.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_create_remove_token_for_repo

> models::AuthenticationToken actions_slash_create_remove_token_for_repo(owner, repo)
Create a remove token for a repository

Returns a token that you can pass to the `config` script to remove a self-hosted runner from an repository. The token expires after one hour.  For example, you can replace `TOKEN` in the following example with the registration token provided by this endpoint to remove your self-hosted runner from an organization:  ``` ./config.sh remove --token TOKEN ```  Authenticated users must have admin access to the repository to use this endpoint.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::AuthenticationToken**](authentication-token.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_create_repo_variable

> serde_json::Value actions_slash_create_repo_variable(owner, repo, actions_create_repo_variable_request)
Create a repository variable

Creates a repository variable that you can reference in a GitHub Actions workflow.  Authenticated users must have collaborator access to a repository to create, update, or read variables.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**actions_create_repo_variable_request** | [**ActionsCreateRepoVariableRequest**](ActionsCreateRepoVariableRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_create_self_hosted_runner_group_for_org

> models::RunnerGroupsOrg actions_slash_create_self_hosted_runner_group_for_org(org, actions_create_self_hosted_runner_group_for_org_request)
Create a self-hosted runner group for an organization

Creates a new self-hosted runner group for an organization.  OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**actions_create_self_hosted_runner_group_for_org_request** | [**ActionsCreateSelfHostedRunnerGroupForOrgRequest**](ActionsCreateSelfHostedRunnerGroupForOrgRequest.md) |  | [required] |

### Return type

[**models::RunnerGroupsOrg**](runner-groups-org.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_create_workflow_dispatch

> actions_slash_create_workflow_dispatch(owner, repo, workflow_id, actions_create_workflow_dispatch_request)
Create a workflow dispatch event

You can use this endpoint to manually trigger a GitHub Actions workflow run. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`.  You must configure your GitHub Actions workflow to run when the [`workflow_dispatch` webhook](/developers/webhooks-and-events/webhook-events-and-payloads#workflow_dispatch) event occurs. The `inputs` are configured in the workflow file. For more information about how to configure the `workflow_dispatch` event in the workflow file, see \"[Events that trigger workflows](/actions/reference/events-that-trigger-workflows#workflow_dispatch).\"  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**workflow_id** | [**ActionsGetWorkflowWorkflowIdParameter**](.md) | The ID of the workflow. You can also pass the workflow file name as a string. | [required] |
**actions_create_workflow_dispatch_request** | [**ActionsCreateWorkflowDispatchRequest**](ActionsCreateWorkflowDispatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_delete_actions_cache_by_id

> actions_slash_delete_actions_cache_by_id(owner, repo, cache_id)
Delete a GitHub Actions cache for a repository (using a cache ID)

Deletes a GitHub Actions cache for a repository, using a cache ID.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**cache_id** | **i32** | The unique identifier of the GitHub Actions cache. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_delete_actions_cache_by_key

> models::ActionsCacheList actions_slash_delete_actions_cache_by_key(owner, repo, key, r#ref)
Delete GitHub Actions caches for a repository (using a cache key)

Deletes one or more GitHub Actions caches for a repository, using a complete cache key. By default, all caches that match the provided key are deleted, but you can optionally provide a Git ref to restrict deletions to caches that match both the provided key and the Git ref.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**key** | **String** | A key for identifying the cache. | [required] |
**r#ref** | Option<**String**> | The full Git reference for narrowing down the cache. The `ref` for a branch should be formatted as `refs/heads/<branch name>`. To reference a pull request use `refs/pull/<number>/merge`. |  |

### Return type

[**models::ActionsCacheList**](actions-cache-list.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_delete_artifact

> actions_slash_delete_artifact(owner, repo, artifact_id)
Delete an artifact

Deletes an artifact for a workflow run. OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**artifact_id** | **i32** | The unique identifier of the artifact. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_delete_environment_secret

> actions_slash_delete_environment_secret(owner, repo, environment_name, secret_name)
Delete an environment secret

Deletes a secret in an environment using the secret name.  Authenticated users must have collaborator access to a repository to create, update, or read secrets.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_delete_environment_variable

> actions_slash_delete_environment_variable(owner, repo, name, environment_name)
Delete an environment variable

Deletes an environment variable using the variable name.  Authenticated users must have collaborator access to a repository to create, update, or read variables.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**name** | **String** | The name of the variable. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_delete_org_secret

> actions_slash_delete_org_secret(org, secret_name)
Delete an organization secret

Deletes a secret in an organization using the secret name.  Authenticated users must have collaborator access to a repository to create, update, or read secrets.  OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_delete_org_variable

> actions_slash_delete_org_variable(org, name)
Delete an organization variable

Deletes an organization variable using the variable name.  Authenticated users must have collaborator access to a repository to create, update, or read variables.  OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**name** | **String** | The name of the variable. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_delete_repo_secret

> actions_slash_delete_repo_secret(owner, repo, secret_name)
Delete a repository secret

Deletes a secret in a repository using the secret name.  Authenticated users must have collaborator access to a repository to create, update, or read secrets.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_delete_repo_variable

> actions_slash_delete_repo_variable(owner, repo, name)
Delete a repository variable

Deletes a repository variable using the variable name.  Authenticated users must have collaborator access to a repository to create, update, or read variables.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**name** | **String** | The name of the variable. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_delete_self_hosted_runner_from_org

> actions_slash_delete_self_hosted_runner_from_org(org, runner_id)
Delete a self-hosted runner from an organization

Forces the removal of a self-hosted runner from an organization. You can use this endpoint to completely remove the runner when the machine you were using no longer exists.  Authenticated users must have admin access to the organization to use this endpoint.  OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**runner_id** | **i32** | Unique identifier of the self-hosted runner. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_delete_self_hosted_runner_from_repo

> actions_slash_delete_self_hosted_runner_from_repo(owner, repo, runner_id)
Delete a self-hosted runner from a repository

Forces the removal of a self-hosted runner from a repository. You can use this endpoint to completely remove the runner when the machine you were using no longer exists.  Authenticated users must have admin access to the repository to use this endpoint.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**runner_id** | **i32** | Unique identifier of the self-hosted runner. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_delete_self_hosted_runner_group_from_org

> actions_slash_delete_self_hosted_runner_group_from_org(org, runner_group_id)
Delete a self-hosted runner group from an organization

Deletes a self-hosted runner group for an organization.  OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**runner_group_id** | **i32** | Unique identifier of the self-hosted runner group. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_delete_workflow_run

> actions_slash_delete_workflow_run(owner, repo, run_id)
Delete a workflow run

Deletes a specific workflow run.  Anyone with write access to the repository can use this endpoint.  If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_delete_workflow_run_logs

> actions_slash_delete_workflow_run_logs(owner, repo, run_id)
Delete workflow run logs

Deletes all logs for a workflow run.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_disable_selected_repository_github_actions_organization

> actions_slash_disable_selected_repository_github_actions_organization(org, repository_id)
Disable a selected repository for GitHub Actions in an organization

Removes a repository from the list of selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be configured to `selected`. For more information, see \"[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization).\"  OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**repository_id** | **i32** | The unique identifier of the repository. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_disable_workflow

> actions_slash_disable_workflow(owner, repo, workflow_id)
Disable a workflow

Disables a workflow and sets the `state` of the workflow to `disabled_manually`. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**workflow_id** | [**ActionsGetWorkflowWorkflowIdParameter**](.md) | The ID of the workflow. You can also pass the workflow file name as a string. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_download_artifact

> actions_slash_download_artifact(owner, repo, artifact_id, archive_format)
Download an artifact

Gets a redirect URL to download an archive for a repository. This URL expires after 1 minute. Look for `Location:` in the response header to find the URL for the download. The `:archive_format` must be `zip`.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**artifact_id** | **i32** | The unique identifier of the artifact. | [required] |
**archive_format** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_download_job_logs_for_workflow_run

> actions_slash_download_job_logs_for_workflow_run(owner, repo, job_id)
Download job logs for a workflow run

Gets a redirect URL to download a plain text file of logs for a workflow job. This link expires after 1 minute. Look for `Location:` in the response header to find the URL for the download.  Anyone with read access to the repository can use this endpoint.  If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**job_id** | **i32** | The unique identifier of the job. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_download_workflow_run_attempt_logs

> actions_slash_download_workflow_run_attempt_logs(owner, repo, run_id, attempt_number)
Download workflow run attempt logs

Gets a redirect URL to download an archive of log files for a specific workflow run attempt. This link expires after 1 minute. Look for `Location:` in the response header to find the URL for the download.  Anyone with read access to the repository can use this endpoint.  If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |
**attempt_number** | **i32** | The attempt number of the workflow run. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_download_workflow_run_logs

> actions_slash_download_workflow_run_logs(owner, repo, run_id)
Download workflow run logs

Gets a redirect URL to download an archive of log files for a workflow run. This link expires after 1 minute. Look for `Location:` in the response header to find the URL for the download.  Anyone with read access to the repository can use this endpoint.  If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_enable_selected_repository_github_actions_organization

> actions_slash_enable_selected_repository_github_actions_organization(org, repository_id)
Enable a selected repository for GitHub Actions in an organization

Adds a repository to the list of selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be must be configured to `selected`. For more information, see \"[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization).\"  OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**repository_id** | **i32** | The unique identifier of the repository. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_enable_workflow

> actions_slash_enable_workflow(owner, repo, workflow_id)
Enable a workflow

Enables a workflow and sets the `state` of the workflow to `active`. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**workflow_id** | [**ActionsGetWorkflowWorkflowIdParameter**](.md) | The ID of the workflow. You can also pass the workflow file name as a string. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_force_cancel_workflow_run

> serde_json::Value actions_slash_force_cancel_workflow_run(owner, repo, run_id)
Force cancel a workflow run

Cancels a workflow run and bypasses conditions that would otherwise cause a workflow execution to continue, such as an `always()` condition on a job. You should only use this endpoint to cancel a workflow run when the workflow run is not responding to [`POST /repos/{owner}/{repo}/actions/runs/{run_id}/cancel`](/rest/actions/workflow-runs#cancel-a-workflow-run).  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_generate_runner_jitconfig_for_org

> models::ActionsGenerateRunnerJitconfigForOrg201Response actions_slash_generate_runner_jitconfig_for_org(org, actions_generate_runner_jitconfig_for_org_request)
Create configuration for a just-in-time runner for an organization

Generates a configuration that can be passed to the runner application at startup.  The authenticated user must have admin access to the organization.  OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**actions_generate_runner_jitconfig_for_org_request** | [**ActionsGenerateRunnerJitconfigForOrgRequest**](ActionsGenerateRunnerJitconfigForOrgRequest.md) |  | [required] |

### Return type

[**models::ActionsGenerateRunnerJitconfigForOrg201Response**](actions_generate_runner_jitconfig_for_org_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_generate_runner_jitconfig_for_repo

> models::ActionsGenerateRunnerJitconfigForOrg201Response actions_slash_generate_runner_jitconfig_for_repo(owner, repo, actions_generate_runner_jitconfig_for_org_request)
Create configuration for a just-in-time runner for a repository

Generates a configuration that can be passed to the runner application at startup.  The authenticated user must have admin access to the repository.  OAuth tokens and personal access tokens (classic) need the`repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**actions_generate_runner_jitconfig_for_org_request** | [**ActionsGenerateRunnerJitconfigForOrgRequest**](ActionsGenerateRunnerJitconfigForOrgRequest.md) |  | [required] |

### Return type

[**models::ActionsGenerateRunnerJitconfigForOrg201Response**](actions_generate_runner_jitconfig_for_org_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_actions_cache_list

> models::ActionsCacheList actions_slash_get_actions_cache_list(owner, repo, per_page, page, r#ref, key, sort, direction)
List GitHub Actions caches for a repository

Lists the GitHub Actions caches for a repository.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**r#ref** | Option<**String**> | The full Git reference for narrowing down the cache. The `ref` for a branch should be formatted as `refs/heads/<branch name>`. To reference a pull request use `refs/pull/<number>/merge`. |  |
**key** | Option<**String**> | An explicit key or prefix for identifying the cache |  |
**sort** | Option<**String**> | The property to sort the results by. `created_at` means when the cache was created. `last_accessed_at` means when the cache was last accessed. `size_in_bytes` is the size of the cache in bytes. |  |[default to last_accessed_at]
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]

### Return type

[**models::ActionsCacheList**](actions-cache-list.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_actions_cache_usage

> models::ActionsCacheUsageByRepository actions_slash_get_actions_cache_usage(owner, repo)
Get GitHub Actions cache usage for a repository

Gets GitHub Actions cache usage for a repository. The data fetched using this API is refreshed approximately every 5 minutes, so values returned from this endpoint may take at least 5 minutes to get updated.  Anyone with read access to the repository can use this endpoint.  If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::ActionsCacheUsageByRepository**](actions-cache-usage-by-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_actions_cache_usage_by_repo_for_org

> models::ActionsGetActionsCacheUsageByRepoForOrg200Response actions_slash_get_actions_cache_usage_by_repo_for_org(org, per_page, page)
List repositories with GitHub Actions cache usage for an organization

Lists repositories and their GitHub Actions cache usage for an organization. The data fetched using this API is refreshed approximately every 5 minutes, so values returned from this endpoint may take at least 5 minutes to get updated.  OAuth tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ActionsGetActionsCacheUsageByRepoForOrg200Response**](actions_get_actions_cache_usage_by_repo_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_actions_cache_usage_for_org

> models::ActionsCacheUsageOrgEnterprise actions_slash_get_actions_cache_usage_for_org(org)
Get GitHub Actions cache usage for an organization

Gets the total GitHub Actions cache usage for an organization. The data fetched using this API is refreshed approximately every 5 minutes, so values returned from this endpoint may take at least 5 minutes to get updated.  OAuth tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::ActionsCacheUsageOrgEnterprise**](actions-cache-usage-org-enterprise.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_allowed_actions_organization

> models::SelectedActions actions_slash_get_allowed_actions_organization(org)
Get allowed actions and reusable workflows for an organization

Gets the selected actions and reusable workflows that are allowed in an organization. To use this endpoint, the organization permission policy for `allowed_actions` must be configured to `selected`. For more information, see \"[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization).\"  OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::SelectedActions**](selected-actions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_allowed_actions_repository

> models::SelectedActions actions_slash_get_allowed_actions_repository(owner, repo)
Get allowed actions and reusable workflows for a repository

Gets the settings for selected actions and reusable workflows that are allowed in a repository. To use this endpoint, the repository policy for `allowed_actions` must be configured to `selected`. For more information, see \"[Set GitHub Actions permissions for a repository](#set-github-actions-permissions-for-a-repository).\"  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::SelectedActions**](selected-actions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_artifact

> models::Artifact actions_slash_get_artifact(owner, repo, artifact_id)
Get an artifact

Gets a specific artifact for a workflow run.  Anyone with read access to the repository can use this endpoint.  If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**artifact_id** | **i32** | The unique identifier of the artifact. | [required] |

### Return type

[**models::Artifact**](artifact.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_custom_oidc_sub_claim_for_repo

> models::OidcCustomSubRepo actions_slash_get_custom_oidc_sub_claim_for_repo(owner, repo)
Get the customization template for an OIDC subject claim for a repository

Gets the customization template for an OpenID Connect (OIDC) subject claim.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::OidcCustomSubRepo**](oidc-custom-sub-repo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_environment_public_key

> models::ActionsPublicKey actions_slash_get_environment_public_key(owner, repo, environment_name)
Get an environment public key

Get the public key for an environment, which you need to encrypt environment secrets. You need to encrypt a secret before you can create or update secrets.  Anyone with read access to the repository can use this endpoint.  If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |

### Return type

[**models::ActionsPublicKey**](actions-public-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_environment_secret

> models::ActionsSecret actions_slash_get_environment_secret(owner, repo, environment_name, secret_name)
Get an environment secret

Gets a single environment secret without revealing its encrypted value.  Authenticated users must have collaborator access to a repository to create, update, or read secrets.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |

### Return type

[**models::ActionsSecret**](actions-secret.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_environment_variable

> models::ActionsVariable actions_slash_get_environment_variable(owner, repo, environment_name, name)
Get an environment variable

Gets a specific variable in an environment.  Authenticated users must have collaborator access to a repository to create, update, or read variables.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**name** | **String** | The name of the variable. | [required] |

### Return type

[**models::ActionsVariable**](actions-variable.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_github_actions_default_workflow_permissions_organization

> models::ActionsGetDefaultWorkflowPermissions actions_slash_get_github_actions_default_workflow_permissions_organization(org)
Get default workflow permissions for an organization

Gets the default workflow permissions granted to the `GITHUB_TOKEN` when running workflows in an organization, as well as whether GitHub Actions can submit approving pull request reviews. For more information, see \"[Setting the permissions of the GITHUB_TOKEN for your organization](https://docs.github.com/organizations/managing-organization-settings/disabling-or-limiting-github-actions-for-your-organization#setting-the-permissions-of-the-github_token-for-your-organization).\"  OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::ActionsGetDefaultWorkflowPermissions**](actions-get-default-workflow-permissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_github_actions_default_workflow_permissions_repository

> models::ActionsGetDefaultWorkflowPermissions actions_slash_get_github_actions_default_workflow_permissions_repository(owner, repo)
Get default workflow permissions for a repository

Gets the default workflow permissions granted to the `GITHUB_TOKEN` when running workflows in a repository, as well as if GitHub Actions can submit approving pull request reviews. For more information, see \"[Setting the permissions of the GITHUB_TOKEN for your repository](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository#setting-the-permissions-of-the-github_token-for-your-repository).\"  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::ActionsGetDefaultWorkflowPermissions**](actions-get-default-workflow-permissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_github_actions_permissions_organization

> models::ActionsOrganizationPermissions actions_slash_get_github_actions_permissions_organization(org)
Get GitHub Actions permissions for an organization

Gets the GitHub Actions permissions policy for repositories and allowed actions and reusable workflows in an organization.  OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::ActionsOrganizationPermissions**](actions-organization-permissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_github_actions_permissions_repository

> models::ActionsRepositoryPermissions actions_slash_get_github_actions_permissions_repository(owner, repo)
Get GitHub Actions permissions for a repository

Gets the GitHub Actions permissions policy for a repository, including whether GitHub Actions is enabled and the actions and reusable workflows allowed to run in the repository.  OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::ActionsRepositoryPermissions**](actions-repository-permissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_job_for_workflow_run

> models::Job actions_slash_get_job_for_workflow_run(owner, repo, job_id)
Get a job for a workflow run

Gets a specific job in a workflow run.  Anyone with read access to the repository can use this endpoint.  If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**job_id** | **i32** | The unique identifier of the job. | [required] |

### Return type

[**models::Job**](job.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_org_public_key

> models::ActionsPublicKey actions_slash_get_org_public_key(org)
Get an organization public key

Gets your public key, which you need to encrypt secrets. You need to encrypt a secret before you can create or update secrets.  The authenticated user must have collaborator access to a repository to create, update, or read secrets.  OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::ActionsPublicKey**](actions-public-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_org_secret

> models::OrganizationActionsSecret actions_slash_get_org_secret(org, secret_name)
Get an organization secret

Gets a single organization secret without revealing its encrypted value.  The authenticated user must have collaborator access to a repository to create, update, or read secrets  OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |

### Return type

[**models::OrganizationActionsSecret**](organization-actions-secret.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_org_variable

> models::OrganizationActionsVariable actions_slash_get_org_variable(org, name)
Get an organization variable

Gets a specific variable in an organization.  The authenticated user must have collaborator access to a repository to create, update, or read variables.  OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**name** | **String** | The name of the variable. | [required] |

### Return type

[**models::OrganizationActionsVariable**](organization-actions-variable.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_pending_deployments_for_run

> Vec<models::PendingDeployment> actions_slash_get_pending_deployments_for_run(owner, repo, run_id)
Get pending deployments for a workflow run

Get all deployment environments for a workflow run that are waiting for protection rules to pass.  Anyone with read access to the repository can use this endpoint.  If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |

### Return type

[**Vec<models::PendingDeployment>**](pending-deployment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_repo_public_key

> models::ActionsPublicKey actions_slash_get_repo_public_key(owner, repo)
Get a repository public key

Gets your public key, which you need to encrypt secrets. You need to encrypt a secret before you can create or update secrets.  Anyone with read access to the repository can use this endpoint.  If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::ActionsPublicKey**](actions-public-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_repo_secret

> models::ActionsSecret actions_slash_get_repo_secret(owner, repo, secret_name)
Get a repository secret

Gets a single repository secret without revealing its encrypted value.  The authenticated user must have collaborator access to the repository to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |

### Return type

[**models::ActionsSecret**](actions-secret.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_repo_variable

> models::ActionsVariable actions_slash_get_repo_variable(owner, repo, name)
Get a repository variable

Gets a specific variable in a repository.  The authenticated user must have collaborator access to the repository to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**name** | **String** | The name of the variable. | [required] |

### Return type

[**models::ActionsVariable**](actions-variable.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_reviews_for_run

> Vec<models::EnvironmentApprovals> actions_slash_get_reviews_for_run(owner, repo, run_id)
Get the review history for a workflow run

Anyone with read access to the repository can use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |

### Return type

[**Vec<models::EnvironmentApprovals>**](environment-approvals.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_self_hosted_runner_for_org

> models::Runner actions_slash_get_self_hosted_runner_for_org(org, runner_id)
Get a self-hosted runner for an organization

Gets a specific self-hosted runner configured in an organization.  Authenticated users must have admin access to the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**runner_id** | **i32** | Unique identifier of the self-hosted runner. | [required] |

### Return type

[**models::Runner**](runner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_self_hosted_runner_for_repo

> models::Runner actions_slash_get_self_hosted_runner_for_repo(owner, repo, runner_id)
Get a self-hosted runner for a repository

Gets a specific self-hosted runner configured in a repository.  Authenticated users must have admin access to the repository to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**runner_id** | **i32** | Unique identifier of the self-hosted runner. | [required] |

### Return type

[**models::Runner**](runner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_self_hosted_runner_group_for_org

> models::RunnerGroupsOrg actions_slash_get_self_hosted_runner_group_for_org(org, runner_group_id)
Get a self-hosted runner group for an organization

Gets a specific self-hosted runner group for an organization.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**runner_group_id** | **i32** | Unique identifier of the self-hosted runner group. | [required] |

### Return type

[**models::RunnerGroupsOrg**](runner-groups-org.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_workflow

> models::Workflow actions_slash_get_workflow(owner, repo, workflow_id)
Get a workflow

Gets a specific workflow. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`.  Anyone with read access to the repository can use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**workflow_id** | [**ActionsGetWorkflowWorkflowIdParameter**](.md) | The ID of the workflow. You can also pass the workflow file name as a string. | [required] |

### Return type

[**models::Workflow**](workflow.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_workflow_access_to_repository

> models::ActionsWorkflowAccessToRepository actions_slash_get_workflow_access_to_repository(owner, repo)
Get the level of access for workflows outside of the repository

Gets the level of access that workflows outside of the repository have to actions and reusable workflows in the repository. This endpoint only applies to private repositories. For more information, see \"[Allowing access to components in a private repository](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository#allowing-access-to-components-in-a-private-repository).\"  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::ActionsWorkflowAccessToRepository**](actions-workflow-access-to-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_workflow_run

> models::WorkflowRun actions_slash_get_workflow_run(owner, repo, run_id, exclude_pull_requests)
Get a workflow run

Gets a specific workflow run.  Anyone with read access to the repository can use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |
**exclude_pull_requests** | Option<**bool**> | If `true` pull requests are omitted from the response (empty array). |  |[default to false]

### Return type

[**models::WorkflowRun**](workflow-run.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_workflow_run_attempt

> models::WorkflowRun actions_slash_get_workflow_run_attempt(owner, repo, run_id, attempt_number, exclude_pull_requests)
Get a workflow run attempt

Gets a specific workflow run attempt.  Anyone with read access to the repository can use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |
**attempt_number** | **i32** | The attempt number of the workflow run. | [required] |
**exclude_pull_requests** | Option<**bool**> | If `true` pull requests are omitted from the response (empty array). |  |[default to false]

### Return type

[**models::WorkflowRun**](workflow-run.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_workflow_run_usage

> models::WorkflowRunUsage actions_slash_get_workflow_run_usage(owner, repo, run_id)
Get workflow run usage

Gets the number of billable minutes and total run time for a specific workflow run. Billable minutes only apply to workflows in private repositories that use GitHub-hosted runners. Usage is listed for each GitHub-hosted runner operating system in milliseconds. Any job re-runs are also included in the usage. The usage does not include the multiplier for macOS and Windows runners and is not rounded up to the nearest whole minute. For more information, see \"[Managing billing for GitHub Actions](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)\".  Anyone with read access to the repository can use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |

### Return type

[**models::WorkflowRunUsage**](workflow-run-usage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_get_workflow_usage

> models::WorkflowUsage actions_slash_get_workflow_usage(owner, repo, workflow_id)
Get workflow usage

Gets the number of billable minutes used by a specific workflow during the current billing cycle. Billable minutes only apply to workflows in private repositories that use GitHub-hosted runners. Usage is listed for each GitHub-hosted runner operating system in milliseconds. Any job re-runs are also included in the usage. The usage does not include the multiplier for macOS and Windows runners and is not rounded up to the nearest whole minute. For more information, see \"[Managing billing for GitHub Actions](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)\".  You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`.  Anyone with read access to the repository can use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**workflow_id** | [**ActionsGetWorkflowWorkflowIdParameter**](.md) | The ID of the workflow. You can also pass the workflow file name as a string. | [required] |

### Return type

[**models::WorkflowUsage**](workflow-usage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_artifacts_for_repo

> models::ActionsListArtifactsForRepo200Response actions_slash_list_artifacts_for_repo(owner, repo, per_page, page, name)
List artifacts for a repository

Lists all artifacts for a repository.  Anyone with read access to the repository can use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**name** | Option<**String**> | The name field of an artifact. When specified, only artifacts with this name will be returned. |  |

### Return type

[**models::ActionsListArtifactsForRepo200Response**](actions_list_artifacts_for_repo_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_environment_secrets

> models::ActionsListRepoOrganizationSecrets200Response actions_slash_list_environment_secrets(owner, repo, environment_name, per_page, page)
List environment secrets

Lists all secrets available in an environment without revealing their encrypted values.  Authenticated users must have collaborator access to a repository to create, update, or read secrets.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ActionsListRepoOrganizationSecrets200Response**](actions_list_repo_organization_secrets_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_environment_variables

> models::ActionsListRepoOrganizationVariables200Response actions_slash_list_environment_variables(owner, repo, environment_name, per_page, page)
List environment variables

Lists all environment variables.  Authenticated users must have collaborator access to a repository to create, update, or read variables.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 30). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 10]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ActionsListRepoOrganizationVariables200Response**](actions_list_repo_organization_variables_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_jobs_for_workflow_run

> models::ActionsListJobsForWorkflowRunAttempt200Response actions_slash_list_jobs_for_workflow_run(owner, repo, run_id, filter, per_page, page)
List jobs for a workflow run

Lists jobs for a workflow run. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#parameters).  Anyone with read access to the repository can use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |
**filter** | Option<**String**> | Filters jobs by their `completed_at` timestamp. `latest` returns jobs from the most recent execution of the workflow run. `all` returns all jobs for a workflow run, including from old executions of the workflow run. |  |[default to latest]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ActionsListJobsForWorkflowRunAttempt200Response**](actions_list_jobs_for_workflow_run_attempt_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_jobs_for_workflow_run_attempt

> models::ActionsListJobsForWorkflowRunAttempt200Response actions_slash_list_jobs_for_workflow_run_attempt(owner, repo, run_id, attempt_number, per_page, page)
List jobs for a workflow run attempt

Lists jobs for a specific workflow run attempt. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#parameters).  Anyone with read access to the repository can use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint  with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |
**attempt_number** | **i32** | The attempt number of the workflow run. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ActionsListJobsForWorkflowRunAttempt200Response**](actions_list_jobs_for_workflow_run_attempt_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_labels_for_self_hosted_runner_for_org

> models::ActionsListLabelsForSelfHostedRunnerForOrg200Response actions_slash_list_labels_for_self_hosted_runner_for_org(org, runner_id)
List labels for a self-hosted runner for an organization

Lists all labels for a self-hosted runner configured in an organization.  Authenticated users must have admin access to the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**runner_id** | **i32** | Unique identifier of the self-hosted runner. | [required] |

### Return type

[**models::ActionsListLabelsForSelfHostedRunnerForOrg200Response**](actions_list_labels_for_self_hosted_runner_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_labels_for_self_hosted_runner_for_repo

> models::ActionsListLabelsForSelfHostedRunnerForOrg200Response actions_slash_list_labels_for_self_hosted_runner_for_repo(owner, repo, runner_id)
List labels for a self-hosted runner for a repository

Lists all labels for a self-hosted runner configured in a repository.  Authenticated users must have admin access to the repository to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**runner_id** | **i32** | Unique identifier of the self-hosted runner. | [required] |

### Return type

[**models::ActionsListLabelsForSelfHostedRunnerForOrg200Response**](actions_list_labels_for_self_hosted_runner_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_org_secrets

> models::ActionsListOrgSecrets200Response actions_slash_list_org_secrets(org, per_page, page)
List organization secrets

Lists all secrets available in an organization without revealing their encrypted values.  Authenticated users must have collaborator access to a repository to create, update, or read secrets.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ActionsListOrgSecrets200Response**](actions_list_org_secrets_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_org_variables

> models::ActionsListOrgVariables200Response actions_slash_list_org_variables(org, per_page, page)
List organization variables

Lists all organization variables.  Authenticated users must have collaborator access to a repository to create, update, or read variables.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 30). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 10]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ActionsListOrgVariables200Response**](actions_list_org_variables_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_repo_access_to_self_hosted_runner_group_in_org

> models::ActionsListRepoAccessToSelfHostedRunnerGroupInOrg200Response actions_slash_list_repo_access_to_self_hosted_runner_group_in_org(org, runner_group_id, page, per_page)
List repository access to a self-hosted runner group in an organization

Lists the repositories with access to a self-hosted runner group configured in an organization.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**runner_group_id** | **i32** | Unique identifier of the self-hosted runner group. | [required] |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**models::ActionsListRepoAccessToSelfHostedRunnerGroupInOrg200Response**](actions_list_repo_access_to_self_hosted_runner_group_in_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_repo_organization_secrets

> models::ActionsListRepoOrganizationSecrets200Response actions_slash_list_repo_organization_secrets(owner, repo, per_page, page)
List repository organization secrets

Lists all organization secrets shared with a repository without revealing their encrypted values.  Authenticated users must have collaborator access to a repository to create, update, or read secrets.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ActionsListRepoOrganizationSecrets200Response**](actions_list_repo_organization_secrets_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_repo_organization_variables

> models::ActionsListRepoOrganizationVariables200Response actions_slash_list_repo_organization_variables(owner, repo, per_page, page)
List repository organization variables

Lists all organization variables shared with a repository.  Authenticated users must have collaborator access to a repository to create, update, or read variables.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 30). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 10]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ActionsListRepoOrganizationVariables200Response**](actions_list_repo_organization_variables_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_repo_secrets

> models::ActionsListRepoOrganizationSecrets200Response actions_slash_list_repo_secrets(owner, repo, per_page, page)
List repository secrets

Lists all secrets available in a repository without revealing their encrypted values.  Authenticated users must have collaborator access to a repository to create, update, or read secrets.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ActionsListRepoOrganizationSecrets200Response**](actions_list_repo_organization_secrets_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_repo_variables

> models::ActionsListRepoOrganizationVariables200Response actions_slash_list_repo_variables(owner, repo, per_page, page)
List repository variables

Lists all repository variables.  Authenticated users must have collaborator access to a repository to create, update, or read variables.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 30). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 10]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ActionsListRepoOrganizationVariables200Response**](actions_list_repo_organization_variables_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_repo_workflows

> models::ActionsListRepoWorkflows200Response actions_slash_list_repo_workflows(owner, repo, per_page, page)
List repository workflows

Lists the workflows in a repository.  Anyone with read access to the repository can use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ActionsListRepoWorkflows200Response**](actions_list_repo_workflows_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_runner_applications_for_org

> Vec<models::RunnerApplication> actions_slash_list_runner_applications_for_org(org)
List runner applications for an organization

Lists binaries for the runner application that you can download and run.  Authenticated users must have admin access to the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.  If the repository is private, the `repo` scope is also required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**Vec<models::RunnerApplication>**](runner-application.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_runner_applications_for_repo

> Vec<models::RunnerApplication> actions_slash_list_runner_applications_for_repo(owner, repo)
List runner applications for a repository

Lists binaries for the runner application that you can download and run.  Authenticated users must have admin access to the repository to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**Vec<models::RunnerApplication>**](runner-application.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_selected_repos_for_org_secret

> models::ActionsListSelectedReposForOrgSecret200Response actions_slash_list_selected_repos_for_org_secret(org, secret_name, page, per_page)
List selected repositories for an organization secret

Lists all repositories that have been selected when the `visibility` for repository access to a secret is set to `selected`.  Authenticated users must have collaborator access to a repository to create, update, or read secrets.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**models::ActionsListSelectedReposForOrgSecret200Response**](actions_list_selected_repos_for_org_secret_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_selected_repos_for_org_variable

> models::ActionsListSelectedReposForOrgSecret200Response actions_slash_list_selected_repos_for_org_variable(org, name, page, per_page)
List selected repositories for an organization variable

Lists all repositories that can access an organization variable that is available to selected repositories.  Authenticated users must have collaborator access to a repository to create, update, or read variables.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**name** | **String** | The name of the variable. | [required] |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**models::ActionsListSelectedReposForOrgSecret200Response**](actions_list_selected_repos_for_org_secret_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_selected_repositories_enabled_github_actions_organization

> models::ActionsListSelectedRepositoriesEnabledGithubActionsOrganization200Response actions_slash_list_selected_repositories_enabled_github_actions_organization(org, per_page, page)
List selected repositories enabled for GitHub Actions in an organization

Lists the selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be configured to `selected`. For more information, see \"[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization).\"  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ActionsListSelectedRepositoriesEnabledGithubActionsOrganization200Response**](actions_list_selected_repositories_enabled_github_actions_organization_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_self_hosted_runner_groups_for_org

> models::ActionsListSelfHostedRunnerGroupsForOrg200Response actions_slash_list_self_hosted_runner_groups_for_org(org, per_page, page, visible_to_repository)
List self-hosted runner groups for an organization

Lists all self-hosted runner groups configured in an organization and inherited from an enterprise.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**visible_to_repository** | Option<**String**> | Only return runner groups that are allowed to be used by this repository. |  |

### Return type

[**models::ActionsListSelfHostedRunnerGroupsForOrg200Response**](actions_list_self_hosted_runner_groups_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_self_hosted_runners_for_org

> models::ActionsListSelfHostedRunnersForOrg200Response actions_slash_list_self_hosted_runners_for_org(org, name, per_page, page)
List self-hosted runners for an organization

Lists all self-hosted runners configured in an organization.  Authenticated users must have admin access to the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**name** | Option<**String**> | The name of a self-hosted runner. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ActionsListSelfHostedRunnersForOrg200Response**](actions_list_self_hosted_runners_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_self_hosted_runners_for_repo

> models::ActionsListSelfHostedRunnersForOrg200Response actions_slash_list_self_hosted_runners_for_repo(owner, repo, name, per_page, page)
List self-hosted runners for a repository

Lists all self-hosted runners configured in a repository.  Authenticated users must have admin access to the repository to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**name** | Option<**String**> | The name of a self-hosted runner. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ActionsListSelfHostedRunnersForOrg200Response**](actions_list_self_hosted_runners_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_self_hosted_runners_in_group_for_org

> models::ActionsListSelfHostedRunnersInGroupForOrg200Response actions_slash_list_self_hosted_runners_in_group_for_org(org, runner_group_id, per_page, page)
List self-hosted runners in a group for an organization

Lists self-hosted runners that are in a specific organization group.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**runner_group_id** | **i32** | Unique identifier of the self-hosted runner group. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ActionsListSelfHostedRunnersInGroupForOrg200Response**](actions_list_self_hosted_runners_in_group_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_workflow_run_artifacts

> models::ActionsListArtifactsForRepo200Response actions_slash_list_workflow_run_artifacts(owner, repo, run_id, per_page, page, name)
List workflow run artifacts

Lists artifacts for a workflow run.  Anyone with read access to the repository can use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**name** | Option<**String**> | The name field of an artifact. When specified, only artifacts with this name will be returned. |  |

### Return type

[**models::ActionsListArtifactsForRepo200Response**](actions_list_artifacts_for_repo_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_workflow_runs

> models::ActionsListWorkflowRunsForRepo200Response actions_slash_list_workflow_runs(owner, repo, workflow_id, actor, branch, event, status, per_page, page, created, exclude_pull_requests, check_suite_id, head_sha)
List workflow runs for a workflow

List all workflow runs for a workflow. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#parameters).  Anyone with read access to the repository can use this endpoint  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**workflow_id** | [**ActionsGetWorkflowWorkflowIdParameter**](.md) | The ID of the workflow. You can also pass the workflow file name as a string. | [required] |
**actor** | Option<**String**> | Returns someone's workflow runs. Use the login for the user who created the `push` associated with the check suite or workflow run. |  |
**branch** | Option<**String**> | Returns workflow runs associated with a branch. Use the name of the branch of the `push`. |  |
**event** | Option<**String**> | Returns workflow run triggered by the event you specify. For example, `push`, `pull_request` or `issue`. For more information, see \"[Events that trigger workflows](https://docs.github.com/actions/automating-your-workflow-with-github-actions/events-that-trigger-workflows).\" |  |
**status** | Option<**String**> | Returns workflow runs with the check run `status` or `conclusion` that you specify. For example, a conclusion can be `success` or a status can be `in_progress`. Only GitHub Actions can set a status of `waiting`, `pending`, or `requested`. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**created** | Option<**String**> | Returns workflow runs created within the given date-time range. For more information on the syntax, see \"[Understanding the search syntax](https://docs.github.com/search-github/getting-started-with-searching-on-github/understanding-the-search-syntax#query-for-dates).\" |  |
**exclude_pull_requests** | Option<**bool**> | If `true` pull requests are omitted from the response (empty array). |  |[default to false]
**check_suite_id** | Option<**i32**> | Returns workflow runs with the `check_suite_id` that you specify. |  |
**head_sha** | Option<**String**> | Only returns workflow runs that are associated with the specified `head_sha`. |  |

### Return type

[**models::ActionsListWorkflowRunsForRepo200Response**](actions_list_workflow_runs_for_repo_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_list_workflow_runs_for_repo

> models::ActionsListWorkflowRunsForRepo200Response actions_slash_list_workflow_runs_for_repo(owner, repo, actor, branch, event, status, per_page, page, created, exclude_pull_requests, check_suite_id, head_sha)
List workflow runs for a repository

Lists all workflow runs for a repository. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#parameters).  Anyone with read access to the repository can use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.  This API will return up to 1,000 results for each search when using the following parameters: `actor`, `branch`, `check_suite_id`, `created`, `event`, `head_sha`, `status`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**actor** | Option<**String**> | Returns someone's workflow runs. Use the login for the user who created the `push` associated with the check suite or workflow run. |  |
**branch** | Option<**String**> | Returns workflow runs associated with a branch. Use the name of the branch of the `push`. |  |
**event** | Option<**String**> | Returns workflow run triggered by the event you specify. For example, `push`, `pull_request` or `issue`. For more information, see \"[Events that trigger workflows](https://docs.github.com/actions/automating-your-workflow-with-github-actions/events-that-trigger-workflows).\" |  |
**status** | Option<**String**> | Returns workflow runs with the check run `status` or `conclusion` that you specify. For example, a conclusion can be `success` or a status can be `in_progress`. Only GitHub Actions can set a status of `waiting`, `pending`, or `requested`. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**created** | Option<**String**> | Returns workflow runs created within the given date-time range. For more information on the syntax, see \"[Understanding the search syntax](https://docs.github.com/search-github/getting-started-with-searching-on-github/understanding-the-search-syntax#query-for-dates).\" |  |
**exclude_pull_requests** | Option<**bool**> | If `true` pull requests are omitted from the response (empty array). |  |[default to false]
**check_suite_id** | Option<**i32**> | Returns workflow runs with the `check_suite_id` that you specify. |  |
**head_sha** | Option<**String**> | Only returns workflow runs that are associated with the specified `head_sha`. |  |

### Return type

[**models::ActionsListWorkflowRunsForRepo200Response**](actions_list_workflow_runs_for_repo_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_re_run_job_for_workflow_run

> serde_json::Value actions_slash_re_run_job_for_workflow_run(owner, repo, job_id, actions_re_run_job_for_workflow_run_request)
Re-run a job from a workflow run

Re-run a job and its dependent jobs in a workflow run.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**job_id** | **i32** | The unique identifier of the job. | [required] |
**actions_re_run_job_for_workflow_run_request** | Option<[**ActionsReRunJobForWorkflowRunRequest**](ActionsReRunJobForWorkflowRunRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_re_run_workflow

> serde_json::Value actions_slash_re_run_workflow(owner, repo, run_id, actions_re_run_job_for_workflow_run_request)
Re-run a workflow

Re-runs your workflow run using its `id`.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |
**actions_re_run_job_for_workflow_run_request** | Option<[**ActionsReRunJobForWorkflowRunRequest**](ActionsReRunJobForWorkflowRunRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_re_run_workflow_failed_jobs

> serde_json::Value actions_slash_re_run_workflow_failed_jobs(owner, repo, run_id, actions_re_run_job_for_workflow_run_request)
Re-run failed jobs from a workflow run

Re-run all of the failed jobs and their dependent jobs in a workflow run using the `id` of the workflow run.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |
**actions_re_run_job_for_workflow_run_request** | Option<[**ActionsReRunJobForWorkflowRunRequest**](ActionsReRunJobForWorkflowRunRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_remove_all_custom_labels_from_self_hosted_runner_for_org

> models::ActionsListLabelsForSelfHostedRunnerForOrg200Response actions_slash_remove_all_custom_labels_from_self_hosted_runner_for_org(org, runner_id)
Remove all custom labels from a self-hosted runner for an organization

Remove all custom labels from a self-hosted runner configured in an organization. Returns the remaining read-only labels from the runner.  Authenticated users must have admin access to the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**runner_id** | **i32** | Unique identifier of the self-hosted runner. | [required] |

### Return type

[**models::ActionsListLabelsForSelfHostedRunnerForOrg200Response**](actions_list_labels_for_self_hosted_runner_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_remove_all_custom_labels_from_self_hosted_runner_for_repo

> models::ActionsListLabelsForSelfHostedRunnerForOrg200Response actions_slash_remove_all_custom_labels_from_self_hosted_runner_for_repo(owner, repo, runner_id)
Remove all custom labels from a self-hosted runner for a repository

Remove all custom labels from a self-hosted runner configured in a repository. Returns the remaining read-only labels from the runner.  Authenticated users must have admin access to the repository to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**runner_id** | **i32** | Unique identifier of the self-hosted runner. | [required] |

### Return type

[**models::ActionsListLabelsForSelfHostedRunnerForOrg200Response**](actions_list_labels_for_self_hosted_runner_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_remove_custom_label_from_self_hosted_runner_for_org

> models::ActionsListLabelsForSelfHostedRunnerForOrg200Response actions_slash_remove_custom_label_from_self_hosted_runner_for_org(org, runner_id, name)
Remove a custom label from a self-hosted runner for an organization

Remove a custom label from a self-hosted runner configured in an organization. Returns the remaining labels from the runner.  This endpoint returns a `404 Not Found` status if the custom label is not present on the runner.  Authenticated users must have admin access to the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**runner_id** | **i32** | Unique identifier of the self-hosted runner. | [required] |
**name** | **String** | The name of a self-hosted runner's custom label. | [required] |

### Return type

[**models::ActionsListLabelsForSelfHostedRunnerForOrg200Response**](actions_list_labels_for_self_hosted_runner_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_remove_custom_label_from_self_hosted_runner_for_repo

> models::ActionsListLabelsForSelfHostedRunnerForOrg200Response actions_slash_remove_custom_label_from_self_hosted_runner_for_repo(owner, repo, runner_id, name)
Remove a custom label from a self-hosted runner for a repository

Remove a custom label from a self-hosted runner configured in a repository. Returns the remaining labels from the runner.  This endpoint returns a `404 Not Found` status if the custom label is not present on the runner.  Authenticated users must have admin access to the repository to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**runner_id** | **i32** | Unique identifier of the self-hosted runner. | [required] |
**name** | **String** | The name of a self-hosted runner's custom label. | [required] |

### Return type

[**models::ActionsListLabelsForSelfHostedRunnerForOrg200Response**](actions_list_labels_for_self_hosted_runner_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_remove_repo_access_to_self_hosted_runner_group_in_org

> actions_slash_remove_repo_access_to_self_hosted_runner_group_in_org(org, runner_group_id, repository_id)
Remove repository access to a self-hosted runner group in an organization

Removes a repository from the list of selected repositories that can access a self-hosted runner group. The runner group must have `visibility` set to `selected`. For more information, see \"[Create a self-hosted runner group for an organization](#create-a-self-hosted-runner-group-for-an-organization).\"  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**runner_group_id** | **i32** | Unique identifier of the self-hosted runner group. | [required] |
**repository_id** | **i32** | The unique identifier of the repository. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_remove_selected_repo_from_org_secret

> actions_slash_remove_selected_repo_from_org_secret(org, secret_name, repository_id)
Remove selected repository from an organization secret

Removes a repository from an organization secret when the `visibility` for repository access is set to `selected`. The visibility is set when you [Create or update an organization secret](https://docs.github.com/rest/actions/secrets#create-or-update-an-organization-secret).  Authenticated users must have collaborator access to a repository to create, update, or read secrets.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |
**repository_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_remove_selected_repo_from_org_variable

> actions_slash_remove_selected_repo_from_org_variable(org, name, repository_id)
Remove selected repository from an organization variable

Removes a repository from an organization variable that is available to selected repositories. Organization variables that are available to selected repositories have their `visibility` field set to `selected`.  Authenticated users must have collaborator access to a repository to create, update, or read variables.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**name** | **String** | The name of the variable. | [required] |
**repository_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_remove_self_hosted_runner_from_group_for_org

> actions_slash_remove_self_hosted_runner_from_group_for_org(org, runner_group_id, runner_id)
Remove a self-hosted runner from a group for an organization

Removes a self-hosted runner from a group configured in an organization. The runner is then returned to the default group.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**runner_group_id** | **i32** | Unique identifier of the self-hosted runner group. | [required] |
**runner_id** | **i32** | Unique identifier of the self-hosted runner. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_review_custom_gates_for_run

> actions_slash_review_custom_gates_for_run(owner, repo, run_id, actions_review_custom_gates_for_run_request)
Review custom deployment protection rules for a workflow run

Approve or reject custom deployment protection rules provided by a GitHub App for a workflow run. For more information, see \"[Using environments for deployment](https://docs.github.com/actions/deployment/targeting-different-environments/using-environments-for-deployment).\"  > [!NOTE] > GitHub Apps can only review their own custom deployment protection rules. To approve or reject pending deployments that are waiting for review from a specific person or team, see [`POST /repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments`](/rest/actions/workflow-runs#review-pending-deployments-for-a-workflow-run).  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |
**actions_review_custom_gates_for_run_request** | [**ActionsReviewCustomGatesForRunRequest**](ActionsReviewCustomGatesForRunRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_review_pending_deployments_for_run

> Vec<models::Deployment> actions_slash_review_pending_deployments_for_run(owner, repo, run_id, actions_review_pending_deployments_for_run_request)
Review pending deployments for a workflow run

Approve or reject pending deployments that are waiting on approval by a required reviewer.  Required reviewers with read access to the repository contents and deployments can use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**run_id** | **i32** | The unique identifier of the workflow run. | [required] |
**actions_review_pending_deployments_for_run_request** | [**ActionsReviewPendingDeploymentsForRunRequest**](ActionsReviewPendingDeploymentsForRunRequest.md) |  | [required] |

### Return type

[**Vec<models::Deployment>**](deployment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_set_allowed_actions_organization

> actions_slash_set_allowed_actions_organization(org, selected_actions)
Set allowed actions and reusable workflows for an organization

Sets the actions and reusable workflows that are allowed in an organization. To use this endpoint, the organization permission policy for `allowed_actions` must be configured to `selected`. For more information, see \"[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization).\"  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**selected_actions** | Option<[**SelectedActions**](SelectedActions.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_set_allowed_actions_repository

> actions_slash_set_allowed_actions_repository(owner, repo, selected_actions)
Set allowed actions and reusable workflows for a repository

Sets the actions and reusable workflows that are allowed in a repository. To use this endpoint, the repository permission policy for `allowed_actions` must be configured to `selected`. For more information, see \"[Set GitHub Actions permissions for a repository](#set-github-actions-permissions-for-a-repository).\"  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**selected_actions** | Option<[**SelectedActions**](SelectedActions.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_set_custom_labels_for_self_hosted_runner_for_org

> models::ActionsListLabelsForSelfHostedRunnerForOrg200Response actions_slash_set_custom_labels_for_self_hosted_runner_for_org(org, runner_id, actions_set_custom_labels_for_self_hosted_runner_for_org_request)
Set custom labels for a self-hosted runner for an organization

Remove all previous custom labels and set the new custom labels for a specific self-hosted runner configured in an organization.  Authenticated users must have admin access to the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**runner_id** | **i32** | Unique identifier of the self-hosted runner. | [required] |
**actions_set_custom_labels_for_self_hosted_runner_for_org_request** | [**ActionsSetCustomLabelsForSelfHostedRunnerForOrgRequest**](ActionsSetCustomLabelsForSelfHostedRunnerForOrgRequest.md) |  | [required] |

### Return type

[**models::ActionsListLabelsForSelfHostedRunnerForOrg200Response**](actions_list_labels_for_self_hosted_runner_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_set_custom_labels_for_self_hosted_runner_for_repo

> models::ActionsListLabelsForSelfHostedRunnerForOrg200Response actions_slash_set_custom_labels_for_self_hosted_runner_for_repo(owner, repo, runner_id, actions_set_custom_labels_for_self_hosted_runner_for_org_request)
Set custom labels for a self-hosted runner for a repository

Remove all previous custom labels and set the new custom labels for a specific self-hosted runner configured in a repository.  Authenticated users must have admin access to the repository to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**runner_id** | **i32** | Unique identifier of the self-hosted runner. | [required] |
**actions_set_custom_labels_for_self_hosted_runner_for_org_request** | [**ActionsSetCustomLabelsForSelfHostedRunnerForOrgRequest**](ActionsSetCustomLabelsForSelfHostedRunnerForOrgRequest.md) |  | [required] |

### Return type

[**models::ActionsListLabelsForSelfHostedRunnerForOrg200Response**](actions_list_labels_for_self_hosted_runner_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_set_custom_oidc_sub_claim_for_repo

> serde_json::Value actions_slash_set_custom_oidc_sub_claim_for_repo(owner, repo, actions_oidc_subject_customization_for_a_repository)
Set the customization template for an OIDC subject claim for a repository

Sets the customization template and `opt-in` or `opt-out` flag for an OpenID Connect (OIDC) subject claim for a repository.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**actions_oidc_subject_customization_for_a_repository** | [**ActionsOidcSubjectCustomizationForARepository**](ActionsOidcSubjectCustomizationForARepository.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_set_github_actions_default_workflow_permissions_organization

> actions_slash_set_github_actions_default_workflow_permissions_organization(org, actions_set_default_workflow_permissions)
Set default workflow permissions for an organization

Sets the default workflow permissions granted to the `GITHUB_TOKEN` when running workflows in an organization, and sets if GitHub Actions can submit approving pull request reviews. For more information, see \"[Setting the permissions of the GITHUB_TOKEN for your organization](https://docs.github.com/organizations/managing-organization-settings/disabling-or-limiting-github-actions-for-your-organization#setting-the-permissions-of-the-github_token-for-your-organization).\"  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**actions_set_default_workflow_permissions** | Option<[**ActionsSetDefaultWorkflowPermissions**](ActionsSetDefaultWorkflowPermissions.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_set_github_actions_default_workflow_permissions_repository

> actions_slash_set_github_actions_default_workflow_permissions_repository(owner, repo, actions_set_default_workflow_permissions)
Set default workflow permissions for a repository

Sets the default workflow permissions granted to the `GITHUB_TOKEN` when running workflows in a repository, and sets if GitHub Actions can submit approving pull request reviews. For more information, see \"[Setting the permissions of the GITHUB_TOKEN for your repository](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository#setting-the-permissions-of-the-github_token-for-your-repository).\"  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**actions_set_default_workflow_permissions** | [**ActionsSetDefaultWorkflowPermissions**](ActionsSetDefaultWorkflowPermissions.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_set_github_actions_permissions_organization

> actions_slash_set_github_actions_permissions_organization(org, actions_set_github_actions_permissions_organization_request)
Set GitHub Actions permissions for an organization

Sets the GitHub Actions permissions policy for repositories and allowed actions and reusable workflows in an organization.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**actions_set_github_actions_permissions_organization_request** | [**ActionsSetGithubActionsPermissionsOrganizationRequest**](ActionsSetGithubActionsPermissionsOrganizationRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_set_github_actions_permissions_repository

> actions_slash_set_github_actions_permissions_repository(owner, repo, actions_set_github_actions_permissions_repository_request)
Set GitHub Actions permissions for a repository

Sets the GitHub Actions permissions policy for enabling GitHub Actions and allowed actions and reusable workflows in the repository.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**actions_set_github_actions_permissions_repository_request** | [**ActionsSetGithubActionsPermissionsRepositoryRequest**](ActionsSetGithubActionsPermissionsRepositoryRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_set_repo_access_to_self_hosted_runner_group_in_org

> actions_slash_set_repo_access_to_self_hosted_runner_group_in_org(org, runner_group_id, actions_set_repo_access_to_self_hosted_runner_group_in_org_request)
Set repository access for a self-hosted runner group in an organization

Replaces the list of repositories that have access to a self-hosted runner group configured in an organization.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**runner_group_id** | **i32** | Unique identifier of the self-hosted runner group. | [required] |
**actions_set_repo_access_to_self_hosted_runner_group_in_org_request** | [**ActionsSetRepoAccessToSelfHostedRunnerGroupInOrgRequest**](ActionsSetRepoAccessToSelfHostedRunnerGroupInOrgRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_set_selected_repos_for_org_secret

> actions_slash_set_selected_repos_for_org_secret(org, secret_name, actions_set_selected_repos_for_org_secret_request)
Set selected repositories for an organization secret

Replaces all repositories for an organization secret when the `visibility` for repository access is set to `selected`. The visibility is set when you [Create or update an organization secret](https://docs.github.com/rest/actions/secrets#create-or-update-an-organization-secret).  Authenticated users must have collaborator access to a repository to create, update, or read secrets.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |
**actions_set_selected_repos_for_org_secret_request** | [**ActionsSetSelectedReposForOrgSecretRequest**](ActionsSetSelectedReposForOrgSecretRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_set_selected_repos_for_org_variable

> actions_slash_set_selected_repos_for_org_variable(org, name, actions_set_selected_repos_for_org_variable_request)
Set selected repositories for an organization variable

Replaces all repositories for an organization variable that is available to selected repositories. Organization variables that are available to selected repositories have their `visibility` field set to `selected`.  Authenticated users must have collaborator access to a repository to create, update, or read variables.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**name** | **String** | The name of the variable. | [required] |
**actions_set_selected_repos_for_org_variable_request** | [**ActionsSetSelectedReposForOrgVariableRequest**](ActionsSetSelectedReposForOrgVariableRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_set_selected_repositories_enabled_github_actions_organization

> actions_slash_set_selected_repositories_enabled_github_actions_organization(org, actions_set_selected_repositories_enabled_github_actions_organization_request)
Set selected repositories enabled for GitHub Actions in an organization

Replaces the list of selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be configured to `selected`. For more information, see \"[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization).\"   OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**actions_set_selected_repositories_enabled_github_actions_organization_request** | [**ActionsSetSelectedRepositoriesEnabledGithubActionsOrganizationRequest**](ActionsSetSelectedRepositoriesEnabledGithubActionsOrganizationRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_set_self_hosted_runners_in_group_for_org

> actions_slash_set_self_hosted_runners_in_group_for_org(org, runner_group_id, actions_set_self_hosted_runners_in_group_for_org_request)
Set self-hosted runners in a group for an organization

Replaces the list of self-hosted runners that are part of an organization runner group.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**runner_group_id** | **i32** | Unique identifier of the self-hosted runner group. | [required] |
**actions_set_self_hosted_runners_in_group_for_org_request** | [**ActionsSetSelfHostedRunnersInGroupForOrgRequest**](ActionsSetSelfHostedRunnersInGroupForOrgRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_set_workflow_access_to_repository

> actions_slash_set_workflow_access_to_repository(owner, repo, actions_workflow_access_to_repository)
Set the level of access for workflows outside of the repository

Sets the level of access that workflows outside of the repository have to actions and reusable workflows in the repository. This endpoint only applies to private repositories. For more information, see \"[Allowing access to components in a private repository](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository#allowing-access-to-components-in-a-private-repository)\".  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**actions_workflow_access_to_repository** | [**ActionsWorkflowAccessToRepository**](ActionsWorkflowAccessToRepository.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_update_environment_variable

> actions_slash_update_environment_variable(owner, repo, name, environment_name, actions_update_repo_variable_request)
Update an environment variable

Updates an environment variable that you can reference in a GitHub Actions workflow.  Authenticated users must have collaborator access to a repository to create, update, or read variables.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**name** | **String** | The name of the variable. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**actions_update_repo_variable_request** | [**ActionsUpdateRepoVariableRequest**](ActionsUpdateRepoVariableRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_update_org_variable

> actions_slash_update_org_variable(org, name, actions_update_org_variable_request)
Update an organization variable

Updates an organization variable that you can reference in a GitHub Actions workflow.  Authenticated users must have collaborator access to a repository to create, update, or read variables.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**name** | **String** | The name of the variable. | [required] |
**actions_update_org_variable_request** | [**ActionsUpdateOrgVariableRequest**](ActionsUpdateOrgVariableRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_update_repo_variable

> actions_slash_update_repo_variable(owner, repo, name, actions_update_repo_variable_request)
Update a repository variable

Updates a repository variable that you can reference in a GitHub Actions workflow.  Authenticated users must have collaborator access to a repository to create, update, or read variables.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**name** | **String** | The name of the variable. | [required] |
**actions_update_repo_variable_request** | [**ActionsUpdateRepoVariableRequest**](ActionsUpdateRepoVariableRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_slash_update_self_hosted_runner_group_for_org

> models::RunnerGroupsOrg actions_slash_update_self_hosted_runner_group_for_org(org, runner_group_id, actions_update_self_hosted_runner_group_for_org_request)
Update a self-hosted runner group for an organization

Updates the `name` and `visibility` of a self-hosted runner group in an organization.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**runner_group_id** | **i32** | Unique identifier of the self-hosted runner group. | [required] |
**actions_update_self_hosted_runner_group_for_org_request** | [**ActionsUpdateSelfHostedRunnerGroupForOrgRequest**](ActionsUpdateSelfHostedRunnerGroupForOrgRequest.md) |  | [required] |

### Return type

[**models::RunnerGroupsOrg**](runner-groups-org.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

