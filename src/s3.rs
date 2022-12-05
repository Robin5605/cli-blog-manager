use std::{path::Path, process};

use aws_sdk_s3::{Client, types::ByteStream};

pub async fn list_posts(client: &Client) -> Result<Vec<String>, aws_sdk_s3::Error> {
    let res = client.list_objects_v2().bucket("robinj-blog-articles").send().await?;

    let keys = Vec::from(res.contents().unwrap_or_default())
        .iter()
        .map(|obj| obj.key().unwrap_or_default())
        .map(String::from).collect();
    Ok(keys)
}

pub async fn delete_post(client: &Client, key: &String) -> Result<(), aws_sdk_s3::Error> {
    client
        .delete_object()
        .bucket("robinj-blog-articles")
        .key(key)
        .send()
        .await?;

    Ok(())
}

pub async fn publish_post(client: &Client, path: &Path) -> Result<(), aws_sdk_s3::Error> {
    let key = path.file_name().unwrap().to_str().unwrap();
    let body = ByteStream::from_path(path).await;
    match body {
        Ok(b) => {
            client.put_object()
                .bucket("robinj-blog-articles")
                .key(key)
                .body(b)
                .send()
                .await?;
        },
        Err(e) => {
            println!("Failed uploading object");
            println!("{}", e);
            process::exit(1);
        }
    }

    Ok(())
}