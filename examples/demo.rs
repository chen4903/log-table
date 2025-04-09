use log_table::{logger::Logger, table_data};

fn main() {
    let logger = Logger::new()
        .with_chain_id(1)
        .with_provider("https://mainnet.infura.io");
    
    let from_token = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
    let to_token = "0x420000000000000000000000000000000000006";
    let amount = "1500000000000000000"; // 1.5 ETH in wei
    let msg_sender = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";

    logger.table("USDT Transfer", table_data! {
        msgSender: msg_sender,
        fromToken: from_token,
        toToken: to_token,
        amount: format!("{} ether", amount)
    });
}
