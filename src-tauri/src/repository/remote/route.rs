use anyhow::Context;
use mongodb::{Collection, Database};
use futures::stream::StreamExt;
use mongodb::bson::{doc, to_document};
use mongodb::bson::oid::ObjectId;
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
    pub async fn add_route(&self, mut route: Route) -> anyhow::Result<(), anyhow::Error> {
        let route_exists = self.coll.find_one(doc! {
            "departure": &route.departure,
            "destination": &route.destination
        })
        .await
        .context("Failed to check if route exists")?;

        if route_exists.is_some() {
            return Err(anyhow::anyhow!("Route already exists"));
        }

        route.id = Some(ObjectId::new());

        self.coll.insert_one(route)
            .await
            .context("Failed to insert user")?;

        Ok(())
    }
    
    // fetch_routes fetches all routes from the database
    pub async fn fetch_routes(&self) -> Result<Vec<Route>, anyhow::Error> {
        let mut cursor = self.coll.find(doc! {})
            .await?;

        let mut routes = Vec::new();

        while let Some(user) = cursor.next().await {
            routes.push(user.context("Failed to get user")?);
        }

        Ok(routes)
    }

    // update_route updates a route in the database
    pub async fn update_route(&self, route: Route) -> Result<(), anyhow::Error> {
        self.coll.update_one(doc! {"_id": route.id}, to_document(&route).unwrap())
            .await
            .context("Failed to update route")?;

        Ok(())
    }
}