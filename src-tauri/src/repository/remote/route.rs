use mongodb::{Collection, Database};
use futures::stream::StreamExt;
use mongodb::bson::{doc, to_document};
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
    pub async fn add_route(&self, route: Route) -> Result<(), String> {
        self.coll.insert_one(route)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
    
    // fetch_routes fetches all routes from the database
    pub async fn fetch_routes(&self) -> Result<Vec<Route>, String> {
        let mut cursor = self.coll.find(doc! {})
            .await
            .map_err(|e| e.to_string())?;

        let mut routes = Vec::new();

        while let Some(route) = cursor.next().await {
            routes.push(route.map_err(|e| e.to_string())?);
        }

        Ok(routes)
    }

    // update_route updates a route in the database
    pub async fn update_route(&self, route: Route) -> Result<(), String> {
        self.coll.update_one(doc! {"_id": route.id}, to_document(&route).unwrap())
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}