/// Struct to hold a latitude and longitude
#[derive(Debug, Copy, Clone)]
pub struct LatLng {
    /// Latitude in degrees
    pub latitude: f64,
    /// Longitude in degrees
    pub longitude: f64,
}

impl LatLng {
    /// Returns the latitude in radians
    pub fn latitude_radians(self) -> f64 {
        self.latitude.to_radians()
    }
    /// Returns the longitude in radians
    pub fn longitude_radians(self) -> f64 {
        self.longitude.to_radians()
    }
    /// Returns a tuple containing latitude and longitude
    pub fn as_lat_lng_tuple(self) -> (f64, f64) {
        (self.latitude, self.longitude)
    }
}
