#![allow(dead_code)]
use std::env;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SystemContext {
    pub host_os: String,
    pub host_arch: String,
    pub target_os: Option<String>,
}

impl SystemContext {
    pub fn new() -> Self {
        Self {
            host_os: env::consts::OS.to_string(),
            host_arch: env::consts::ARCH.to_string(),
            target_os: None,
        }
    }

    pub fn detect_target_os(&mut self, input: &str) {
        self.target_os = detect_target_os_from_input(input);
    }

    pub fn get_effective_os(&self) -> &str {
        self.target_os.as_deref().unwrap_or(&self.host_os)
    }

    pub fn get_os_display_name(&self) -> String {
        match self.get_effective_os() {
            "macos" => "macOS".to_string(),
            "linux" => "Linux".to_string(),
            "windows" => "Windows".to_string(),
            other => other.to_string(),
        }
    }

    pub fn get_powershell_command(&self) -> &str {
        match self.get_effective_os() {
            "windows" => "powershell",
            _ => "pwsh", // macOS and Linux use PowerShell Core
        }
    }

    pub fn get_package_manager(&self) -> &str {
        match self.get_effective_os() {
            "macos" => "brew",
            "windows" => "winget", // or choco
            _ => "apt",            // Default to apt for Linux, could be more sophisticated
        }
    }
}

fn detect_target_os_from_input(input: &str) -> Option<String> {
    let input_lower = input.to_lowercase();

    // Windows indicators
    if input_lower.contains("windows")
        || input_lower.contains("powershell.exe")
        || input_lower.contains("c:\\")
        || input_lower.contains("registry")
        || input_lower.contains("services.msc")
        || input_lower.contains("event log")
        || input_lower.contains("wuauserv")
        || input_lower.contains("get-service")
        || input_lower.contains("get-eventlog")
    {
        return Some("windows".to_string());
    }

    // Linux indicators
    if input_lower.contains("linux")
        || input_lower.contains("/etc/")
        || input_lower.contains("systemctl")
        || input_lower.contains("apt ")
        || input_lower.contains("yum ")
        || input_lower.contains("dnf ")
        || input_lower.contains("pacman ")
        || input_lower.contains("ubuntu")
        || input_lower.contains("centos")
        || input_lower.contains("rhel")
        || input_lower.contains("debian")
        || input_lower.contains("linux server")
    {
        return Some("linux".to_string());
    }

    // macOS indicators
    if input_lower.contains("macos")
        || input_lower.contains("mac os")
        || input_lower.contains("brew ")
        || input_lower.contains("launchctl")
        || input_lower.contains("/usr/local/")
        || input_lower.contains("homebrew")
    {
        return Some("macos".to_string());
    }

    // Container/Remote context - could be any OS, but often Linux
    if input_lower.contains("container")
        || input_lower.contains("docker")
        || input_lower.contains("kubectl exec")
        || input_lower.contains("inside")
        || input_lower.contains("remote server")
    {
        // Don't assume OS for containers, let other indicators decide
        return None;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_host_os_detection() {
        let ctx = SystemContext::new();
        assert!(!ctx.host_os.is_empty());
        assert!(!ctx.host_arch.is_empty());
    }

    #[test]
    fn test_windows_detection() {
        let mut ctx = SystemContext::new();
        ctx.detect_target_os("Check if the Windows Update service is running");
        assert_eq!(ctx.target_os, Some("windows".to_string()));
        assert_eq!(ctx.get_powershell_command(), "powershell");
    }

    #[test]
    fn test_linux_detection() {
        let mut ctx = SystemContext::new();
        ctx.detect_target_os("On a Linux server, how do I check running processes?");
        assert_eq!(ctx.target_os, Some("linux".to_string()));
        assert_eq!(ctx.get_powershell_command(), "pwsh");
    }

    #[test]
    fn test_macos_detection() {
        let mut ctx = SystemContext::new();
        ctx.detect_target_os("Use brew to install something on macOS");
        assert_eq!(ctx.target_os, Some("macos".to_string()));
        assert_eq!(ctx.get_powershell_command(), "pwsh");
    }

    #[test]
    fn test_no_target_os_uses_host() {
        let ctx = SystemContext::new();
        // Should use host OS when no target detected
        assert_eq!(ctx.get_effective_os(), &ctx.host_os);
    }
}
