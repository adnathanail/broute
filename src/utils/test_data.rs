use crate::geography::datastructures::LatLng;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

/// Get a list of random lat/lngs
pub fn get_random_lat_lngs(num_lls: usize, rng: &mut Pcg64Mcg) -> Vec<LatLng> {
    (0..num_lls)
        .map(|_| LatLng {
            latitude: rng.gen_range(43.7247599..43.7519311),
            longitude: rng.gen_range(7.4090279..7.4398704),
        })
        .collect()
}
