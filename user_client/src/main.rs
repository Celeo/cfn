use msfs::sim_connect::{
    data_definition, Period, SimConnect, SimConnectRecv, SIMCONNECT_OBJECT_ID_USER,
};

#[data_definition]
#[derive(Debug)]
struct Data {
    #[name = "INDICATED ALTITUDE"]
    #[epsilon = 0.01]
    altitude: f64,
    #[name = "AIRSPEED INDICATED"]
    #[epsilon = 0.01]
    airspeed: f64,
    #[name = "PLANE LONGITUDE"]
    #[unit = "degree longitude"]
    #[epsilon = 0.01]
    longitude: f64,
    #[name = "PLANE LATITUDE"]
    #[unit = "degree latitude"]
    #[epsilon = 0.01]
    latitude: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sim = SimConnect::open("CFN", |sim, recv| match recv {
        SimConnectRecv::SimObjectData(event) => match event.dwRequestID {
            0 => {
                println!("{:?}", event.into::<Data>(sim).unwrap());
            }
            _ => {}
        },
        _ => {}
    })?;

    sim.request_data_on_sim_object::<Data>(0, SIMCONNECT_OBJECT_ID_USER, Period::SimFrame)?;

    loop {
        sim.call_dispatch()?;
        println!("");
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
