use can_config_rs::config::MessageRef;

use crate::cnl::{
    can_adapter::{TCanFrame, timestamped::Timestamped},
    errors::Result,
    frame::TFrame,
    deserialize::FrameDeserializer,
};

pub struct CommandRespFrameHandler {
    frame_deserializer: FrameDeserializer,
}

impl CommandRespFrameHandler {
    pub fn create(message_config : &MessageRef) -> Self {
        Self { frame_deserializer : FrameDeserializer::new(message_config) }
    }
    pub async fn handle(&self, can_frame: &TCanFrame) -> Result<TFrame> {
        let frame = self.frame_deserializer.deserialize(can_frame.get_data_u64());

        // TODO implement handling

        Ok(Timestamped::new(can_frame.timestamp().clone(), frame))
    }
}
