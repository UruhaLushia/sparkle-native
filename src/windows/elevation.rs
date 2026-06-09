use std::mem::size_of;

use anyhow::{Result, anyhow};
use windows::Win32::Foundation::ERROR_CANCELLED;
use windows::Win32::System::Threading::{GetExitCodeProcess, INFINITE, WaitForSingleObject};
use windows::Win32::UI::Shell::{
    SEE_MASK_NOASYNC, SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW, ShellExecuteExW,
};
use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;
use windows::core::PCWSTR;

use super::handle::Handle;

const HRESULT_FROM_WIN32_ERROR_CANCELLED: i32 = 0x800704C7u32 as i32;

fn quote_windows_arg(arg: &str) -> String {
    if !arg.is_empty()
        && !arg
            .chars()
            .any(|ch| matches!(ch, ' ' | '\t' | '\n' | '\r' | '"'))
    {
        return arg.to_string();
    }

    let mut quoted = String::from("\"");
    let mut backslashes = 0usize;
    for ch in arg.chars() {
        match ch {
            '\\' => backslashes += 1,
            '"' => {
                quoted.push_str(&"\\".repeat(backslashes * 2 + 1));
                quoted.push('"');
                backslashes = 0;
            }
            _ => {
                quoted.push_str(&"\\".repeat(backslashes));
                backslashes = 0;
                quoted.push(ch);
            }
        }
    }
    quoted.push_str(&"\\".repeat(backslashes * 2));
    quoted.push('"');
    quoted
}

fn to_wide_null(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

fn shell_execute_and_exit_code(info: &mut SHELLEXECUTEINFOW) -> Result<u32> {
    unsafe {
        ShellExecuteExW(info).map_err(|error| {
            if error.code().0 == HRESULT_FROM_WIN32_ERROR_CANCELLED
                || error.code().0 == ERROR_CANCELLED.to_hresult().0
            {
                anyhow!("User canceled")
            } else {
                anyhow!("ShellExecuteExW failed: {error}")
            }
        })?;
        if info.hProcess.is_invalid() {
            return Ok(0);
        }

        let process = Handle(info.hProcess);
        let mut exit_code = 0u32;
        WaitForSingleObject(process.0, INFINITE);
        GetExitCodeProcess(process.0, &mut exit_code)
            .map_err(|error| anyhow!("GetExitCodeProcess failed: {error}"))?;
        Ok(exit_code)
    }
}

pub fn run_elevated(command: &str, args: &[String]) -> Result<u32> {
    let verb = to_wide_null("runas");
    let file = to_wide_null(command);
    let parameters = args
        .iter()
        .map(|arg| quote_windows_arg(arg))
        .collect::<Vec<_>>()
        .join(" ");
    let parameters = to_wide_null(&parameters);

    let mut info = SHELLEXECUTEINFOW {
        cbSize: size_of::<SHELLEXECUTEINFOW>() as u32,
        fMask: SEE_MASK_NOCLOSEPROCESS | SEE_MASK_NOASYNC,
        lpVerb: PCWSTR(verb.as_ptr()),
        lpFile: PCWSTR(file.as_ptr()),
        lpParameters: PCWSTR(parameters.as_ptr()),
        nShow: SW_HIDE.0,
        ..Default::default()
    };

    shell_execute_and_exit_code(&mut info)
}
