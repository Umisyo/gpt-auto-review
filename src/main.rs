use std::fs;
use std::env;
use chatgpt::types::CompletionResponse;
use clap::Parser;
use chatgpt::prelude::*;

#[derive(Parser)]
struct Args {
    file: String,
}

async fn get_gpt_response(content: String) -> Result<String> {
    let gpt_api_key = env::var("GPT_API_KEY").expect("KEY is not defined");
    let client = ChatGPT::new(gpt_api_key)?;
    let message = format!("Here is the information from the GitHub pull request. Please see this and write a review in markdown format.: {}", content);
    let response: CompletionResponse = client.send_message(message).await?;
    Ok((response.message().content).to_string())
}


#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let content = fs::read_to_string(args.file)?;

    let response = get_gpt_response(content).await?;

    println!("{}", response);
    Ok(())
}