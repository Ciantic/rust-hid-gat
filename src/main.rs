use gatt::*;

mod c1;
mod command;
mod controller;
mod event;
mod gatt;
mod packets;
mod parrot;
mod smp;

fn build_hid_db() -> AttributeDatabase {
    let mut db = AttributeDatabase::new();
    let mut handle = 1;

    // Service 1800
    let service_1800 = Service {
        handle: Handle(handle),
        uuid: Uuid::U16(0x1800),
        primary: true,
        characteristics: vec![
            Characteristic {
                declaration_handle: Handle(handle + 1),
                value_handle: Handle(handle + 2),
                uuid: Uuid::U16(0x2A00), // Device Name
                properties: CharacteristicProperties::from_bytes(&[0x02]).unwrap(),
                descriptors: vec![],
                value: b"My Pi".to_vec(),
            },
            Characteristic {
                declaration_handle: Handle(handle + 3),
                value_handle: Handle(handle + 4),
                uuid: Uuid::U16(0x2A01), // Appearance
                properties: CharacteristicProperties::from_bytes(&[0x02]).unwrap(),
                descriptors: vec![],
                value: vec![0x00, 0x00],
            },
        ],
    };
    handle += 5;

    // Service 180A
    let service_180A = Service {
        handle: Handle(handle),
        uuid: Uuid::U16(0x180A),
        primary: true,
        characteristics: vec![Characteristic {
            declaration_handle: Handle(handle + 1),
            value_handle: Handle(handle + 2),
            uuid: Uuid::U16(0x2A50), // PnP ID
            properties: CharacteristicProperties::from_bytes(&[0x02]).unwrap(),
            descriptors: vec![],
            value: vec![0x01, 0x02, 0xE5, 0x00, 0x01, 0x02, 0x03],
        }],
    };
    handle += 3;

    // Service 1812 (HID)
    let service_1812 = Service {
        handle: Handle(handle),
        uuid: Uuid::U16(0x1812),
        primary: true,
        characteristics: vec![
            Characteristic {
                declaration_handle: Handle(handle + 1),
                value_handle: Handle(handle + 2),
                uuid: Uuid::U16(0x2A4E), // Protocol Mode
                properties: CharacteristicProperties::from_bytes(&[0x06]).unwrap(),
                descriptors: vec![],
                value: vec![0x01],
            },
            Characteristic {
                declaration_handle: Handle(handle + 3),
                value_handle: Handle(handle + 4),
                uuid: Uuid::U16(0x2A4A), // HID Info
                properties: CharacteristicProperties::from_bytes(&[0x02]).unwrap(),
                descriptors: vec![],
                value: vec![0x11, 0x01, 0x00, 0x02],
            },
            Characteristic {
                declaration_handle: Handle(handle + 5),
                value_handle: Handle(handle + 6),
                uuid: Uuid::U16(0x2A4C), // HID Control Point
                properties: CharacteristicProperties::from_bytes(&[0x04]).unwrap(),
                descriptors: vec![],
                value: vec![0x00; 8],
            },
            Characteristic {
                declaration_handle: Handle(handle + 7),
                value_handle: Handle(handle + 8),
                uuid: Uuid::U16(0x2A4B), // Report Map
                properties: CharacteristicProperties::from_bytes(&[0x02]).unwrap(),
                descriptors: vec![],
                value: vec![0x00; 47],
            },
            Characteristic {
                declaration_handle: Handle(handle + 9),
                value_handle: Handle(handle + 10),
                uuid: Uuid::U16(0x2A4D), // Report
                properties: CharacteristicProperties::from_bytes(&[0x92]).unwrap(),
                descriptors: vec![],
                value: vec![0x00; 8],
            },
        ],
    };

    // Add all services and characteristics to the attribute map
    for service in vec![service_1800, service_180A, service_1812] {
        db.insert(Attribute::Service(service.clone()));
        for ch in service.characteristics {
            db.insert(Attribute::Characteristic(ch));
        }
    }

    db
}

#[tokio::main]
async fn main() {
    let db = build_hid_db();
    parrot::run_parrot(db).unwrap();

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
