use std::fmt::{self, Debug, Display};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat > 0.0 { "N" } else { "S" };
        let lon_c = if self.lon > 0.0 { "E" } else { "W" };
        write!(
            f,
            "{}:{:.3} {} {:.3} {}",
            self.name, self.lat, lat_c, self.lon, lon_c
        )
    }
}

#[test]
fn test_fmt() {
    let city = City {
        name: "beijing",
        lat: 120f32,
        lon: 80f32,
    };
    println!("{}", &city) //beijing:120.000 N 80.000 E
}
