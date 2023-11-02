#[allow(unused)]
use crate::info;
#[allow(unused)]
use crate::warn;
#[allow(unused)]
use crate::error;

use rand::seq::SliceRandom;

use std::time::Duration;

use rumqttc::Client;
use rumqttc::MqttOptions;

const MQTT_HOST: &str = "192.168.0.103";
const MQTT_USER: &str = "";
const MQTT_PASS: &str = "";
const MQTT_PORT: u16 = 1883;
pub const MQTT_TOPIC_BASE: &str = "/grid_eye";

const MQTT_CLIENT_ID: &str = "grideye_simulator";

pub const LEN: usize = 8;
pub const POW: usize = LEN * LEN;
//const CHUNK_SIZE: usize = 4;
//const MQTT_PAYLOAD_SIZE: usize = POW * CHUNK_SIZE;

pub type Temperature = f32;
//pub type Array =  [Temperature; POW];

pub const TEMPERATURE_MIN: Temperature = 15.0;
pub const TEMPERATURE_MAX: Temperature = 35.0;
pub const TEMPERATURE_STEP: Temperature = 0.25;
pub const TEMPERATURE_ERROR_VALUE: Temperature = 85.0;

pub fn create_topic(parts: &[&str]) -> String {

    let mut path = std::path::PathBuf::new();
    path.push(MQTT_TOPIC_BASE);

    
    let topic = parts
        .iter()
        .fold(path, |topic, part|
              topic.join(part)
        );

    match topic
        .to_str() {
            Some(t) => String::from(t),
            None => {
                format!("error create_topic to_str() -> {} + {:?}",
                        MQTT_TOPIC_BASE,
                        parts,
                )
            },
        }
}

//
pub fn new() -> (rumqttc::Client, rumqttc::Connection) {
    let mqtt_uniq_id = format!("{}_{}",
                               MQTT_CLIENT_ID,
                               uuid::Uuid::new_v4().simple(),
    );
    
    let mut options = MqttOptions::new(
        mqtt_uniq_id,
        MQTT_HOST,
        MQTT_PORT,
    );
    
    options.set_credentials(MQTT_USER, MQTT_PASS);
    options.set_keep_alive(Duration::from_secs(5));
    info!("MQTT Options: {options:?}");
    
    Client::new(options.clone(), 10)
}

//
/*
let payload: [u8; 256] = [65, 174, 0, 0, 65, 196, 0, 0, 65, 198, 0, 0, 65, 208, 0, 0, 65, 210, 0, 0, 65, 186, 0, 0, 65, 182, 0, 0, 65, 186, 0, 0, 65, 176, 0, 0, 65, 194, 0, 0, 65, 206, 0, 0, 65, 208, 0, 0, 65, 210, 0, 0, 65, 200, 0, 0, 65, 190, 0, 0, 65, 196, 0, 0, 65, 178, 0, 0, 65, 188, 0, 0, 65, 202, 0, 0, 65, 200, 0, 0, 65, 198, 0, 0, 65, 192, 0, 0, 65, 208, 0, 0, 65, 214, 0, 0, 65, 178, 0, 0, 65, 182, 0, 0, 65, 184, 0, 0, 65, 190, 0, 0, 65, 196, 0, 0, 65, 212, 0, 0, 65, 216, 0, 0, 65, 210, 0, 0, 65, 184, 0, 0, 65, 184, 0, 0, 65, 186, 0, 0, 65, 188, 0, 0, 65, 184, 0, 0, 65, 184, 0, 0, 65, 194, 0, 0, 65, 190, 0, 0, 65, 190, 0, 0, 65, 190, 0, 0, 65, 188, 0, 0, 65, 188, 0, 0, 65, 186, 0, 0, 65, 178, 0, 0, 65, 184, 0, 0, 65, 182, 0, 0, 65, 202, 0, 0, 65, 198, 0, 0, 65, 188, 0, 0, 65, 186, 0, 0, 65, 180, 0, 0, 65, 188, 0, 0, 65, 180, 0, 0, 65, 184, 0, 0, 65, 202, 0, 0, 65, 200, 0, 0, 65, 186, 0, 0, 65, 190, 0, 0, 65, 184, 0, 0, 65, 188, 0, 0, 65, 182, 0, 0, 65, 180, 0, 0];
*/
pub fn measurement() -> Vec<u8> {
    let sample_data = generate_temperatures();
    let mut rng = rand::thread_rng();
    
    (0..POW)
        .flat_map(|_| {
            match sample_data.choose(&mut rng) {
                Some(t) => *t,
                None => TEMPERATURE_ERROR_VALUE
            }.to_be_bytes()
            
        })
        .collect()
}

//
fn generate_temperatures() -> Vec<Temperature> {
    let mut temperatures: Vec<Temperature> = vec!();
    let mut actual_temp = TEMPERATURE_MIN;
    
    while actual_temp <= TEMPERATURE_MAX {
        temperatures.push(actual_temp);

        actual_temp += TEMPERATURE_STEP;
    }

    temperatures
}
