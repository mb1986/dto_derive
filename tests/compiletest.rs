use compiletest_rs as compiletest;
use std::path::PathBuf;

fn run_mode(mode: &'static str, path: &'static str) {
    let mut config = compiletest::Config {
        mode: mode.parse().expect("Invalid mode"),
        src_base: PathBuf::from(format!("tests/{}", path)),
        ..Default::default()
    };
    config.link_deps();
    config.clean_rmeta();

    compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode("compile-fail", "ui-fail");
    run_mode("ui", "ui-fail");
}
