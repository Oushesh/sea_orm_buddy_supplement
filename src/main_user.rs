use sea_orm::{ActiveModelTrait, ActiveValue::Set, Database, EntityTrait, ModelTrait};

mod user;

use user::*;

#[tokio::main]
async fn main() {
    // 1. Connect to the database (using SQLite in-memory for this example)
    let connection = Database::connect("sqlite::memory:").await.unwrap();

    //Migrations of Data --> syncing changes in the database.

    //2. Define User 1 (Activemodel is for changing data).
    let user1 = user::ActiveModel{
        name : Set("Alice".to_owned()),
        ..Default::default() //Handles the ID auto-increment
    };

    // 3. Insert Data
    let user1_result = user1.insert(&db).await.unwrap();
    println!("Insert result: {:?}", user1_result);

    // 4. Define and Insert user 2
    let user2 = user::ActiveModel {
        name: Set("Bob".to_owned()),
        ..Default::default()
    };

   user2.insert(&db).awat.unwrap();
    //Define User2
    //insert data into database,

    //Query all rows from database

    // 5. Query all rows
    let all_users = User::find().all(&db).await.unwrap();

    println!("--- All Users in Database ---");
    for u in all_users {
        println!("ID: {}, Name: {}", u.id, u.name);
    }
    
}