// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mushroom_types::{MushroomEntry, MushroomTypes, MushroomPath};
use network_table_handler::{NetworkTableHandler, NetworkTableHandlerId, SubscriptionPackage};
use network_tables::v4::SubscriptionOptions;
use std::cell::RefCell;
use std::collections::HashMap;
use std::net::Ipv4Addr;

use crate::mushroom_types::MushroomTable;

pub mod mushroom_types;
mod network_table_handler;

thread_local! {

    static THREAD_POOL: RefCell<Option<tokio::runtime::Runtime>> = RefCell::new(
    Some(tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap()));

    static NETWORK_CLIENT_MAP: RefCell<HashMap<NetworkTableHandlerId, NetworkTableHandler>> = RefCell::new(HashMap::new());
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_network_table_handler,
            stop_network_table_handler,
            does_network_table_handler_exist,
            subscribe_to_topic,
            set_boolean_topic,
            set_float_topic,
            set_double_topic,
            set_string_topic,
            set_int_topic,
            set_boolean_array_topic,
            set_float_array_topic,
            set_double_array_topic,
            set_string_array_topic,
            set_int_array_topic,
            get_subbed_entries_values,
            get_handler_timestamp,
            get_subbed_entry_value,
            close
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn close() {
    tracing::warn!("Closing");
    THREAD_POOL.with(|pool| (pool.replace(None)).unwrap().shutdown_background());
    NETWORK_CLIENT_MAP.with(|map| map.borrow_mut().clear());
    std::process::exit(0);
}

/**
* Starts the network table handler
*
* address The IP address of the network table server as an array of 4 bytes
* in typescript pass in an array of 4 numbers
*
* port The port of the network table server as a 16-bit unsigned integer
* in typescript pass in a number
*/
#[tauri::command]
fn start_network_table_handler(
    address: [u8; 4],
    port: u16,
    identity: String,
) -> NetworkTableHandlerId {
    let ip = Ipv4Addr::from(address);
    let id = NetworkTableHandlerId::new(ip, port, identity.clone());

    if let Some(handler) = NETWORK_CLIENT_MAP.with(|map| map.borrow_mut().remove(&id)) {
        tracing::info!("Stopping network table handler for {}", id);
        handler.stop();
    }

    tracing::info!("Starting network table handler for {}", id);
    let handler = network_table_handler::nt4(ip, port, identity).unwrap();

    NETWORK_CLIENT_MAP.with(|map| {
        map.borrow_mut().insert(id.clone(), handler);
    });

    return id;
}

#[tauri::command]
fn does_network_table_handler_exist(handler_id: NetworkTableHandlerId) -> bool {
    NETWORK_CLIENT_MAP.with(|map| map.borrow().contains_key(&handler_id))
}

#[tauri::command]
fn stop_network_table_handler(handler_id: NetworkTableHandlerId) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().remove(&handler_id) {
            tracing::info!("Stopping network table handler for {}", handler_id);
            handler.stop();
        } else {
            tracing::warn!("No network table handler found for {}", handler_id);
        }
    });
}

#[tauri::command]
fn subscribe_to_topic(
    handler_id: NetworkTableHandlerId,
    topic: String,
    periodic: Option<f64>,
    all: Option<bool>,
    prefix: Option<bool>,
) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let data = SubscriptionPackage::new(
                topic.clone(),
                SubscriptionOptions {
                    all,
                    prefix,
                    periodic,
                    ..Default::default()
                },
            );
            handler.subscribe(vec![data]);
            tracing::info!("Subscribed to topic {}", topic);
        } else {
            tracing::warn!("No network table handler found for {}", handler_id);
        }
    });
}

#[tauri::command]
fn set_boolean_topic(handler_id: NetworkTableHandlerId, topic: String, value: bool) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry = MushroomEntry::new(
                MushroomTypes::Boolean(value), topic.clone().into(), None);
            handler.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set boolean topic {} to {}", topic, value);
        } else {
            tracing::warn!("No network table handler found for {}", handler_id);
        }
    });
}

