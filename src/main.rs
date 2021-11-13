use minigrep::{run, Config};
use std::env;
use std::process;

fn main() {
    let args = env::args();

    // 获取参数
    let config = Config::new(args).unwrap_or_else(|err| {
        eprint!("在解析参数时发生错误：{}", err);
        process::exit(1)
    });

    if let Err(e) = run(&config) {
        eprint!("运行错误{}", e);
        process::exit(1);
    };
}
