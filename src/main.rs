use firebase_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

#[tokio::main]
async fn main() {
    // Create the user
    let user = User {
        name: "Paolo García".to_string(),
        age: 25,
        email: "carlos@test.com".to_string(),
    };

    // Create the Firebase Instance
    let firebase = Firebase::new("https://myproject.firebaseio.com").unwrap();

    // Create the user
    let response = set_user(&firebase, &user).await;
    // Get the user
    let mut user = get_user(&firebase, &response.name).await;
    println!("{:?}", user);

    // Update the user
    user.email = "updated.mail@gmail.com".to_string();
    let updated_user = update_user(&firebase, &response.name, &user).await;
    println!("{:?}", updated_user);

    // Delete the user
    delete_user(&firebase, &response.name).await;
    println!("User deleted");
}

// Create a user
async fn set_user(firebase_client: &Firebase, user: &User) -> Response {
    let firebase = firebase_client.at("users");
    let _users = firebase.set::<User>(&user).await;
    return string_to_reponse(&_users.unwrap().data);
}

// Get a user
async fn get_user(firebase_client: &Firebase, id: &String) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let user = firebase.get::<User>().await;
    return user.unwrap();
}

// Update a user
async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let _user = firebase.update::<User>(&user).await;
    return string_to_user(&_user.unwrap().data);
}

async fn delete_user(firebase_client: &Firebase, id: &String) {
    let firebase = firebase_client.at("users").at(&id);
    let _result = firebase.delete().await;
}

// Convert a string to a Response
fn string_to_reponse(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

// Convert a string to a User
fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}
