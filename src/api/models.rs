// Define RISE API Request, Response model

pub struct Account {

}

pub struct Block {

}

pub struct Transaction {

}

pub struct Delegate {

}

pub struct Loader {

}

pub struct Peer {
    pub ip: String,
    pub port: u32,
    pub state : u8,
    pub os: String,
    pub version: String,
    pub broadhash: String,
    pub height: u64,
    pub clock: u64
    pub updated: u64,
    pub nonce: String,
}

pub struct Signature {

}

pub struct Multisignature {

}
