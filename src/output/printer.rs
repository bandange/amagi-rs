//! Printer implementation for startup messages.

use std::{
    cell::Cell,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
};

use serde::Serialize;
use serde_json::json;

use crate::config::{OutputConfig, OutputFormat};
use crate::error::AppError;

/// Emit startup messages in either plain text or JSON.
pub struct Printer {
    config: OutputConfig,
    wrote_once: Cell<bool>,
}

impl Printer {
    /// Create a [`Printer`] that uses the configured [`OutputFormat`].
    pub fn new(config: OutputConfig) -> Self {
        Self {
            config,
            wrote_once: Cell::new(false),
        }
    }

    /// Print the application banner.
    ///
    /// In JSON mode this method is a no-op because readiness messages already
    /// include the application metadata.
    ///
    /// # Errors
    ///
    /// Returns an error when writing to stdout fails.
    pub fn print_banner(&self, app_name: &str, version: &str) -> Result<(), AppError> {
        if self.config.format == OutputFormat::Text {
            self.with_writer(|writer| {
                writeln!(writer, "{app_name} {version}")?;
                Ok(())
            })?;
        }

        Ok(())
    }

    /// Print the readiness payload for CLI mode.
    ///
    /// # Errors
    ///
    /// Returns an error when writing to stdout fails.
    pub fn print_run_ready(&self, app_name: &str, version: &str) -> Result<(), AppError> {
        match self.config.format {
            OutputFormat::Text => self.with_writer(|writer| {
                writeln!(writer, "mode: cli")?;
                writeln!(writer, "status: ready")?;
                writeln!(
                    writer,
                    "next: run `amagi run douyin emoji-list`, `amagi run kuaishou emoji-list`, or start `amagi serve`"
                )?;
                Ok(())
            }),
            OutputFormat::Json => self.write_json(&json!({
                "kind": "run_ready",
                "app": app_name,
                "version": version,
                "mode": "cli",
                "status": "ready",
                "next": "run `amagi run douyin emoji-list`, `amagi run kuaishou emoji-list`, or start `amagi serve`"
            })),
        }
    }

    /// Print the readiness payload for Web API mode.
    ///
    /// # Errors
    ///
    /// Returns an error when writing to stdout fails.
    pub fn print_server_ready(
        &self,
        app_name: &str,
        version: &str,
        bind_addr: &str,
    ) -> Result<(), AppError> {
        let root = format!("http://{bind_addr}/");
        let health = format!("http://{bind_addr}/health");
        let douyin_emoji = format!("http://{bind_addr}/api/douyin/emoji");
        let kuaishou_emoji = format!("http://{bind_addr}/api/kuaishou/emoji");

        match self.config.format {
            OutputFormat::Text => self.with_writer(|writer| {
                writeln!(writer, "mode: server")?;
                writeln!(writer, "status: listening")?;
                writeln!(writer, "addr: {bind_addr}")?;
                writeln!(writer, "root: {root}")?;
                writeln!(writer, "health: {health}")?;
                writeln!(writer, "douyin_emoji: {douyin_emoji}")?;
                writeln!(writer, "kuaishou_emoji: {kuaishou_emoji}")?;
                Ok(())
            }),
            OutputFormat::Json => self.write_json(&json!({
                "kind": "server_ready",
                "app": app_name,
                "version": version,
                "mode": "server",
                "status": "listening",
                "addr": bind_addr,
                "root": root,
                "health": health,
                "douyin_emoji": douyin_emoji,
                "kuaishou_emoji": kuaishou_emoji
            })),
        }
    }

    /// Print a serialized payload emitted by a fetch command.
    ///
    /// # Errors
    ///
    /// Returns an error when writing to stdout fails.
    pub fn print_payload<T: Serialize>(&self, value: &T) -> Result<(), AppError> {
        self.with_writer(|writer| {
            match self.config.format {
                OutputFormat::Text => serde_json::to_writer_pretty(&mut *writer, value)?,
                OutputFormat::Json if self.config.pretty => {
                    serde_json::to_writer_pretty(&mut *writer, value)?
                }
                OutputFormat::Json => serde_json::to_writer(&mut *writer, value)?,
            }

            writeln!(writer)?;
            Ok(())
        })
    }

    fn write_json<T: Serialize>(&self, value: &T) -> Result<(), AppError> {
        self.with_writer(|writer| {
            if self.config.pretty {
                serde_json::to_writer_pretty(&mut *writer, value)?;
            } else {
                serde_json::to_writer(&mut *writer, value)?;
            }

            writeln!(writer)?;
            Ok(())
        })
    }

    fn with_writer<F>(&self, write: F) -> Result<(), AppError>
    where
        F: FnOnce(&mut dyn Write) -> Result<(), AppError>,
    {
        match self.config.file.as_deref() {
            Some(path) => {
                let path = Path::new(path);

                if self.config.create_parent_dirs {
                    if let Some(parent) =
                        path.parent().filter(|value| !value.as_os_str().is_empty())
                    {
                        fs::create_dir_all(parent)?;
                    }
                }

                let mut options = OpenOptions::new();
                options.create(true).write(true);

                if self.config.append || self.wrote_once.get() {
                    options.append(true);
                } else {
                    options.truncate(true);
                }

                let mut file = options.open(path)?;
                write(&mut file)?;
                file.flush()?;
                self.wrote_once.set(true);
                Ok(())
            }
            None => {
                let stdout = io::stdout();
                let mut stdout = stdout.lock();
                write(&mut stdout)?;
                stdout.flush()?;
                Ok(())
            }
        }
    }
}
