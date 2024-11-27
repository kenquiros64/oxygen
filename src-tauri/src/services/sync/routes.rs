use anyhow::Context;
use crate::db::{mongo, polo};
use crate::repository::local::route::RoutesRepository as PoloURoutesRepository;
use crate::repository::remote::route::RoutesRepository as MongoRoutesRepository;

// sync_routes syncs the routes from mongodb to polo db
pub async fn sync_routes() -> Result<(), anyhow::Error> {
    let mongodb = mongo::connect_mongo()
        .await
        .context("Failed to connect to MongoDB")?;

    let polo = polo::connect_polo();

    let polo_repo = PoloURoutesRepository::new(&polo);
    let mongo_repo = MongoRoutesRepository::new(&mongodb);

    let routes = mongo_repo.fetch_routes().await
        .context("Failed to fetch routes from MongoDB")?;

    match polo_repo.clear().context("Failed to clear PoloDB") {
        Ok(_) => println!("PoloDB cleared successfully."),
        Err(e) => println!("Failed to clear PoloDB: {}", e),
    }

    for route in routes {
        polo_repo.add_route(route).context("Failed to add route to PoloDB")?;
    }

    println!("Route synchronization completed successfully.");
    Ok(())
}