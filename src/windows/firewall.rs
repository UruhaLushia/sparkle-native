use anyhow::{Result, anyhow};
use windows::Win32::Foundation::VARIANT_TRUE;
use windows::Win32::NetworkManagement::WindowsFirewall::{
    INetFwPolicy2, INetFwRule, INetFwRules, NET_FW_ACTION_ALLOW, NET_FW_PROFILE2_ALL,
    NET_FW_RULE_DIR_IN, NetFwPolicy2, NetFwRule,
};
use windows::Win32::System::Com::{
    CLSCTX_ALL, COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE, CoCreateInstance, CoInitializeEx,
    CoUninitialize,
};
use windows::core::BSTR;

use crate::FirewallRule;

struct ComApartment;

impl ComApartment {
    fn init() -> Result<Self> {
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE)
                .ok()
                .map_err(|error| anyhow!("CoInitializeEx failed: {error}"))?;
        }
        Ok(Self)
    }
}

impl Drop for ComApartment {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}

fn firewall_rules() -> Result<INetFwRules> {
    unsafe {
        let policy: INetFwPolicy2 = CoCreateInstance(&NetFwPolicy2, None, CLSCTX_ALL)
            .map_err(|error| anyhow!("CoCreateInstance(NetFwPolicy2) failed: {error}"))?;
        policy
            .Rules()
            .map_err(|error| anyhow!("INetFwPolicy2.Rules failed: {error}"))
    }
}

fn remove_firewall_rule(fw_rules: &INetFwRules, name: &str) {
    unsafe {
        let _ = fw_rules.Remove(&BSTR::from(name));
    }
}

fn add_firewall_rule(fw_rules: &INetFwRules, rule: &FirewallRule) -> Result<()> {
    unsafe {
        let fw_rule: INetFwRule = CoCreateInstance(&NetFwRule, None, CLSCTX_ALL)
            .map_err(|error| anyhow!("CoCreateInstance(NetFwRule) failed: {error}"))?;
        fw_rule
            .SetName(&BSTR::from(rule.name.as_str()))
            .map_err(|error| anyhow!("SetName({}) failed: {error}", rule.name))?;
        fw_rule
            .SetApplicationName(&BSTR::from(rule.application_path.as_str()))
            .map_err(|error| anyhow!("SetApplicationName({}) failed: {error}", rule.name))?;
        fw_rule
            .SetDirection(NET_FW_RULE_DIR_IN)
            .map_err(|error| anyhow!("SetDirection({}) failed: {error}", rule.name))?;
        fw_rule
            .SetAction(NET_FW_ACTION_ALLOW)
            .map_err(|error| anyhow!("SetAction({}) failed: {error}", rule.name))?;
        fw_rule
            .SetProfiles(NET_FW_PROFILE2_ALL.0)
            .map_err(|error| anyhow!("SetProfiles({}) failed: {error}", rule.name))?;
        fw_rule
            .SetEnabled(VARIANT_TRUE)
            .map_err(|error| anyhow!("SetEnabled({}) failed: {error}", rule.name))?;
        fw_rules
            .Add(&fw_rule)
            .map_err(|error| anyhow!("Add({}) failed: {error}", rule.name))?;
    }
    Ok(())
}

pub fn setup_firewall_rules(rules: Vec<FirewallRule>) -> Result<()> {
    let _com = ComApartment::init()?;
    let fw_rules = firewall_rules()?;

    for rule in &rules {
        remove_firewall_rule(&fw_rules, &rule.name);
    }

    for rule in rules {
        add_firewall_rule(&fw_rules, &rule)?;
    }

    Ok(())
}
