pub mod s3;
mod create_post;
mod utils;

use std::{path::Path};

use aws_sdk_s3::Client;
use create_post::create_post;
use clap::{Parser, Subcommand};
use s3::{publish_post, list_posts, delete_post};


#[derive(Subcommand)]
#[derive(Debug)]
enum ExecutionMode {
    /// Create a blog template file locally
    Create {
        filename: String
    },

    /// Uplod a local blog template file to S3 to reflect on site
    Publish {
        path_str: String
    },

    Delete {
        filename: String
    },

    List,
}

#[derive(Parser, Debug)]
#[command(author="Robin", version="1.0", about="CLI tool to make blog posts", long_about=None)]
struct Args {
    #[command(subcommand)]
    execution_mode: ExecutionMode,
}

#[tokio::main]
async fn main() -> Result<(), aws_sdk_s3::Error> {
    let args = Args::parse();

    let shared_config = aws_config::from_env().region("us-east-1").load().await;
    let client = Client::new(&shared_config);

    match &args.execution_mode {
        ExecutionMode::Create { filename } => {
            create_post(filename).unwrap();
            println!("Successfully created unpublished post file.");
        },
        ExecutionMode::Publish { path_str } => {
            let path = Path::new(path_str);
            publish_post(&client, path).await?;
            println!("Successfully published post to S3.");
        },
        ExecutionMode::List => {
            let posts = list_posts(&client).await?;
            println!("Current posts: ");
            println!("{}", 
                posts
                    .iter()
                    .map(|i| format!("> {}", i))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
        },
        ExecutionMode::Delete { filename } => {
            delete_post(&client, filename).await?
        }
    }

    Ok(())
}