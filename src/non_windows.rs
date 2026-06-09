use crate::FirewallRule;

pub fn current_user_sid() -> anyhow::Result<String> {
    Ok(String::new())
}

pub fn is_running_as_admin() -> anyhow::Result<bool> {
    Ok(false)
}

pub fn run_elevated(_command: &str, _args: &[String]) -> anyhow::Result<u32> {
    Ok(0)
}

pub fn setup_firewall_rules(_rules: Vec<FirewallRule>) -> anyhow::Result<()> {
    Ok(())
}
