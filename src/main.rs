use std::env;
use rand::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if 1 >= args.len() {
        println!("要素を指定してください");
        println!("例) ./lottery foo bar hoge fuga");
        return;
    }

    let m = &args[1..args.len()];
    println!("候補: {:?}", m);

    let i = rand::thread_rng().gen_range(0..m.len());
    println!("抽選結果: {}", m[i]);
}
