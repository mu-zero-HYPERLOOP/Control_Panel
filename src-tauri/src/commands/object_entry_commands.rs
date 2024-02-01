use std::time::Duration;

use serde::Serialize;

use crate::cnl::frame::{Value, Attribute};
use crate::{
    cnl::network::object_entry_object::ObjectEntryEvent,
    state::cnl_state::CNLState, notification::notify_error,
};

use can_config_rs::config;
use can_config_rs::config::{SignalType, Type};

use serde_json;

#[tauri::command]
pub async fn set_object_entry_value(
    state: tauri::State<'_, CNLState>,
    node_name: String,
    object_entry_name: String,
    new_value_json: String,
) -> Result<(), ()> {
    println!(
        "view invoked set_object_entry_value({node_name}, {object_entry_name}, {new_value_json})"
    );
    let cnl = state.lock().await;

    let Some(node) = cnl.nodes().iter().find(|no| no.name() == &node_name) else {
        return Err(());
    };
    let Some(object_entry_object) = node
        .object_entries()
        .iter()
        .find(|oe| oe.name() == &object_entry_name)
    else {
        return Err(());
    };
    let oe_type = object_entry_object.ty();

    let json_value = match serde_json::from_str::<serde_json::Value>(&new_value_json) {
        Ok(v) => v,
        Err(_) => return Err(()),
    };

    fn parse_value(
        oe_type: &config::TypeRef,
        json_value: &serde_json::Value,
    ) -> Result<Value, ()> {
        match oe_type.as_ref() {
            Type::Primitive(SignalType::SignedInt { size }) => {
                if let Some(val) = json_value.as_i64() {
                    if 2i64.pow((*size - 1) as u32) > val && 2i64.pow((*size - 1) as u32) >= -val {
                        Ok(Value::SignedValue(val))
                    } else {
                        return Err(());
                    }
                } else {
                    return Err(());
                }
            }
            Type::Primitive(SignalType::UnsignedInt { size }) => {
                if let Some(val) = json_value.as_u64() {
                    if 2u64.pow(*size as u32) > val {
                        Ok(Value::UnsignedValue(val))
                    } else {
                        return Err(());
                    }
                } else {
                    return Err(());
                }
            }
            Type::Primitive(SignalType::Decimal {
                size,
                offset,
                scale,
            }) => {
                if let Some(val) = json_value.as_f64() {
                    let min = *offset;
                    let max = (0xffffffffffffffff as u64 >> (64 - size)) as f64 * scale + offset;
                    if val >= min && val <= max {
                        Ok(Value::RealValue(val))
                    } else {
                        return Err(());
                    }
                } else {
                    return Err(());
                }
            }

            Type::Struct {
                name: _,
                description: _,
                attribs,
                visibility: _,
            } => {
                if let Some(map) = json_value.as_object() {
                    let mut attributes : Vec<Attribute> = vec![];

                    for (name, attr_type) in attribs {
                        if let Some(val) = map.get(name) {
                            if let Ok(type_val) = parse_value(attr_type, val) {
                                attributes.push(Attribute::new(name, type_val));
                            } else {
                                return Err(());
                            }
                        } else {
                            return Err(());
                        }
                    }
                    Ok(Value::StructValue(attributes))
                } else {
                    return Err(());
                }
            }
            Type::Enum {
                name: _,
                description: _,
                size: _,
                entries,
                visibility: _,
            } => {
                if let Some(variant_str) = json_value.as_str() {
                    if entries.iter().any(|e| e.0 == variant_str) {
                        Ok(Value::EnumValue(variant_str.to_string()))
                        // Ok(TypeValue::Enum(oe_type.clone(), variant_str.to_string()))
                    } else {
                        return Err(());
                    }
                } else {
                    return Err(());
                }
            }
            Type::Array { len: _, ty: _ } => todo!(),
        }
    }

    let value = match parse_value(oe_type, &json_value) {
        Ok(x) => x,
        Err(_) => return Err(()),
    };

    println!("parsed value: {value:?}");
    object_entry_object.set_request(value);

    Ok(())
}

#[derive(Debug, Clone, Serialize)]
pub struct ObjectEntryListenLatestResponse {
    event_name: String,
    latest: Option<ObjectEntryEvent>,
}

