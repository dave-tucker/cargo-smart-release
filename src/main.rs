mod options;
use options::Args;

mod command;

fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();
    init_logging();

    let Args {
        execute,
        bump,
        crates,
        allow_dirty,
        ignore_instability,
        skip_publish,
        skip_tag,
        dangerously_pass_no_verify,
    } = args;

    command::release(
        command::release::Options {
            dry_run: !execute,
            allow_dirty,
            ignore_instability,
            skip_publish,
            skip_tag,
            no_verify: dangerously_pass_no_verify,
        },
        bump.unwrap_or_else(|| "keep".into()),
        crates,
    )?;

    Ok(())
}

fn init_logging() {
    env_logger::Builder::new()
        .format_module_path(false)
        .format_target(false)
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Info)
        .init();
}
