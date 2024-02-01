use can_config_rs::config;


pub struct TxCom {
    // needed to find right bus for messages
    network_ref: config::NetworkRef,

}

impl TxCom {
    pub fn create(network_ref: config::NetworkRef) -> TxCom {
        TxCom { network_ref }
    }

    pub fn send_set_request(&self, server_id: u16, oe_id: u32, val: &Vec<u64>) {
        println!("attempted send of {val:?}");

    }

}
