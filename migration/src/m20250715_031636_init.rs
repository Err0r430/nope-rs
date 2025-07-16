use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create() 
                    .table(Nopes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Nopes::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Nopes::Language)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Nopes::Nope)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(
                Table::drop()
                    .table(Nopes::Table)
                    .to_owned(),
            )
            .await?;
            
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Nopes {
    Table,
    Id,
    Language,
    Nope
}
