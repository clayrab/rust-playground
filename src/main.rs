
use std::fmt::Debug;

use serde::{Serialize, Deserialize};

pub fn hash_bytes(_data: &[u8]) -> [u8 ;32] {
    [0; 32]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionProto {
    pub value: i32,
}

pub struct MiniblockProto {
    pub txprotos: Vec<TransactionProto>,
}

pub struct TransactionMeta {
    pub hash: [u8;32],
}

pub struct Miniblock {
    pub tx_meta: Vec<TransactionMeta>,
    pub proto: MiniblockProto,
}
#[derive(Clone, Debug)] 
pub struct Transaction<'a> {
    pub hash: &'a [u8;32],
    pub proto: &'a TransactionProto,
}


#[derive(Clone, Debug)] 
pub struct MempoolTransaction {
    pub hash: [u8;32],
    pub proto: TransactionProto,
}

impl Miniblock {
    pub fn new(txs: Vec<TransactionProto>) -> Self {
        Miniblock {
            tx_meta: txs.iter().map(|txproto| TransactionMeta{ hash: hash_bytes(&bincode::serialize(&txproto).unwrap())}).collect(),
            proto: MiniblockProto {txprotos: txs}
        }
    }
    pub fn get_transaction<'a>(&'a self, index: usize) -> Transaction {
        Transaction {
            hash: &self.tx_meta[index].hash,
            proto: &self.proto.txprotos[index],
        }

    }
    pub fn get_transactions<'a>(&'a self) -> Vec<Transaction> {
        self.proto.txprotos.iter().enumerate().map(|(index, txproto)| Transaction { hash: &self.tx_meta[index].hash, proto: txproto }).collect()
    }
}

impl MempoolTransaction {
    pub fn new(txproto: TransactionProto) -> Self {
        MempoolTransaction {
            hash: hash_bytes(&bincode::serialize(&txproto).unwrap()),
            proto: txproto,
        }
    }
    pub fn get_transaction<'a>(&'a self) -> Transaction {
        Transaction {
            hash: &self.hash,
            proto: &self.proto
        }
    }
}
pub fn main() {
    let txs = vec![TransactionProto{ value: 1 }, TransactionProto { value: 2 }];
    let mini_block = Miniblock::new(txs);

    let mempooltx = MempoolTransaction::new(TransactionProto { value: 4 });

    println!("tx {:?}", mini_block.get_transaction(1));
    println!("tx {:?}", mempooltx.get_transaction());

    for tx in mini_block.get_transactions() {
        println!("tx {:?}", tx);
    }
}
