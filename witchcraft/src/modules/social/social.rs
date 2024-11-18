use crate::{core::types::Closure, modules::social::qrcode::gen_qrcode_from_argsv};

pub fn api() -> Closure {
    vec![("qrcode", gen_qrcode_from_argsv)]
}
