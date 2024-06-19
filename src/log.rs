// Q: Why not use an existing solution?
// A: I need something extremely simple, and having a logging library that depends
//    on 7 others and provides 1000 features is extremely overkill for the scope of this project.

enum LogType {
    Info,
    Warn,
    Error,
    Success,
}

fn log(_type: LogType, ctx: &str, message: &str) {
    let context_color = 107;
    let color = match _type {
        LogType::Info => 104,
        LogType::Warn => 103,
        LogType::Error => 101,
        LogType::Success => 102,
    };

    let prefix_type = match _type {
        LogType::Info => "inf",
        LogType::Warn => "wrn",
        LogType::Error => "err",
        LogType::Success => "suc",
    };

    let prefix = format!("\x1b[1;37;{}m  {}  \x1b[0m", color, prefix_type);
    let ctx = format!("\x1b[37;{}m  {}  \x1b[0m", context_color, ctx);
    let message = format!("\x1b[37;{}m   \x1b[0m {}", color, message);

    println!("{}{}{}", prefix, ctx, message);
}

pub fn info(ctx: &str, message: &str) {
    log(LogType::Info, ctx, message);
}

pub fn warn(ctx: &str, message: &str) {
    log(LogType::Warn, ctx, message);
}

pub fn error(ctx: &str, message: &str) {
    log(LogType::Error, ctx, message);
}

pub fn success(ctx: &str, message: &str) {
    log(LogType::Success, ctx, message);
}
