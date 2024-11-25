use anyhow::{Result, Context};
use polodb_core::{Collection, CollectionT, Database};
use polodb_core::bson::{doc, to_document};
use crate::models::route::Route;

const ROUTE_COLLECTION: &str = "routes";

// RoutesRepository is the repository for the routes collection
pub struct RoutesRepository {
    coll: Collection<Route>
}

impl RoutesRepository{
    // new creates a new instance of the RoutesRepository
    pub fn new(db: &Database) -> Self {
        let coll = db.collection(ROUTE_COLLECTION);
        RoutesRepository { coll }
    }

    // add_route adds a new route in the database
    pub fn add_route(&self, route: Route) -> Result<()> {
        self.coll.insert_one(route).context("Failed to insert route")?;

        Ok(())
    }
    
    // fetch_routes fetches all routes from the database
    pub fn fetch_routes(&self) -> Result<Vec<Route>> {
        let mut cursor = self.coll.find(doc! {}).run()?;

        let mut routes = Vec::new();

        while let Some(route) = cursor.next() {
            routes.push(route.context("Failed to get route")?);
        }

        Ok(routes)
    }

    // update_route updates a route in the database
    pub fn update_route(&self, route: Route) -> Result<()> {
        self.coll.update_one(doc! {"_id": route.id}, to_document(&route).unwrap())
            .context("Failed to update route")?;

        Ok(())
    }
}