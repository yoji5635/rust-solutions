use clap::{App, Arg};

fn main() {
    // println!("Hello, world!");

    // std::env::args 引数を受け取る方法
    // {}だと、Displayのトレイトを継承していないのでエラーになる
    println!("{:?}", std::env::args());

    // cargo rn hello world -n OK
    // cargo run -n hello world NG
    // cargo run -- -n hello world OK

    // clapを利用したオプション解析
    // -hと-vはデフォルトで存在する

    let matches = App::new("echor")
        .version("0.1.0")
        .author("Ken")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    // {:#?} <- pretty-printing、整形表示
    println!("{:#?}", matches);

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    // let mut ending = "\n";
    // if omit_newline {
    //     ending = "";
    // }

    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text.join(""), ending);
}
