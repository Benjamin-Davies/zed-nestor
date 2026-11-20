use std::fs;
use zed_extension_api::{self as zed, LanguageServerId, Result};

const REPO_NAME: &str = "Benjamin-Davies/nestor";
const BIN_NAME: &str = "nestor";

struct NestorExtension {
    cached_binary_path: Option<String>,
}

impl NestorExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // Prefer a binary the user already has on their PATH.
        if let Some(path) = worktree.which(BIN_NAME) {
            return Ok(path);
        }

        // Reuse the binary we already resolved this session.
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|m| m.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = match zed::latest_github_release(
            REPO_NAME,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ) {
            Ok(release) => release,
            // Offline or rate limited: fall back to any previously installed version.
            Err(err) => {
                return self
                    .find_installed_binary()
                    .ok_or_else(|| format!("failed to check for updates: {err}"));
            }
        };

        let asset_name = build_asset_name()?;
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {asset_name:?}"))?;

        let version_dir = format!("{BIN_NAME}-{}", release.version);
        let binary_path = format!("{version_dir}/{BIN_NAME}");

        if !fs::metadata(&binary_path).is_ok_and(|m| m.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::GzipTar,
            )
            .map_err(|err| format!("failed to download {asset_name}: {err}"))?;
            zed::make_file_executable(&binary_path)?;

            // Remove older versions.
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let name = name.to_string_lossy();
                    if name.starts_with(&format!("{BIN_NAME}-")) && name != version_dir {
                        fs::remove_dir_all(entry.path()).ok();
                    }
                }
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }

    fn find_installed_binary(&self) -> Option<String> {
        fs::read_dir(".")
            .ok()?
            .flatten()
            .map(|entry| format!("{}/{BIN_NAME}", entry.file_name().to_string_lossy()))
            .find(|path| {
                path.starts_with(&format!("{BIN_NAME}-"))
                    && fs::metadata(path).is_ok_and(|m| m.is_file())
            })
    }
}

impl zed::Extension for NestorExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec![],
            env: worktree.shell_env(),
        })
    }
}

fn build_asset_name() -> Result<String> {
    let (os, arch) = zed::current_platform();

    let target = match (os, arch) {
        (zed::Os::Linux, zed::Architecture::X8664) => "x86_64-unknown-linux-musl",
        (zed::Os::Mac, zed::Architecture::Aarch64) => "aarch64-apple-darwin",
        _ => return Err("nestor does not publish binaries for this platform yet".to_owned()),
    };

    Ok(format!("{BIN_NAME}-{target}.tar.gz"))
}

zed::register_extension!(NestorExtension);
