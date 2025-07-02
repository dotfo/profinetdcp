pub struct DCPBlock {
    pub option: u8,
    pub suboption: u8,
    pub length: u16,
    pub status: u16,
    pub payload: Vec<u8>,
}

impl DCPBlock {
    pub fn new(option: u8, suboption: u8, status: u16, payload: Vec<u8>) -> DCPBlock {
        let length = payload.len() as u16;
        DCPBlock {
            option: option,
            suboption: suboption,
            length: length,
            status: status,
            payload: payload,
        }
    }

    pub fn parse(block: &[u8]) -> DCPBlock {
        let option = block[0];
        let suboption = block[1];
        let length = u16::from_be_bytes([block[2], block[3]]);
        let status = u16::from_be_bytes([block[4], block[5]]);
        let payload_end = (6 + length - 2) as usize;
        let payload = &block[4..payload_end];
        return DCPBlock {
            option: option,
            suboption: suboption,
            status: status,
            length: length,
            payload: payload.to_vec(),
        };
    }

    pub fn compile(&self) -> Vec<u8> {
        let mut packet: Vec<u8> = Vec::new();
        let mut header: Vec<u8> = Vec::new();
        header.push(self.option);
        header.push(self.suboption);
        let paylen = self.payload.len() as u16;
        let mut pay = self.payload.clone();
        if (paylen % 2) != 0 {
            pay.push(0x00);
        }
        header.extend_from_slice(&paylen.to_be_bytes());
        header.extend_from_slice(&self.status.to_be_bytes());

        packet.extend(&header);
        packet.extend(&pay);
        return packet;
    }
}

// NEW FUNCTION for setting IP:
use crate::dcpblockrequest::DCPBlockRequest;
pub fn create_ip_parameter_block(ip: &str, netmask: &str, gateway: &str) -> DCPBlockRequest {
    let mut data = Vec::new();

    for octet in ip.split('.') {
        data.push(octet.parse::<u8>().unwrap());
    }
    for octet in netmask.split('.') {
        data.push(octet.parse::<u8>().unwrap());
    }
    for octet in gateway.split('.') {
        data.push(octet.parse::<u8>().unwrap());
    }

    DCPBlockRequest {
        option: crate::constants::OPTION_IP_PARAMETER,
        suboption: 1,
        payload: data,
    }
}

