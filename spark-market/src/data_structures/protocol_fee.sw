library;

use std::storage::storage_vec::*;

pub struct ProtocolFee {
    pub maker_fee: u64,
    pub taker_fee: u64,
    pub volume_threshold: u64,
}

impl ProtocolFee {
    fn new() -> Self {
        Self {
            maker_fee: 0,
            taker_fee: 0,
            volume_threshold: 0,
        }
    }
}

impl Vec<ProtocolFee> {
    pub fn is_volume_threshold_valid(self) -> bool {
        let mut iter = self.iter();
        let mut item = iter.next();
        let mut prev = 0u64;
        let mut valid = true;

        while item.is_some() {
            let volume_threshold = item.unwrap().volume_threshold;
            if prev < volume_threshold
                || (prev == 0
                && volume_threshold == 0)
            {
                prev = volume_threshold;
                item = iter.next();
            } else {
                valid = false;
                break;
            }
        }
        valid
    }
}

impl StorageKey<StorageVec<ProtocolFee>> {
    #[storage(read)]
    pub fn get_volume_protocol_fee(self, volume: u64) -> ProtocolFee {
        let len = self.len();
        if len == 0 {
            return ProtocolFee::new();
        }
        let mut index = 0;
        if volume > 0 {
            let mut left = 0;
            let mut right = self.len() - 1;
            while left <= right {
                let mid = left + (right - left) / 2;
                if self.get(mid).unwrap().read().volume_threshold <= volume
                {
                    index = mid;
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        self.get(index).unwrap().read()
    }
}
