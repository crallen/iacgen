use time::UtcOffset;
use tracing::Level;
use tracing_subscriber::fmt::time::OffsetTime;

pub fn init(args: &crate::cli::Args) {
    let level = if args.debug {
        Level::DEBUG
    } else {
        Level::INFO
    };

    let time_format =
        time::format_description::parse("[hour]:[minute]:[second].[subsecond digits:3]").unwrap();
    let time_offset = UtcOffset::current_local_offset().unwrap_or_else(|_| UtcOffset::UTC);
    let timer = OffsetTime::new(time_offset, time_format);

    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_timer(timer)
        .init();
}
