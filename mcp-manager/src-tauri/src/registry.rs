use crate::adapters::ConfigAdapter;
use crate::adapters::{
    claude_desktop::ClaudeDesktopAdapter,
    claude_code::ClaudeCodeAdapter,
    vscode::VscodeAdapter,
    cline::ClineAdapter,
    roo_code::RooCodeAdapter,
    cursor::CursorAdapter,
    windsurf::WindsurfAdapter,
    jetbrains::JetBrainsAdapter,
    visual_studio::VisualStudioAdapter,
    copilot_cli::CopilotCliAdapter,
    amazon_q::AmazonQAdapter,
    zed::ZedAdapter,
    continue_dev::ContinueDevAdapter,
    gemini_cli::GeminiCliAdapter,
};

pub fn build_adapter_registry() -> Vec<Box<dyn ConfigAdapter>> {
    let mut adapters: Vec<Box<dyn ConfigAdapter>> = vec![
        Box::new(ClaudeDesktopAdapter::new()),
        Box::new(ClaudeCodeAdapter::new()),
        Box::new(VscodeAdapter::new()),
        Box::new(ClineAdapter::new()),
        Box::new(RooCodeAdapter::new()),
        Box::new(CursorAdapter::new()),
        Box::new(WindsurfAdapter::new()),
        Box::new(VisualStudioAdapter::new()),
        Box::new(CopilotCliAdapter::new()),
        Box::new(AmazonQAdapter::new()),
        Box::new(ZedAdapter::new()),
        Box::new(ContinueDevAdapter::new()),
        Box::new(GeminiCliAdapter::new()),
    ];
    for jb_adapter in JetBrainsAdapter::detect_all() {
        adapters.push(Box::new(jb_adapter));
    }
    adapters
}

pub fn detect_installed_tools() -> Vec<Box<dyn ConfigAdapter>> {
    build_adapter_registry()
        .into_iter()
        .filter(|a| a.detect())
        .collect()
}
