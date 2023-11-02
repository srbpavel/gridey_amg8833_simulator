#[allow(unused)]
use log::info;
#[allow(unused)]
use log::warn;
#[allow(unused)]
use log::error;

mod mqtt;

use std::time::Duration;

const DELAY: u64 = 100 * 3;
const FIXED_UUID: &str = "3aadce91aeeb4e41828802a0a5fd7bc1";

//
fn main() {
    env_logger::init();

    info!("grideye_amg8833_simulator");

    // todo(!) --> conf or arg
    
    // got client to push data in
    let (mut mqtt_client, mut mqtt_connection) = mqtt::new();

    // /* // SIMULATOR
    let machine_uuid = format!("{}", uuid::Uuid::new_v4().simple());

    let topic = mqtt::create_topic(&["simulator",
                                     // dynamic, every time new and different
                                     &machine_uuid,
    ]);
    // */

    /* // TRANSFORMATOR
    let topic = mqtt::create_topic(&["transformator",
                                     // fixed
                                     FIXED_UUID,
    ]);
    */
    
    let topic_error = mqtt::create_topic(&["error"]);
    if let Err(error) = mqtt_client
        .publish(
            topic_error.clone(),
            rumqttc::QoS::AtLeastOnce,
            false,
            format!("{} : boot",
                    topic,
            ).as_bytes(),
        ) {
            error!("mqtt_client publish error: {}",
                   format!("{:?}",
                           error,
                   ))
        };
    
    std::thread::spawn(move || loop {
        let payload = mqtt::measurement();

        /*
        info!("going to publish: {:?}\n{:?}",
              topic,
              payload,
        );
        */
        
        if let Err(error) = mqtt_client
            .publish(
                topic.clone(),
                rumqttc::QoS::AtLeastOnce,
                false,
                payload,
            ) {
                error!("mqtt_client publish error: {}",
                       format!("{:?}",
                               error,
                       ))
            };
        
        //info!("going to sleep");
        std::thread::sleep(Duration::from_millis(DELAY));
    });

    // 
    for (_index, _notification) in mqtt_connection.iter().enumerate() {
        /*
        warn!("[{}]Notification = {:?}",
              _index,
              _notification,
        );
        */
    }
}
