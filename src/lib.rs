use ::std::fs;
use std::error::Error;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // 参数获取到之后就需要读取文件
    let contents = fs::read_to_string(&config.filename)?;
    let result = search(&config.query, &contents);
    println!("{:?}", result);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("输入的参数不完整\n");
        // }
        // 不需要第一个参数，直接消耗掉
        args.next();
        // 我们的命令需要两个参数： 1. 需要查询的字符  2. 需要查询的参数
        let query = match args.next() {
            Some(s) => s,
            None => return Err("缺少参数1 - 需要查询的字符串"),
        };
        let filename = match args.next() {
            Some(s) => s,
            None => return Err("缺少参数2 - 需要查询的文件"),
        };

        Ok(Config { query, filename })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         result.push(line);
    //     }
    // }

    // result

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_search() {
        let query = "这";
        let contents = "\
这是一行
第二行";
        assert_eq!(search(query, contents), vec!["这是一行"]);
    }
}
