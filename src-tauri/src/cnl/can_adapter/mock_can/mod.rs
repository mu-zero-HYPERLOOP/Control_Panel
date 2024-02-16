use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use can_config_rs::config::{self};
use rand::{rngs::{ThreadRng, StdRng}, Rng, SeedableRng};

use super::{can_frame::CanFrame, timestamped::Timestamped, TCanError, TCanFrame, can_error::CanError, CanAdapterInterface};

pub struct MockCanAdapter {
    network_ref: config::NetworkRef,
    rng: Mutex<StdRng>,
}

impl MockCanAdapter {
    pub fn create(_bus_config : &config::bus::BusRef, network_ref: &config::NetworkRef) -> Self {
        Self {
            network_ref: network_ref.clone(),
            rng: Mutex::new(StdRng::seed_from_u64(rand::thread_rng().gen())),
        }
    }
}

impl CanAdapterInterface for MockCanAdapter {

    async fn send(&self, frame: CanFrame) {
        println!("mock-can : sending {frame:?}");
    }

    async fn receive(&self) -> Result<TCanFrame, TCanError> {
        // await for random amount of time.
        let timeout: u64 = self
            .rng
            .lock()
            .expect("failed to acquire mock can lock")
            .gen_range(10..100);
        tokio::time::sleep(Duration::from_micros(timeout)).await;


        let mut rng = self.rng.lock().expect("failed to acquire mock can lock");
        let t: bool = rng.gen_bool(0.5);
        match t {
            false => Ok(Timestamped::now(random_get_resp(&mut rng, &self.network_ref))),
            true => Ok(Timestamped::now(random_stream_frame(&mut rng, &self.network_ref))),
        }
    }
    async fn receive_err(&self) -> TCanError {
        // await for random amount of time.
        let timeout: u64 = self
            .rng
            .lock()
            .expect("failed to acquire mock can lock")
            .gen_range(2..10);
        tokio::time::sleep(Duration::from_secs(timeout)).await;

        let error = Timestamped::now(CanError::Can(42));

        error
    }
}

fn random_get_resp(rng: &mut StdRng, network_config: &config::NetworkRef) -> CanFrame {
    let object_entries: Vec<&Arc<config::ObjectEntry>> = network_config
        .nodes()
        .iter()
        .map(|node| node.object_entries().iter())
        .flatten()
        .collect();
    let object_entry = object_entries[rng.gen_range(0..object_entries.len())];
    let server_id = object_entry.node().id();

    let msg = network_config.get_resp_message();

    let (id, ide) = match msg.id() {
        config::MessageId::StandardId(id) => (*id, false),
        config::MessageId::ExtendedId(id) => (*id, true),
    };

    // let [sof_signal,
    //     eof_signal,
    //     toggle_signal,
    //     object_entry_id_signal,
    //     client_id_signal,
    //     server_id_signal,
    //     value_signal]: &[Arc<config::signal::Signal>] = &msg.signals().as_slice() else {
    //     panic!();
    // };
    let oe_value: u32 = rng.gen();
    let sof = 1;
    let eof = 1;
    let toggle = 0;
    let object_entry_id = object_entry.id();
    let server_id = server_id;
    let client_id = network_config.nodes().len();

    let mut value = 0;
    value |= sof;
    value |= eof << 1;
    value |= toggle << 2;
    value |= (object_entry_id as u64) << 3;
    value |= (client_id as u64) << 16;
    value |= (server_id as u64) << 24;
    value |= (oe_value as u64) << 32;

    // value |= (((0xFFFFFFFFFFFFFFFF as u64 >> (64 - sof_signal.size())) & (sof))
    //     << (64 - sof_signal.size()))
    //     >> sof_signal.byte_offset();
    // value |= (((0xFFFFFFFFFFFFFFFF as u64 >> (64 - eof_signal.size())) & (eof))
    //     << (64 - eof_signal.size()))
    //     >> eof_signal.byte_offset();
    // value |= (((0xFFFFFFFFFFFFFFFF as u64 >> (64 - toggle_signal.size())) & (toggle))
    //     << (64 - toggle_signal.size()))
    //     >> toggle_signal.byte_offset();
    //
    // value |= (((0xFFFFFFFFFFFFFFFF as u64 >> (64 - object_entry_id_signal.size()))
    //     & (object_entry_id as u64))
    //     << (64 - object_entry_id_signal.size()))
    //     >> object_entry_id_signal.byte_offset();
    // value |= (((0xFFFFFFFFFFFFFFFF as u64 >> (64 - client_id_signal.size())) & (client_id as u64))
    //     >> (64 - client_id_signal.size()))
    //     << client_id_signal.byte_offset();
    // value |= (((0xFFFFFFFFFFFFFFFF as u64 >> (64 - server_id_signal.size())) & (server_id as u64))
    //     << (64 - server_id_signal.size()))
    //     >> server_id_signal.byte_offset();
    // value |= (((0xFFFFFFFFFFFFFFFF as u64 >> (64 - value_signal.size())) & (oe_value as u64))
    //     << (64 - value_signal.size()))
    //     >> value_signal.byte_offset();

    CanFrame::new(id, ide, false, msg.dlc(), value)
}

fn random_stream_frame(rng: &mut StdRng, network_config: &config::NetworkRef) -> CanFrame {
    let streams: Vec<&Arc<config::stream::Stream>> = network_config
        .nodes()
        .iter()
        .map(|node| node.tx_streams().iter())
        .flatten()
        .collect();
    let stream = streams[rng.gen_range(0..streams.len())];
    let msg = stream.message();

    let (id, ide) = match msg.id() {
        config::MessageId::StandardId(id) => (*id, false),
        config::MessageId::ExtendedId(id) => (*id, true),
    };
    let dlc = msg.dlc();

    let data = rng.gen();

    CanFrame::new(id, ide, false, dlc, data)
}

// NOTE: this is allowed only because it is only used during testing.
// For release builds no unsafe blocks unless very obviously safe
// are allowed! 
// NOTE: unsafe is required because NetworkRef doesn't 
// implement send and sync, even tho it's immutable DUH!
