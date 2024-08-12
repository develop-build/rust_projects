use image::Luma;
use qrcode::QrCode;

fn main() {
    let data = "https://www.rust-lang.org/";
    let code = QrCode::new(data).unwrap();
    let image = code.render::<Luma<u8>>().build();
    image.save("qrcode.png").unwrap();
    println!("QR code generated");
}
