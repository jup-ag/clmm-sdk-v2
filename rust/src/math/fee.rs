fn ui_amount_to_amount(ui_amount: f64, decimals: u8) -> u64 {
    (ui_amount * 10_usize.pow(decimals as u32) as f64) as u64
}

pub fn ui_fee_to_lamport(ui_fee: f64) -> u64 {
    ui_amount_to_amount(ui_fee, 6)
}

pub fn ui_fee_to_lamport_4(ui_fee: f64) -> u64 {
    ui_amount_to_amount(ui_fee, 4)
}
