use zed_extension_api::{self as zed, LanguageServerId, Result};

struct MyExtension;

impl zed::Extension for MyExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: "nestor".to_owned(),
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(MyExtension);
