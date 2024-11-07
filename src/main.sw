contract;

use std::{
    auth::msg_sender,
    bytes::Bytes,
    hash::{
        Hash,
        sha256,
    },
    storage::{
        storage_bytes::*,
        storage_map::StorageMapError,
        storage_string::*,
        storage_vec::*,
    },
};

abi VotingContract {
    #[storage(read, write)]
    fn cast_vote(option: u64) -> u64;

    #[storage(read)]
    fn has_voted() -> bool;

    #[storage(read)]
    fn get_count(option: u64) -> u64;

    #[storage(read)]
    fn is_option(option: u64) -> bool;

    #[storage(read, write)]
    fn add_option(option_id: u64);
}


storage {

    vote_counts: StorageMap<u64, u64> = StorageMap::<u64, u64> {},
    voters: StorageMap<b256, bool> = StorageMap::<b256, bool> {},

    options: StorageMap<u64, b256> = StorageMap::<u64, b256> {},
}

impl VotingContract for Contract {
    #[storage(read, write)]
    fn add_option(option_id: u64){
        storage.options.insert(option_id, msg_sender().unwrap().bits());
    }

    #[storage(read)]
    fn is_option(option: u64) -> bool {
        !storage.options.get(option).try_read().is_none()
    }

    #[storage(read, write)]
    fn cast_vote(option: u64) -> u64 {
        let user_address = msg_sender().unwrap().bits();
        let result = storage.options.get(option).try_read();
        match result {
            Some(_) => {        
                match storage.voters.try_insert(user_address, true) {
                    Err(StorageMapError::OccupiedError(_)) => {
                        1
                    },
                    Ok(_) => {
                        let mut count: u64 = storage.vote_counts.get(option).try_read().unwrap_or(0);
                        storage.vote_counts.insert(option, count + 1);
                        option
                    },
        }},
            None => {0}
        }
    }

    #[storage(read)]
    fn has_voted() -> bool {
        let user_address = msg_sender().unwrap().bits();
        match storage.voters.get(user_address).try_read() {
            Some(value) => {value},
            None => {false}
        }

    }

    #[storage(read)]
    fn get_count(option: u64) -> u64 {
        match storage.vote_counts.get(option).try_read() {
            Some(value) => {value},
            None => {0}
        }

    }
}
