# \ReposApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**repos_slash_accept_invitation_for_authenticated_user**](ReposApi.md#repos_slash_accept_invitation_for_authenticated_user) | **PATCH** /user/repository_invitations/{invitation_id} | Accept a repository invitation
[**repos_slash_add_app_access_restrictions**](ReposApi.md#repos_slash_add_app_access_restrictions) | **POST** /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps | Add app access restrictions
[**repos_slash_add_collaborator**](ReposApi.md#repos_slash_add_collaborator) | **PUT** /repos/{owner}/{repo}/collaborators/{username} | Add a repository collaborator
[**repos_slash_add_status_check_contexts**](ReposApi.md#repos_slash_add_status_check_contexts) | **POST** /repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts | Add status check contexts
[**repos_slash_add_team_access_restrictions**](ReposApi.md#repos_slash_add_team_access_restrictions) | **POST** /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams | Add team access restrictions
[**repos_slash_add_user_access_restrictions**](ReposApi.md#repos_slash_add_user_access_restrictions) | **POST** /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users | Add user access restrictions
[**repos_slash_cancel_pages_deployment**](ReposApi.md#repos_slash_cancel_pages_deployment) | **POST** /repos/{owner}/{repo}/pages/deployments/{pages_deployment_id}/cancel | Cancel a GitHub Pages deployment
[**repos_slash_check_automated_security_fixes**](ReposApi.md#repos_slash_check_automated_security_fixes) | **GET** /repos/{owner}/{repo}/automated-security-fixes | Check if automated security fixes are enabled for a repository
[**repos_slash_check_collaborator**](ReposApi.md#repos_slash_check_collaborator) | **GET** /repos/{owner}/{repo}/collaborators/{username} | Check if a user is a repository collaborator
[**repos_slash_check_private_vulnerability_reporting**](ReposApi.md#repos_slash_check_private_vulnerability_reporting) | **GET** /repos/{owner}/{repo}/private-vulnerability-reporting | Check if private vulnerability reporting is enabled for a repository
[**repos_slash_check_vulnerability_alerts**](ReposApi.md#repos_slash_check_vulnerability_alerts) | **GET** /repos/{owner}/{repo}/vulnerability-alerts | Check if vulnerability alerts are enabled for a repository
[**repos_slash_codeowners_errors**](ReposApi.md#repos_slash_codeowners_errors) | **GET** /repos/{owner}/{repo}/codeowners/errors | List CODEOWNERS errors
[**repos_slash_compare_commits**](ReposApi.md#repos_slash_compare_commits) | **GET** /repos/{owner}/{repo}/compare/{basehead} | Compare two commits
[**repos_slash_create_attestation**](ReposApi.md#repos_slash_create_attestation) | **POST** /repos/{owner}/{repo}/attestations | Create an attestation
[**repos_slash_create_autolink**](ReposApi.md#repos_slash_create_autolink) | **POST** /repos/{owner}/{repo}/autolinks | Create an autolink reference for a repository
[**repos_slash_create_commit_comment**](ReposApi.md#repos_slash_create_commit_comment) | **POST** /repos/{owner}/{repo}/commits/{commit_sha}/comments | Create a commit comment
[**repos_slash_create_commit_signature_protection**](ReposApi.md#repos_slash_create_commit_signature_protection) | **POST** /repos/{owner}/{repo}/branches/{branch}/protection/required_signatures | Create commit signature protection
[**repos_slash_create_commit_status**](ReposApi.md#repos_slash_create_commit_status) | **POST** /repos/{owner}/{repo}/statuses/{sha} | Create a commit status
[**repos_slash_create_deploy_key**](ReposApi.md#repos_slash_create_deploy_key) | **POST** /repos/{owner}/{repo}/keys | Create a deploy key
[**repos_slash_create_deployment**](ReposApi.md#repos_slash_create_deployment) | **POST** /repos/{owner}/{repo}/deployments | Create a deployment
[**repos_slash_create_deployment_branch_policy**](ReposApi.md#repos_slash_create_deployment_branch_policy) | **POST** /repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies | Create a deployment branch policy
[**repos_slash_create_deployment_protection_rule**](ReposApi.md#repos_slash_create_deployment_protection_rule) | **POST** /repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules | Create a custom deployment protection rule on an environment
[**repos_slash_create_deployment_status**](ReposApi.md#repos_slash_create_deployment_status) | **POST** /repos/{owner}/{repo}/deployments/{deployment_id}/statuses | Create a deployment status
[**repos_slash_create_dispatch_event**](ReposApi.md#repos_slash_create_dispatch_event) | **POST** /repos/{owner}/{repo}/dispatches | Create a repository dispatch event
[**repos_slash_create_for_authenticated_user**](ReposApi.md#repos_slash_create_for_authenticated_user) | **POST** /user/repos | Create a repository for the authenticated user
[**repos_slash_create_fork**](ReposApi.md#repos_slash_create_fork) | **POST** /repos/{owner}/{repo}/forks | Create a fork
[**repos_slash_create_in_org**](ReposApi.md#repos_slash_create_in_org) | **POST** /orgs/{org}/repos | Create an organization repository
[**repos_slash_create_or_update_custom_properties_values**](ReposApi.md#repos_slash_create_or_update_custom_properties_values) | **PATCH** /repos/{owner}/{repo}/properties/values | Create or update custom property values for a repository
[**repos_slash_create_or_update_environment**](ReposApi.md#repos_slash_create_or_update_environment) | **PUT** /repos/{owner}/{repo}/environments/{environment_name} | Create or update an environment
[**repos_slash_create_or_update_file_contents**](ReposApi.md#repos_slash_create_or_update_file_contents) | **PUT** /repos/{owner}/{repo}/contents/{path} | Create or update file contents
[**repos_slash_create_org_ruleset**](ReposApi.md#repos_slash_create_org_ruleset) | **POST** /orgs/{org}/rulesets | Create an organization repository ruleset
[**repos_slash_create_pages_deployment**](ReposApi.md#repos_slash_create_pages_deployment) | **POST** /repos/{owner}/{repo}/pages/deployments | Create a GitHub Pages deployment
[**repos_slash_create_pages_site**](ReposApi.md#repos_slash_create_pages_site) | **POST** /repos/{owner}/{repo}/pages | Create a GitHub Pages site
[**repos_slash_create_release**](ReposApi.md#repos_slash_create_release) | **POST** /repos/{owner}/{repo}/releases | Create a release
[**repos_slash_create_repo_ruleset**](ReposApi.md#repos_slash_create_repo_ruleset) | **POST** /repos/{owner}/{repo}/rulesets | Create a repository ruleset
[**repos_slash_create_tag_protection**](ReposApi.md#repos_slash_create_tag_protection) | **POST** /repos/{owner}/{repo}/tags/protection | Closing down - Create a tag protection state for a repository
[**repos_slash_create_using_template**](ReposApi.md#repos_slash_create_using_template) | **POST** /repos/{template_owner}/{template_repo}/generate | Create a repository using a template
[**repos_slash_create_webhook**](ReposApi.md#repos_slash_create_webhook) | **POST** /repos/{owner}/{repo}/hooks | Create a repository webhook
[**repos_slash_decline_invitation_for_authenticated_user**](ReposApi.md#repos_slash_decline_invitation_for_authenticated_user) | **DELETE** /user/repository_invitations/{invitation_id} | Decline a repository invitation
[**repos_slash_delete**](ReposApi.md#repos_slash_delete) | **DELETE** /repos/{owner}/{repo} | Delete a repository
[**repos_slash_delete_access_restrictions**](ReposApi.md#repos_slash_delete_access_restrictions) | **DELETE** /repos/{owner}/{repo}/branches/{branch}/protection/restrictions | Delete access restrictions
[**repos_slash_delete_admin_branch_protection**](ReposApi.md#repos_slash_delete_admin_branch_protection) | **DELETE** /repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins | Delete admin branch protection
[**repos_slash_delete_an_environment**](ReposApi.md#repos_slash_delete_an_environment) | **DELETE** /repos/{owner}/{repo}/environments/{environment_name} | Delete an environment
[**repos_slash_delete_autolink**](ReposApi.md#repos_slash_delete_autolink) | **DELETE** /repos/{owner}/{repo}/autolinks/{autolink_id} | Delete an autolink reference from a repository
[**repos_slash_delete_branch_protection**](ReposApi.md#repos_slash_delete_branch_protection) | **DELETE** /repos/{owner}/{repo}/branches/{branch}/protection | Delete branch protection
[**repos_slash_delete_commit_comment**](ReposApi.md#repos_slash_delete_commit_comment) | **DELETE** /repos/{owner}/{repo}/comments/{comment_id} | Delete a commit comment
[**repos_slash_delete_commit_signature_protection**](ReposApi.md#repos_slash_delete_commit_signature_protection) | **DELETE** /repos/{owner}/{repo}/branches/{branch}/protection/required_signatures | Delete commit signature protection
[**repos_slash_delete_deploy_key**](ReposApi.md#repos_slash_delete_deploy_key) | **DELETE** /repos/{owner}/{repo}/keys/{key_id} | Delete a deploy key
[**repos_slash_delete_deployment**](ReposApi.md#repos_slash_delete_deployment) | **DELETE** /repos/{owner}/{repo}/deployments/{deployment_id} | Delete a deployment
[**repos_slash_delete_deployment_branch_policy**](ReposApi.md#repos_slash_delete_deployment_branch_policy) | **DELETE** /repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies/{branch_policy_id} | Delete a deployment branch policy
[**repos_slash_delete_file**](ReposApi.md#repos_slash_delete_file) | **DELETE** /repos/{owner}/{repo}/contents/{path} | Delete a file
[**repos_slash_delete_invitation**](ReposApi.md#repos_slash_delete_invitation) | **DELETE** /repos/{owner}/{repo}/invitations/{invitation_id} | Delete a repository invitation
[**repos_slash_delete_org_ruleset**](ReposApi.md#repos_slash_delete_org_ruleset) | **DELETE** /orgs/{org}/rulesets/{ruleset_id} | Delete an organization repository ruleset
[**repos_slash_delete_pages_site**](ReposApi.md#repos_slash_delete_pages_site) | **DELETE** /repos/{owner}/{repo}/pages | Delete a GitHub Pages site
[**repos_slash_delete_pull_request_review_protection**](ReposApi.md#repos_slash_delete_pull_request_review_protection) | **DELETE** /repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews | Delete pull request review protection
[**repos_slash_delete_release**](ReposApi.md#repos_slash_delete_release) | **DELETE** /repos/{owner}/{repo}/releases/{release_id} | Delete a release
[**repos_slash_delete_release_asset**](ReposApi.md#repos_slash_delete_release_asset) | **DELETE** /repos/{owner}/{repo}/releases/assets/{asset_id} | Delete a release asset
[**repos_slash_delete_repo_ruleset**](ReposApi.md#repos_slash_delete_repo_ruleset) | **DELETE** /repos/{owner}/{repo}/rulesets/{ruleset_id} | Delete a repository ruleset
[**repos_slash_delete_tag_protection**](ReposApi.md#repos_slash_delete_tag_protection) | **DELETE** /repos/{owner}/{repo}/tags/protection/{tag_protection_id} | Closing down - Delete a tag protection state for a repository
[**repos_slash_delete_webhook**](ReposApi.md#repos_slash_delete_webhook) | **DELETE** /repos/{owner}/{repo}/hooks/{hook_id} | Delete a repository webhook
[**repos_slash_disable_automated_security_fixes**](ReposApi.md#repos_slash_disable_automated_security_fixes) | **DELETE** /repos/{owner}/{repo}/automated-security-fixes | Disable automated security fixes
[**repos_slash_disable_deployment_protection_rule**](ReposApi.md#repos_slash_disable_deployment_protection_rule) | **DELETE** /repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules/{protection_rule_id} | Disable a custom protection rule for an environment
[**repos_slash_disable_private_vulnerability_reporting**](ReposApi.md#repos_slash_disable_private_vulnerability_reporting) | **DELETE** /repos/{owner}/{repo}/private-vulnerability-reporting | Disable private vulnerability reporting for a repository
[**repos_slash_disable_vulnerability_alerts**](ReposApi.md#repos_slash_disable_vulnerability_alerts) | **DELETE** /repos/{owner}/{repo}/vulnerability-alerts | Disable vulnerability alerts
[**repos_slash_download_tarball_archive**](ReposApi.md#repos_slash_download_tarball_archive) | **GET** /repos/{owner}/{repo}/tarball/{ref} | Download a repository archive (tar)
[**repos_slash_download_zipball_archive**](ReposApi.md#repos_slash_download_zipball_archive) | **GET** /repos/{owner}/{repo}/zipball/{ref} | Download a repository archive (zip)
[**repos_slash_enable_automated_security_fixes**](ReposApi.md#repos_slash_enable_automated_security_fixes) | **PUT** /repos/{owner}/{repo}/automated-security-fixes | Enable automated security fixes
[**repos_slash_enable_private_vulnerability_reporting**](ReposApi.md#repos_slash_enable_private_vulnerability_reporting) | **PUT** /repos/{owner}/{repo}/private-vulnerability-reporting | Enable private vulnerability reporting for a repository
[**repos_slash_enable_vulnerability_alerts**](ReposApi.md#repos_slash_enable_vulnerability_alerts) | **PUT** /repos/{owner}/{repo}/vulnerability-alerts | Enable vulnerability alerts
[**repos_slash_generate_release_notes**](ReposApi.md#repos_slash_generate_release_notes) | **POST** /repos/{owner}/{repo}/releases/generate-notes | Generate release notes content for a release
[**repos_slash_get**](ReposApi.md#repos_slash_get) | **GET** /repos/{owner}/{repo} | Get a repository
[**repos_slash_get_access_restrictions**](ReposApi.md#repos_slash_get_access_restrictions) | **GET** /repos/{owner}/{repo}/branches/{branch}/protection/restrictions | Get access restrictions
[**repos_slash_get_admin_branch_protection**](ReposApi.md#repos_slash_get_admin_branch_protection) | **GET** /repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins | Get admin branch protection
[**repos_slash_get_all_deployment_protection_rules**](ReposApi.md#repos_slash_get_all_deployment_protection_rules) | **GET** /repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules | Get all deployment protection rules for an environment
[**repos_slash_get_all_environments**](ReposApi.md#repos_slash_get_all_environments) | **GET** /repos/{owner}/{repo}/environments | List environments
[**repos_slash_get_all_status_check_contexts**](ReposApi.md#repos_slash_get_all_status_check_contexts) | **GET** /repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts | Get all status check contexts
[**repos_slash_get_all_topics**](ReposApi.md#repos_slash_get_all_topics) | **GET** /repos/{owner}/{repo}/topics | Get all repository topics
[**repos_slash_get_apps_with_access_to_protected_branch**](ReposApi.md#repos_slash_get_apps_with_access_to_protected_branch) | **GET** /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps | Get apps with access to the protected branch
[**repos_slash_get_autolink**](ReposApi.md#repos_slash_get_autolink) | **GET** /repos/{owner}/{repo}/autolinks/{autolink_id} | Get an autolink reference of a repository
[**repos_slash_get_branch**](ReposApi.md#repos_slash_get_branch) | **GET** /repos/{owner}/{repo}/branches/{branch} | Get a branch
[**repos_slash_get_branch_protection**](ReposApi.md#repos_slash_get_branch_protection) | **GET** /repos/{owner}/{repo}/branches/{branch}/protection | Get branch protection
[**repos_slash_get_branch_rules**](ReposApi.md#repos_slash_get_branch_rules) | **GET** /repos/{owner}/{repo}/rules/branches/{branch} | Get rules for a branch
[**repos_slash_get_clones**](ReposApi.md#repos_slash_get_clones) | **GET** /repos/{owner}/{repo}/traffic/clones | Get repository clones
[**repos_slash_get_code_frequency_stats**](ReposApi.md#repos_slash_get_code_frequency_stats) | **GET** /repos/{owner}/{repo}/stats/code_frequency | Get the weekly commit activity
[**repos_slash_get_collaborator_permission_level**](ReposApi.md#repos_slash_get_collaborator_permission_level) | **GET** /repos/{owner}/{repo}/collaborators/{username}/permission | Get repository permissions for a user
[**repos_slash_get_combined_status_for_ref**](ReposApi.md#repos_slash_get_combined_status_for_ref) | **GET** /repos/{owner}/{repo}/commits/{ref}/status | Get the combined status for a specific reference
[**repos_slash_get_commit**](ReposApi.md#repos_slash_get_commit) | **GET** /repos/{owner}/{repo}/commits/{ref} | Get a commit
[**repos_slash_get_commit_activity_stats**](ReposApi.md#repos_slash_get_commit_activity_stats) | **GET** /repos/{owner}/{repo}/stats/commit_activity | Get the last year of commit activity
[**repos_slash_get_commit_comment**](ReposApi.md#repos_slash_get_commit_comment) | **GET** /repos/{owner}/{repo}/comments/{comment_id} | Get a commit comment
[**repos_slash_get_commit_signature_protection**](ReposApi.md#repos_slash_get_commit_signature_protection) | **GET** /repos/{owner}/{repo}/branches/{branch}/protection/required_signatures | Get commit signature protection
[**repos_slash_get_community_profile_metrics**](ReposApi.md#repos_slash_get_community_profile_metrics) | **GET** /repos/{owner}/{repo}/community/profile | Get community profile metrics
[**repos_slash_get_content**](ReposApi.md#repos_slash_get_content) | **GET** /repos/{owner}/{repo}/contents/{path} | Get repository content
[**repos_slash_get_contributors_stats**](ReposApi.md#repos_slash_get_contributors_stats) | **GET** /repos/{owner}/{repo}/stats/contributors | Get all contributor commit activity
[**repos_slash_get_custom_deployment_protection_rule**](ReposApi.md#repos_slash_get_custom_deployment_protection_rule) | **GET** /repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules/{protection_rule_id} | Get a custom deployment protection rule
[**repos_slash_get_custom_properties_values**](ReposApi.md#repos_slash_get_custom_properties_values) | **GET** /repos/{owner}/{repo}/properties/values | Get all custom property values for a repository
[**repos_slash_get_deploy_key**](ReposApi.md#repos_slash_get_deploy_key) | **GET** /repos/{owner}/{repo}/keys/{key_id} | Get a deploy key
[**repos_slash_get_deployment**](ReposApi.md#repos_slash_get_deployment) | **GET** /repos/{owner}/{repo}/deployments/{deployment_id} | Get a deployment
[**repos_slash_get_deployment_branch_policy**](ReposApi.md#repos_slash_get_deployment_branch_policy) | **GET** /repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies/{branch_policy_id} | Get a deployment branch policy
[**repos_slash_get_deployment_status**](ReposApi.md#repos_slash_get_deployment_status) | **GET** /repos/{owner}/{repo}/deployments/{deployment_id}/statuses/{status_id} | Get a deployment status
[**repos_slash_get_environment**](ReposApi.md#repos_slash_get_environment) | **GET** /repos/{owner}/{repo}/environments/{environment_name} | Get an environment
[**repos_slash_get_latest_pages_build**](ReposApi.md#repos_slash_get_latest_pages_build) | **GET** /repos/{owner}/{repo}/pages/builds/latest | Get latest Pages build
[**repos_slash_get_latest_release**](ReposApi.md#repos_slash_get_latest_release) | **GET** /repos/{owner}/{repo}/releases/latest | Get the latest release
[**repos_slash_get_org_rule_suite**](ReposApi.md#repos_slash_get_org_rule_suite) | **GET** /orgs/{org}/rulesets/rule-suites/{rule_suite_id} | Get an organization rule suite
[**repos_slash_get_org_rule_suites**](ReposApi.md#repos_slash_get_org_rule_suites) | **GET** /orgs/{org}/rulesets/rule-suites | List organization rule suites
[**repos_slash_get_org_ruleset**](ReposApi.md#repos_slash_get_org_ruleset) | **GET** /orgs/{org}/rulesets/{ruleset_id} | Get an organization repository ruleset
[**repos_slash_get_org_rulesets**](ReposApi.md#repos_slash_get_org_rulesets) | **GET** /orgs/{org}/rulesets | Get all organization repository rulesets
[**repos_slash_get_pages**](ReposApi.md#repos_slash_get_pages) | **GET** /repos/{owner}/{repo}/pages | Get a GitHub Pages site
[**repos_slash_get_pages_build**](ReposApi.md#repos_slash_get_pages_build) | **GET** /repos/{owner}/{repo}/pages/builds/{build_id} | Get GitHub Pages build
[**repos_slash_get_pages_deployment**](ReposApi.md#repos_slash_get_pages_deployment) | **GET** /repos/{owner}/{repo}/pages/deployments/{pages_deployment_id} | Get the status of a GitHub Pages deployment
[**repos_slash_get_pages_health_check**](ReposApi.md#repos_slash_get_pages_health_check) | **GET** /repos/{owner}/{repo}/pages/health | Get a DNS health check for GitHub Pages
[**repos_slash_get_participation_stats**](ReposApi.md#repos_slash_get_participation_stats) | **GET** /repos/{owner}/{repo}/stats/participation | Get the weekly commit count
[**repos_slash_get_pull_request_review_protection**](ReposApi.md#repos_slash_get_pull_request_review_protection) | **GET** /repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews | Get pull request review protection
[**repos_slash_get_punch_card_stats**](ReposApi.md#repos_slash_get_punch_card_stats) | **GET** /repos/{owner}/{repo}/stats/punch_card | Get the hourly commit count for each day
[**repos_slash_get_readme**](ReposApi.md#repos_slash_get_readme) | **GET** /repos/{owner}/{repo}/readme | Get a repository README
[**repos_slash_get_readme_in_directory**](ReposApi.md#repos_slash_get_readme_in_directory) | **GET** /repos/{owner}/{repo}/readme/{dir} | Get a repository README for a directory
[**repos_slash_get_release**](ReposApi.md#repos_slash_get_release) | **GET** /repos/{owner}/{repo}/releases/{release_id} | Get a release
[**repos_slash_get_release_asset**](ReposApi.md#repos_slash_get_release_asset) | **GET** /repos/{owner}/{repo}/releases/assets/{asset_id} | Get a release asset
[**repos_slash_get_release_by_tag**](ReposApi.md#repos_slash_get_release_by_tag) | **GET** /repos/{owner}/{repo}/releases/tags/{tag} | Get a release by tag name
[**repos_slash_get_repo_rule_suite**](ReposApi.md#repos_slash_get_repo_rule_suite) | **GET** /repos/{owner}/{repo}/rulesets/rule-suites/{rule_suite_id} | Get a repository rule suite
[**repos_slash_get_repo_rule_suites**](ReposApi.md#repos_slash_get_repo_rule_suites) | **GET** /repos/{owner}/{repo}/rulesets/rule-suites | List repository rule suites
[**repos_slash_get_repo_ruleset**](ReposApi.md#repos_slash_get_repo_ruleset) | **GET** /repos/{owner}/{repo}/rulesets/{ruleset_id} | Get a repository ruleset
[**repos_slash_get_repo_rulesets**](ReposApi.md#repos_slash_get_repo_rulesets) | **GET** /repos/{owner}/{repo}/rulesets | Get all repository rulesets
[**repos_slash_get_status_checks_protection**](ReposApi.md#repos_slash_get_status_checks_protection) | **GET** /repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks | Get status checks protection
[**repos_slash_get_teams_with_access_to_protected_branch**](ReposApi.md#repos_slash_get_teams_with_access_to_protected_branch) | **GET** /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams | Get teams with access to the protected branch
[**repos_slash_get_top_paths**](ReposApi.md#repos_slash_get_top_paths) | **GET** /repos/{owner}/{repo}/traffic/popular/paths | Get top referral paths
[**repos_slash_get_top_referrers**](ReposApi.md#repos_slash_get_top_referrers) | **GET** /repos/{owner}/{repo}/traffic/popular/referrers | Get top referral sources
[**repos_slash_get_users_with_access_to_protected_branch**](ReposApi.md#repos_slash_get_users_with_access_to_protected_branch) | **GET** /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users | Get users with access to the protected branch
[**repos_slash_get_views**](ReposApi.md#repos_slash_get_views) | **GET** /repos/{owner}/{repo}/traffic/views | Get page views
[**repos_slash_get_webhook**](ReposApi.md#repos_slash_get_webhook) | **GET** /repos/{owner}/{repo}/hooks/{hook_id} | Get a repository webhook
[**repos_slash_get_webhook_config_for_repo**](ReposApi.md#repos_slash_get_webhook_config_for_repo) | **GET** /repos/{owner}/{repo}/hooks/{hook_id}/config | Get a webhook configuration for a repository
[**repos_slash_get_webhook_delivery**](ReposApi.md#repos_slash_get_webhook_delivery) | **GET** /repos/{owner}/{repo}/hooks/{hook_id}/deliveries/{delivery_id} | Get a delivery for a repository webhook
[**repos_slash_list_activities**](ReposApi.md#repos_slash_list_activities) | **GET** /repos/{owner}/{repo}/activity | List repository activities
[**repos_slash_list_attestations**](ReposApi.md#repos_slash_list_attestations) | **GET** /repos/{owner}/{repo}/attestations/{subject_digest} | List attestations
[**repos_slash_list_autolinks**](ReposApi.md#repos_slash_list_autolinks) | **GET** /repos/{owner}/{repo}/autolinks | Get all autolinks of a repository
[**repos_slash_list_branches**](ReposApi.md#repos_slash_list_branches) | **GET** /repos/{owner}/{repo}/branches | List branches
[**repos_slash_list_branches_for_head_commit**](ReposApi.md#repos_slash_list_branches_for_head_commit) | **GET** /repos/{owner}/{repo}/commits/{commit_sha}/branches-where-head | List branches for HEAD commit
[**repos_slash_list_collaborators**](ReposApi.md#repos_slash_list_collaborators) | **GET** /repos/{owner}/{repo}/collaborators | List repository collaborators
[**repos_slash_list_comments_for_commit**](ReposApi.md#repos_slash_list_comments_for_commit) | **GET** /repos/{owner}/{repo}/commits/{commit_sha}/comments | List commit comments
[**repos_slash_list_commit_comments_for_repo**](ReposApi.md#repos_slash_list_commit_comments_for_repo) | **GET** /repos/{owner}/{repo}/comments | List commit comments for a repository
[**repos_slash_list_commit_statuses_for_ref**](ReposApi.md#repos_slash_list_commit_statuses_for_ref) | **GET** /repos/{owner}/{repo}/commits/{ref}/statuses | List commit statuses for a reference
[**repos_slash_list_commits**](ReposApi.md#repos_slash_list_commits) | **GET** /repos/{owner}/{repo}/commits | List commits
[**repos_slash_list_contributors**](ReposApi.md#repos_slash_list_contributors) | **GET** /repos/{owner}/{repo}/contributors | List repository contributors
[**repos_slash_list_custom_deployment_rule_integrations**](ReposApi.md#repos_slash_list_custom_deployment_rule_integrations) | **GET** /repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules/apps | List custom deployment rule integrations available for an environment
[**repos_slash_list_deploy_keys**](ReposApi.md#repos_slash_list_deploy_keys) | **GET** /repos/{owner}/{repo}/keys | List deploy keys
[**repos_slash_list_deployment_branch_policies**](ReposApi.md#repos_slash_list_deployment_branch_policies) | **GET** /repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies | List deployment branch policies
[**repos_slash_list_deployment_statuses**](ReposApi.md#repos_slash_list_deployment_statuses) | **GET** /repos/{owner}/{repo}/deployments/{deployment_id}/statuses | List deployment statuses
[**repos_slash_list_deployments**](ReposApi.md#repos_slash_list_deployments) | **GET** /repos/{owner}/{repo}/deployments | List deployments
[**repos_slash_list_for_authenticated_user**](ReposApi.md#repos_slash_list_for_authenticated_user) | **GET** /user/repos | List repositories for the authenticated user
[**repos_slash_list_for_org**](ReposApi.md#repos_slash_list_for_org) | **GET** /orgs/{org}/repos | List organization repositories
[**repos_slash_list_for_user**](ReposApi.md#repos_slash_list_for_user) | **GET** /users/{username}/repos | List repositories for a user
[**repos_slash_list_forks**](ReposApi.md#repos_slash_list_forks) | **GET** /repos/{owner}/{repo}/forks | List forks
[**repos_slash_list_invitations**](ReposApi.md#repos_slash_list_invitations) | **GET** /repos/{owner}/{repo}/invitations | List repository invitations
[**repos_slash_list_invitations_for_authenticated_user**](ReposApi.md#repos_slash_list_invitations_for_authenticated_user) | **GET** /user/repository_invitations | List repository invitations for the authenticated user
[**repos_slash_list_languages**](ReposApi.md#repos_slash_list_languages) | **GET** /repos/{owner}/{repo}/languages | List repository languages
[**repos_slash_list_pages_builds**](ReposApi.md#repos_slash_list_pages_builds) | **GET** /repos/{owner}/{repo}/pages/builds | List GitHub Pages builds
[**repos_slash_list_public**](ReposApi.md#repos_slash_list_public) | **GET** /repositories | List public repositories
[**repos_slash_list_pull_requests_associated_with_commit**](ReposApi.md#repos_slash_list_pull_requests_associated_with_commit) | **GET** /repos/{owner}/{repo}/commits/{commit_sha}/pulls | List pull requests associated with a commit
[**repos_slash_list_release_assets**](ReposApi.md#repos_slash_list_release_assets) | **GET** /repos/{owner}/{repo}/releases/{release_id}/assets | List release assets
[**repos_slash_list_releases**](ReposApi.md#repos_slash_list_releases) | **GET** /repos/{owner}/{repo}/releases | List releases
[**repos_slash_list_tag_protection**](ReposApi.md#repos_slash_list_tag_protection) | **GET** /repos/{owner}/{repo}/tags/protection | Closing down - List tag protection states for a repository
[**repos_slash_list_tags**](ReposApi.md#repos_slash_list_tags) | **GET** /repos/{owner}/{repo}/tags | List repository tags
[**repos_slash_list_teams**](ReposApi.md#repos_slash_list_teams) | **GET** /repos/{owner}/{repo}/teams | List repository teams
[**repos_slash_list_webhook_deliveries**](ReposApi.md#repos_slash_list_webhook_deliveries) | **GET** /repos/{owner}/{repo}/hooks/{hook_id}/deliveries | List deliveries for a repository webhook
[**repos_slash_list_webhooks**](ReposApi.md#repos_slash_list_webhooks) | **GET** /repos/{owner}/{repo}/hooks | List repository webhooks
[**repos_slash_merge**](ReposApi.md#repos_slash_merge) | **POST** /repos/{owner}/{repo}/merges | Merge a branch
[**repos_slash_merge_upstream**](ReposApi.md#repos_slash_merge_upstream) | **POST** /repos/{owner}/{repo}/merge-upstream | Sync a fork branch with the upstream repository
[**repos_slash_ping_webhook**](ReposApi.md#repos_slash_ping_webhook) | **POST** /repos/{owner}/{repo}/hooks/{hook_id}/pings | Ping a repository webhook
[**repos_slash_redeliver_webhook_delivery**](ReposApi.md#repos_slash_redeliver_webhook_delivery) | **POST** /repos/{owner}/{repo}/hooks/{hook_id}/deliveries/{delivery_id}/attempts | Redeliver a delivery for a repository webhook
[**repos_slash_remove_app_access_restrictions**](ReposApi.md#repos_slash_remove_app_access_restrictions) | **DELETE** /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps | Remove app access restrictions
[**repos_slash_remove_collaborator**](ReposApi.md#repos_slash_remove_collaborator) | **DELETE** /repos/{owner}/{repo}/collaborators/{username} | Remove a repository collaborator
[**repos_slash_remove_status_check_contexts**](ReposApi.md#repos_slash_remove_status_check_contexts) | **DELETE** /repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts | Remove status check contexts
[**repos_slash_remove_status_check_protection**](ReposApi.md#repos_slash_remove_status_check_protection) | **DELETE** /repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks | Remove status check protection
[**repos_slash_remove_team_access_restrictions**](ReposApi.md#repos_slash_remove_team_access_restrictions) | **DELETE** /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams | Remove team access restrictions
[**repos_slash_remove_user_access_restrictions**](ReposApi.md#repos_slash_remove_user_access_restrictions) | **DELETE** /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users | Remove user access restrictions
[**repos_slash_rename_branch**](ReposApi.md#repos_slash_rename_branch) | **POST** /repos/{owner}/{repo}/branches/{branch}/rename | Rename a branch
[**repos_slash_replace_all_topics**](ReposApi.md#repos_slash_replace_all_topics) | **PUT** /repos/{owner}/{repo}/topics | Replace all repository topics
[**repos_slash_request_pages_build**](ReposApi.md#repos_slash_request_pages_build) | **POST** /repos/{owner}/{repo}/pages/builds | Request a GitHub Pages build
[**repos_slash_set_admin_branch_protection**](ReposApi.md#repos_slash_set_admin_branch_protection) | **POST** /repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins | Set admin branch protection
[**repos_slash_set_app_access_restrictions**](ReposApi.md#repos_slash_set_app_access_restrictions) | **PUT** /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps | Set app access restrictions
[**repos_slash_set_status_check_contexts**](ReposApi.md#repos_slash_set_status_check_contexts) | **PUT** /repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts | Set status check contexts
[**repos_slash_set_team_access_restrictions**](ReposApi.md#repos_slash_set_team_access_restrictions) | **PUT** /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams | Set team access restrictions
[**repos_slash_set_user_access_restrictions**](ReposApi.md#repos_slash_set_user_access_restrictions) | **PUT** /repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users | Set user access restrictions
[**repos_slash_test_push_webhook**](ReposApi.md#repos_slash_test_push_webhook) | **POST** /repos/{owner}/{repo}/hooks/{hook_id}/tests | Test the push repository webhook
[**repos_slash_transfer**](ReposApi.md#repos_slash_transfer) | **POST** /repos/{owner}/{repo}/transfer | Transfer a repository
[**repos_slash_update**](ReposApi.md#repos_slash_update) | **PATCH** /repos/{owner}/{repo} | Update a repository
[**repos_slash_update_branch_protection**](ReposApi.md#repos_slash_update_branch_protection) | **PUT** /repos/{owner}/{repo}/branches/{branch}/protection | Update branch protection
[**repos_slash_update_commit_comment**](ReposApi.md#repos_slash_update_commit_comment) | **PATCH** /repos/{owner}/{repo}/comments/{comment_id} | Update a commit comment
[**repos_slash_update_deployment_branch_policy**](ReposApi.md#repos_slash_update_deployment_branch_policy) | **PUT** /repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies/{branch_policy_id} | Update a deployment branch policy
[**repos_slash_update_information_about_pages_site**](ReposApi.md#repos_slash_update_information_about_pages_site) | **PUT** /repos/{owner}/{repo}/pages | Update information about a GitHub Pages site
[**repos_slash_update_invitation**](ReposApi.md#repos_slash_update_invitation) | **PATCH** /repos/{owner}/{repo}/invitations/{invitation_id} | Update a repository invitation
[**repos_slash_update_org_ruleset**](ReposApi.md#repos_slash_update_org_ruleset) | **PUT** /orgs/{org}/rulesets/{ruleset_id} | Update an organization repository ruleset
[**repos_slash_update_pull_request_review_protection**](ReposApi.md#repos_slash_update_pull_request_review_protection) | **PATCH** /repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews | Update pull request review protection
[**repos_slash_update_release**](ReposApi.md#repos_slash_update_release) | **PATCH** /repos/{owner}/{repo}/releases/{release_id} | Update a release
[**repos_slash_update_release_asset**](ReposApi.md#repos_slash_update_release_asset) | **PATCH** /repos/{owner}/{repo}/releases/assets/{asset_id} | Update a release asset
[**repos_slash_update_repo_ruleset**](ReposApi.md#repos_slash_update_repo_ruleset) | **PUT** /repos/{owner}/{repo}/rulesets/{ruleset_id} | Update a repository ruleset
[**repos_slash_update_status_check_protection**](ReposApi.md#repos_slash_update_status_check_protection) | **PATCH** /repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks | Update status check protection
[**repos_slash_update_webhook**](ReposApi.md#repos_slash_update_webhook) | **PATCH** /repos/{owner}/{repo}/hooks/{hook_id} | Update a repository webhook
[**repos_slash_update_webhook_config_for_repo**](ReposApi.md#repos_slash_update_webhook_config_for_repo) | **PATCH** /repos/{owner}/{repo}/hooks/{hook_id}/config | Update a webhook configuration for a repository
[**repos_slash_upload_release_asset**](ReposApi.md#repos_slash_upload_release_asset) | **POST** /repos/{owner}/{repo}/releases/{release_id}/assets | Upload a release asset



## repos_slash_accept_invitation_for_authenticated_user

> repos_slash_accept_invitation_for_authenticated_user(invitation_id)
Accept a repository invitation



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitation_id** | **i32** | The unique identifier of the invitation. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_add_app_access_restrictions

> Vec<models::Integration> repos_slash_add_app_access_restrictions(owner, repo, branch, repos_set_app_access_restrictions_request)
Add app access restrictions

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Grants the specified apps push access for this branch. Only GitHub Apps that are installed on the repository and that have been granted write access to the repository contents can be added as authorized actors on a protected branch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |
**repos_set_app_access_restrictions_request** | [**ReposSetAppAccessRestrictionsRequest**](ReposSetAppAccessRestrictionsRequest.md) |  | [required] |

### Return type

[**Vec<models::Integration>**](integration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_add_collaborator

> models::RepositoryInvitation repos_slash_add_collaborator(owner, repo, username, repos_add_collaborator_request)
Add a repository collaborator

This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see \"[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)\" and \"[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api).\"  Adding an outside collaborator may be restricted by enterprise administrators. For more information, see \"[Enforcing repository management policies in your enterprise](https://docs.github.com/admin/policies/enforcing-policies-for-your-enterprise/enforcing-repository-management-policies-in-your-enterprise#enforcing-a-policy-for-inviting-outside-collaborators-to-repositories).\"  For more information on permission levels, see \"[Repository permission levels for an organization](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/repository-permission-levels-for-an-organization#permission-levels-for-repositories-owned-by-an-organization)\". There are restrictions on which permissions can be granted to organization members when an organization base role is in place. In this case, the permission being given must be equal to or higher than the org base permission. Otherwise, the request will fail with:  ``` Cannot assign {member} permission of {role name} ```  Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see \"[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method).\"  The invitee will receive a notification that they have been invited to the repository, which they must accept or decline. They may do this via the notifications page, the email they receive, or by using the [API](https://docs.github.com/rest/collaborators/invitations).  **Updating an existing collaborator's permission level**  The endpoint can also be used to change the permissions of an existing collaborator without first removing and re-adding the collaborator. To change the permissions, use the same endpoint and pass a different `permission` parameter. The response will be a `204`, with no other indication that the permission level changed.  **Rate limits**  You are limited to sending 50 invitations to a repository per 24 hour period. Note there is no limit if you are inviting organization members to an organization repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |
**repos_add_collaborator_request** | Option<[**ReposAddCollaboratorRequest**](ReposAddCollaboratorRequest.md)> |  |  |

### Return type

[**models::RepositoryInvitation**](repository-invitation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_add_status_check_contexts

> Vec<String> repos_slash_add_status_check_contexts(owner, repo, branch, repos_set_status_check_contexts_request)
Add status check contexts

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |
**repos_set_status_check_contexts_request** | Option<[**ReposSetStatusCheckContextsRequest**](ReposSetStatusCheckContextsRequest.md)> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_add_team_access_restrictions

> Vec<models::Team> repos_slash_add_team_access_restrictions(owner, repo, branch, repos_add_team_access_restrictions_request)
Add team access restrictions

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Grants the specified teams push access for this branch. You can also give push access to child teams.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |
**repos_add_team_access_restrictions_request** | Option<[**ReposAddTeamAccessRestrictionsRequest**](ReposAddTeamAccessRestrictionsRequest.md)> |  |  |

### Return type

[**Vec<models::Team>**](team.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_add_user_access_restrictions

> Vec<models::SimpleUser> repos_slash_add_user_access_restrictions(owner, repo, branch, repos_set_user_access_restrictions_request)
Add user access restrictions

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Grants the specified people push access for this branch.  | Type    | Description                                                                                                                   | | ------- | ----------------------------------------------------------------------------------------------------------------------------- | | `array` | Usernames for people who can have push access. **Note**: The list of users, apps, and teams in total is limited to 100 items. |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |
**repos_set_user_access_restrictions_request** | [**ReposSetUserAccessRestrictionsRequest**](ReposSetUserAccessRestrictionsRequest.md) |  | [required] |

### Return type

[**Vec<models::SimpleUser>**](simple-user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_cancel_pages_deployment

> repos_slash_cancel_pages_deployment(owner, repo, pages_deployment_id)
Cancel a GitHub Pages deployment

Cancels a GitHub Pages deployment.  The authenticated user must have write permissions for the GitHub Pages site.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**pages_deployment_id** | [**ActionsGetWorkflowWorkflowIdParameter**](.md) | The ID of the Pages deployment. You can also give the commit SHA of the deployment. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_check_automated_security_fixes

> models::CheckAutomatedSecurityFixes repos_slash_check_automated_security_fixes(owner, repo)
Check if automated security fixes are enabled for a repository

Shows whether automated security fixes are enabled, disabled or paused for a repository. The authenticated user must have admin read access to the repository. For more information, see \"[Configuring automated security fixes](https://docs.github.com/articles/configuring-automated-security-fixes)\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::CheckAutomatedSecurityFixes**](check-automated-security-fixes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_check_collaborator

> repos_slash_check_collaborator(owner, repo, username)
Check if a user is a repository collaborator

For organization-owned repositories, the list of collaborators includes outside collaborators, organization members that are direct collaborators, organization members with access through team memberships, organization members with access through default organization permissions, and organization owners.  Team members will include the members of child teams.  The authenticated user must have push access to the repository to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `read:org` and `repo` scopes to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_check_private_vulnerability_reporting

> models::ReposCheckPrivateVulnerabilityReporting200Response repos_slash_check_private_vulnerability_reporting(owner, repo)
Check if private vulnerability reporting is enabled for a repository

Returns a boolean indicating whether or not private vulnerability reporting is enabled for the repository. For more information, see \"[Evaluating the security settings of a repository](https://docs.github.com/code-security/security-advisories/working-with-repository-security-advisories/evaluating-the-security-settings-of-a-repository)\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::ReposCheckPrivateVulnerabilityReporting200Response**](repos_check_private_vulnerability_reporting_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_check_vulnerability_alerts

> repos_slash_check_vulnerability_alerts(owner, repo)
Check if vulnerability alerts are enabled for a repository

Shows whether dependency alerts are enabled or disabled for a repository. The authenticated user must have admin read access to the repository. For more information, see \"[About security alerts for vulnerable dependencies](https://docs.github.com/articles/about-security-alerts-for-vulnerable-dependencies)\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_codeowners_errors

> models::CodeownersErrors repos_slash_codeowners_errors(owner, repo, r#ref)
List CODEOWNERS errors

List any syntax errors that are detected in the CODEOWNERS file.  For more information about the correct CODEOWNERS syntax, see \"[About code owners](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**r#ref** | Option<**String**> | A branch, tag or commit name used to determine which version of the CODEOWNERS file to use. Default: the repository's default branch (e.g. `main`) |  |

### Return type

[**models::CodeownersErrors**](codeowners-errors.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_compare_commits

> models::CommitComparison repos_slash_compare_commits(owner, repo, basehead, page, per_page)
Compare two commits

Compares two commits against one another. You can compare refs (branches or tags) and commit SHAs in the same repository, or you can compare refs and commit SHAs that exist in different repositories within the same repository network, including fork branches. For more information about how to view a repository's network, see \"[Understanding connections between repositories](https://docs.github.com/repositories/viewing-activity-and-data-for-your-repository/understanding-connections-between-repositories).\"  This endpoint is equivalent to running the `git log BASE..HEAD` command, but it returns commits in a different order. The `git log BASE..HEAD` command returns commits in reverse chronological order, whereas the API returns commits in chronological order.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github.diff`**: Returns the diff of the commit. - **`application/vnd.github.patch`**: Returns the patch of the commit. Diffs with binary data will have no `patch` property.  The API response includes details about the files that were changed between the two commits. This includes the status of the change (if a file was added, removed, modified, or renamed), and details of the change itself. For example, files with a `renamed` status have a `previous_filename` field showing the previous filename of the file, and files with a `modified` status have a `patch` field showing the changes made to the file.  When calling this endpoint without any paging parameter (`per_page` or `page`), the returned list is limited to 250 commits, and the last commit in the list is the most recent of the entire comparison.  **Working with large comparisons**  To process a response with a large number of commits, use a query parameter (`per_page` or `page`) to paginate the results. When using pagination:  - The list of changed files is only shown on the first page of results, and it includes up to 300 changed files for the entire comparison. - The results are returned in chronological order, but the last commit in the returned list may not be the most recent one in the entire set if there are more pages of results.  For more information on working with pagination, see \"[Using pagination in the REST API](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api).\"  **Signature verification object**  The response will include a `verification` object that describes the result of verifying the commit's signature. The `verification` object includes the following fields:  | Name | Type | Description | | ---- | ---- | ----------- | | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. | | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. | | `signature` | `string` | The signature that was extracted from the commit. | | `payload` | `string` | The value that was signed. |  These are the possible values for `reason` in the `verification` object:  | Value | Description | | ----- | ----------- | | `expired_key` | The key that made the signature is expired. | | `not_signing_key` | The \"signing\" flag is not among the usage flags in the GPG key that made the signature. | | `gpgverify_error` | There was an error communicating with the signature verification service. | | `gpgverify_unavailable` | The signature verification service is currently unavailable. | | `unsigned` | The object does not include a signature. | | `unknown_signature_type` | A non-PGP signature was found in the commit. | | `no_user` | No user was associated with the `committer` email address in the commit. | | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on their account. | | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. | | `unknown_key` | The key that made the signature has not been registered with any user's account. | | `malformed_signature` | There was an error parsing the signature. | | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. | | `valid` | None of the above errors applied, so the signature is considered to be verified. |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**basehead** | **String** | The base branch and head branch to compare. This parameter expects the format `BASE...HEAD`. Both must be branch names in `repo`. To compare with a branch that exists in a different repository in the same network as `repo`, the `basehead` parameter expects the format `USERNAME:BASE...USERNAME:HEAD`. | [required] |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**models::CommitComparison**](commit-comparison.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_attestation

> models::ReposCreateAttestation201Response repos_slash_create_attestation(owner, repo, repos_create_attestation_request)
Create an attestation

Store an artifact attestation and associate it with a repository.  The authenticated user must have write permission to the repository and, if using a fine-grained access token, the `attestations:write` permission is required.  Artifact attestations are meant to be created using the [attest action](https://github.com/actions/attest). For more information, see our guide on [using artifact attestations to establish a build's provenance](https://docs.github.com/actions/security-guides/using-artifact-attestations-to-establish-provenance-for-builds).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_create_attestation_request** | [**ReposCreateAttestationRequest**](ReposCreateAttestationRequest.md) |  | [required] |

### Return type

[**models::ReposCreateAttestation201Response**](repos_create_attestation_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_autolink

> models::Autolink repos_slash_create_autolink(owner, repo, repos_create_autolink_request)
Create an autolink reference for a repository

Users with admin access to the repository can create an autolink.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_create_autolink_request** | [**ReposCreateAutolinkRequest**](ReposCreateAutolinkRequest.md) |  | [required] |

### Return type

[**models::Autolink**](autolink.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_commit_comment

> models::CommitComment repos_slash_create_commit_comment(owner, repo, commit_sha, repos_create_commit_comment_request)
Create a commit comment

Create a comment for a commit using its `:commit_sha`.  This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see \"[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)\" and \"[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api).\"  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type. - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`. - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`. - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**commit_sha** | **String** | The SHA of the commit. | [required] |
**repos_create_commit_comment_request** | [**ReposCreateCommitCommentRequest**](ReposCreateCommitCommentRequest.md) |  | [required] |

### Return type

[**models::CommitComment**](commit-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_commit_signature_protection

> models::ProtectedBranchAdminEnforced repos_slash_create_commit_signature_protection(owner, repo, branch)
Create commit signature protection

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  When authenticated with admin or owner permissions to the repository, you can use this endpoint to require signed commits on a branch. You must enable branch protection to require signed commits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

[**models::ProtectedBranchAdminEnforced**](protected-branch-admin-enforced.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_commit_status

> models::Status repos_slash_create_commit_status(owner, repo, sha, repos_create_commit_status_request)
Create a commit status

Users with push access in a repository can create commit statuses for a given SHA.  Note: there is a limit of 1000 statuses per `sha` and `context` within a repository. Attempts to create more than 1000 statuses will result in a validation error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**sha** | **String** |  | [required] |
**repos_create_commit_status_request** | [**ReposCreateCommitStatusRequest**](ReposCreateCommitStatusRequest.md) |  | [required] |

### Return type

[**models::Status**](status.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_deploy_key

> models::DeployKey repos_slash_create_deploy_key(owner, repo, repos_create_deploy_key_request)
Create a deploy key

You can create a read-only deploy key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_create_deploy_key_request** | [**ReposCreateDeployKeyRequest**](ReposCreateDeployKeyRequest.md) |  | [required] |

### Return type

[**models::DeployKey**](deploy-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_deployment

> models::Deployment repos_slash_create_deployment(owner, repo, repos_create_deployment_request)
Create a deployment

Deployments offer a few configurable parameters with certain defaults.  The `ref` parameter can be any named branch, tag, or SHA. At GitHub we often deploy branches and verify them before we merge a pull request.  The `environment` parameter allows deployments to be issued to different runtime environments. Teams often have multiple environments for verifying their applications, such as `production`, `staging`, and `qa`. This parameter makes it easier to track which environments have requested deployments. The default environment is `production`.  The `auto_merge` parameter is used to ensure that the requested ref is not behind the repository's default branch. If the ref _is_ behind the default branch for the repository, we will attempt to merge it for you. If the merge succeeds, the API will return a successful merge commit. If merge conflicts prevent the merge from succeeding, the API will return a failure response.  By default, [commit statuses](https://docs.github.com/rest/commits/statuses) for every submitted context must be in a `success` state. The `required_contexts` parameter allows you to specify a subset of contexts that must be `success`, or to specify contexts that have not yet been submitted. You are not required to use commit statuses to deploy. If you do not require any contexts or create any commit statuses, the deployment will always succeed.  The `payload` parameter is available for any extra information that a deployment system might need. It is a JSON text field that will be passed on when a deployment event is dispatched.  The `task` parameter is used by the deployment system to allow different execution paths. In the web world this might be `deploy:migrations` to run schema changes on the system. In the compiled world this could be a flag to compile an application with debugging enabled.  Merged branch response:  You will see this response when GitHub automatically merges the base branch into the topic branch instead of creating a deployment. This auto-merge happens when: *   Auto-merge option is enabled in the repository *   Topic branch does not include the latest changes on the base branch, which is `master` in the response example *   There are no merge conflicts  If there are no new commits in the base branch, a new request to create a deployment should give a successful response.  Merge conflict response:  This error happens when the `auto_merge` option is enabled and when the default branch (in this case `master`), can't be merged into the branch that's being deployed (in this case `topic-branch`), due to merge conflicts.  Failed commit status checks:  This error happens when the `required_contexts` parameter indicates that one or more contexts need to have a `success` status for the commit to be deployed, but one or more of the required contexts do not have a state of `success`.  OAuth app tokens and personal access tokens (classic) need the `repo` or `repo_deployment` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_create_deployment_request** | [**ReposCreateDeploymentRequest**](ReposCreateDeploymentRequest.md) |  | [required] |

### Return type

[**models::Deployment**](deployment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_deployment_branch_policy

> models::DeploymentBranchPolicy repos_slash_create_deployment_branch_policy(owner, repo, environment_name, deployment_branch_policy_name_pattern_with_type)
Create a deployment branch policy

Creates a deployment branch or tag policy for an environment.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**deployment_branch_policy_name_pattern_with_type** | [**DeploymentBranchPolicyNamePatternWithType**](DeploymentBranchPolicyNamePatternWithType.md) |  | [required] |

### Return type

[**models::DeploymentBranchPolicy**](deployment-branch-policy.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_deployment_protection_rule

> models::DeploymentProtectionRule repos_slash_create_deployment_protection_rule(environment_name, repo, owner, repos_create_deployment_protection_rule_request)
Create a custom deployment protection rule on an environment

Enable a custom deployment protection rule for an environment.  The authenticated user must have admin or owner permissions to the repository to use this endpoint.  For more information about the app that is providing this custom deployment rule, see the [documentation for the `GET /apps/{app_slug}` endpoint](https://docs.github.com/rest/apps/apps#get-an-app).  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repos_create_deployment_protection_rule_request** | [**ReposCreateDeploymentProtectionRuleRequest**](ReposCreateDeploymentProtectionRuleRequest.md) |  | [required] |

### Return type

[**models::DeploymentProtectionRule**](deployment-protection-rule.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_deployment_status

> models::DeploymentStatus repos_slash_create_deployment_status(owner, repo, deployment_id, repos_create_deployment_status_request)
Create a deployment status

Users with `push` access can create deployment statuses for a given deployment.  OAuth app tokens and personal access tokens (classic) need the `repo_deployment` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**deployment_id** | **i32** | deployment_id parameter | [required] |
**repos_create_deployment_status_request** | [**ReposCreateDeploymentStatusRequest**](ReposCreateDeploymentStatusRequest.md) |  | [required] |

### Return type

[**models::DeploymentStatus**](deployment-status.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_dispatch_event

> repos_slash_create_dispatch_event(owner, repo, repos_create_dispatch_event_request)
Create a repository dispatch event

You can use this endpoint to trigger a webhook event called `repository_dispatch` when you want activity that happens outside of GitHub to trigger a GitHub Actions workflow or GitHub App webhook. You must configure your GitHub Actions workflow or GitHub App to run when the `repository_dispatch` event occurs. For an example `repository_dispatch` webhook payload, see \"[RepositoryDispatchEvent](https://docs.github.com/webhooks/event-payloads/#repository_dispatch).\"  The `client_payload` parameter is available for any extra information that your workflow might need. This parameter is a JSON payload that will be passed on when the webhook event is dispatched. For example, the `client_payload` can include a message that a user would like to send using a GitHub Actions workflow. Or the `client_payload` can be used as a test to debug your workflow.  This input example shows how you can use the `client_payload` as a test to debug your workflow.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_create_dispatch_event_request** | [**ReposCreateDispatchEventRequest**](ReposCreateDispatchEventRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_for_authenticated_user

> models::FullRepository repos_slash_create_for_authenticated_user(repos_create_for_authenticated_user_request)
Create a repository for the authenticated user

Creates a new repository for the authenticated user.  OAuth app tokens and personal access tokens (classic) need the `public_repo` or `repo` scope to create a public repository, and `repo` scope to create a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repos_create_for_authenticated_user_request** | [**ReposCreateForAuthenticatedUserRequest**](ReposCreateForAuthenticatedUserRequest.md) |  | [required] |

### Return type

[**models::FullRepository**](full-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_fork

> models::FullRepository repos_slash_create_fork(owner, repo, repos_create_fork_request)
Create a fork

Create a fork for the authenticated user.  > [!NOTE] > Forking a Repository happens asynchronously. You may have to wait a short period of time before you can access the git objects. If this takes longer than 5 minutes, be sure to contact [GitHub Support](https://support.github.com/contact?tags=dotcom-rest-api).  > [!NOTE] > Although this endpoint works with GitHub Apps, the GitHub App must be installed on the destination account with access to all repositories and on the source account with access to the source repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_create_fork_request** | Option<[**ReposCreateForkRequest**](ReposCreateForkRequest.md)> |  |  |

### Return type

[**models::FullRepository**](full-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_in_org

> models::FullRepository repos_slash_create_in_org(org, repos_create_in_org_request)
Create an organization repository

Creates a new repository in the specified organization. The authenticated user must be a member of the organization.  OAuth app tokens and personal access tokens (classic) need the `public_repo` or `repo` scope to create a public repository, and `repo` scope to create a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**repos_create_in_org_request** | [**ReposCreateInOrgRequest**](ReposCreateInOrgRequest.md) |  | [required] |

### Return type

[**models::FullRepository**](full-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_or_update_custom_properties_values

> repos_slash_create_or_update_custom_properties_values(owner, repo, repos_create_or_update_custom_properties_values_request)
Create or update custom property values for a repository

Create new or update existing custom property values for a repository. Using a value of `null` for a custom property will remove or 'unset' the property value from the repository.  Repository admins and other users with the repository-level \"edit custom property values\" fine-grained permission can use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_create_or_update_custom_properties_values_request** | [**ReposCreateOrUpdateCustomPropertiesValuesRequest**](ReposCreateOrUpdateCustomPropertiesValuesRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_or_update_environment

> models::Environment repos_slash_create_or_update_environment(owner, repo, environment_name, repos_create_or_update_environment_request)
Create or update an environment

Create or update an environment with protection rules, such as required reviewers. For more information about environment protection rules, see \"[Environments](/actions/reference/environments#environment-protection-rules).\"  > [!NOTE] > To create or update name patterns that branches must match in order to deploy to this environment, see \"[Deployment branch policies](/rest/deployments/branch-policies).\"  > [!NOTE] > To create or update secrets for an environment, see \"[GitHub Actions secrets](/rest/actions/secrets).\"  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**repos_create_or_update_environment_request** | Option<[**ReposCreateOrUpdateEnvironmentRequest**](ReposCreateOrUpdateEnvironmentRequest.md)> |  |  |

### Return type

[**models::Environment**](environment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_or_update_file_contents

> models::FileCommit repos_slash_create_or_update_file_contents(owner, repo, path, repos_create_or_update_file_contents_request)
Create or update file contents

Creates a new file or replaces an existing file in a repository.  > [!NOTE] > If you use this endpoint and the \"[Delete a file](https://docs.github.com/rest/repos/contents/#delete-a-file)\" endpoint in parallel, the concurrent requests will conflict and you will receive errors. You must use these endpoints serially instead.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint. The `workflow` scope is also required in order to modify files in the `.github/workflows` directory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**path** | **String** | path parameter | [required] |
**repos_create_or_update_file_contents_request** | [**ReposCreateOrUpdateFileContentsRequest**](ReposCreateOrUpdateFileContentsRequest.md) |  | [required] |

### Return type

[**models::FileCommit**](file-commit.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_org_ruleset

> models::RepositoryRuleset repos_slash_create_org_ruleset(org, repos_create_org_ruleset_request)
Create an organization repository ruleset

Create a repository ruleset for an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**repos_create_org_ruleset_request** | [**ReposCreateOrgRulesetRequest**](ReposCreateOrgRulesetRequest.md) | Request body | [required] |

### Return type

[**models::RepositoryRuleset**](repository-ruleset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_pages_deployment

> models::PageDeployment repos_slash_create_pages_deployment(owner, repo, repos_create_pages_deployment_request)
Create a GitHub Pages deployment

Create a GitHub Pages deployment for a repository.  The authenticated user must have write permission to the repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_create_pages_deployment_request** | [**ReposCreatePagesDeploymentRequest**](ReposCreatePagesDeploymentRequest.md) |  | [required] |

### Return type

[**models::PageDeployment**](page-deployment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_pages_site

> models::Page repos_slash_create_pages_site(owner, repo, repos_create_pages_site_request)
Create a GitHub Pages site

Configures a GitHub Pages site. For more information, see \"[About GitHub Pages](/github/working-with-github-pages/about-github-pages).\"  The authenticated user must be a repository administrator, maintainer, or have the 'manage GitHub Pages settings' permission.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_create_pages_site_request** | Option<[**ReposCreatePagesSiteRequest**](ReposCreatePagesSiteRequest.md)> |  | [required] |

### Return type

[**models::Page**](page.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_release

> models::Release repos_slash_create_release(owner, repo, repos_create_release_request)
Create a release

Users with push access to the repository can create a release.  This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see \"[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)\" and \"[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_create_release_request** | [**ReposCreateReleaseRequest**](ReposCreateReleaseRequest.md) |  | [required] |

### Return type

[**models::Release**](release.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_repo_ruleset

> models::RepositoryRuleset repos_slash_create_repo_ruleset(owner, repo, repos_create_repo_ruleset_request)
Create a repository ruleset

Create a ruleset for a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_create_repo_ruleset_request** | [**ReposCreateRepoRulesetRequest**](ReposCreateRepoRulesetRequest.md) | Request body | [required] |

### Return type

[**models::RepositoryRuleset**](repository-ruleset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_tag_protection

> models::TagProtection repos_slash_create_tag_protection(owner, repo, repos_create_tag_protection_request)
Closing down - Create a tag protection state for a repository

> [!WARNING] > **Closing down notice:** This operation is closing down and will be removed after August 30, 2024. Use the \"[Repository Rulesets](https://docs.github.com/rest/repos/rules#create-a-repository-ruleset)\" endpoint instead.  This creates a tag protection state for a repository. This endpoint is only available to repository administrators.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_create_tag_protection_request** | [**ReposCreateTagProtectionRequest**](ReposCreateTagProtectionRequest.md) |  | [required] |

### Return type

[**models::TagProtection**](tag-protection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_using_template

> models::FullRepository repos_slash_create_using_template(template_owner, template_repo, repos_create_using_template_request)
Create a repository using a template

Creates a new repository using a repository template. Use the `template_owner` and `template_repo` route parameters to specify the repository to use as the template. If the repository is not public, the authenticated user must own or be a member of an organization that owns the repository. To check if a repository is available to use as a template, get the repository's information using the [Get a repository](https://docs.github.com/rest/repos/repos#get-a-repository) endpoint and check that the `is_template` key is `true`.  OAuth app tokens and personal access tokens (classic) need the `public_repo` or `repo` scope to create a public repository, and `repo` scope to create a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_owner** | **String** | The account owner of the template repository. The name is not case sensitive. | [required] |
**template_repo** | **String** | The name of the template repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_create_using_template_request** | [**ReposCreateUsingTemplateRequest**](ReposCreateUsingTemplateRequest.md) |  | [required] |

### Return type

[**models::FullRepository**](full-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_create_webhook

> models::Hook repos_slash_create_webhook(owner, repo, repos_create_webhook_request)
Create a repository webhook

Repositories can have multiple webhooks installed. Each webhook should have a unique `config`. Multiple webhooks can share the same `config` as long as those webhooks do not have any `events` that overlap.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_create_webhook_request** | Option<[**ReposCreateWebhookRequest**](ReposCreateWebhookRequest.md)> |  |  |

### Return type

[**models::Hook**](hook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_decline_invitation_for_authenticated_user

> repos_slash_decline_invitation_for_authenticated_user(invitation_id)
Decline a repository invitation



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitation_id** | **i32** | The unique identifier of the invitation. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete

> repos_slash_delete(owner, repo)
Delete a repository

Deleting a repository requires admin access.  If an organization owner has configured the organization to prevent members from deleting organization-owned repositories, you will get a `403 Forbidden` response.  OAuth app tokens and personal access tokens (classic) need the `delete_repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_access_restrictions

> repos_slash_delete_access_restrictions(owner, repo, branch)
Delete access restrictions

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Disables the ability to restrict who can push to this branch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_admin_branch_protection

> repos_slash_delete_admin_branch_protection(owner, repo, branch)
Delete admin branch protection

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Removing admin enforcement requires admin or owner permissions to the repository and branch protection to be enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_an_environment

> repos_slash_delete_an_environment(owner, repo, environment_name)
Delete an environment

OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_autolink

> repos_slash_delete_autolink(owner, repo, autolink_id)
Delete an autolink reference from a repository

This deletes a single autolink reference by ID that was configured for the given repository.  Information about autolinks are only available to repository administrators.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**autolink_id** | **i32** | The unique identifier of the autolink. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_branch_protection

> repos_slash_delete_branch_protection(owner, repo, branch)
Delete branch protection

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_commit_comment

> repos_slash_delete_commit_comment(owner, repo, comment_id)
Delete a commit comment



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**comment_id** | **i64** | The unique identifier of the comment. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_commit_signature_protection

> repos_slash_delete_commit_signature_protection(owner, repo, branch)
Delete commit signature protection

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  When authenticated with admin or owner permissions to the repository, you can use this endpoint to disable required signed commits on a branch. You must enable branch protection to require signed commits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_deploy_key

> repos_slash_delete_deploy_key(owner, repo, key_id)
Delete a deploy key

Deploy keys are immutable. If you need to update a key, remove the key and create a new one instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**key_id** | **i32** | The unique identifier of the key. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_deployment

> repos_slash_delete_deployment(owner, repo, deployment_id)
Delete a deployment

If the repository only has one deployment, you can delete the deployment regardless of its status. If the repository has more than one deployment, you can only delete inactive deployments. This ensures that repositories with multiple deployments will always have an active deployment.  To set a deployment as inactive, you must:  *   Create a new deployment that is active so that the system has a record of the current state, then delete the previously active deployment. *   Mark the active deployment as inactive by adding any non-successful deployment status.  For more information, see \"[Create a deployment](https://docs.github.com/rest/deployments/deployments/#create-a-deployment)\" and \"[Create a deployment status](https://docs.github.com/rest/deployments/statuses#create-a-deployment-status).\"  OAuth app tokens and personal access tokens (classic) need the `repo` or `repo_deployment` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**deployment_id** | **i32** | deployment_id parameter | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_deployment_branch_policy

> repos_slash_delete_deployment_branch_policy(owner, repo, environment_name, branch_policy_id)
Delete a deployment branch policy

Deletes a deployment branch or tag policy for an environment.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**branch_policy_id** | **i32** | The unique identifier of the branch policy. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_file

> models::FileCommit repos_slash_delete_file(owner, repo, path, repos_delete_file_request)
Delete a file

Deletes a file in a repository.  You can provide an additional `committer` parameter, which is an object containing information about the committer. Or, you can provide an `author` parameter, which is an object containing information about the author.  The `author` section is optional and is filled in with the `committer` information if omitted. If the `committer` information is omitted, the authenticated user's information is used.  You must provide values for both `name` and `email`, whether you choose to use `author` or `committer`. Otherwise, you'll receive a `422` status code.  > [!NOTE] > If you use this endpoint and the \"[Create or update file contents](https://docs.github.com/rest/repos/contents/#create-or-update-file-contents)\" endpoint in parallel, the concurrent requests will conflict and you will receive errors. You must use these endpoints serially instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**path** | **String** | path parameter | [required] |
**repos_delete_file_request** | [**ReposDeleteFileRequest**](ReposDeleteFileRequest.md) |  | [required] |

### Return type

[**models::FileCommit**](file-commit.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_invitation

> repos_slash_delete_invitation(owner, repo, invitation_id)
Delete a repository invitation



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**invitation_id** | **i32** | The unique identifier of the invitation. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_org_ruleset

> repos_slash_delete_org_ruleset(org, ruleset_id)
Delete an organization repository ruleset

Delete a ruleset for an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**ruleset_id** | **i32** | The ID of the ruleset. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_pages_site

> repos_slash_delete_pages_site(owner, repo)
Delete a GitHub Pages site

Deletes a GitHub Pages site. For more information, see \"[About GitHub Pages](/github/working-with-github-pages/about-github-pages).  The authenticated user must be a repository administrator, maintainer, or have the 'manage GitHub Pages settings' permission.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_pull_request_review_protection

> repos_slash_delete_pull_request_review_protection(owner, repo, branch)
Delete pull request review protection

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_release

> repos_slash_delete_release(owner, repo, release_id)
Delete a release

Users with push access to the repository can delete a release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**release_id** | **i32** | The unique identifier of the release. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_release_asset

> repos_slash_delete_release_asset(owner, repo, asset_id)
Delete a release asset



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**asset_id** | **i32** | The unique identifier of the asset. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_repo_ruleset

> repos_slash_delete_repo_ruleset(owner, repo, ruleset_id)
Delete a repository ruleset

Delete a ruleset for a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**ruleset_id** | **i32** | The ID of the ruleset. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_tag_protection

> repos_slash_delete_tag_protection(owner, repo, tag_protection_id)
Closing down - Delete a tag protection state for a repository

> [!WARNING] > **Closing down notice:** This operation is closing down and will be removed after August 30, 2024. Use the \"[Repository Rulesets](https://docs.github.com/rest/repos/rules#delete-a-repository-ruleset)\" endpoint instead.  This deletes a tag protection state for a repository. This endpoint is only available to repository administrators.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**tag_protection_id** | **i32** | The unique identifier of the tag protection. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_delete_webhook

> repos_slash_delete_webhook(owner, repo, hook_id)
Delete a repository webhook



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_disable_automated_security_fixes

> repos_slash_disable_automated_security_fixes(owner, repo)
Disable automated security fixes

Disables automated security fixes for a repository. The authenticated user must have admin access to the repository. For more information, see \"[Configuring automated security fixes](https://docs.github.com/articles/configuring-automated-security-fixes)\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_disable_deployment_protection_rule

> repos_slash_disable_deployment_protection_rule(environment_name, repo, owner, protection_rule_id)
Disable a custom protection rule for an environment

Disables a custom deployment protection rule for an environment.  The authenticated user must have admin or owner permissions to the repository to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**protection_rule_id** | **i32** | The unique identifier of the protection rule. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_disable_private_vulnerability_reporting

> repos_slash_disable_private_vulnerability_reporting(owner, repo)
Disable private vulnerability reporting for a repository

Disables private vulnerability reporting for a repository. The authenticated user must have admin access to the repository. For more information, see \"[Privately reporting a security vulnerability](https://docs.github.com/code-security/security-advisories/guidance-on-reporting-and-writing/privately-reporting-a-security-vulnerability)\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_disable_vulnerability_alerts

> repos_slash_disable_vulnerability_alerts(owner, repo)
Disable vulnerability alerts

Disables dependency alerts and the dependency graph for a repository. The authenticated user must have admin access to the repository. For more information, see \"[About security alerts for vulnerable dependencies](https://docs.github.com/articles/about-security-alerts-for-vulnerable-dependencies)\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_download_tarball_archive

> repos_slash_download_tarball_archive(owner, repo, r#ref)
Download a repository archive (tar)

Gets a redirect URL to download a tar archive for a repository. If you omit `:ref`, the repository’s default branch (usually `main`) will be used. Please make sure your HTTP framework is configured to follow redirects or you will need to use the `Location` header to make a second `GET` request.  > [!NOTE] > For private repositories, these links are temporary and expire after five minutes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**r#ref** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_download_zipball_archive

> repos_slash_download_zipball_archive(owner, repo, r#ref)
Download a repository archive (zip)

Gets a redirect URL to download a zip archive for a repository. If you omit `:ref`, the repository’s default branch (usually `main`) will be used. Please make sure your HTTP framework is configured to follow redirects or you will need to use the `Location` header to make a second `GET` request.  > [!NOTE] > For private repositories, these links are temporary and expire after five minutes. If the repository is empty, you will receive a 404 when you follow the redirect.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**r#ref** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_enable_automated_security_fixes

> repos_slash_enable_automated_security_fixes(owner, repo)
Enable automated security fixes

Enables automated security fixes for a repository. The authenticated user must have admin access to the repository. For more information, see \"[Configuring automated security fixes](https://docs.github.com/articles/configuring-automated-security-fixes)\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_enable_private_vulnerability_reporting

> repos_slash_enable_private_vulnerability_reporting(owner, repo)
Enable private vulnerability reporting for a repository

Enables private vulnerability reporting for a repository. The authenticated user must have admin access to the repository. For more information, see \"[Privately reporting a security vulnerability](https://docs.github.com/code-security/security-advisories/guidance-on-reporting-and-writing/privately-reporting-a-security-vulnerability).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_enable_vulnerability_alerts

> repos_slash_enable_vulnerability_alerts(owner, repo)
Enable vulnerability alerts

Enables dependency alerts and the dependency graph for a repository. The authenticated user must have admin access to the repository. For more information, see \"[About security alerts for vulnerable dependencies](https://docs.github.com/articles/about-security-alerts-for-vulnerable-dependencies)\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_generate_release_notes

> models::ReleaseNotesContent repos_slash_generate_release_notes(owner, repo, repos_generate_release_notes_request)
Generate release notes content for a release

Generate a name and body describing a [release](https://docs.github.com/rest/releases/releases#get-a-release). The body content will be markdown formatted and contain information like the changes since last release and users who contributed. The generated release notes are not saved anywhere. They are intended to be generated and used when creating a new release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_generate_release_notes_request** | [**ReposGenerateReleaseNotesRequest**](ReposGenerateReleaseNotesRequest.md) |  | [required] |

### Return type

[**models::ReleaseNotesContent**](release-notes-content.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get

> models::FullRepository repos_slash_get(owner, repo)
Get a repository

The `parent` and `source` objects are present when the repository is a fork. `parent` is the repository this repository was forked from, `source` is the ultimate source for the network.  > [!NOTE] > In order to see the `security_and_analysis` block for a repository you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see \"[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::FullRepository**](full-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_access_restrictions

> models::BranchRestrictionPolicy repos_slash_get_access_restrictions(owner, repo, branch)
Get access restrictions

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Lists who has access to this protected branch.  > [!NOTE] > Users, apps, and teams `restrictions` are only available for organization-owned repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

[**models::BranchRestrictionPolicy**](branch-restriction-policy.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_admin_branch_protection

> models::ProtectedBranchAdminEnforced repos_slash_get_admin_branch_protection(owner, repo, branch)
Get admin branch protection

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

[**models::ProtectedBranchAdminEnforced**](protected-branch-admin-enforced.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_all_deployment_protection_rules

> models::ReposGetAllDeploymentProtectionRules200Response repos_slash_get_all_deployment_protection_rules(environment_name, repo, owner)
Get all deployment protection rules for an environment

Gets all custom deployment protection rules that are enabled for an environment. Anyone with read access to the repository can use this endpoint. For more information about environments, see \"[Using environments for deployment](https://docs.github.com/actions/deployment/targeting-different-environments/using-environments-for-deployment).\"  For more information about the app that is providing this custom deployment rule, see the [documentation for the `GET /apps/{app_slug}` endpoint](https://docs.github.com/rest/apps/apps#get-an-app).  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |

### Return type

[**models::ReposGetAllDeploymentProtectionRules200Response**](repos_get_all_deployment_protection_rules_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_all_environments

> models::ReposGetAllEnvironments200Response repos_slash_get_all_environments(owner, repo, per_page, page)
List environments

Lists the environments for a repository.  Anyone with read access to the repository can use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ReposGetAllEnvironments200Response**](repos_get_all_environments_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_all_status_check_contexts

> Vec<String> repos_slash_get_all_status_check_contexts(owner, repo, branch)
Get all status check contexts

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_all_topics

> models::Topic repos_slash_get_all_topics(owner, repo, page, per_page)
Get all repository topics



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**models::Topic**](topic.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_apps_with_access_to_protected_branch

> Vec<models::Integration> repos_slash_get_apps_with_access_to_protected_branch(owner, repo, branch)
Get apps with access to the protected branch

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Lists the GitHub Apps that have push access to this branch. Only GitHub Apps that are installed on the repository and that have been granted write access to the repository contents can be added as authorized actors on a protected branch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

[**Vec<models::Integration>**](integration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_autolink

> models::Autolink repos_slash_get_autolink(owner, repo, autolink_id)
Get an autolink reference of a repository

This returns a single autolink reference by ID that was configured for the given repository.  Information about autolinks are only available to repository administrators.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**autolink_id** | **i32** | The unique identifier of the autolink. | [required] |

### Return type

[**models::Autolink**](autolink.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_branch

> models::BranchWithProtection repos_slash_get_branch(owner, repo, branch)
Get a branch



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

[**models::BranchWithProtection**](branch-with-protection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_branch_protection

> models::BranchProtection repos_slash_get_branch_protection(owner, repo, branch)
Get branch protection

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

[**models::BranchProtection**](branch-protection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_branch_rules

> Vec<models::RepositoryRuleDetailed> repos_slash_get_branch_rules(owner, repo, branch, per_page, page)
Get rules for a branch

Returns all active rules that apply to the specified branch. The branch does not need to exist; rules that would apply to a branch with that name will be returned. All active rules that apply will be returned, regardless of the level at which they are configured (e.g. repository or organization). Rules in rulesets with \"evaluate\" or \"disabled\" enforcement statuses are not returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::RepositoryRuleDetailed>**](repository-rule-detailed.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_clones

> models::CloneTraffic repos_slash_get_clones(owner, repo, per)
Get repository clones

Get the total number of clones and breakdown per day or week for the last 14 days. Timestamps are aligned to UTC midnight of the beginning of the day or week. Week begins on Monday.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per** | Option<**String**> | The time frame to display results for. |  |[default to day]

### Return type

[**models::CloneTraffic**](clone-traffic.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_code_frequency_stats

> Vec<Vec<i32>> repos_slash_get_code_frequency_stats(owner, repo)
Get the weekly commit activity

Returns a weekly aggregate of the number of additions and deletions pushed to a repository.  > [!NOTE] > This endpoint can only be used for repositories with fewer than 10,000 commits. If the repository contains 10,000 or more commits, a 422 status code will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**Vec<Vec<i32>>**](Vec.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_collaborator_permission_level

> models::RepositoryCollaboratorPermission repos_slash_get_collaborator_permission_level(owner, repo, username)
Get repository permissions for a user

Checks the repository permission of a collaborator. The possible repository permissions are `admin`, `write`, `read`, and `none`.  *Note*: The `permission` attribute provides the legacy base roles of `admin`, `write`, `read`, and `none`, where the `maintain` role is mapped to `write` and the `triage` role is mapped to `read`. To determine the role assigned to the collaborator, see the `role_name` attribute, which will provide the full role name, including custom roles. The `permissions` hash can also be used to determine which base level of access the collaborator has to the repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

[**models::RepositoryCollaboratorPermission**](repository-collaborator-permission.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_combined_status_for_ref

> models::CombinedCommitStatus repos_slash_get_combined_status_for_ref(owner, repo, r#ref, per_page, page)
Get the combined status for a specific reference

Users with pull access in a repository can access a combined view of commit statuses for a given ref. The ref can be a SHA, a branch name, or a tag name.   Additionally, a combined `state` is returned. The `state` is one of:  *   **failure** if any of the contexts report as `error` or `failure` *   **pending** if there are no statuses or a context is `pending` *   **success** if the latest status for all contexts is `success`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**r#ref** | **String** | The commit reference. Can be a commit SHA, branch name (`heads/BRANCH_NAME`), or tag name (`tags/TAG_NAME`). For more information, see \"[Git References](https://git-scm.com/book/en/v2/Git-Internals-Git-References)\" in the Git documentation. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::CombinedCommitStatus**](combined-commit-status.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_commit

> models::Commit repos_slash_get_commit(owner, repo, r#ref, page, per_page)
Get a commit

Returns the contents of a single commit reference. You must have `read` access for the repository to use this endpoint.  > [!NOTE] > If there are more than 300 files in the commit diff and the default JSON media type is requested, the response will include pagination link headers for the remaining files, up to a limit of 3000 files. Each page contains the static commit information, and the only changes are to the file listing.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\" Pagination query parameters are not supported for these media types.  - **`application/vnd.github.diff`**: Returns the diff of the commit. Larger diffs may time out and return a 5xx status code. - **`application/vnd.github.patch`**: Returns the patch of the commit. Diffs with binary data will have no `patch` property. Larger diffs may time out and return a 5xx status code. - **`application/vnd.github.sha`**: Returns the commit's SHA-1 hash. You can use this endpoint to check if a remote reference's SHA-1 hash is the same as your local reference's SHA-1 hash by providing the local SHA-1 reference as the ETag.  **Signature verification object**  The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:  | Name | Type | Description | | ---- | ---- | ----------- | | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. | | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. | | `signature` | `string` | The signature that was extracted from the commit. | | `payload` | `string` | The value that was signed. |  These are the possible values for `reason` in the `verification` object:  | Value | Description | | ----- | ----------- | | `expired_key` | The key that made the signature is expired. | | `not_signing_key` | The \"signing\" flag is not among the usage flags in the GPG key that made the signature. | | `gpgverify_error` | There was an error communicating with the signature verification service. | | `gpgverify_unavailable` | The signature verification service is currently unavailable. | | `unsigned` | The object does not include a signature. | | `unknown_signature_type` | A non-PGP signature was found in the commit. | | `no_user` | No user was associated with the `committer` email address in the commit. | | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on their account. | | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. | | `unknown_key` | The key that made the signature has not been registered with any user's account. | | `malformed_signature` | There was an error parsing the signature. | | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. | | `valid` | None of the above errors applied, so the signature is considered to be verified. |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**r#ref** | **String** | The commit reference. Can be a commit SHA, branch name (`heads/BRANCH_NAME`), or tag name (`tags/TAG_NAME`). For more information, see \"[Git References](https://git-scm.com/book/en/v2/Git-Internals-Git-References)\" in the Git documentation. | [required] |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**models::Commit**](commit.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_commit_activity_stats

> Vec<models::CommitActivity> repos_slash_get_commit_activity_stats(owner, repo)
Get the last year of commit activity

Returns the last year of commit activity grouped by week. The `days` array is a group of commits per day, starting on `Sunday`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**Vec<models::CommitActivity>**](commit-activity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_commit_comment

> models::CommitComment repos_slash_get_commit_comment(owner, repo, comment_id)
Get a commit comment

Gets a specified commit comment.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type. - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`. - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`. - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**comment_id** | **i64** | The unique identifier of the comment. | [required] |

### Return type

[**models::CommitComment**](commit-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_commit_signature_protection

> models::ProtectedBranchAdminEnforced repos_slash_get_commit_signature_protection(owner, repo, branch)
Get commit signature protection

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  When authenticated with admin or owner permissions to the repository, you can use this endpoint to check whether a branch requires signed commits. An enabled status of `true` indicates you must sign commits on this branch. For more information, see [Signing commits with GPG](https://docs.github.com/articles/signing-commits-with-gpg) in GitHub Help.  > [!NOTE] > You must enable branch protection to require signed commits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

[**models::ProtectedBranchAdminEnforced**](protected-branch-admin-enforced.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_community_profile_metrics

> models::CommunityProfile repos_slash_get_community_profile_metrics(owner, repo)
Get community profile metrics

Returns all community profile metrics for a repository. The repository cannot be a fork.  The returned metrics include an overall health score, the repository description, the presence of documentation, the detected code of conduct, the detected license, and the presence of ISSUE\\_TEMPLATE, PULL\\_REQUEST\\_TEMPLATE, README, and CONTRIBUTING files.  The `health_percentage` score is defined as a percentage of how many of the recommended community health files are present. For more information, see \"[About community profiles for public repositories](https://docs.github.com/communities/setting-up-your-project-for-healthy-contributions/about-community-profiles-for-public-repositories).\"  `content_reports_enabled` is only returned for organization-owned repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::CommunityProfile**](community-profile.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_content

> models::ContentTree repos_slash_get_content(owner, repo, path, r#ref)
Get repository content

Gets the contents of a file or directory in a repository. Specify the file path or directory with the `path` parameter. If you omit the `path` parameter, you will receive the contents of the repository's root directory.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github.raw+json`**: Returns the raw file contents for files and symlinks. - **`application/vnd.github.html+json`**: Returns the file contents in HTML. Markup languages are rendered to HTML using GitHub's open-source [Markup library](https://github.com/github/markup). - **`application/vnd.github.object+json`**: Returns the contents in a consistent object format regardless of the content type. For example, instead of an array of objects for a directory, the response will be an object with an `entries` attribute containing the array of objects.  If the content is a directory, the response will be an array of objects, one object for each item in the directory. When listing the contents of a directory, submodules have their \"type\" specified as \"file\". Logically, the value _should_ be \"submodule\". This behavior exists [for backwards compatibility purposes](https://git.io/v1YCW). In the next major version of the API, the type will be returned as \"submodule\".  If the content is a symlink and the symlink's target is a normal file in the repository, then the API responds with the content of the file. Otherwise, the API responds with an object describing the symlink itself.  If the content is a submodule, the `submodule_git_url` field identifies the location of the submodule repository, and the `sha` identifies a specific commit within the submodule repository. Git uses the given URL when cloning the submodule repository, and checks out the submodule at that specific commit. If the submodule repository is not hosted on github.com, the Git URLs (`git_url` and `_links[\"git\"]`) and the github.com URLs (`html_url` and `_links[\"html\"]`) will have null values.  **Notes**:  - To get a repository's contents recursively, you can [recursively get the tree](https://docs.github.com/rest/git/trees#get-a-tree). - This API has an upper limit of 1,000 files for a directory. If you need to retrieve more files, use the [Git Trees API](https://docs.github.com/rest/git/trees#get-a-tree). - Download URLs expire and are meant to be used just once. To ensure the download URL does not expire, please use the contents API to obtain a fresh download URL for each download. - If the requested file's size is:   - 1 MB or smaller: All features of this endpoint are supported.   - Between 1-100 MB: Only the `raw` or `object` custom media types are supported. Both will work as normal, except that when using the `object` media type, the `content` field will be an empty string and the `encoding` field will be `\"none\"`. To get the contents of these larger files, use the `raw` media type.   - Greater than 100 MB: This endpoint is not supported.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**path** | **String** | path parameter | [required] |
**r#ref** | Option<**String**> | The name of the commit/branch/tag. Default: the repository’s default branch. |  |

### Return type

[**models::ContentTree**](content-tree.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.github.object, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_contributors_stats

> Vec<models::ContributorActivity> repos_slash_get_contributors_stats(owner, repo)
Get all contributor commit activity

 Returns the `total` number of commits authored by the contributor. In addition, the response includes a Weekly Hash (`weeks` array) with the following information:  *   `w` - Start of the week, given as a [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time). *   `a` - Number of additions *   `d` - Number of deletions *   `c` - Number of commits  > [!NOTE] > This endpoint will return `0` values for all addition and deletion counts in repositories with 10,000 or more commits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**Vec<models::ContributorActivity>**](contributor-activity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_custom_deployment_protection_rule

> models::DeploymentProtectionRule repos_slash_get_custom_deployment_protection_rule(owner, repo, environment_name, protection_rule_id)
Get a custom deployment protection rule

Gets an enabled custom deployment protection rule for an environment. Anyone with read access to the repository can use this endpoint. For more information about environments, see \"[Using environments for deployment](https://docs.github.com/actions/deployment/targeting-different-environments/using-environments-for-deployment).\"  For more information about the app that is providing this custom deployment rule, see [`GET /apps/{app_slug}`](https://docs.github.com/rest/apps/apps#get-an-app).  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**protection_rule_id** | **i32** | The unique identifier of the protection rule. | [required] |

### Return type

[**models::DeploymentProtectionRule**](deployment-protection-rule.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_custom_properties_values

> Vec<models::CustomPropertyValue> repos_slash_get_custom_properties_values(owner, repo)
Get all custom property values for a repository

Gets all custom property values that are set for a repository. Users with read access to the repository can use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**Vec<models::CustomPropertyValue>**](custom-property-value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_deploy_key

> models::DeployKey repos_slash_get_deploy_key(owner, repo, key_id)
Get a deploy key



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**key_id** | **i32** | The unique identifier of the key. | [required] |

### Return type

[**models::DeployKey**](deploy-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_deployment

> models::Deployment repos_slash_get_deployment(owner, repo, deployment_id)
Get a deployment



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**deployment_id** | **i32** | deployment_id parameter | [required] |

### Return type

[**models::Deployment**](deployment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_deployment_branch_policy

> models::DeploymentBranchPolicy repos_slash_get_deployment_branch_policy(owner, repo, environment_name, branch_policy_id)
Get a deployment branch policy

Gets a deployment branch or tag policy for an environment.  Anyone with read access to the repository can use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**branch_policy_id** | **i32** | The unique identifier of the branch policy. | [required] |

### Return type

[**models::DeploymentBranchPolicy**](deployment-branch-policy.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_deployment_status

> models::DeploymentStatus repos_slash_get_deployment_status(owner, repo, deployment_id, status_id)
Get a deployment status

Users with pull access can view a deployment status for a deployment:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**deployment_id** | **i32** | deployment_id parameter | [required] |
**status_id** | **i32** |  | [required] |

### Return type

[**models::DeploymentStatus**](deployment-status.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_environment

> models::Environment repos_slash_get_environment(owner, repo, environment_name)
Get an environment

> [!NOTE] > To get information about name patterns that branches must match in order to deploy to this environment, see \"[Get a deployment branch policy](/rest/deployments/branch-policies#get-a-deployment-branch-policy).\"  Anyone with read access to the repository can use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |

### Return type

[**models::Environment**](environment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_latest_pages_build

> models::PageBuild repos_slash_get_latest_pages_build(owner, repo)
Get latest Pages build

Gets information about the single most recent build of a GitHub Pages site.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::PageBuild**](page-build.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_latest_release

> models::Release repos_slash_get_latest_release(owner, repo)
Get the latest release

View the latest published full release for the repository.  The latest release is the most recent non-prerelease, non-draft release, sorted by the `created_at` attribute. The `created_at` attribute is the date of the commit used for the release, and not the date when the release was drafted or published.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::Release**](release.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_org_rule_suite

> models::RuleSuite repos_slash_get_org_rule_suite(org, rule_suite_id)
Get an organization rule suite

Gets information about a suite of rule evaluations from within an organization. For more information, see \"[Managing rulesets for repositories in your organization](https://docs.github.com/organizations/managing-organization-settings/managing-rulesets-for-repositories-in-your-organization#viewing-insights-for-rulesets).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**rule_suite_id** | **i32** | The unique identifier of the rule suite result. To get this ID, you can use [GET /repos/{owner}/{repo}/rulesets/rule-suites](https://docs.github.com/rest/repos/rule-suites#list-repository-rule-suites) for repositories and [GET /orgs/{org}/rulesets/rule-suites](https://docs.github.com/rest/orgs/rule-suites#list-organization-rule-suites) for organizations. | [required] |

### Return type

[**models::RuleSuite**](rule-suite.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_org_rule_suites

> Vec<models::RuleSuitesInner> repos_slash_get_org_rule_suites(org, r#ref, repository_name, time_period, actor_name, rule_suite_result, per_page, page)
List organization rule suites

Lists suites of rule evaluations at the organization level. For more information, see \"[Managing rulesets for repositories in your organization](https://docs.github.com/organizations/managing-organization-settings/managing-rulesets-for-repositories-in-your-organization#viewing-insights-for-rulesets).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**r#ref** | Option<**String**> | The name of the ref. Cannot contain wildcard characters. Optionally prefix with `refs/heads/` to limit to branches or `refs/tags/` to limit to tags. Omit the prefix to search across all refs. When specified, only rule evaluations triggered for this ref will be returned. |  |
**repository_name** | Option<**i32**> | The name of the repository to filter on. When specified, only rule evaluations from this repository will be returned. |  |
**time_period** | Option<**String**> | The time period to filter by.  For example, `day` will filter for rule suites that occurred in the past 24 hours, and `week` will filter for insights that occurred in the past 7 days (168 hours). |  |[default to day]
**actor_name** | Option<**String**> | The handle for the GitHub user account to filter on. When specified, only rule evaluations triggered by this actor will be returned. |  |
**rule_suite_result** | Option<**String**> | The rule results to filter on. When specified, only suites with this result will be returned. |  |[default to all]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::RuleSuitesInner>**](rule_suites_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_org_ruleset

> models::RepositoryRuleset repos_slash_get_org_ruleset(org, ruleset_id)
Get an organization repository ruleset

Get a repository ruleset for an organization.  **Note:** To prevent leaking sensitive information, the `bypass_actors` property is only returned if the user making the API request has write access to the ruleset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**ruleset_id** | **i32** | The ID of the ruleset. | [required] |

### Return type

[**models::RepositoryRuleset**](repository-ruleset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_org_rulesets

> Vec<models::RepositoryRuleset> repos_slash_get_org_rulesets(org, per_page, page, targets)
Get all organization repository rulesets

Get all the repository rulesets for an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**targets** | Option<**String**> | A comma-separated list of rule targets to filter by. If provided, only rulesets that apply to the specified targets will be returned. For example, `branch,tag,push`.  |  |

### Return type

[**Vec<models::RepositoryRuleset>**](repository-ruleset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_pages

> models::Page repos_slash_get_pages(owner, repo)
Get a GitHub Pages site

Gets information about a GitHub Pages site.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::Page**](page.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_pages_build

> models::PageBuild repos_slash_get_pages_build(owner, repo, build_id)
Get GitHub Pages build

Gets information about a GitHub Pages build.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**build_id** | **i32** |  | [required] |

### Return type

[**models::PageBuild**](page-build.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_pages_deployment

> models::PagesDeploymentStatus repos_slash_get_pages_deployment(owner, repo, pages_deployment_id)
Get the status of a GitHub Pages deployment

Gets the current status of a GitHub Pages deployment.  The authenticated user must have read permission for the GitHub Pages site.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**pages_deployment_id** | [**ActionsGetWorkflowWorkflowIdParameter**](.md) | The ID of the Pages deployment. You can also give the commit SHA of the deployment. | [required] |

### Return type

[**models::PagesDeploymentStatus**](pages-deployment-status.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_pages_health_check

> models::PagesHealthCheck repos_slash_get_pages_health_check(owner, repo)
Get a DNS health check for GitHub Pages

Gets a health check of the DNS settings for the `CNAME` record configured for a repository's GitHub Pages.  The first request to this endpoint returns a `202 Accepted` status and starts an asynchronous background task to get the results for the domain. After the background task completes, subsequent requests to this endpoint return a `200 OK` status with the health check results in the response.  The authenticated user must be a repository administrator, maintainer, or have the 'manage GitHub Pages settings' permission to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::PagesHealthCheck**](pages-health-check.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_participation_stats

> models::ParticipationStats repos_slash_get_participation_stats(owner, repo)
Get the weekly commit count

Returns the total commit counts for the `owner` and total commit counts in `all`. `all` is everyone combined, including the `owner` in the last 52 weeks. If you'd like to get the commit counts for non-owners, you can subtract `owner` from `all`.  The array order is oldest week (index 0) to most recent week.  The most recent week is seven days ago at UTC midnight to today at UTC midnight.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::ParticipationStats**](participation-stats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_pull_request_review_protection

> models::ProtectedBranchPullRequestReview repos_slash_get_pull_request_review_protection(owner, repo, branch)
Get pull request review protection

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

[**models::ProtectedBranchPullRequestReview**](protected-branch-pull-request-review.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_punch_card_stats

> Vec<Vec<i32>> repos_slash_get_punch_card_stats(owner, repo)
Get the hourly commit count for each day

Each array contains the day number, hour number, and number of commits:  *   `0-6`: Sunday - Saturday *   `0-23`: Hour of day *   Number of commits  For example, `[2, 14, 25]` indicates that there were 25 total commits, during the 2:00pm hour on Tuesdays. All times are based on the time zone of individual commits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**Vec<Vec<i32>>**](Vec.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_readme

> models::ContentFile repos_slash_get_readme(owner, repo, r#ref)
Get a repository README

Gets the preferred README for a repository.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github.raw+json`**: Returns the raw file contents. This is the default if you do not specify a media type. - **`application/vnd.github.html+json`**: Returns the README in HTML. Markup languages are rendered to HTML using GitHub's open-source [Markup library](https://github.com/github/markup).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**r#ref** | Option<**String**> | The name of the commit/branch/tag. Default: the repository’s default branch. |  |

### Return type

[**models::ContentFile**](content-file.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_readme_in_directory

> models::ContentFile repos_slash_get_readme_in_directory(owner, repo, dir, r#ref)
Get a repository README for a directory

Gets the README from a repository directory.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github.raw+json`**: Returns the raw file contents. This is the default if you do not specify a media type. - **`application/vnd.github.html+json`**: Returns the README in HTML. Markup languages are rendered to HTML using GitHub's open-source [Markup library](https://github.com/github/markup).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**dir** | **String** | The alternate path to look for a README file | [required] |
**r#ref** | Option<**String**> | The name of the commit/branch/tag. Default: the repository’s default branch. |  |

### Return type

[**models::ContentFile**](content-file.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_release

> models::Release repos_slash_get_release(owner, repo, release_id)
Get a release

Gets a public release with the specified release ID.  > [!NOTE] > This returns an `upload_url` key corresponding to the endpoint for uploading release assets. This key is a hypermedia resource. For more information, see \"[Getting started with the REST API](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#hypermedia).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**release_id** | **i32** | The unique identifier of the release. | [required] |

### Return type

[**models::Release**](release.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_release_asset

> models::ReleaseAsset repos_slash_get_release_asset(owner, repo, asset_id)
Get a release asset

To download the asset's binary content, set the `Accept` header of the request to [`application/octet-stream`](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types). The API will either redirect the client to the location, or stream it directly if possible. API clients should handle both a `200` or `302` response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**asset_id** | **i32** | The unique identifier of the asset. | [required] |

### Return type

[**models::ReleaseAsset**](release-asset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_release_by_tag

> models::Release repos_slash_get_release_by_tag(owner, repo, tag)
Get a release by tag name

Get a published release with the specified tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**tag** | **String** | tag parameter | [required] |

### Return type

[**models::Release**](release.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_repo_rule_suite

> models::RuleSuite repos_slash_get_repo_rule_suite(owner, repo, rule_suite_id)
Get a repository rule suite

Gets information about a suite of rule evaluations from within a repository. For more information, see \"[Managing rulesets for a repository](https://docs.github.com/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/managing-rulesets-for-a-repository#viewing-insights-for-rulesets).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**rule_suite_id** | **i32** | The unique identifier of the rule suite result. To get this ID, you can use [GET /repos/{owner}/{repo}/rulesets/rule-suites](https://docs.github.com/rest/repos/rule-suites#list-repository-rule-suites) for repositories and [GET /orgs/{org}/rulesets/rule-suites](https://docs.github.com/rest/orgs/rule-suites#list-organization-rule-suites) for organizations. | [required] |

### Return type

[**models::RuleSuite**](rule-suite.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_repo_rule_suites

> Vec<models::RuleSuitesInner> repos_slash_get_repo_rule_suites(owner, repo, r#ref, time_period, actor_name, rule_suite_result, per_page, page)
List repository rule suites

Lists suites of rule evaluations at the repository level. For more information, see \"[Managing rulesets for a repository](https://docs.github.com/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/managing-rulesets-for-a-repository#viewing-insights-for-rulesets).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**r#ref** | Option<**String**> | The name of the ref. Cannot contain wildcard characters. Optionally prefix with `refs/heads/` to limit to branches or `refs/tags/` to limit to tags. Omit the prefix to search across all refs. When specified, only rule evaluations triggered for this ref will be returned. |  |
**time_period** | Option<**String**> | The time period to filter by.  For example, `day` will filter for rule suites that occurred in the past 24 hours, and `week` will filter for insights that occurred in the past 7 days (168 hours). |  |[default to day]
**actor_name** | Option<**String**> | The handle for the GitHub user account to filter on. When specified, only rule evaluations triggered by this actor will be returned. |  |
**rule_suite_result** | Option<**String**> | The rule results to filter on. When specified, only suites with this result will be returned. |  |[default to all]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::RuleSuitesInner>**](rule_suites_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_repo_ruleset

> models::RepositoryRuleset repos_slash_get_repo_ruleset(owner, repo, ruleset_id, includes_parents)
Get a repository ruleset

Get a ruleset for a repository.  **Note:** To prevent leaking sensitive information, the `bypass_actors` property is only returned if the user making the API request has write access to the ruleset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**ruleset_id** | **i32** | The ID of the ruleset. | [required] |
**includes_parents** | Option<**bool**> | Include rulesets configured at higher levels that apply to this repository |  |[default to true]

### Return type

[**models::RepositoryRuleset**](repository-ruleset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_repo_rulesets

> Vec<models::RepositoryRuleset> repos_slash_get_repo_rulesets(owner, repo, per_page, page, includes_parents, targets)
Get all repository rulesets

Get all the rulesets for a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**includes_parents** | Option<**bool**> | Include rulesets configured at higher levels that apply to this repository |  |[default to true]
**targets** | Option<**String**> | A comma-separated list of rule targets to filter by. If provided, only rulesets that apply to the specified targets will be returned. For example, `branch,tag,push`.  |  |

### Return type

[**Vec<models::RepositoryRuleset>**](repository-ruleset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_status_checks_protection

> models::StatusCheckPolicy repos_slash_get_status_checks_protection(owner, repo, branch)
Get status checks protection

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

[**models::StatusCheckPolicy**](status-check-policy.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_teams_with_access_to_protected_branch

> Vec<models::Team> repos_slash_get_teams_with_access_to_protected_branch(owner, repo, branch)
Get teams with access to the protected branch

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Lists the teams who have push access to this branch. The list includes child teams.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

[**Vec<models::Team>**](team.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_top_paths

> Vec<models::ContentTraffic> repos_slash_get_top_paths(owner, repo)
Get top referral paths

Get the top 10 popular contents over the last 14 days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**Vec<models::ContentTraffic>**](content-traffic.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_top_referrers

> Vec<models::ReferrerTraffic> repos_slash_get_top_referrers(owner, repo)
Get top referral sources

Get the top 10 referrers over the last 14 days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**Vec<models::ReferrerTraffic>**](referrer-traffic.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_users_with_access_to_protected_branch

> Vec<models::SimpleUser> repos_slash_get_users_with_access_to_protected_branch(owner, repo, branch)
Get users with access to the protected branch

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Lists the people who have push access to this branch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

[**Vec<models::SimpleUser>**](simple-user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_views

> models::ViewTraffic repos_slash_get_views(owner, repo, per)
Get page views

Get the total number of views and breakdown per day or week for the last 14 days. Timestamps are aligned to UTC midnight of the beginning of the day or week. Week begins on Monday.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per** | Option<**String**> | The time frame to display results for. |  |[default to day]

### Return type

[**models::ViewTraffic**](view-traffic.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_webhook

> models::Hook repos_slash_get_webhook(owner, repo, hook_id)
Get a repository webhook

Returns a webhook configured in a repository. To get only the webhook `config` properties, see \"[Get a webhook configuration for a repository](/rest/webhooks/repo-config#get-a-webhook-configuration-for-a-repository).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |

### Return type

[**models::Hook**](hook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_webhook_config_for_repo

> models::WebhookConfig repos_slash_get_webhook_config_for_repo(owner, repo, hook_id)
Get a webhook configuration for a repository

Returns the webhook configuration for a repository. To get more information about the webhook, including the `active` state and `events`, use \"[Get a repository webhook](/rest/webhooks/repos#get-a-repository-webhook).\"  OAuth app tokens and personal access tokens (classic) need the `read:repo_hook` or `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |

### Return type

[**models::WebhookConfig**](webhook-config.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_get_webhook_delivery

> models::HookDelivery repos_slash_get_webhook_delivery(owner, repo, hook_id, delivery_id)
Get a delivery for a repository webhook

Returns a delivery for a webhook configured in a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |
**delivery_id** | **i32** |  | [required] |

### Return type

[**models::HookDelivery**](hook-delivery.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_activities

> Vec<models::Activity> repos_slash_list_activities(owner, repo, direction, per_page, before, after, r#ref, actor, time_period, activity_type)
List repository activities

Lists a detailed history of changes to a repository, such as pushes, merges, force pushes, and branch changes, and associates these changes with commits and users.  For more information about viewing repository activity, see \"[Viewing activity and data for your repository](https://docs.github.com/repositories/viewing-activity-and-data-for-your-repository).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**before** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**after** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**r#ref** | Option<**String**> | The Git reference for the activities you want to list.  The `ref` for a branch can be formatted either as `refs/heads/BRANCH_NAME` or `BRANCH_NAME`, where `BRANCH_NAME` is the name of your branch. |  |
**actor** | Option<**String**> | The GitHub username to use to filter by the actor who performed the activity. |  |
**time_period** | Option<**String**> | The time period to filter by.  For example, `day` will filter for activity that occurred in the past 24 hours, and `week` will filter for activity that occurred in the past 7 days (168 hours). |  |
**activity_type** | Option<**String**> | The activity type to filter by.  For example, you can choose to filter by \"force_push\", to see all force pushes to the repository. |  |

### Return type

[**Vec<models::Activity>**](activity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_attestations

> models::OrgsListAttestations200Response repos_slash_list_attestations(owner, repo, subject_digest, per_page, before, after)
List attestations

List a collection of artifact attestations with a given subject digest that are associated with a repository.  The authenticated user making the request must have read access to the repository. In addition, when using a fine-grained access token the `attestations:read` permission is required.  **Please note:** in order to offer meaningful security benefits, an attestation's signature and timestamps **must** be cryptographically verified, and the identity of the attestation signer **must** be validated. Attestations can be verified using the [GitHub CLI `attestation verify` command](https://cli.github.com/manual/gh_attestation_verify). For more information, see [our guide on how to use artifact attestations to establish a build's provenance](https://docs.github.com/actions/security-guides/using-artifact-attestations-to-establish-provenance-for-builds).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**subject_digest** | **String** | The parameter should be set to the attestation's subject's SHA256 digest, in the form `sha256:HEX_DIGEST`. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**before** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**after** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |

### Return type

[**models::OrgsListAttestations200Response**](orgs_list_attestations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_autolinks

> Vec<models::Autolink> repos_slash_list_autolinks(owner, repo)
Get all autolinks of a repository

Gets all autolinks that are configured for a repository.  Information about autolinks are only available to repository administrators.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**Vec<models::Autolink>**](autolink.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_branches

> Vec<models::ShortBranch> repos_slash_list_branches(owner, repo, protected, per_page, page)
List branches



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**protected** | Option<**bool**> | Setting to `true` returns only branches protected by branch protections or rulesets. When set to `false`, only unprotected branches are returned. Omitting this parameter returns all branches. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::ShortBranch>**](short-branch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_branches_for_head_commit

> Vec<models::BranchShort> repos_slash_list_branches_for_head_commit(owner, repo, commit_sha)
List branches for HEAD commit

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Returns all branches where the given commit SHA is the HEAD, or latest commit for the branch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**commit_sha** | **String** | The SHA of the commit. | [required] |

### Return type

[**Vec<models::BranchShort>**](branch-short.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_collaborators

> Vec<models::Collaborator> repos_slash_list_collaborators(owner, repo, affiliation, permission, per_page, page)
List repository collaborators

For organization-owned repositories, the list of collaborators includes outside collaborators, organization members that are direct collaborators, organization members with access through team memberships, organization members with access through default organization permissions, and organization owners. Organization members with write, maintain, or admin privileges on the organization-owned repository can use this endpoint.  Team members will include the members of child teams.  The authenticated user must have push access to the repository to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `read:org` and `repo` scopes to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**affiliation** | Option<**String**> | Filter collaborators returned by their affiliation. `outside` means all outside collaborators of an organization-owned repository. `direct` means all collaborators with permissions to an organization-owned repository, regardless of organization membership status. `all` means all collaborators the authenticated user can see. |  |[default to all]
**permission** | Option<**String**> | Filter collaborators by the permissions they have on the repository. If not specified, all collaborators will be returned. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Collaborator>**](collaborator.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_comments_for_commit

> Vec<models::CommitComment> repos_slash_list_comments_for_commit(owner, repo, commit_sha, per_page, page)
List commit comments

Lists the comments for a specified commit.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type. - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`. - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`. - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**commit_sha** | **String** | The SHA of the commit. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::CommitComment>**](commit-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_commit_comments_for_repo

> Vec<models::CommitComment> repos_slash_list_commit_comments_for_repo(owner, repo, per_page, page)
List commit comments for a repository

Lists the commit comments for a specified repository. Comments are ordered by ascending ID.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type. - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`. - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`. - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::CommitComment>**](commit-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_commit_statuses_for_ref

> Vec<models::Status> repos_slash_list_commit_statuses_for_ref(owner, repo, r#ref, per_page, page)
List commit statuses for a reference

Users with pull access in a repository can view commit statuses for a given ref. The ref can be a SHA, a branch name, or a tag name. Statuses are returned in reverse chronological order. The first status in the list will be the latest one.  This resource is also available via a legacy route: `GET /repos/:owner/:repo/statuses/:ref`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**r#ref** | **String** | The commit reference. Can be a commit SHA, branch name (`heads/BRANCH_NAME`), or tag name (`tags/TAG_NAME`). For more information, see \"[Git References](https://git-scm.com/book/en/v2/Git-Internals-Git-References)\" in the Git documentation. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Status>**](status.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_commits

> Vec<models::Commit> repos_slash_list_commits(owner, repo, sha, path, author, committer, since, until, per_page, page)
List commits

**Signature verification object**  The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:  | Name | Type | Description | | ---- | ---- | ----------- | | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. | | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. | | `signature` | `string` | The signature that was extracted from the commit. | | `payload` | `string` | The value that was signed. |  These are the possible values for `reason` in the `verification` object:  | Value | Description | | ----- | ----------- | | `expired_key` | The key that made the signature is expired. | | `not_signing_key` | The \"signing\" flag is not among the usage flags in the GPG key that made the signature. | | `gpgverify_error` | There was an error communicating with the signature verification service. | | `gpgverify_unavailable` | The signature verification service is currently unavailable. | | `unsigned` | The object does not include a signature. | | `unknown_signature_type` | A non-PGP signature was found in the commit. | | `no_user` | No user was associated with the `committer` email address in the commit. | | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on their account. | | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. | | `unknown_key` | The key that made the signature has not been registered with any user's account. | | `malformed_signature` | There was an error parsing the signature. | | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. | | `valid` | None of the above errors applied, so the signature is considered to be verified. |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**sha** | Option<**String**> | SHA or branch to start listing commits from. Default: the repository’s default branch (usually `main`). |  |
**path** | Option<**String**> | Only commits containing this file path will be returned. |  |
**author** | Option<**String**> | GitHub username or email address to use to filter by commit author. |  |
**committer** | Option<**String**> | GitHub username or email address to use to filter by commit committer. |  |
**since** | Option<**String**> | Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. Due to limitations of Git, timestamps must be between 1970-01-01 and 2099-12-31 (inclusive) or unexpected results may be returned. |  |
**until** | Option<**String**> | Only commits before this date will be returned. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. Due to limitations of Git, timestamps must be between 1970-01-01 and 2099-12-31 (inclusive) or unexpected results may be returned. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Commit>**](commit.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_contributors

> Vec<models::Contributor> repos_slash_list_contributors(owner, repo, anon, per_page, page)
List repository contributors

Lists contributors to the specified repository and sorts them by the number of commits per contributor in descending order. This endpoint may return information that is a few hours old because the GitHub REST API caches contributor data to improve performance.  GitHub identifies contributors by author email address. This endpoint groups contribution counts by GitHub user, which includes all associated email addresses. To improve performance, only the first 500 author email addresses in the repository link to GitHub users. The rest will appear as anonymous contributors without associated GitHub user information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**anon** | Option<**String**> | Set to `1` or `true` to include anonymous contributors in results. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Contributor>**](contributor.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_custom_deployment_rule_integrations

> models::ReposListCustomDeploymentRuleIntegrations200Response repos_slash_list_custom_deployment_rule_integrations(environment_name, repo, owner, page, per_page)
List custom deployment rule integrations available for an environment

Gets all custom deployment protection rule integrations that are available for an environment.  The authenticated user must have admin or owner permissions to the repository to use this endpoint.  For more information about environments, see \"[Using environments for deployment](https://docs.github.com/actions/deployment/targeting-different-environments/using-environments-for-deployment).\"  For more information about the app that is providing this custom deployment rule, see \"[GET an app](https://docs.github.com/rest/apps/apps#get-an-app)\".  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**models::ReposListCustomDeploymentRuleIntegrations200Response**](repos_list_custom_deployment_rule_integrations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_deploy_keys

> Vec<models::DeployKey> repos_slash_list_deploy_keys(owner, repo, per_page, page)
List deploy keys



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::DeployKey>**](deploy-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_deployment_branch_policies

> models::ReposListDeploymentBranchPolicies200Response repos_slash_list_deployment_branch_policies(owner, repo, environment_name, per_page, page)
List deployment branch policies

Lists the deployment branch policies for an environment.  Anyone with read access to the repository can use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ReposListDeploymentBranchPolicies200Response**](repos_list_deployment_branch_policies_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_deployment_statuses

> Vec<models::DeploymentStatus> repos_slash_list_deployment_statuses(owner, repo, deployment_id, per_page, page)
List deployment statuses

Users with pull access can view deployment statuses for a deployment:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**deployment_id** | **i32** | deployment_id parameter | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::DeploymentStatus>**](deployment-status.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_deployments

> Vec<models::Deployment> repos_slash_list_deployments(owner, repo, sha, r#ref, task, environment, per_page, page)
List deployments

Simple filtering of deployments is available via query parameters:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**sha** | Option<**String**> | The SHA recorded at creation time. |  |[default to none]
**r#ref** | Option<**String**> | The name of the ref. This can be a branch, tag, or SHA. |  |[default to none]
**task** | Option<**String**> | The name of the task for the deployment (e.g., `deploy` or `deploy:migrations`). |  |[default to none]
**environment** | Option<**String**> | The name of the environment that was deployed to (e.g., `staging` or `production`). |  |[default to none]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Deployment>**](deployment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_for_authenticated_user

> Vec<models::Repository> repos_slash_list_for_authenticated_user(visibility, affiliation, r#type, sort, direction, per_page, page, since, before)
List repositories for the authenticated user

Lists repositories that the authenticated user has explicit permission (`:read`, `:write`, or `:admin`) to access.  The authenticated user has explicit permission to access repositories they own, repositories where they are a collaborator, and repositories that they can access through an organization membership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**visibility** | Option<**String**> | Limit results to repositories with the specified visibility. |  |[default to all]
**affiliation** | Option<**String**> | Comma-separated list of values. Can include:    * `owner`: Repositories that are owned by the authenticated user.    * `collaborator`: Repositories that the user has been added to as a collaborator.    * `organization_member`: Repositories that the user has access to through being a member of an organization. This includes every repository on every team that the user is on. |  |[default to owner,collaborator,organization_member]
**r#type** | Option<**String**> | Limit results to repositories of the specified type. Will cause a `422` error if used in the same request as **visibility** or **affiliation**. |  |[default to all]
**sort** | Option<**String**> | The property to sort the results by. |  |[default to full_name]
**direction** | Option<**String**> | The order to sort by. Default: `asc` when using `full_name`, otherwise `desc`. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**since** | Option<**String**> | Only show repositories updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. |  |
**before** | Option<**String**> | Only show repositories updated before the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. |  |

### Return type

[**Vec<models::Repository>**](repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_for_org

> Vec<models::MinimalRepository> repos_slash_list_for_org(org, r#type, sort, direction, per_page, page)
List organization repositories

Lists repositories for the specified organization.  > [!NOTE] > In order to see the `security_and_analysis` block for a repository you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see \"[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**r#type** | Option<**String**> | Specifies the types of repositories you want returned. |  |[default to all]
**sort** | Option<**String**> | The property to sort the results by. |  |[default to created]
**direction** | Option<**String**> | The order to sort by. Default: `asc` when using `full_name`, otherwise `desc`. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::MinimalRepository>**](minimal-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_for_user

> Vec<models::MinimalRepository> repos_slash_list_for_user(username, r#type, sort, direction, per_page, page)
List repositories for a user

Lists public repositories for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**r#type** | Option<**String**> | Limit results to repositories of the specified type. |  |[default to owner]
**sort** | Option<**String**> | The property to sort the results by. |  |[default to full_name]
**direction** | Option<**String**> | The order to sort by. Default: `asc` when using `full_name`, otherwise `desc`. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::MinimalRepository>**](minimal-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_forks

> Vec<models::MinimalRepository> repos_slash_list_forks(owner, repo, sort, per_page, page)
List forks



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**sort** | Option<**String**> | The sort order. `stargazers` will sort by star count. |  |[default to newest]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::MinimalRepository>**](minimal-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_invitations

> Vec<models::RepositoryInvitation> repos_slash_list_invitations(owner, repo, per_page, page)
List repository invitations

When authenticating as a user with admin rights to a repository, this endpoint will list all currently open repository invitations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::RepositoryInvitation>**](repository-invitation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_invitations_for_authenticated_user

> Vec<models::RepositoryInvitation> repos_slash_list_invitations_for_authenticated_user(per_page, page)
List repository invitations for the authenticated user

When authenticating as a user, this endpoint will list all currently open repository invitations for that user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::RepositoryInvitation>**](repository-invitation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_languages

> std::collections::HashMap<String, i32> repos_slash_list_languages(owner, repo)
List repository languages

Lists languages for the specified repository. The value shown for each language is the number of bytes of code written in that language.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

**std::collections::HashMap<String, i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_pages_builds

> Vec<models::PageBuild> repos_slash_list_pages_builds(owner, repo, per_page, page)
List GitHub Pages builds

Lists builts of a GitHub Pages site.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::PageBuild>**](page-build.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_public

> Vec<models::MinimalRepository> repos_slash_list_public(since)
List public repositories

Lists all public repositories in the order that they were created.  Note: - For GitHub Enterprise Server, this endpoint will only list repositories available to all users on the enterprise. - Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers) to get the URL for the next page of repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since** | Option<**i32**> | A repository ID. Only return repositories with an ID greater than this ID. |  |

### Return type

[**Vec<models::MinimalRepository>**](minimal-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_pull_requests_associated_with_commit

> Vec<models::PullRequestSimple> repos_slash_list_pull_requests_associated_with_commit(owner, repo, commit_sha, per_page, page)
List pull requests associated with a commit

Lists the merged pull request that introduced the commit to the repository. If the commit is not present in the default branch, will only return open pull requests associated with the commit.  To list the open or merged pull requests associated with a branch, you can set the `commit_sha` parameter to the branch name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**commit_sha** | **String** | The SHA of the commit. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::PullRequestSimple>**](pull-request-simple.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_release_assets

> Vec<models::ReleaseAsset> repos_slash_list_release_assets(owner, repo, release_id, per_page, page)
List release assets



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**release_id** | **i32** | The unique identifier of the release. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::ReleaseAsset>**](release-asset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_releases

> Vec<models::Release> repos_slash_list_releases(owner, repo, per_page, page)
List releases

This returns a list of releases, which does not include regular Git tags that have not been associated with a release. To get a list of Git tags, use the [Repository Tags API](https://docs.github.com/rest/repos/repos#list-repository-tags).  Information about published releases are available to everyone. Only users with push access will receive listings for draft releases.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Release>**](release.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_tag_protection

> Vec<models::TagProtection> repos_slash_list_tag_protection(owner, repo)
Closing down - List tag protection states for a repository

> [!WARNING] > **Closing down notice:** This operation is closing down and will be removed after August 30, 2024. Use the \"[Repository Rulesets](https://docs.github.com/rest/repos/rules#get-all-repository-rulesets)\" endpoint instead.  This returns the tag protection states of a repository.  This information is only available to repository administrators.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**Vec<models::TagProtection>**](tag-protection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_tags

> Vec<models::Tag> repos_slash_list_tags(owner, repo, per_page, page)
List repository tags



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Tag>**](tag.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_teams

> Vec<models::Team> repos_slash_list_teams(owner, repo, per_page, page)
List repository teams

Lists the teams that have access to the specified repository and that are also visible to the authenticated user.  For a public repository, a team is listed only if that team added the public repository explicitly.  OAuth app tokens and personal access tokens (classic) need the `public_repo` or `repo` scope to use this endpoint with a public repository, and `repo` scope to use this endpoint with a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Team>**](team.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_webhook_deliveries

> Vec<models::HookDeliveryItem> repos_slash_list_webhook_deliveries(owner, repo, hook_id, per_page, cursor)
List deliveries for a repository webhook

Returns a list of webhook deliveries for a webhook configured in a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**cursor** | Option<**String**> | Used for pagination: the starting delivery from which the page of deliveries is fetched. Refer to the `link` header for the next and previous page cursors. |  |

### Return type

[**Vec<models::HookDeliveryItem>**](hook-delivery-item.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_list_webhooks

> Vec<models::Hook> repos_slash_list_webhooks(owner, repo, per_page, page)
List repository webhooks

Lists webhooks for a repository. `last response` may return null if there have not been any deliveries within 30 days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Hook>**](hook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_merge

> models::Commit repos_slash_merge(owner, repo, repos_merge_request)
Merge a branch



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_merge_request** | [**ReposMergeRequest**](ReposMergeRequest.md) |  | [required] |

### Return type

[**models::Commit**](commit.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_merge_upstream

> models::MergedUpstream repos_slash_merge_upstream(owner, repo, repos_merge_upstream_request)
Sync a fork branch with the upstream repository

Sync a branch of a forked repository to keep it up-to-date with the upstream repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_merge_upstream_request** | [**ReposMergeUpstreamRequest**](ReposMergeUpstreamRequest.md) |  | [required] |

### Return type

[**models::MergedUpstream**](merged-upstream.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_ping_webhook

> repos_slash_ping_webhook(owner, repo, hook_id)
Ping a repository webhook

This will trigger a [ping event](https://docs.github.com/webhooks/#ping-event) to be sent to the hook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_redeliver_webhook_delivery

> serde_json::Value repos_slash_redeliver_webhook_delivery(owner, repo, hook_id, delivery_id)
Redeliver a delivery for a repository webhook

Redeliver a webhook delivery for a webhook configured in a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |
**delivery_id** | **i32** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_remove_app_access_restrictions

> Vec<models::Integration> repos_slash_remove_app_access_restrictions(owner, repo, branch, repos_set_app_access_restrictions_request)
Remove app access restrictions

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Removes the ability of an app to push to this branch. Only GitHub Apps that are installed on the repository and that have been granted write access to the repository contents can be added as authorized actors on a protected branch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |
**repos_set_app_access_restrictions_request** | [**ReposSetAppAccessRestrictionsRequest**](ReposSetAppAccessRestrictionsRequest.md) |  | [required] |

### Return type

[**Vec<models::Integration>**](integration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_remove_collaborator

> repos_slash_remove_collaborator(owner, repo, username)
Remove a repository collaborator

Removes a collaborator from a repository.  To use this endpoint, the authenticated user must either be an administrator of the repository or target themselves for removal.  This endpoint also: - Cancels any outstanding invitations - Unasigns the user from any issues - Removes access to organization projects if the user is not an organization member and is not a collaborator on any other organization repositories. - Unstars the repository - Updates access permissions to packages  Removing a user as a collaborator has the following effects on forks:  - If the user had access to a fork through their membership to this repository, the user will also be removed from the fork.  - If the user had their own fork of the repository, the fork will be deleted.  - If the user still has read access to the repository, open pull requests by this user from a fork will be denied.  > [!NOTE] > A user can still have access to the repository through organization permissions like base repository permissions.  Although the API responds immediately, the additional permission updates might take some extra time to complete in the background.  For more information on fork permissions, see \"[About permissions and visibility of forks](https://docs.github.com/pull-requests/collaborating-with-pull-requests/working-with-forks/about-permissions-and-visibility-of-forks)\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_remove_status_check_contexts

> Vec<String> repos_slash_remove_status_check_contexts(owner, repo, branch, repos_set_status_check_contexts_request)
Remove status check contexts

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |
**repos_set_status_check_contexts_request** | Option<[**ReposSetStatusCheckContextsRequest**](ReposSetStatusCheckContextsRequest.md)> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_remove_status_check_protection

> repos_slash_remove_status_check_protection(owner, repo, branch)
Remove status check protection

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_remove_team_access_restrictions

> Vec<models::Team> repos_slash_remove_team_access_restrictions(owner, repo, branch, repos_add_team_access_restrictions_request)
Remove team access restrictions

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Removes the ability of a team to push to this branch. You can also remove push access for child teams.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |
**repos_add_team_access_restrictions_request** | Option<[**ReposAddTeamAccessRestrictionsRequest**](ReposAddTeamAccessRestrictionsRequest.md)> |  |  |

### Return type

[**Vec<models::Team>**](team.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_remove_user_access_restrictions

> Vec<models::SimpleUser> repos_slash_remove_user_access_restrictions(owner, repo, branch, repos_set_user_access_restrictions_request)
Remove user access restrictions

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Removes the ability of a user to push to this branch.  | Type    | Description                                                                                                                                   | | ------- | --------------------------------------------------------------------------------------------------------------------------------------------- | | `array` | Usernames of the people who should no longer have push access. **Note**: The list of users, apps, and teams in total is limited to 100 items. |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |
**repos_set_user_access_restrictions_request** | [**ReposSetUserAccessRestrictionsRequest**](ReposSetUserAccessRestrictionsRequest.md) |  | [required] |

### Return type

[**Vec<models::SimpleUser>**](simple-user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_rename_branch

> models::BranchWithProtection repos_slash_rename_branch(owner, repo, branch, repos_rename_branch_request)
Rename a branch

Renames a branch in a repository.  > [!NOTE] > Although the API responds immediately, the branch rename process might take some extra time to complete in the background. You won't be able to push to the old branch name while the rename process is in progress. For more information, see \"[Renaming a branch](https://docs.github.com/github/administering-a-repository/renaming-a-branch)\".  The authenticated user must have push access to the branch. If the branch is the default branch, the authenticated user must also have admin or owner permissions.  In order to rename the default branch, fine-grained access tokens also need the `administration:write` repository permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |
**repos_rename_branch_request** | [**ReposRenameBranchRequest**](ReposRenameBranchRequest.md) |  | [required] |

### Return type

[**models::BranchWithProtection**](branch-with-protection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_replace_all_topics

> models::Topic repos_slash_replace_all_topics(owner, repo, repos_replace_all_topics_request)
Replace all repository topics



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_replace_all_topics_request** | [**ReposReplaceAllTopicsRequest**](ReposReplaceAllTopicsRequest.md) |  | [required] |

### Return type

[**models::Topic**](topic.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_request_pages_build

> models::PageBuildStatus repos_slash_request_pages_build(owner, repo)
Request a GitHub Pages build

You can request that your site be built from the latest revision on the default branch. This has the same effect as pushing a commit to your default branch, but does not require an additional commit. Manually triggering page builds can be helpful when diagnosing build warnings and failures.  Build requests are limited to one concurrent build per repository and one concurrent build per requester. If you request a build while another is still in progress, the second request will be queued until the first completes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::PageBuildStatus**](page-build-status.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_set_admin_branch_protection

> models::ProtectedBranchAdminEnforced repos_slash_set_admin_branch_protection(owner, repo, branch)
Set admin branch protection

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Adding admin enforcement requires admin or owner permissions to the repository and branch protection to be enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |

### Return type

[**models::ProtectedBranchAdminEnforced**](protected-branch-admin-enforced.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_set_app_access_restrictions

> Vec<models::Integration> repos_slash_set_app_access_restrictions(owner, repo, branch, repos_set_app_access_restrictions_request)
Set app access restrictions

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Replaces the list of apps that have push access to this branch. This removes all apps that previously had push access and grants push access to the new list of apps. Only GitHub Apps that are installed on the repository and that have been granted write access to the repository contents can be added as authorized actors on a protected branch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |
**repos_set_app_access_restrictions_request** | [**ReposSetAppAccessRestrictionsRequest**](ReposSetAppAccessRestrictionsRequest.md) |  | [required] |

### Return type

[**Vec<models::Integration>**](integration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_set_status_check_contexts

> Vec<String> repos_slash_set_status_check_contexts(owner, repo, branch, repos_set_status_check_contexts_request)
Set status check contexts

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |
**repos_set_status_check_contexts_request** | Option<[**ReposSetStatusCheckContextsRequest**](ReposSetStatusCheckContextsRequest.md)> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_set_team_access_restrictions

> Vec<models::Team> repos_slash_set_team_access_restrictions(owner, repo, branch, repos_set_team_access_restrictions_request)
Set team access restrictions

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Replaces the list of teams that have push access to this branch. This removes all teams that previously had push access and grants push access to the new list of teams. Team restrictions include child teams.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |
**repos_set_team_access_restrictions_request** | Option<[**ReposSetTeamAccessRestrictionsRequest**](ReposSetTeamAccessRestrictionsRequest.md)> |  |  |

### Return type

[**Vec<models::Team>**](team.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_set_user_access_restrictions

> Vec<models::SimpleUser> repos_slash_set_user_access_restrictions(owner, repo, branch, repos_set_user_access_restrictions_request)
Set user access restrictions

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Replaces the list of people that have push access to this branch. This removes all people that previously had push access and grants push access to the new list of people.  | Type    | Description                                                                                                                   | | ------- | ----------------------------------------------------------------------------------------------------------------------------- | | `array` | Usernames for people who can have push access. **Note**: The list of users, apps, and teams in total is limited to 100 items. |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |
**repos_set_user_access_restrictions_request** | [**ReposSetUserAccessRestrictionsRequest**](ReposSetUserAccessRestrictionsRequest.md) |  | [required] |

### Return type

[**Vec<models::SimpleUser>**](simple-user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_test_push_webhook

> repos_slash_test_push_webhook(owner, repo, hook_id)
Test the push repository webhook

This will trigger the hook with the latest push to the current repository if the hook is subscribed to `push` events. If the hook is not subscribed to `push` events, the server will respond with 204 but no test POST will be generated.  > [!NOTE] > Previously `/repos/:owner/:repo/hooks/:hook_id/test`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_transfer

> models::MinimalRepository repos_slash_transfer(owner, repo, repos_transfer_request)
Transfer a repository

A transfer request will need to be accepted by the new owner when transferring a personal repository to another user. The response will contain the original `owner`, and the transfer will continue asynchronously. For more details on the requirements to transfer personal and organization-owned repositories, see [about repository transfers](https://docs.github.com/articles/about-repository-transfers/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_transfer_request** | [**ReposTransferRequest**](ReposTransferRequest.md) |  | [required] |

### Return type

[**models::MinimalRepository**](minimal-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_update

> models::FullRepository repos_slash_update(owner, repo, repos_update_request)
Update a repository

**Note**: To edit a repository's topics, use the [Replace all repository topics](https://docs.github.com/rest/repos/repos#replace-all-repository-topics) endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_update_request** | Option<[**ReposUpdateRequest**](ReposUpdateRequest.md)> |  |  |

### Return type

[**models::FullRepository**](full-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_update_branch_protection

> models::ProtectedBranch repos_slash_update_branch_protection(owner, repo, branch, repos_update_branch_protection_request)
Update branch protection

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Protecting a branch requires admin or owner permissions to the repository.  > [!NOTE] > Passing new arrays of `users` and `teams` replaces their previous values.  > [!NOTE] > The list of users, apps, and teams in total is limited to 100 items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |
**repos_update_branch_protection_request** | [**ReposUpdateBranchProtectionRequest**](ReposUpdateBranchProtectionRequest.md) |  | [required] |

### Return type

[**models::ProtectedBranch**](protected-branch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_update_commit_comment

> models::CommitComment repos_slash_update_commit_comment(owner, repo, comment_id, repos_update_commit_comment_request)
Update a commit comment

Updates the contents of a specified commit comment.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type. - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`. - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`. - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**comment_id** | **i64** | The unique identifier of the comment. | [required] |
**repos_update_commit_comment_request** | [**ReposUpdateCommitCommentRequest**](ReposUpdateCommitCommentRequest.md) |  | [required] |

### Return type

[**models::CommitComment**](commit-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_update_deployment_branch_policy

> models::DeploymentBranchPolicy repos_slash_update_deployment_branch_policy(owner, repo, environment_name, branch_policy_id, deployment_branch_policy_name_pattern)
Update a deployment branch policy

Updates a deployment branch or tag policy for an environment.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**environment_name** | **String** | The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`. | [required] |
**branch_policy_id** | **i32** | The unique identifier of the branch policy. | [required] |
**deployment_branch_policy_name_pattern** | [**DeploymentBranchPolicyNamePattern**](DeploymentBranchPolicyNamePattern.md) |  | [required] |

### Return type

[**models::DeploymentBranchPolicy**](deployment-branch-policy.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_update_information_about_pages_site

> repos_slash_update_information_about_pages_site(owner, repo, repos_update_information_about_pages_site_request)
Update information about a GitHub Pages site

Updates information for a GitHub Pages site. For more information, see \"[About GitHub Pages](/github/working-with-github-pages/about-github-pages).  The authenticated user must be a repository administrator, maintainer, or have the 'manage GitHub Pages settings' permission.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repos_update_information_about_pages_site_request** | Option<[**ReposUpdateInformationAboutPagesSiteRequest**](ReposUpdateInformationAboutPagesSiteRequest.md)> |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_update_invitation

> models::RepositoryInvitation repos_slash_update_invitation(owner, repo, invitation_id, repos_update_invitation_request)
Update a repository invitation



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**invitation_id** | **i32** | The unique identifier of the invitation. | [required] |
**repos_update_invitation_request** | Option<[**ReposUpdateInvitationRequest**](ReposUpdateInvitationRequest.md)> |  |  |

### Return type

[**models::RepositoryInvitation**](repository-invitation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_update_org_ruleset

> models::RepositoryRuleset repos_slash_update_org_ruleset(org, ruleset_id, repos_update_org_ruleset_request)
Update an organization repository ruleset

Update a ruleset for an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**ruleset_id** | **i32** | The ID of the ruleset. | [required] |
**repos_update_org_ruleset_request** | Option<[**ReposUpdateOrgRulesetRequest**](ReposUpdateOrgRulesetRequest.md)> | Request body |  |

### Return type

[**models::RepositoryRuleset**](repository-ruleset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_update_pull_request_review_protection

> models::ProtectedBranchPullRequestReview repos_slash_update_pull_request_review_protection(owner, repo, branch, repos_update_pull_request_review_protection_request)
Update pull request review protection

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Updating pull request review enforcement requires admin or owner permissions to the repository and branch protection to be enabled.  > [!NOTE] > Passing new arrays of `users` and `teams` replaces their previous values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |
**repos_update_pull_request_review_protection_request** | Option<[**ReposUpdatePullRequestReviewProtectionRequest**](ReposUpdatePullRequestReviewProtectionRequest.md)> |  |  |

### Return type

[**models::ProtectedBranchPullRequestReview**](protected-branch-pull-request-review.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_update_release

> models::Release repos_slash_update_release(owner, repo, release_id, repos_update_release_request)
Update a release

Users with push access to the repository can edit a release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**release_id** | **i32** | The unique identifier of the release. | [required] |
**repos_update_release_request** | Option<[**ReposUpdateReleaseRequest**](ReposUpdateReleaseRequest.md)> |  |  |

### Return type

[**models::Release**](release.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_update_release_asset

> models::ReleaseAsset repos_slash_update_release_asset(owner, repo, asset_id, repos_update_release_asset_request)
Update a release asset

Users with push access to the repository can edit a release asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**asset_id** | **i32** | The unique identifier of the asset. | [required] |
**repos_update_release_asset_request** | Option<[**ReposUpdateReleaseAssetRequest**](ReposUpdateReleaseAssetRequest.md)> |  |  |

### Return type

[**models::ReleaseAsset**](release-asset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_update_repo_ruleset

> models::RepositoryRuleset repos_slash_update_repo_ruleset(owner, repo, ruleset_id, repos_update_repo_ruleset_request)
Update a repository ruleset

Update a ruleset for a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**ruleset_id** | **i32** | The ID of the ruleset. | [required] |
**repos_update_repo_ruleset_request** | Option<[**ReposUpdateRepoRulesetRequest**](ReposUpdateRepoRulesetRequest.md)> | Request body |  |

### Return type

[**models::RepositoryRuleset**](repository-ruleset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_update_status_check_protection

> models::StatusCheckPolicy repos_slash_update_status_check_protection(owner, repo, branch, repos_update_status_check_protection_request)
Update status check protection

Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  Updating required status checks requires admin or owner permissions to the repository and branch protection to be enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**branch** | **String** | The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql). | [required] |
**repos_update_status_check_protection_request** | Option<[**ReposUpdateStatusCheckProtectionRequest**](ReposUpdateStatusCheckProtectionRequest.md)> |  |  |

### Return type

[**models::StatusCheckPolicy**](status-check-policy.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_update_webhook

> models::Hook repos_slash_update_webhook(owner, repo, hook_id, repos_update_webhook_request)
Update a repository webhook

Updates a webhook configured in a repository. If you previously had a `secret` set, you must provide the same `secret` or set a new `secret` or the secret will be removed. If you are only updating individual webhook `config` properties, use \"[Update a webhook configuration for a repository](/rest/webhooks/repo-config#update-a-webhook-configuration-for-a-repository).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |
**repos_update_webhook_request** | [**ReposUpdateWebhookRequest**](ReposUpdateWebhookRequest.md) |  | [required] |

### Return type

[**models::Hook**](hook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_update_webhook_config_for_repo

> models::WebhookConfig repos_slash_update_webhook_config_for_repo(owner, repo, hook_id, repos_update_webhook_config_for_repo_request)
Update a webhook configuration for a repository

Updates the webhook configuration for a repository. To update more information about the webhook, including the `active` state and `events`, use \"[Update a repository webhook](/rest/webhooks/repos#update-a-repository-webhook).\"  OAuth app tokens and personal access tokens (classic) need the `write:repo_hook` or `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |
**repos_update_webhook_config_for_repo_request** | Option<[**ReposUpdateWebhookConfigForRepoRequest**](ReposUpdateWebhookConfigForRepoRequest.md)> |  |  |

### Return type

[**models::WebhookConfig**](webhook-config.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repos_slash_upload_release_asset

> models::ReleaseAsset repos_slash_upload_release_asset(owner, repo, release_id, name, label, body)
Upload a release asset

This endpoint makes use of a [Hypermedia relation](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#hypermedia) to determine which URL to access. The endpoint you call to upload release assets is specific to your release. Use the `upload_url` returned in the response of the [Create a release endpoint](https://docs.github.com/rest/releases/releases#create-a-release) to upload a release asset.  You need to use an HTTP client which supports [SNI](http://en.wikipedia.org/wiki/Server_Name_Indication) to make calls to this endpoint.  Most libraries will set the required `Content-Length` header automatically. Use the required `Content-Type` header to provide the media type of the asset. For a list of media types, see [Media Types](https://www.iana.org/assignments/media-types/media-types.xhtml). For example:   `application/zip`  GitHub expects the asset data in its raw binary form, rather than JSON. You will send the raw binary content of the asset as the request body. Everything else about the endpoint is the same as the rest of the API. For example, you'll still need to pass your authentication to be able to upload an asset.  When an upstream failure occurs, you will receive a `502 Bad Gateway` status. This may leave an empty asset with a state of `starter`. It can be safely deleted.  **Notes:** *   GitHub renames asset filenames that have special characters, non-alphanumeric characters, and leading or trailing periods. The \"[List release assets](https://docs.github.com/rest/releases/assets#list-release-assets)\" endpoint lists the renamed filenames. For more information and help, contact [GitHub Support](https://support.github.com/contact?tags=dotcom-rest-api). *   To find the `release_id` query the [`GET /repos/{owner}/{repo}/releases/latest` endpoint](https://docs.github.com/rest/releases/releases#get-the-latest-release).  *   If you upload an asset with the same filename as another uploaded asset, you'll receive an error and must delete the old file before you can re-upload the new asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**release_id** | **i32** | The unique identifier of the release. | [required] |
**name** | **String** |  | [required] |
**label** | Option<**String**> |  |  |
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**models::ReleaseAsset**](release-asset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

