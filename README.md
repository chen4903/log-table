# log-table

A simple Rust library providing beautiful and colorful table logging functionality with file output support.

## Installation

Add `log-table` to your `Cargo.toml`.

```toml
log-table = "0.1"
```

## Features

* **Colored Output**: Customize the appearance of your tables with different colors for borders and content
* **File Logging**: Save table output to log files with automatic directory creation
* **Dynamic Sizing**: Tables automatically adjust to content width
* **Unicode Borders**: Beautiful box-drawing characters for clean table presentation
* **Builder Pattern**: Fluent interface for easy configuration
* **Memory Efficient**: No unnecessary allocations or copies

## Usage

```rust
use alloy::primitives::{address, U256};
use log_table::{color::Color, logger::Logger, table_data};

fn main() {
    let logger = Logger::new()
        .with_frame_color(Color::Blue)
        .with_save_log("logs/data.log");

    let from_token = address!("874eD100ad6C7006d000050099c0f00f900bC74E");
    let to_token = address!("4200000000000000000000000000000000000060");
    let amount = U256::from(1000000000000000000_u128); // 1 ETH in wei
    let msg_sender = address!("874eD100ad6C7006d000050099c0f00f900bC74E");

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
```

Output will look like:
```
┌────────────────────────────────────────────────────────┐
│                     USDT Transfer                      │
├───────────┬────────────────────────────────────────────┤
│ Key       │ Value                                      │
├───────────┼────────────────────────────────────────────┤
│ msgSender │ 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 │
├───────────┼────────────────────────────────────────────┤
│ fromToken │ 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 │
├───────────┼────────────────────────────────────────────┤
│ toToken   │ 0x4200000000000000000000000000000000000060 │
├───────────┼────────────────────────────────────────────┤
│ amount    │ 1000000000000000000                        │
└───────────┴────────────────────────────────────────────┘
```

## License

This project is licensed under the MIT License.
