#![deny(clippy::all)]

mod icons;
mod rules;

#[cfg(not(target_os = "windows"))]
mod non_windows;
#[cfg(target_os = "windows")]
mod windows;

pub use icons::{file_to_data_url, get_app_name};
#[cfg(not(target_os = "windows"))]
pub use non_windows::{current_user_sid, is_running_as_admin, run_elevated, setup_firewall_rules};
pub use rules::{
    RuleConvertOptions, RuleOutputInfo, RuleSkippedItem, RuleStringResult, rule_file_to_string,
};
#[cfg(target_os = "windows")]
pub use windows::{current_user_sid, is_running_as_admin, run_elevated, setup_firewall_rules};

#[derive(Debug, Clone)]
pub struct FirewallRule {
    pub name: String,
    pub application_path: String,
}
