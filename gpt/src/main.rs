use chatgpt::prelude::*;

#[tokio::main]
async fn main() -> chatgpt::Result<()> {
    let token: String = std::env::var("SESSION_TOKEN").unwrap(); 
    let mut client = ChatGPT::new(token)?;
    client.refresh_token().await?;
    
    // We create a new empty conversation
    let mut conversation = client.new_conversation();
    let response: String = conversation.send_message(&client, "Write me a simple HTTP server in Rust").await?;

    // Now we can refer to our previous message when talking to ChatGPT
    let response: String = conversation.send_message(&client, "Now can you rewrite in Kotlin using the ktor framework?").await?;

    // Streamed responses are also supported
    let mut stream = conversation.send_message_streaming(&client, "Now can you rewrite it in TypeScript?").await?;

    while let Some(response) = stream.next() {
        // ...
    }

    Ok(())
}