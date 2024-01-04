use super::*;
use crate::*;

#[test]
fn test_bind_curl() {
    let debug = true;
    let instance = CurlBind {
        method: "get",
        auth_type: "",
        auth_token: "",
        url: "http://example.com/",
        ctn_type: "",
        data: "",
    };
    let output = curl_request(instance, debug);

    assert_eq!(output, true);
}
