use axum::extract::State;
use mongodb::{bson::doc, Client, Collection};

use crate::structs::user::User;

use super::templates::Login;

pub async fn user_login_page(State(client): State<Client>) -> Login {
    let query = doc! {"email": "tom.sorabella@gmail.com"};
    let coll: Collection<User> = client.database("test").collection::<User>("users");

    let result = coll.find_one(query, None).await;

    return Login {
        name: result.unwrap().unwrap().username,
    };
}
