use cloud_storage::{Bucket, Object};
use tokio::fs;
use tokio_compat_02::FutureExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const BUCKET_NAME: &str = "neocallisto";
    const FILENAME: &str = "Cargo.toml";

    // Connecting to an existing bucket in Google Cloud Storage
    let bucket = Bucket::read(BUCKET_NAME).compat().await?;

    println!("{:#?}", bucket);

    // Read a file from disk and store it on googles server
    let contents = fs::read(FILENAME).await?;

    Object::create(BUCKET_NAME, contents, FILENAME, "text/plain")
        .compat()
        .await?;

    println!(
        "Successfully uploaded {} to Google Cloud Service!",
        FILENAME
    );

    Ok(())
}
