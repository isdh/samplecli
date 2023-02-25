use clap::{Command, Arg, ArgAction};

fn main() {
    let matches = Command::new("My RPN program")
    // コマンドラインのバージョン
    .version("1.0.0")
    // 作者名
    .author("isdh")
    // 説明
    .about("Super awesome sample RPN calculator")
    // デフォルトのヘルプでは作者名やコマンド名は表示されなくなった
    // Command::help_templateを使用すると表示できる
    .help_template("\
{name} v{version} created by {author}
    ")
    // コマンドライン引数定義
    .arg(
        Arg::new("formula_file")
        // .aboutは@deprecated
        .help("Formulas written in RPN")
        .value_name("FILE")
        .index(1)
        .required(false),
    )
    // コマンドライン引数定義
    .arg(
        // -vで表示するような内容
        Arg::new("verbose")
        .help("Sets the level of verbosity")
        .short('v')
        .long("verbose")
        .required(false)
        // v4からflagで管理するのを推奨っぽい
        // https://github.com/clap-rs/clap/blob/master/CHANGELOG.md#migrating
        .action(ArgAction::SetTrue),
    )
    .get_matches();

    // 以下はget_one/get_manyに変わった
    // match matches.value_of("formula_file") {
    match matches.get_one::<String>("formula_file") {
        Some(file) => println!("File speecified: {}", file),
        None => println!("No file specified."),
    }
    // 以下はget_flagに変わった
    // let verbose = matches.is_present("verbose");
    let verbose = matches.get_flag("verbose");
    println!("Is verbosity specified?: {} ", verbose );
}