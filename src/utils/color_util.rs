pub struct ColorUtil();

impl ColorUtil {
    pub fn hsv2rgb(h:f64, s:f64, v:f64)->(u8, u8, u8) {
        let mut vv = js_sys::Math::floor(v);
        let mut ss = js_sys::Math::floor(s);
        vv = vv / 100.0;
        ss = ss / 100.0;
        hsv::hsv_to_rgb(h, ss, vv)
    }

    pub fn rgb2hex(r: u8, g: u8, b: u8)->String {
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
 }

 #[test]
 fn it_works() {
     let rgb = ColorUtil::hsv2rgb(100.0, 100.0, 20.0);
     println!("fffffff");
 }