pub const VERSION: u8 = 0x05;
pub const NO_AUTH: u8 = 0x00;
pub const SUCCESS: u8 = 0x00;
pub const RESERVED: u8 = 0x00;
pub const CONNECT: u8 = 0x01;

pub const V4: u8 = 0x01;
pub const DOMAIN: u8 = 0x03;
pub const V6: u8 = 0x04;

pub const BIND: u8 = 0x02;
pub const UDP: u8 = 0x3;

#[derive(Debug, Clone, Copy)]
pub enum Atyp {
    V4 = 0x01,
    DOMAIN = 0x03,
    V6 = 0x04,
}

impl From<u8> for Atyp {
    fn from(value: u8) -> Self {
        match value {
            0x01 => Atyp::V4,
            0x03 => Atyp::DOMAIN,
            0x04 => Atyp::V6,
            _ => panic!("unsupport address type"),
        }
    }
}

impl From<Atyp> for u8 {
    fn from(value: Atyp) -> Self {
        match value {
            Atyp::V4 => V4,
            Atyp::DOMAIN => DOMAIN,
            Atyp::V6 => V6,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Cmd {
    CONNECT = 0x01,
    BIND = 0x02,
    UDP = 0x3,
}

impl From<Cmd> for u8 {
    fn from(value: Cmd) -> Self {
        match value {
            Cmd::CONNECT => CONNECT,
            Cmd::BIND => BIND,
            Cmd::UDP => UDP,
        }
    }
}

#[derive(Debug)]
pub struct Rsp {
    ver: u8,
    rep: u8,
    rsv: u8,
    atyp: u8,
    baddr: Vec<u8>,
    bport: u16,
}

impl Rsp {
    pub fn new(rep: u8, atyp: u8) -> Self {
        Self {
            ver: VERSION,
            rep,
            rsv: RESERVED,
            atyp,
            baddr: vec![0, 0, 0, 0],
            bport: 0,
        }
    }

    pub fn to_bytes(self) -> [u8; 10] {
        let ports: [u8; 2] = self.bport.to_be_bytes();
        let rsp = [
            self.ver,
            self.rep,
            self.rsv,
            self.atyp,
            self.baddr[0],
            self.baddr[1],
            self.baddr[2],
            self.baddr[3],
            ports[0],
            ports[1],
        ];
        rsp
    }
}
