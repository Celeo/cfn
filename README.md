# cfn

"Collaborative Flight Network"

Programs, scripts, configurations, and instructions to enable users to stream MSFS 2020 flight data to a central server and populate their games with other players' information. Think [VATSIM](vatsim.net/), but a for fun, open source, no-attempt-to-actually-launch version.

Again, this is _not_ an attempt to supplant VATSIM. I have no intention of running such a community. None of the code in this project is copied from VATSIM. I have no official affiliation with VATSIM, VATUSA, etc.

## Tech

- [Rust](https://www.rust-lang.org/) programming language
  - [msfs-rs library](https://github.com/flybywiresim/msfs-rs) to interface with [SimConnect](https://docs.flightsimulator.com/html/Programming_Tools/SimConnect/SimConnect_SDK.htm) to get data from the simulator
  - [axum](https://crates.io/crates/axum) web server library
- [Redis](https://redis.io/) for fast in-memory data storage
- [SurrealDB](surrealdb.com/) for long-term data storage

### Components

1. A program that runs on the user's Windows computer
    - Streams data from their simulator to the server
    - Receives streamed data from the server
    - Must run on Windows
1. A web service that handles data streams, incoming and outgoing
    - Uses redis to store the data across threads/processes
    - Persists interval-sampled data to longer-term storage
    - Must run on Linux
1. A website to view data about connected clients
    - Must run in the browser (obviously)

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Please feel free to contribute. Please open an issue first (or comment on an existing one) so that I know that you want to add/change something.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
