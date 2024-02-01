use std::{collections::HashMap, sync::Arc};

use can_config_rs::config;

use crate::cnl::{
    can_adapter::{timestamped::Timestamped, TCanFrame},
    deserialize::{type_deserializer::TypeDeserializer, FrameDeserializer},
    errors::{Error, Result},
    frame::{Frame, TFrame, Value},
    network::{object_entry_object::ObjectEntryObject, NetworkObject},
};

struct GetRespFrame {
    sof: bool,
    eof: bool,
    toggle: bool,
    client_id: u8,
    server_id: u8,
    object_entry_id: u16,
    data: u32,
}

impl GetRespFrame {
    pub fn new(frame: &Frame) -> Self {
        let Some(header) = frame.attribute("header") else {
            panic!("DETECTED INVALID CONFIG: invalid format of get_resp_frame");
        };
        let Some(Value::UnsignedValue(sof)) = header.attribute("sof") else {
            panic!("DETECTED INVALID CONFIG: invalid format of get_resp_frame");
        };
        let Some(Value::UnsignedValue(eof)) = header.attribute("eof") else {
            panic!("DETECTED INVALID CONFIG: invalid format of get_resp_frame");
        };
        let Some(Value::UnsignedValue(toggle)) = header.attribute("toggle") else {
            panic!("DETECTED INVALID CONFIG: invalid format of get_resp_frame");
        };
        let Some(Value::UnsignedValue(object_entry_id)) = header.attribute("od_index") else {
            panic!("DETECTED INVALID CONFIG: invalid format of get_resp_frame");
        };
        let Some(Value::UnsignedValue(client_id)) = header.attribute("client_id") else {
            panic!("DETECTED INVALID CONFIG: invalid format of get_resp_frame");
        };
        let Some(Value::UnsignedValue(server_id)) = header.attribute("server_id") else {
            panic!("DETECTED INVALID CONFIG: invalid format of get_resp_frame");
        };
        let Some(Value::UnsignedValue(data)) = header.attribute("data") else {
            panic!("DETECTED INVALID CONFIG: invalid format of get_resp_frame");
        };
        Self {
            sof: *sof != 0,
            eof: *eof != 0,
            toggle: *toggle != 0,
            client_id: *client_id as u8,
            server_id: *server_id as u8,
            object_entry_id: *object_entry_id as u16,
            data : *data as u32,
        }
    }
}

enum GetRespState {
    Ready,
    // NOTE expecting toggle low on the next frame!
    FragmentationToggleLow,
    FragmentationToggleHigh,
}

struct GetResp {
    state: GetRespState,
    object_entry: Arc<ObjectEntryObject>,
    type_deserializer: TypeDeserializer,
    size: u32,
    buffer: Vec<u32>,
}

impl GetResp {
    fn receive(&mut self, frame: GetRespFrame, timestamp : &std::time::Instant) -> Result<()> {
        let (expected_sof, expected_toggle) = match &self.state {
            GetRespState::Ready => (true, false),
            GetRespState::FragmentationToggleLow => (false, false),
            GetRespState::FragmentationToggleHigh => (false, true),
        };
        let expected_eof = (self.buffer.len() + 1) as u32 == self.size;

        if expected_sof != frame.sof {
            return Err(Error::InvalidGetResponseSofFlag);
        }
        if expected_toggle != frame.toggle {
            return Err(Error::InvalidGetResponseToggleFlag);
        }
        if expected_eof != frame.eof {
            return Err(Error::InvalidGetResponseEofFlag);
        }

        assert_eq!(frame.object_entry_id, self.object_entry.id() as u16);
        self.buffer.push(frame.data);

        if frame.eof {
            let value = self.type_deserializer
                .deserialize(unsafe { std::mem::transmute(self.buffer.as_slice()) });
            self.object_entry.push_value(value, timestamp);
            self.state = GetRespState::Ready;
        }else {
            // update fragmentation state!
            self.state = match self.state {
                GetRespState::Ready => GetRespState::FragmentationToggleHigh,
                GetRespState::FragmentationToggleLow => GetRespState::FragmentationToggleHigh,
                GetRespState::FragmentationToggleHigh => GetRespState::FragmentationToggleLow,
            }
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Hash)]
struct GetRespIdentifier {
    server_id: u8,
    object_entry_id: u16,
}

pub struct GetRespFrameHandler {
    frame_deserializer: FrameDeserializer,
    get_resp_lookup: HashMap<GetRespIdentifier, tokio::sync::Mutex<GetResp>>,
    get_resp_msg: config::MessageRef,
}

impl GetRespFrameHandler {
    pub fn create(network: &Arc<NetworkObject>, get_resp_msg: &config::MessageRef) -> Self {
        let frame_deserializer = FrameDeserializer::new(get_resp_msg);
        let mut get_resp_lookup = HashMap::new();
        for node in network.nodes() {
            let node_id = node.id() as u8;
            for object_entry in node.object_entries() {
                get_resp_lookup.insert(
                    GetRespIdentifier {
                        server_id: node_id,
                        object_entry_id: object_entry.id() as u16,
                    },
                    tokio::sync::Mutex::new(GetResp {
                        object_entry: object_entry.clone(),
                        buffer: vec![],
                        state: GetRespState::Ready,
                        size: object_entry.ty().size(),
                        type_deserializer: TypeDeserializer::new(object_entry.ty(), 32),
                    }),
                );
            }
        }
        Self {
            frame_deserializer: FrameDeserializer::new(get_resp_msg),
            get_resp_lookup,
            get_resp_msg: get_resp_msg.clone(),
        }
    }

    // gets invoked in rx.rs -> fn can_receiver(..).
    // for each frame a lookup is done to get the correct handler afterwards.
    // This handler is only invoked for the get resp message of the config therefor the
    // format can be assumed to be the same for every frame!
    pub async fn handle(&self, can_frame: &TCanFrame) -> Result<TFrame> {
        // a small example of how to parse the type frame!
        let frame = self
            .frame_deserializer
            .deserialize(can_frame.get_data_u64());

        let get_resp_frame = GetRespFrame::new(&frame);

        let get_resp_identifier = GetRespIdentifier {
            server_id: get_resp_frame.server_id,
            object_entry_id: get_resp_frame.object_entry_id,
        };

        // lookup
        let Some(get_resp) = self.get_resp_lookup.get(&get_resp_identifier) else {
            return Err(Error::InvalidGetResponseServerOrObjectEntryNotFound);
        };

        get_resp.lock().await.receive(get_resp_frame, can_frame.timestamp())?;

        Ok(Timestamped::new(can_frame.timestamp().clone(), frame))
    }
}
