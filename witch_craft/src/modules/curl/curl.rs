use crate::core::messages::standard_messages;
use crate::core::structs::ProcessInit;
use crate::core::utils::*;
use crate::modules::curl::curl_structs::*;

pub fn curl_request(curl: CurlBind, debug: bool) -> bool {
    // binds
    let url = curl.url;
    let data = curl.data;
    let mut method = "";
    let mut auth = "";
    let mut ctn_type = "";

    // content types
    let ctn_type_json = "-H Content-Type: application/json";
    let ctn_type_xml = "-H Content-Type: application/xml";
    let ctn_type_form_data = "-H Content-Type: application/x-www-form-urlencoded";
    let ctn_type_text = "-H Content-Type: text/plain";
    let ctn_type_multi_part_form_data = "-F \"file=@/path/to/file.txt\"";
    let ctn_type_custom = "-H Content-Type: application/custom";

    // ??????????????
    let auth_basic = &format!("-H -u \"{}\"", curl.auth_token);
    let auth_bearer = &format!("-H -u \"Authorization: Bearer {}\"", curl.auth_token);
    let auth_api_key = &format!("-H \"X-API-Key: {}\"", curl.auth_token);
    let auth_ntlm = &format!("--ntlm -u {}", curl.auth_token);

    match curl.auth_type {
        "basic" => auth = auth_basic,
        "bearer" => auth = auth_bearer,
        "api_key" => auth = auth_api_key,
        "ntlm" => auth = auth_ntlm,
        _ => auth = "",
    }

    match curl.method {
        "get" => method = "GET",
        "post" => method = "POST",
        "put" => method = "PUT",
        "patch" => method = "PATCH",
        "delete" => method = "DELETE",
        "head" => method = "HEAD",
        "options" => method = "OPTIONS",
        "connect" => method = "CONNECT",
        "trace" => method = "TRACE",
        _ => method = "",
    }

    match curl.ctn_type {
        "json" => ctn_type = ctn_type_json,
        "xml" => ctn_type = ctn_type_xml,
        "form_data" => ctn_type = ctn_type_form_data,
        "text" => ctn_type = ctn_type_text,
        "multi_part_form_data" => ctn_type = ctn_type_multi_part_form_data,
        "custom" => ctn_type = ctn_type_custom,
        _ => ctn_type = "",
    }

    let instance = ProcessInit {
        source: &format!("curl {} {} {} {} {}", method, auth, url, ctn_type, data),
        source_from: "bcurl",
        source_description: "Executing bcurl requests",
        debug: debug,
    };
    system_command_exec(instance)
}

pub fn shell_curl(system_input: &mut Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {
        "--curl_bind" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let instance = CurlBind {
                method: &take_system_args(system_input, "--method"),
                auth_type: &take_system_args(system_input, "--auth_type"),
                auth_token: &take_system_args(system_input, "--auth_token"),
                url: &take_system_args(system_input, "--url"),
                ctn_type: &take_system_args(system_input, "--ctn_type"),
                data: &take_system_args(system_input, "--data"),
            };

            standard_messages("debug", "BCurl bindings", "", "cute");

            curl_request(instance, debug)
        }

        "--header" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let curl = &take_system_args(system_input, "--url");

            let instance = ProcessInit {
                source: &format!("curl -I {}", curl),
                source_from: "bcurl",
                source_description: "Curl http header lookup",
                debug: debug,
            };
            system_command_exec(instance)
        }

        "--status_code" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let curl = &take_system_args(system_input, "--url");

            let instance = ProcessInit {
                source: &format!("curl -o /dev/null -s -w \"%{{http_code}}\n\" {}", curl),
                source_from: "bcurl",
                source_description: "Bcurl GET request status code",
                debug: debug,
            };
            system_command_exec(instance)
        }

        _ => {
            standard_messages("warning", "Invalid user input", "shell_curl", "cute");
            standard_messages("warning", "Trying exec command", cmd_arg_name, "cute");
            return false;
        }
    }
}
