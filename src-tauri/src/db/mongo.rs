use anyhow::{Result, Context};
use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client, Database};

const MONGO_URL: &str = "mongodb+srv://neon:qmdwHgtheAX3gelm@titaniumcluster.yblhh.mongodb.net/\
                         ?retryWrites=true&w=majority&appName=TitaniumCluster";
const MONGO_DB: &str = "titanium";

// connect_mongo initializes the remote database
pub async fn connect_mongo() -> Result<Database, anyhow::Error> {
    // Parse MongoDB URI
    let mut client_options = ClientOptions::parse(MONGO_URL)
        .await
        .context("Failed to parse MongoDB URI")?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)
        .context("Failed to initialize MongoDB client")?;

    let db = client.database(MONGO_DB);

    db.run_command(doc! { "ping": 1 })
        .await
        .context("Failed to ping MongoDB deployment")?;

    println!("Pinged your deployment. Successfully connected to MongoDB!");
    Ok(db)
}