#[tauri::command]
pub async fn listen_to_latest_object_entry_value(
    state: tauri::State<'_, CNLState>,
    node_name: String,
    object_entry_name: String,
) -> Result<ObjectEntryListenLatestResponse, ()> {
    println!("view invoked listen_to_latest_object_entry_value({node_name}, {object_entry_name})");
    let cnl = state.lock().await;

    let Some(node) = cnl.nodes().iter().find(|no| no.name() == &node_name) else {
        return Err(());
    };
    let Some(object_entry_object) = node
        .object_entries()
        .iter()
        .find(|oe| oe.name() == &object_entry_name)
    else {
        return Err(());
    };
    object_entry_object.listen_to_latest();

    let x = ObjectEntryListenLatestResponse {
        event_name: object_entry_object.latest_event_name().to_owned(),
        latest: object_entry_object.latest().await,
    };

    Ok(x)
}

#[tauri::command]
pub async fn unlisten_from_latest_object_entry_value(
    state: tauri::State<'_, CNLState>,
    node_name: String,
    object_entry_name: String,
) -> Result<(), ()> {
    println!(
        "view invoked unlisten_from_latest_object_entry_value({node_name}, {object_entry_name})"
    );
    let cnl = state.lock().await;

    let Some(node) = cnl.nodes().iter().find(|no| no.name() == &node_name) else {
        return Err(());
    };
    let Some(object_entry_object) = node
        .object_entries()
        .iter()
        .find(|oe| oe.name() == &object_entry_name)
    else {
        return Err(());
    };
    object_entry_object.unlisten_from_latest().await;
    Ok(())
}

#[derive(Debug, Clone, Serialize)]
pub struct ObjectEntryListenHistoryResponse {
    event_name: String,
    history: Vec<ObjectEntryEvent>,
}

#[tauri::command]
pub async fn listen_to_history_of_object_entry(
    state: tauri::State<'_, CNLState>,
    node_name: String,
    object_entry_name: String,
    frame_size: u64,
    min_interval: u64,
) -> Result<ObjectEntryListenHistoryResponse, ()> {
    println!("view invoked listen_to_history_of_object_entry({node_name}, {object_entry_name}, {frame_size})");
    let frame_size = Duration::from_millis(frame_size);
    let min_interval = Duration::from_millis(min_interval);

    let cnl = state.lock().await;

    let Some(node) = cnl.nodes().iter().find(|no| no.name() == &node_name) else {
        println!("error during listen_to_history_of_object_entry");
        return Err(());
    };
    let Some(object_entry_object) = node
        .object_entries()
        .iter()
        .find(|oe| oe.name() == &object_entry_name)
    else {
        println!("error during listen_to_history_of_object_entry");
        return Err(());
    };
    let (event_name, history) = object_entry_object
        .listen_to_history(frame_size, min_interval)
        .await;

    let x = ObjectEntryListenHistoryResponse {
        event_name,
        history,
    };

    Ok(x)
}

#[tauri::command]
pub async fn unlisten_from_history_of_object_entry(
    state: tauri::State<'_, CNLState>,
    node_name: String,
    object_entry_name: String,
    event_name: String,
) -> Result<(), ()> {
    println!("view invoked unlisten_from_history_object_entry({node_name}, {object_entry_name})");
    let cnl = state.lock().await;

    let Some(node) = cnl.nodes().iter().find(|no| no.name() == &node_name) else {
        return Err(());
    };
    let Some(object_entry_object) = node
        .object_entries()
        .iter()
        .find(|oe| oe.name() == &object_entry_name)
    else {
        return Err(());
    };
    object_entry_object.unlisten_from_history(&event_name).await;
    Ok(())
}

#[tauri::command]
pub async fn request_object_entry_value(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, CNLState>,
    node_name: String,
    object_entry_name: String,
) -> Result<(), ()> {
    println!("invoked request_object_entry_value({node_name}, {object_entry_name})");
    let cnl = state.lock().await;

    let Some(node) = cnl.nodes().iter().find(|no| no.name() == &node_name) else {
        return Err(());
    };
    let Some(_) = node
        .object_entries()
        .iter()
        .find(|oe| oe.name() == &object_entry_name)
    else {
        return Err(());
    };

    tokio::time::sleep(Duration::from_millis(1000)).await;

    notify_error(
        &app_handle,
        "Unimplemented",
        "request object entry value is not yet implemented",
    );

    Ok(())
}
