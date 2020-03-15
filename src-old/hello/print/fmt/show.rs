use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // 緯度
    lat: f32,
    // 経度
    lon: f32,
}

impl Display for City {
    // `f` はバッファです。このメソッドは
    // ここにフォーマットされた文字列を書き込みます。
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!`は`format!`に似ていますが、フォーマットされた文字列を
        // バッファ(第一引数)に書き込みます。
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // fmt::Displayに実装を追加したら、 {} を使用するように変更してください。
        println!("{:?}", *color)
    }
}
