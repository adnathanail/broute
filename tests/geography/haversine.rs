use broute::geography::algorithms::haversine;
use broute::geography::datastructures::LatLng;

#[test]
fn haversine_test() {
    assert_eq!(
        haversine(
            LatLng {
                latitude: 51.34,
                longitude: -1.34
            },
            LatLng {
                latitude: 51.24,
                longitude: -1.74
            }
        ),
        29.955746002468217
    );
}
