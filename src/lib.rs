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
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let command = worktree
            .which("nestor")
            .ok_or("nestor must be installed and available on your PATH")?;

        Ok(zed::Command {
            command,
            args: vec![],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(MyExtension);
