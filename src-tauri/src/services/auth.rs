use crate::db::{mongo, polo};
use crate::models::user::User;
use bcrypt;
use crate::enums::error::ErrorType;
use crate::models::error::ErrorResponse as Error;
use crate::repository::local::user as local;
use crate::repository::remote::user as remote;

// login logs in a user
pub fn login(username: &str, password: &str) -> Result<User, Error> {
    let polo = polo::connect_polo();
    let polo_repo = local::UserRepository::new(&polo);

    // Find the user in the repository
    let user = polo_repo
        .find_user(username)
        .map_err(|e| {
            Error::new(
                ErrorType::Error,
                &format!("error while finding an user: {}", e))
        }
        )?
        .ok_or_else(|| Error::new(ErrorType::UserNotFound, "Usuario no encontrado"))?;

    let verified = bcrypt::verify(password, &user.password)
        .map_err(|e| {
            Error::new(
                ErrorType::Error,
                &format!("error while verifying credentials: {}", e))
        }
        )?;
    if !verified {
        return Err(Error::new(ErrorType::InvalidPassword, "Contraseña incorrecta"));
    }
    Ok(user)
}

// register registers a new user
pub async fn register(user: User) -> Result<(), Error> {
    let mongodb = mongo::connect_mongo()
        .await
        .map_err(|e| {
            Error::new(
                ErrorType::Error,
                &format!("error while connecting to mongodb: {}", e))
        }
        )?;
    let mongo_repo = remote::UserRepository::new(&mongodb);

    // Connect to Polo
    let polo = polo::connect_polo();
    let polo_repo = local::UserRepository::new(&polo);

    // Add user to MongoDB
    if let Err(e) = mongo_repo.add_user(user.clone()).await {
        return Err(e);
    }
    println!("User added to MongoDB successfully.");

    // Add user to Polo
    polo_repo
        .add_user(user)
        .map_err(|e| Error::new(
            ErrorType::Error,
            &format!("error while adding an user: {}", e))
        )?;

    println!("User added to Polo successfully.");
    Ok(())
}
