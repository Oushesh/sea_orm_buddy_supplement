mod migration;
mod supplement;
mod user;

use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, Database, EntityTrait};

// Aliasing for cleaner code
use supplement::Entity as Supplement;
use user::Entity as User;

#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    // 1. Connect – uses a persistent file-based SQLite database.
    let db = Database::connect("sqlite:local.db?mode=rwc").await?;

    // 2. Run all pending migrations (idempotent: safe to call on every start).
    //    SeaORM records applied migrations in the `seaql_migrations` table so
    //    each migration is only ever executed once.
    Migrator::up(&db, None).await?;
    println!("Migrations applied successfully.");

    // 3. Insert a user.
    let alice = user::ActiveModel {
        name: Set("Alice".to_owned()),
        ..Default::default()
    };
    alice.insert(&db).await?;

    // 4. Insert a supplement.
    let vit_c = supplement::ActiveModel {
        name: Set("Vitamin C".to_owned()),
        dose: Set("1000mg".to_owned()),
        ..Default::default()
    };
    vit_c.insert(&db).await?;

    // 5. Query both tables to confirm the rows exist independently.
    let user_count = User::find().all(&db).await?.len();
    let supplement_count = Supplement::find().all(&db).await?.len();
    println!(
        "Database contains {} user(s) and {} supplement(s).",
        user_count, supplement_count
    );

    Ok(())
}