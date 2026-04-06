mod supplement;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ConnectionTrait, Database, DatabaseConnection, DbErr,
    EntityTrait, Schema,
};
use supplement::Entity as SupplementEntity;

/// Create the `supplement` table if it does not already exist.
async fn create_table(db: &DatabaseConnection) -> Result<(), DbErr> {
    let backend = db.get_database_backend();
    let schema = Schema::new(backend);
    let stmt = schema.create_table_from_entity(SupplementEntity);
    db.execute(backend.build(&stmt)).await?;
    Ok(())
}

/// Insert a new supplement record and return the saved model.
async fn add_supplement(
    db: &DatabaseConnection,
    name: impl Into<String>,
    dose: impl Into<String>,
    description: Option<String>,
) -> Result<supplement::Model, DbErr> {
    let new_supplement = supplement::ActiveModel {
        name: Set(name.into()),
        dose: Set(dose.into()),
        description: Set(description),
        ..Default::default()
    };
    let result = new_supplement.insert(db).await?;
    Ok(result)
}

/// Delete a supplement record by its primary key.
async fn delete_supplement(db: &DatabaseConnection, id: i32) -> Result<(), DbErr> {
    SupplementEntity::delete_by_id(id).exec(db).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    // Connect to an in-memory SQLite database.
    let db = Database::connect("sqlite::memory:").await?;

    // Set up the schema.
    create_table(&db).await?;

    // Add a couple of supplements.
    let vitamin_c = add_supplement(
        &db,
        "Vitamin C",
        "1000mg",
        Some("Immune support antioxidant".to_string()),
    )
    .await?;
    println!("Added: {:?}", vitamin_c);

    let magnesium = add_supplement(&db, "Magnesium", "400mg", None).await?;
    println!("Added: {:?}", magnesium);

    // Delete the first one.
    delete_supplement(&db, vitamin_c.id).await?;
    println!("Deleted supplement with id={}", vitamin_c.id);

    // Confirm what remains.
    let remaining = SupplementEntity::find().all(&db).await?;
    println!("Remaining supplements: {:?}", remaining);

    Ok(())
}
