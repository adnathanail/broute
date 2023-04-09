use crate::geography::datastructures::LatLng;
use broute::geography::datastructures::LatLng;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

pub fn get_random_lat_lngs(num_lls: usize, rng: &mut Pcg64Mcg) -> Vec<LatLng> {
    (0..num_lls)
        .map(|_| LatLng {
            latitude: rng.gen_range(51.518898..51.526952),
            longitude: rng.gen_range(-0.151246..-0.121154),
        })
        .collect()
}
