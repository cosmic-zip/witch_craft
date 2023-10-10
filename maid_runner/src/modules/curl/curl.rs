use crate::core::messages::standard_messages;
use crate::core::utils::*;
use crate::modules::curl::curl_structs::*;

pub fn curl_request(curl: CurlBind, debug: bool) -> bool {
    standard_messages("warning", "Curl binds", "", "cute");

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

    let cmd = format!("curl {} {} {} {} {}", method, auth, url, ctn_type, data);
    if debug == true {
        standard_messages("warning", "Trying exec command", &cmd, "cute");
    }
    system_command_exec(&cmd, debug)
}

pub fn shell_curl(system_input: Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {
        "--curl_bind" => {
            let debug = gsv_debug(gsv(system_input.clone(), "--debug"));
            let instance = CurlBind {
                method: &gsv(system_input.clone(), "--method"),
                auth_type: &gsv(system_input.clone(), "--auth_type"),
                auth_token: &gsv(system_input.clone(), "--auth_token"),
                url: &gsv(system_input.clone(), "--url"),
                ctn_type: &gsv(system_input.clone(), "--ctn_type"),
                data: &gsv(system_input.clone(), "--data"),
            };
            curl_request(instance, debug)
        }

        "--header" => {
            standard_messages("debug", "Curl header lookup", "", "cute");
            let debug = gsv_debug(gsv(system_input.clone(), "--debug"));
            let curl = &gsv(system_input.clone(), "--url");
            let cmd = format!("curl -I {}", curl);
            if debug == true {
                standard_messages("debug", "Curl header lookup", &cmd, "cute");
            }
            system_command_exec(&cmd, debug)
        }

        "--status_code" => {
            standard_messages("debug", "Curl request status_code", "", "cute");
            let debug = gsv_debug(gsv(system_input.clone(), "--debug"));
            let curl = &gsv(system_input.clone(), "--url");
            let cmd = format!("curl -o /dev/null -s -w \"%{{http_code}}\n\" {}", curl);
            if debug == true {
                standard_messages("debug", "Curl request status_code", &cmd, "cute");
            }
            system_command_exec(&cmd, debug)
        }

        _ => {
            standard_messages("warning", "Invalid user input", "shell_curl", "cute");
            standard_messages("warning", "Trying exec command", cmd_arg_name, "cute");
            return false;
        }
    }
}
