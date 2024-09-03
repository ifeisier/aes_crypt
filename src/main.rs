#![warn(missing_docs)]

//! 会遍历当前目录下的所有文件, 然后使用 [AES Crypt](https://www.aescrypt.com/) 对其进行加密.

use std::env;
use std::process::Command;
use walkdir::WalkDir;

/// 加密和解密密码
///
/// 生成随机密码
/// https://suijimimashengcheng.bmcx.com/
const PASS_WORD: &str = "6ANTEWJ^&^eYWaE%KbQX!pC2tGW7kzYbYBv#54Kj#65y9m$txXRAKzCWe$$az*Jf";

fn main() {
    let pass_word = match env::var("PASS_WORD") {
        Ok(v) => v,
        Err(_) => PASS_WORD.to_owned(),
    };

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] == "-d" {
        decrypt(pass_word);
    } else if args[1] == "-e" {
        encrypt(pass_word);
    }
}

/// 加密
#[allow(dead_code)]
fn encrypt(pass_word: String) {
    let file_format = vec!["rs", "java", "py"];
    let command = |f: String| {
        let output = Command::new("./aescrypt")
            .arg("-p")
            .arg(pass_word.clone())
            .arg("-e")
            .arg(f)
            .output();
        println!("{:?}", output);
    };
    iteration(&file_format, command);
}

/// 解密
#[allow(dead_code)]
fn decrypt(pass_word: String) {
    let file_format = vec!["aes"];
    let command = |f: String| {
        let output = Command::new("./aescrypt")
            .arg("-p")
            .arg(pass_word.clone())
            .arg("-d")
            .arg(f)
            .output();
        println!("{:?}", output);
    };
    iteration(&file_format, command);
}

/// 遍历当前文件下的所有文件, 并执行指定的操作
fn iteration<F>(file_format: &Vec<&str>, command: F)
where
    F: Fn(String),
{
    for entry in WalkDir::new("../") {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                eprintln!("无法访问条目: {}", err);
                continue;
            }
        };

        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        if let Some(f) = path.extension().and_then(std::ffi::OsStr::to_str) {
            if file_format.contains(&f) {
                println!("{:?}", path.display().to_string());
                command(path.display().to_string());
            }
        }
    }
}
