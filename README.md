<div id="top"></div>

<p align="center">
    <a href="https://github.com/libninjacom/twilio-rs/stargazers">
        <img src="https://img.shields.io/github/stars/libninjacom/twilio-rs.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/libninjacom/twilio-rs/actions">
        <img src="https://img.shields.io/github/workflow/status/libninjacom/twilio-rs/ci?style=flat-square" alt="Build Status" />
    </a>
    
<a href="https://crates.io/crates/twilio">
    <img src="https://img.shields.io/crates/d/twilio?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/twilio">
    <img src="https://img.shields.io/crates/v/twilio?style=flat-square" alt="Crates.io" />
</a>

</p>

Twilio client, generated from the OpenAPI spec.

# Usage

```rust
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let response = client
        .list_account()
        .friendly_name("your friendly name")
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .status("your status")
        .await
        .unwrap();
    println!("{:#?}", response);
}
```

This example loads configuration from environment variables, specifically:



# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
twilio = "0.1.0"
```


# Documentation



* [Client Library Documentation](https://docs.rs/twilio)


You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*