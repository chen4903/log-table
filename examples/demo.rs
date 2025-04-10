use alloy::primitives::{address, U256};
use log_table::{color::Color, logger::Logger, table_data};

fn main() {
    let logger = Logger::new()
        .with_frame_color(Color::Blue)
        .with_save_log("logs/data.log");

    let from_token = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    let to_token = address!("4200000000000000000000000000000000000060");
    let amount = U256::from(1000000000000000000_u128); // 1 ETH in wei
    let msg_sender = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");

    logger.table(
        "USDT Transfer",
        table_data! {
            msgSender: msg_sender,
            fromToken: from_token,
            toToken: to_token,
            amount: amount
        },
    );
}
