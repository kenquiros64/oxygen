use anyhow::Context;
use crate::db::polo;
use crate::models::route::Route;
use crate::repository::local;

// fetch_routes fetches routes from polo db
pub fn fetch_routes() -> Result<Vec<Route>, anyhow::Error> {
    let polo = polo::connect_polo();

    let polo_repo = local::route::RoutesRepository::new(&polo);

    let routes = polo_repo
        .fetch_routes()
        .context("Failed to fetch routes from PoloDB")?;

    Ok(routes)
}
