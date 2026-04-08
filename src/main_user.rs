use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ConnectionTrait,
    Database, EntityTrait, Schema
};

mod user;
use user::Entity as User; // This is the 'Gateway' mentioned before

#[tokio::main]
async fn main() {
    // 1. Connect
    let db = Database::connect("sqlite::memory:").await.unwrap();

    // 2. Setup Schema (This builds the 'users' table)
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let create_table_stmt = schema.create_table_from_entity(User);

    // Now that ConnectionTrait is imported, execute() will work
    db.execute(builder.build(&create_table_stmt))
        .await
        .unwrap();

    println!("Database initialized and table created.");

    // 3. Define User 1
    let user1 = user::ActiveModel {
        name: Set("Alice".to_owned()),
        ..Default::default()
    };
    let user1_result = user1.insert(&db).await.unwrap();
    println!("Inserted User 1: {:?}", user1_result);

    // 4. Define and Insert User 2
    let user2 = user::ActiveModel {
        name: Set("Bob".to_owned()),
        ..Default::default()
    };

    user2.insert(&db).await.unwrap();

    // 5. Query all data from the databank.
    let all_users = User::find().all(&db).await.unwrap();
    println!("--- All Users in Database ---");
    for u in all_users {
        println!("ID: {}, Name: {}", u.id, u.name);
    }
}

