// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use serde::{ser::Serializer, Serialize};
use tauri::{
  command,
  plugin::{Builder, TauriPlugin},
  Manager, Runtime, State,
};

use std::env::current_exe;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Copy, Clone)]
pub enum MacosLauncher {
  LaunchAgent,
  AppleScript,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

pub struct AutoLaunchManager(AutoLaunch);

impl AutoLaunchManager {
  pub fn enable(&self) -> Result<()> {
    self.0.enable().map_err(Into::into)
  }

  pub fn disable(&self) -> Result<()> {
    self.0.disable().map_err(Into::into)
  }

  pub fn is_enabled(&self) -> Result<bool> {
    self.0.is_enabled().map_err(Into::into)
  }
}

pub trait ManagerExt<R: Runtime> {
  fn autolaunch(&self) -> State<'_, AutoLaunchManager>;
}

impl<R: Runtime, T: Manager<R>> ManagerExt<R> for T {
  fn autolaunch(&self) -> State<'_, AutoLaunchManager> {
    self.state::<AutoLaunchManager>()
  }
}

#[command]
async fn enable(manager: State<'_, AutoLaunchManager>) -> Result<()> {
  manager.enable()
}

#[command]
async fn disable(manager: State<'_, AutoLaunchManager>) -> Result<()> {
  manager.disable()
}

#[command]
async fn is_enabled(manager: State<'_, AutoLaunchManager>) -> Result<bool> {
  manager.is_enabled()
}

/// Initializes the plugin.
pub fn init<R: Runtime>(macos_launcher: MacosLauncher, hidden: bool) -> TauriPlugin<R> {
  Builder::new("autostart")
    .invoke_handler(tauri::generate_handler![enable, disable, is_enabled])
    .setup(move |app| {
      let mut builder = AutoLaunchBuilder::new();

      builder.set_app_name(&app.package_info().name);
      builder.set_hidden(hidden);
      builder.set_use_launch_agent(matches!(macos_launcher, MacosLauncher::LaunchAgent));

      let current_exe = current_exe()?;

      #[cfg(windows)]
      builder.set_app_path(&current_exe.display().to_string());
      #[cfg(target_os = "macos")]
      builder.set_app_path(&current_exe.canonicalize()?.display().to_string());
      #[cfg(target_os = "linux")]
      if let Some(appimage) = app
        .env()
        .appimage
        .and_then(|p| p.to_str().map(|s| s.to_string()))
      {
        builder.set_app_path(&appimage);
      } else {
        builder.set_app_path(&current_exe.display().to_string());
      }

      app.manage(AutoLaunchManager(builder.build()));
      Ok(())
    })
    .build()
}
