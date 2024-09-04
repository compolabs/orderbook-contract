library;

pub struct UserVolume {
    epoch: u64,
    volume: u64,
}

impl UserVolume {
    pub fn new() -> Self {
        Self {
            epoch: 0,
            volume: 0,
        }
    }

    pub fn get(self, protocol_epoch: u64) -> u64 {
        if self.epoch < protocol_epoch {
            0
        } else {
            self.volume
        }
    }

    pub fn update(ref mut self, protocol_epoch: u64, volume: u64) -> Self {
        if self.epoch < protocol_epoch {
            self.epoch = protocol_epoch;
            self.volume = volume;
        } else {
            self.volume += volume;
        }
        self
    }
}
