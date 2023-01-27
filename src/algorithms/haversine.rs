pub fn haversine(start_longitude: f64, start_latitude: f64, end_longitude: f64, end_latitude: f64) -> f64 {
    let earth_radius = 6371.0;

    let start_latitude_radians = start_latitude.to_radians();
    let end_latitude_radians = end_latitude.to_radians();

    let delta_latitude_radians = (end_latitude - start_latitude).to_radians();
    let delta_longitude_radians = (end_longitude - start_longitude).to_radians();

    let a = (delta_latitude_radians / 2.0).sin().powi(2)
        + start_latitude_radians.cos()
            * end_latitude_radians.cos()
            * (delta_longitude_radians / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    earth_radius * c
}
