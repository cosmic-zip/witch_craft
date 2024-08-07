use crate::modules::binds::binds::*;
use crate::modules::core::core::*;
use crate::modules::core::data::*;
use crate::modules::shell::fancy::*;
use crate::modules::shell::shell::*;

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
    let mut newvec = Vec::<String>::new();
    for x in argsv {
        newvec.push(x.to_string());
    }

    let meta_string = format!(
        "command -S {}some -A {}another -N {}numeric -P {}path",
        TONK, TONK, TONK, TONK
    );
    let out = lazy_loop(&meta_string, newvec);

    assert_eq!(expected, out);
}

#[test]
fn test_lazy_pipeline() {
    let argsv = vec![
        "complete/path/to/here/".to_string(),
        "--pretty".to_string(),
        "-lha".to_string(),
    ];

    let cmds = "ls @@pretty";
    let out = lazy_loop(cmds, argsv.clone());
    let a = lazy_exec(out, true);

    assert_eq!(0, a);
}

#[test]
fn test_check() {
    let exit = check_install();
    assert_eq!(exit, true);
}
