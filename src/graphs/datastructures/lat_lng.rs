#[derive(Debug, Copy, Clone)]
pub struct LatLng {
    pub latitude: f64,
    pub longitude: f64,
}

impl LatLng {
    pub fn latitude_radians(self) -> f64 {
        self.latitude.to_radians()
    }
    pub fn longitude_radians(self) -> f64 {
        self.longitude.to_radians()
    }
    pub fn to_lat_lng_tuple(self) -> (f64, f64) {
        (self.latitude, self.longitude)
    }
}
