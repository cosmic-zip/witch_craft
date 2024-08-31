use crate::core::consts::*;
use crate::core::core::*;
use crate::core::structs::DataSet;
use crate::modules::network::structs::*;

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
    let out = lazy_parser(&meta_string, &new_argsv);

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
    let parsed_cmd = lazy_parser(meta, &argsv);
    let out = lazy_exec(parsed_cmd);

    assert_eq!(0, out);
}

#[test]
fn flawless_exec_pipeline_argsv_without_args() {
    let argsv = vec!["test.local".to_string()];

    let set = DataSet::from_str("", "test.local", "ps -aux");

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

    let set = DataSet::from_str("", "test.local", "ping -c1 @@some");

    let out = flawless_exec(set, &argsv);

    assert_eq!(0, out);
}

#[test]
fn test_network_request_response_fail() {
    let mut request = Request::new();
    request.url = "http://example.com/clover".to_string();
    request.method = "GET".to_string();

    let response = request.make();
    let url = response.url;
    let status = response.status;
    let body = response.body;

    assert_eq!(url, "http://example.com/clover".to_string());
    assert_eq!(status, "500 Internal Server Error");
    // assert_eq!(body, "");
}

#[test]
fn test_network_request_response_ok() {
    let mut request = Request::new();
    request.url = "http://example.com".to_string();
    request.method = "GET".to_string();

    let response = request.make();
    let url = response.url;
    let status = response.status;
    let body = response.body;

    assert_eq!(url, "http://example.com".to_string());
    assert_eq!(status, "200 OK");
    // assert_eq!(body, "");
}
