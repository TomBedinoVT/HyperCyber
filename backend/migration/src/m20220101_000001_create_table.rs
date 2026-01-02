use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create users table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Users::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(Users::PasswordHash).string().not_null())
                    .col(ColumnDef::new(Users::FirstName).string())
                    .col(ColumnDef::new(Users::LastName).string())
                    .col(ColumnDef::new(Users::IsActive).boolean().not_null().default(false))
                    .col(ColumnDef::new(Users::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create entities table
        manager
            .create_table(
                Table::create()
                    .table(Entities::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Entities::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Entities::Name).string().not_null())
                    .col(ColumnDef::new(Entities::Description).string())
                    .col(ColumnDef::new(Entities::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Entities::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create user_entities table (junction table)
        manager
            .create_table(
                Table::create()
                    .table(UserEntities::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserEntities::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(UserEntities::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserEntities::EntityId).uuid().not_null())
                    .col(ColumnDef::new(UserEntities::Role).string().not_null())
                    .col(ColumnDef::new(UserEntities::CreatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_entities_user_id")
                            .from(UserEntities::Table, UserEntities::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_entities_entity_id")
                            .from(UserEntities::Table, UserEntities::EntityId)
                            .to(Entities::Table, Entities::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create rgpd_register table
        manager
            .create_table(
                Table::create()
                    .table(RgpdRegister::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RgpdRegister::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(RgpdRegister::EntityId).uuid().not_null())
                    .col(ColumnDef::new(RgpdRegister::ProcessingName).string().not_null())
                    .col(ColumnDef::new(RgpdRegister::Purpose).string().not_null())
                    .col(ColumnDef::new(RgpdRegister::LegalBasis).string().not_null())
                    .col(ColumnDef::new(RgpdRegister::DataCategories).json_binary())
                    .col(ColumnDef::new(RgpdRegister::DataSubjects).json_binary())
                    .col(ColumnDef::new(RgpdRegister::Recipients).json_binary())
                    .col(ColumnDef::new(RgpdRegister::RetentionPeriod).string())
                    .col(ColumnDef::new(RgpdRegister::SecurityMeasures).string())
                    .col(ColumnDef::new(RgpdRegister::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(RgpdRegister::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_rgpd_register_entity_id")
                            .from(RgpdRegister::Table, RgpdRegister::EntityId)
                            .to(Entities::Table, Entities::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create rgpd_access_requests table
        manager
            .create_table(
                Table::create()
                    .table(RgpdAccessRequests::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RgpdAccessRequests::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(RgpdAccessRequests::EntityId).uuid().not_null())
                    .col(ColumnDef::new(RgpdAccessRequests::RequesterName).string().not_null())
                    .col(ColumnDef::new(RgpdAccessRequests::RequesterEmail).string().not_null())
                    .col(ColumnDef::new(RgpdAccessRequests::RequestType).string().not_null())
                    .col(ColumnDef::new(RgpdAccessRequests::Description).string())
                    .col(ColumnDef::new(RgpdAccessRequests::Status).string().not_null())
                    .col(ColumnDef::new(RgpdAccessRequests::Response).string())
                    .col(ColumnDef::new(RgpdAccessRequests::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(RgpdAccessRequests::UpdatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(RgpdAccessRequests::CompletedAt).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_rgpd_access_requests_entity_id")
                            .from(RgpdAccessRequests::Table, RgpdAccessRequests::EntityId)
                            .to(Entities::Table, Entities::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create rgpd_breaches table
        manager
            .create_table(
                Table::create()
                    .table(RgpdBreaches::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RgpdBreaches::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(RgpdBreaches::EntityId).uuid().not_null())
                    .col(ColumnDef::new(RgpdBreaches::BreachDate).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(RgpdBreaches::DiscoveryDate).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(RgpdBreaches::Description).string().not_null())
                    .col(ColumnDef::new(RgpdBreaches::DataCategoriesAffected).json_binary())
                    .col(ColumnDef::new(RgpdBreaches::NumberOfSubjects).integer())
                    .col(ColumnDef::new(RgpdBreaches::Severity).string().not_null())
                    .col(ColumnDef::new(RgpdBreaches::Status).string().not_null())
                    .col(ColumnDef::new(RgpdBreaches::ContainmentMeasures).string())
                    .col(ColumnDef::new(RgpdBreaches::NotificationDate).timestamp_with_time_zone())
                    .col(ColumnDef::new(RgpdBreaches::AuthorityNotified).boolean().not_null().default(false))
                    .col(ColumnDef::new(RgpdBreaches::SubjectsNotified).boolean().not_null().default(false))
                    .col(ColumnDef::new(RgpdBreaches::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(RgpdBreaches::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_rgpd_breaches_entity_id")
                            .from(RgpdBreaches::Table, RgpdBreaches::EntityId)
                            .to(Entities::Table, Entities::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order (respecting foreign key constraints)
        manager
            .drop_table(Table::drop().table(RgpdBreaches::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(RgpdAccessRequests::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(RgpdRegister::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserEntities::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Entities::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Email,
    PasswordHash,
    FirstName,
    LastName,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Entities {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum UserEntities {
    Table,
    Id,
    UserId,
    EntityId,
    Role,
    CreatedAt,
}

#[derive(DeriveIden)]
enum RgpdRegister {
    Table,
    Id,
    EntityId,
    ProcessingName,
    Purpose,
    LegalBasis,
    DataCategories,
    DataSubjects,
    Recipients,
    RetentionPeriod,
    SecurityMeasures,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum RgpdAccessRequests {
    Table,
    Id,
    EntityId,
    RequesterName,
    RequesterEmail,
    RequestType,
    Description,
    Status,
    Response,
    CreatedAt,
    UpdatedAt,
    CompletedAt,
}

#[derive(DeriveIden)]
enum RgpdBreaches {
    Table,
    Id,
    EntityId,
    BreachDate,
    DiscoveryDate,
    Description,
    DataCategoriesAffected,
    NumberOfSubjects,
    Severity,
    Status,
    ContainmentMeasures,
    NotificationDate,
    AuthorityNotified,
    SubjectsNotified,
    CreatedAt,
    UpdatedAt,
}
