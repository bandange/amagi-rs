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
                eprintln!("[amagi] error: cli runtime thread panicked");
                std::process::exit(1);
            }
        },
        Err(error) => {
            eprintln!("[amagi] error: failed to spawn cli runtime thread: {error}");
            std::process::exit(1);
        }
    }
}
