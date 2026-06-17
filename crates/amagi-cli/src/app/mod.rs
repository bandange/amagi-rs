//! Application runtime orchestration for CLI and server modes.

mod bilibili;
mod douyin;
mod kuaishou;
mod twitter;
mod xiaohongshu;

use tracing::info;

use crate::APP_NAME;
use crate::config::{AppConfig, CommandConfig, RunConfig, RunTask};
use crate::output::Printer;
use crate::telemetry;
use amagi_client::AmagiClient;
use amagi_core::{AppError, build_info};

/// Initialize shared services and execute the selected runtime command.
///
/// This function configures telemetry, constructs a [`Printer`], and dispatches
/// the resolved [`crate::config::CommandConfig`].
///
/// # Errors
///
/// Returns an error if startup output cannot be written or if the selected
/// command fails during execution.
pub async fn run(config: AppConfig) -> Result<(), AppError> {
    let AppConfig {
        command,
        output,
        logging,
        client,
    } = config;

    telemetry::try_init(logging).map_err(|error| {
        AppError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to initialize telemetry: {error}"),
        ))
    })?;

    let printer = Printer::new(output);
    let client = AmagiClient::new(client);
    match command {
        CommandConfig::Run(run) => run_command(&printer, &client, &run).await,
        #[cfg(feature = "server")]
        CommandConfig::Serve(serve) => amagi_server::serve(serve, client, &printer).await,
    }
}

async fn run_command(
    printer: &Printer,
    client: &AmagiClient,
    config: &RunConfig,
) -> Result<(), AppError> {
    if matches!(config.task, RunTask::Ready) && !config.quiet {
        printer.print_banner(APP_NAME, build_info::DISPLAY_VERSION)?;
        printer.print_run_ready(APP_NAME, build_info::DISPLAY_VERSION)?;
    }

    match &config.task {
        RunTask::Ready => {
            info!(
                app = APP_NAME,
                mode = "cli",
                quiet = config.quiet,
                "cli workflow initialized"
            );
        }
        RunTask::Bilibili(task) => {
            bilibili::run_task(printer, client, task).await?;
        }
        RunTask::Douyin(task) => {
            douyin::run_task(printer, client, task).await?;
        }
        RunTask::Kuaishou(task) => {
            kuaishou::run_task(printer, client, task).await?;
        }
        RunTask::Twitter(task) => {
            twitter::run_task(printer, client, task).await?;
        }
        RunTask::Xiaohongshu(task) => {
            xiaohongshu::run_task(printer, client, task).await?;
        }
    }

    Ok(())
}
