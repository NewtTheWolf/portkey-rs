# Portkey Rust SDK

The `portkey` crate is a lightweight wrapper around `async-openai` designed to provide a Portkey-compatible OpenAI client. It simplifies the process of configuring custom clients to work seamlessly with the Portkey AI API.

## Features

- Easy initialization with API and virtual keys.
- Fully compatible with `async-openai`.
- Pre-configured headers for Portkey's API requirements.

### Future Plans

Currently, this SDK serves as a **simple wrapper around `async-openai`**, making it Portkey-compatible. However, the goal is to expand it in the future to fully support the **entire Portkey API**, offering advanced functionality for developers.

## Installation

Install the crate using Cargo:

```bash
cargo add portkey
```

## Usage

Here's how to use the `portkey` client:

```rust
use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
    },
    Models,
};
use portkey::Client as PortkeyClient;

#[tokio::main]
async fn main() {
    let api_key = "your-portkey-api-key";
    let virtual_key = "your-portkey-virtual-key";

    let client = PortkeyClient::new(&api_key, &virtual_key);
    let openai = client.openai();

    let mut messages: Vec<ChatCompletionRequestMessage> = Vec::new();

    messages.push(
        ChatCompletionRequestSystemMessageArgs::default()
            .content("Hello, Portkey!".to_string())
            .build()
            .unwrap()
            .into(),
    );

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o".to_string())
        .messages(messages)
        .build()
        .unwrap();

    let res = openai.chat().create(request).await.unwrap();

    println!("{:?}", res.choices);
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
