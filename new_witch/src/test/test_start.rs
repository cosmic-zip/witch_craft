use super::*;
#[allow(unused_imports)]
use crate::modules::core::data::*;
use crate::*;

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
