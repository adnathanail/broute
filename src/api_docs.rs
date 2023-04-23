//! Documentation for consuming the API
//!
//! # General usage
//! By default the API runs on http://localhost:8000
//!
//! All URLs should be relative to this, e.g.
//! ```text
//! http://localhost:8000/shortest_path/51.52385798325193/-0.1408910751342773/51.52452551441979/-0.137822628021240/
//! ```
//!
//! # Shortest path
//! The shortest path API is available at
//! ```text
//! /shortest_path/<start_lat>/<start_lon>/<end_lat>/<end_lon>
//! ```
//! where the coordinates are in decimal degrees.
//!
//! ### Example response
//! ```json
//! {
//!     from_point: (52.00323, -1.4345),
//!     to_point: (52.02323, -1.4365),
//!     path: [(52.546734, -1.5436), ...],
//!     path_length: 15.43,
//! }
//! ```
//!
//! # Travelling salesman
//! The TSP API is available at
//! ```text
//! /route_optimisation/<points_str>
//! ```
//! Where `points_str` is a list of longitude and latitude pairs (`<lat>,<lon>`) separated by `|` characters, e.g. `52.432,-1.543|57.356,-3.425`
//!
//! ### Example response
//! ```json
//! {
//!     legs: [
//!         [(52.00323, -1.4345), ...],
//!         ...
//!     ],
//!     total_path_length: 15.43,
//! }
//! ```
