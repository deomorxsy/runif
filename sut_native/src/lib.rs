use rustler::{Env, Error, NifStruct, Resource, ResourceArc};
use tokio::runtime::Runtime;


#[derive(NifStruct)]
#[module = ""]

struct Handle {
    //pub sensor: sensor:Sensor,
}

#[rustler::resource_impl]
impl Resource for Handle {}

#[rustler::nif]
fn init() -> Result<Handle, Error> {
    let rt = Runtime::new().unwrap();
    match rt.block_on(isensor::SensorManager::init(none)){
        Ok(sensor) => Ok(ResourceArc::new(Handle { sensor })),
        Err(_err) => Err(Error::Term(Box::new("Failed to load sensor")))

    }
}

rustler::init!("Elixir.AirSensor.Native");


use rustler::{Env, Error, NifStruct, Resource, ResourceArc};
use tokio::runtime::Runtime;


#[derive(NifStruct)]
#[module = ""]

struct Handle {
    //pub sensor: sensor:Sensor,
}

#[rustler::resource_impl]
impl Resource for Handle {}

#[rustler::nif]
fn init() -> Result<Handle, Error> {
    let rt = Runtime::new().unwrap();
    match rt.block_on(isensor::SensorManager::init(none)){
        Ok(sensor) => Ok(ResourceArc::new(Handle { sensor })),
        Err(_err) => Err(Error::Term(Box::new("Failed to load sensor")))

    }
}

rustler::init!("Elixir.AirSensor.Native");

