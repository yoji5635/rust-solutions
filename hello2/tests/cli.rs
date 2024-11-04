// use std::process::Command;
use assert_cmd::Command;

#[test]
fn works() {
    assert!(true)
    // assert!(false)
}

// std::process::Command
// #[test]
// fn runs() {
//     let mut cmd = Command::new("ls");
//     let res = cmd.output();
//     assert!(res.is_ok());
// }

// assert_cmd::Commandを使ったテスト
#[test]
fn runs() {
    // 対象のプログラムがあるかを確認するテスト、cargoプロジェクトの場合はプロジェクト名
    let mut cmd = Command::cargo_bin("hello2").unwrap();
    // 成功を確認する
    // cmd.assert().success();

    // 実際の出力をテストする
    cmd.assert().success().stdout("Hello, world!");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    // コマンドの失敗を検証する
    cmd.assert().failure();
}