#[tauri::command]
fn set_float_topic(handler_id: NetworkTableHandlerId, topic: String, value: f64) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry = MushroomEntry::new(
                MushroomTypes::Float(value), topic.clone().into(), None);
            handler.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set float topic {} to {}", topic, value);
        } else {
            tracing::warn!("No network table handler found for {}", handler_id);
        }
    });
}

#[tauri::command]
fn set_double_topic(handler_id: NetworkTableHandlerId, topic: String, value: f64) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry = MushroomEntry::new(
                MushroomTypes::Double(value), topic.clone().into(), None);
            handler.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set double topic {} to {}", topic, value);
        } else {
            tracing::warn!("No network table handler found for {}", handler_id);
        }
    });
}

#[tauri::command]
fn set_string_topic(handler_id: NetworkTableHandlerId, topic: String, value: String) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry = MushroomEntry::new(
                MushroomTypes::String(value.clone()), topic.clone().into(), None);
            handler.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set string topic {} to {}", topic, value);
        } else {
            tracing::warn!("No network table handler found for {}", handler_id);
        }
    });
}

#[tauri::command]
fn set_int_topic(handler_id: NetworkTableHandlerId, topic: String, value: i64) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry = MushroomEntry::new(
                MushroomTypes::Int(value), topic.clone().into(), None);
            handler.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set int topic {} to {} for {}", topic, value, handler_id);
        } else {
            tracing::warn!("No network table handler found for {}", handler_id);
        }
    });
}

#[tauri::command]
fn set_boolean_array_topic(handler_id: NetworkTableHandlerId, topic: String, value: Vec<bool>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry = MushroomEntry::new(
                MushroomTypes::BooleanArray(value.clone()), topic.clone().into(), None);
            handler.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set boolean array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table handler found for {}", handler_id);
        }
    });
}

#[tauri::command]
fn set_float_array_topic(handler_id: NetworkTableHandlerId, topic: String, value: Vec<f64>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry = MushroomEntry::new(
                MushroomTypes::FloatArray(value.clone()), topic.clone().into(), None);
            handler.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set float array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table handler found for {}", handler_id);
        }
    });
}

#[tauri::command]
fn set_double_array_topic(handler_id: NetworkTableHandlerId, topic: String, value: Vec<f64>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry = MushroomEntry::new(
                MushroomTypes::DoubleArray(value.clone()), topic.clone().into(), None);
            handler.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set double array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table handler found for {}", handler_id);
        }
    });
}

#[tauri::command]
fn set_string_array_topic(handler_id: NetworkTableHandlerId, topic: String, value: Vec<String>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry = MushroomEntry::new(
                MushroomTypes::StringArray(value.clone()), topic.clone().into(), None);
            handler.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set string array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table handler found for {}", handler_id);
        }
    });
}

#[tauri::command]
fn set_int_array_topic(handler_id: NetworkTableHandlerId, topic: String, value: Vec<i64>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry =
                MushroomEntry::new(MushroomTypes::IntArray(value.clone()), topic.clone().into(), None);
            handler.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set int array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table handler found for {}", handler_id);
        }
    });
}

#[tauri::command]
fn get_subbed_entries_values(handler_id: NetworkTableHandlerId) -> MushroomTable {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            tracing::info!("Getting subbed entries values for {}", handler_id);
            handler.poll()
        } else {
            tracing::warn!("No network table handler found for {}", handler_id);
            MushroomTable::new(0)
        }
    })
}

#[tauri::command]
fn get_subbed_entry_value(handler_id: NetworkTableHandlerId, path: MushroomPath) -> Option<MushroomEntry> {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            tracing::info!("Getting subbed entry value for {}", handler_id);
            handler.poll().get_entry(&path)
        } else {
            tracing::warn!("No network table handler found for {}", handler_id);
            None
        }
    })
}

#[tauri::command]
fn get_handler_timestamp(handler_id: NetworkTableHandlerId) -> f64 {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            tracing::info!("Getting handler timestamp for {}", handler_id);
            handler.poll().get_timestamp() as f64 / 1000000_f64
        } else {
            tracing::warn!("No network table handler found for {}", handler_id);
            0_f64
        }
    })
}