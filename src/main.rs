use bt_hci::cmd::le::*;
use bt_hci::controller::ControllerCmdSync;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_time::{Duration, Timer};
use log::*;
use trouble_host::prelude::*;

mod controller;
// mod ble_bas_peripheral;
mod parrot;
mod c1;

/// Size of L2CAP packets
const L2CAP_MTU: usize = 27;

/*
#[embassy_executor::task]
async fn run() {
    info!("Create socket");
    let socket = controller::LibBtSocket::new().unwrap();
    ble_bas_peripheral::run::<_, L2CAP_MTU>(socket).await;
    // run_advertise(socket)
    //     .await;
    
}

#[embassy_executor::task]
async fn run2() {
    loop {
        info!("tick");
        Timer::after_secs(1).await;
    }
}


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("HERE?");
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_nanos()
        .init();

    spawner.spawn(run()).unwrap();
}
*/


#[tokio::main]
async fn main() {
    parrot::run_parrot().unwrap();

    std::io::stdin().read_line(&mut String::new()).unwrap();
    /* 
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_nanos()
        .init();

    

    info!("Create socket");
    let socket = controller::LibBtSocket::new().unwrap();
    ble_bas_peripheral::run::<_, L2CAP_MTU>(socket).await;
    */
}