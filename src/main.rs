mod logger;

use std::collections::HashMap;
use logger::Logger;

fn main() {
    let logger = Logger::new();
    
    let mut data = HashMap::new();
    data.insert("chainId", "1".to_string());
    data.insert("fromToken", "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string());
    data.insert("toToken", "0x420000000000000000000000000000000000006".to_string());
    data.insert("amount", "1.5 ether".to_string());
    data.insert("fromAddress", "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string());
    data.insert("options", r#"{"slippage": "0.5%", "deadline": 3600}"#.to_string());

    logger.table("Processing Missed Event", data);
}
