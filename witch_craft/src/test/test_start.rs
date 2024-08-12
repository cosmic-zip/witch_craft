use crate::modules::binds::binds::*;
use crate::modules::core::core::*;
use crate::modules::core::data::*;
use crate::modules::core::structs::DataSet;
use crate::modules::shell::shell::*;
use crate::modules::core::consts::*;

#[test]
fn test_start() {
    println!("TEST HERE!");

    let exit = true;
    assert_eq!(exit, true);
}

#[test]
fn test_string_to_command() {
    let argsv = vec![
        "some.rs",
        "--some",
        "val",
        "--another",
        "ant",
        "--numeric",
        "123",
        "--path",
        "/etc/hosts",
    ];

    let expected = "command -S val -A ant -N 123 -P /etc/hosts";
    let mut new_argsv = Vec::<String>::new();
    for x in argsv {
        new_argsv.push(x.to_string());
    }

    let meta_string = format!(
        "command -S {}some -A {}another -N {}numeric -P {}path",
        TONK, TONK, TONK, TONK
    );
    let out = lazy_loop(&meta_string, &new_argsv);

    assert_eq!(expected, out);
}

#[test]
fn test_lazy_pipeline() {
    let argsv = vec![
        "complete/path/to/here/".to_string(),
        "--pretty".to_string(),
        "-lha".to_string(),
    ];

    let meta = "ls @@pretty";
    let parsed_cmd = lazy_loop(meta, &argsv);
    let out = lazy_exec(parsed_cmd, true);

    assert_eq!(0, out);
}

#[test]
fn flawless_exec_pipeline_argsv_without_args() {
    let argsv = vec![
        "test.local".to_string(),
    ];

    let set = DataSet::from_str(
        "", "test.local", "ps -aux");

    let out = flawless_exec(set, &argsv);

    assert_eq!(0, out);
}

#[test]
fn flawless_exec_pipeline_argsv_with_args() {
    let argsv = vec![
        "test.local".to_string(),
        "--some".to_string(),
        "localhost".to_string(),
    ];

    let set = DataSet::from_str(
        "", "test.local", "ping -c1 @@some");

    let out = flawless_exec(set, &argsv);

    assert_eq!(0, out);
}
