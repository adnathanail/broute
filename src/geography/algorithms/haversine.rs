use crate::geography::datastructures::LatLng;

/// Find the distance between two `LatLng`s using the Haversine formula
pub fn haversine(start_point: LatLng, end_point: LatLng) -> f64 {
    let earth_radius = 6371.0;

    let delta_latitude_radians = (end_point.latitude - start_point.latitude).to_radians();
    let delta_longitude_radians = (end_point.longitude - start_point.longitude).to_radians();

    let a = (delta_latitude_radians / 2.0).sin().powi(2)
        + start_point.latitude_radians().cos()
            * end_point.latitude_radians().cos()
            * (delta_longitude_radians / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    earth_radius * c
}
