library;

pub enum OrderChangeType {
    OrderOpened: (),
    OrderCancelled: (),
    OrderMatched: (),
}

pub struct OrderChangeInfo {
    change_type: OrderChangeType,
    block_height: u32,
    sender: Identity,
    tx_id: b256,
    amount_before: u64,
    amount_after: u64,
}

impl OrderChangeInfo {
    pub fn new(
        change_type: OrderChangeType,
        block_height: u32,
        sender: Identity,
        tx_id: b256,
        amount_before: u64,
        amount_after: u64,
    ) -> Self {
        Self {
            change_type,
            block_height,
            sender,
            tx_id,
            amount_before,
            amount_after,
        }
    }
}
