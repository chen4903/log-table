use log_table::{logger::Logger, table_data};

fn main() {
    let logger = Logger::new();
    
    let chain_id = "1";
    let from_token = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
    let to_token = "0x420000000000000000000000000000000000006";
    let from_amount = "1500000000000000000"; // 1.5 ETH in wei
    let from_address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
    let options = r#"{"slippage": "0.5%", "deadline": 3600}"#;

    logger.table("LiFi Routes", table_data! {
        "chainId" => chain_id,
        "fromToken" => from_token,
        "toToken" => to_token,
        "amount" => format!("{} ether", from_amount),
        "fromAddress" => from_address,
        "options" => options,
    });
}
