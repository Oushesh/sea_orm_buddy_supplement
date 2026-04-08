mod user;
mod supplement;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ConnectionTrait,
    Database, EntityTrait, Schema
};


// Aliasing for cleaner
use user::Entity as User;
use supplement::Entity as Supplement;

#[tokio::main]
async fn main() -> Result<(),sea_orm::DbErr> {
    //1. One connection to rule them all
    //let db = Database::connect("sqlite::memory:").await?;

    // This creates a file in your project folder called "local.db"
    let db = Database::connect("sqlite:local.db?mode=rwc").await?;
    //2. Initialize both tables
    let builder = db.get_database_backend();
    let _schema = Schema::new(builder); //schema

    //Actually execute the creation commands
    let create_user_stmt = builder.build(&_schema.create_table_from_entity(User));
    db.execute(create_user_stmt).await?;

    let create_supp_stmt = builder.build(&_schema.create_table_from_entity(Supplement));
    db.execute(create_supp_stmt).await?;
    println!("Tables  'users' and 'supplement' created.");

    //3. Manipulate Users
    let alice = user::ActiveModel {
        name: Set("Alice".to_owned()),
        ..Default::default()
    };
    alice.insert(&db).await?;

    //4. Manipulate Supplements
    let vit_c = supplement::ActiveModel {
        name: Set("Vitamin C".to_owned()),
        dose: Set("1000mg".to_owned()),
        ..Default::default()
    };
    vit_c.insert(&db).await?;

    // 5. Query both to prove the exist independently
    let user_count = User::find().all(&db).await?.len();
    let supplement_count = Supplement::find().all(&db).await?.len();

    println!("Database contains {} users and {} supplements.",user_count,supplement_count);

    Ok(())
}


//#todo: assume both share the same db connection otherwise we need 2 objects.

//#todo: Migrations next.