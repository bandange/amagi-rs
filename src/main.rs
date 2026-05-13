const CLI_STACK_SIZE: usize = 16 * 1024 * 1024;

fn main() {
    let worker = std::thread::Builder::new()
        .name("amagi-main".to_owned())
        .stack_size(CLI_STACK_SIZE)
        .spawn(|| -> Result<(), amagi::error::AppError> {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .map_err(amagi::error::AppError::from)?;
            runtime.block_on(amagi::run_env())
        });

    match worker {
        Ok(handle) => match handle.join() {
            Ok(Ok(())) => {}
            Ok(Err(error)) => {
                amagi::print_startup_error(&error);
                std::process::exit(1);
            }
            Err(_) => {
                amagi::output::print_startup_message(
                    amagi::APP_NAME,
                    "cli runtime thread panicked",
                    "CLI 运行时线程发生 panic",
                );
                std::process::exit(1);
            }
        },
        Err(error) => {
            amagi::output::print_startup_message(
                amagi::APP_NAME,
                &format!("failed to spawn cli runtime thread: {error}"),
                &format!("启动 CLI 运行时线程失败: {error}"),
            );
            std::process::exit(1);
        }
    }
}
