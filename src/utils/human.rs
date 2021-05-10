pub(crate) fn human(yocto: u128) -> u128 {
    yocto / 1000000000000000000000000
}

pub(crate) fn current_position_in_epoch(start: u64, current: u64) -> u64 {
    (current - start) * 100 / crate::EPOCH_LENGTH
}
