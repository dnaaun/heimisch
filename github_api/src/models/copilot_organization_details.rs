/*
 * GitHub v3 REST API
 *
 * GitHub's v3 REST API.
 *
 * The version of the OpenAPI document: 1.1.4
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CopilotOrganizationDetails : Information about the seat breakdown and policies set for an organization with a Copilot Business or Copilot Enterprise subscription.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CopilotOrganizationDetails {
    #[serde(rename = "seat_breakdown")]
    pub seat_breakdown: Box<models::CopilotSeatBreakdown>,
    /// The organization policy for allowing or disallowing Copilot to make suggestions that match public code.
    #[serde(rename = "public_code_suggestions")]
    pub public_code_suggestions: PublicCodeSuggestions,
    /// The organization policy for allowing or disallowing organization members to use Copilot Chat within their editor.
    #[serde(rename = "ide_chat", skip_serializing_if = "Option::is_none")]
    pub ide_chat: Option<IdeChat>,
    /// The organization policy for allowing or disallowing organization members to use Copilot features within github.com.
    #[serde(rename = "platform_chat", skip_serializing_if = "Option::is_none")]
    pub platform_chat: Option<PlatformChat>,
    /// The organization policy for allowing or disallowing organization members to use Copilot within their CLI.
    #[serde(rename = "cli", skip_serializing_if = "Option::is_none")]
    pub cli: Option<Cli>,
    /// The mode of assigning new seats.
    #[serde(rename = "seat_management_setting")]
    pub seat_management_setting: SeatManagementSetting,
    /// The Copilot plan of the organization, or the parent enterprise, when applicable.
    #[serde(rename = "plan_type", skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<PlanType>,
}

impl CopilotOrganizationDetails {
    /// Information about the seat breakdown and policies set for an organization with a Copilot Business or Copilot Enterprise subscription.
    pub fn new(
        seat_breakdown: models::CopilotSeatBreakdown,
        public_code_suggestions: PublicCodeSuggestions,
        seat_management_setting: SeatManagementSetting,
    ) -> CopilotOrganizationDetails {
        CopilotOrganizationDetails {
            seat_breakdown: Box::new(seat_breakdown),
            public_code_suggestions,
            ide_chat: None,
            platform_chat: None,
            cli: None,
            seat_management_setting,
            plan_type: None,
        }
    }
}
/// The organization policy for allowing or disallowing Copilot to make suggestions that match public code.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PublicCodeSuggestions {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "unconfigured")]
    Unconfigured,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for PublicCodeSuggestions {
    fn default() -> PublicCodeSuggestions {
        Self::Allow
    }
}
/// The organization policy for allowing or disallowing organization members to use Copilot Chat within their editor.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IdeChat {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "unconfigured")]
    Unconfigured,
}

impl Default for IdeChat {
    fn default() -> IdeChat {
        Self::Enabled
    }
}
/// The organization policy for allowing or disallowing organization members to use Copilot features within github.com.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlatformChat {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "unconfigured")]
    Unconfigured,
}

impl Default for PlatformChat {
    fn default() -> PlatformChat {
        Self::Enabled
    }
}
/// The organization policy for allowing or disallowing organization members to use Copilot within their CLI.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Cli {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "unconfigured")]
    Unconfigured,
}

impl Default for Cli {
    fn default() -> Cli {
        Self::Enabled
    }
}
/// The mode of assigning new seats.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SeatManagementSetting {
    #[serde(rename = "assign_all")]
    AssignAll,
    #[serde(rename = "assign_selected")]
    AssignSelected,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "unconfigured")]
    Unconfigured,
}

impl Default for SeatManagementSetting {
    fn default() -> SeatManagementSetting {
        Self::AssignAll
    }
}
/// The Copilot plan of the organization, or the parent enterprise, when applicable.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlanType {
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "enterprise")]
    Enterprise,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for PlanType {
    fn default() -> PlanType {
        Self::Business
    }
}
