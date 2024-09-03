use crate::core::core::*;

use image::Luma;
use qrcode::QrCode;

pub fn gen_qrcode_from_argsv(argsv: &[String]) -> i32 {
    let data = search_value("data", argsv);
    let code = match QrCode::new(data.as_bytes()) {
        Ok(val) => val,
        Err(err) => {
            raise(&err.to_string(), 2);
            return 42;
        }
    };

    let now = datetime_now();
    let image = code.render::<Luma<u8>>().build();
    image.save(format!("./qrcode-{now}.png")).unwrap();

    let string = code.render().light_color(' ').dark_color('#').build();
    raise(&string, 6);
    0
}
