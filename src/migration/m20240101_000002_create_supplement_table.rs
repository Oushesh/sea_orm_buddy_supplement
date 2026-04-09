use sea_orm_migration::prelude::*;

/// Migration that creates the `supplement` table.
pub struct Migration;

/// Column identifiers for the `supplement` table.
#[derive(DeriveIden)]
enum Supplement {
    Table,
    Id,
    Name,
    Dose,
    Description,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240101_000002_create_supplement_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Apply the migration: create the `supplement` table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Supplement::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Supplement::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Supplement::Name).string().not_null())
                    .col(ColumnDef::new(Supplement::Dose).string().not_null())
                    .col(ColumnDef::new(Supplement::Description).string().null())
                    .to_owned(),
            )
            .await
    }

    /// Revert the migration: drop the `supplement` table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Supplement::Table).to_owned())
            .await
    }
}
