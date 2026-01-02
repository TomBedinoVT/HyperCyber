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

        // Create catalogue_endpoints table
        manager
            .create_table(
                Table::create()
                    .table(CatalogueEndpoints::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CatalogueEndpoints::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(CatalogueEndpoints::Name).string().not_null())
                    .col(ColumnDef::new(CatalogueEndpoints::EndpointType).string().not_null())
                    .col(ColumnDef::new(CatalogueEndpoints::Description).string())
                    .col(ColumnDef::new(CatalogueEndpoints::Address).string())
                    .col(ColumnDef::new(CatalogueEndpoints::Metadata).json_binary())
                    .col(ColumnDef::new(CatalogueEndpoints::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(CatalogueEndpoints::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create catalogue_license_keys table
        manager
            .create_table(
                Table::create()
                    .table(CatalogueLicenseKeys::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CatalogueLicenseKeys::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(CatalogueLicenseKeys::Name).string().not_null())
                    .col(ColumnDef::new(CatalogueLicenseKeys::LicenseType).string().not_null())
                    .col(ColumnDef::new(CatalogueLicenseKeys::KeyValue).string())
                    .col(ColumnDef::new(CatalogueLicenseKeys::FilePath).string())
                    .col(ColumnDef::new(CatalogueLicenseKeys::FileName).string())
                    .col(ColumnDef::new(CatalogueLicenseKeys::FileSize).big_integer())
                    .col(ColumnDef::new(CatalogueLicenseKeys::StorageType).string().not_null())
                    .col(ColumnDef::new(CatalogueLicenseKeys::Description).string())
                    .col(ColumnDef::new(CatalogueLicenseKeys::ExpiresAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(CatalogueLicenseKeys::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(CatalogueLicenseKeys::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create catalogue_software_versions table
        manager
            .create_table(
                Table::create()
                    .table(CatalogueSoftwareVersions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CatalogueSoftwareVersions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(CatalogueSoftwareVersions::Name).string().not_null())
                    .col(ColumnDef::new(CatalogueSoftwareVersions::Version).string().not_null())
                    .col(ColumnDef::new(CatalogueSoftwareVersions::Description).string())
                    .col(ColumnDef::new(CatalogueSoftwareVersions::ReleaseDate).timestamp_with_time_zone())
                    .col(ColumnDef::new(CatalogueSoftwareVersions::EndOfLife).timestamp_with_time_zone())
                    .col(ColumnDef::new(CatalogueSoftwareVersions::Metadata).json_binary())
                    .col(ColumnDef::new(CatalogueSoftwareVersions::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(CatalogueSoftwareVersions::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create catalogue_encryption_algorithms table
        manager
            .create_table(
                Table::create()
                    .table(CatalogueEncryptionAlgorithms::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CatalogueEncryptionAlgorithms::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(CatalogueEncryptionAlgorithms::Name).string().not_null())
                    .col(ColumnDef::new(CatalogueEncryptionAlgorithms::AlgorithmType).string().not_null())
                    .col(ColumnDef::new(CatalogueEncryptionAlgorithms::KeySize).integer())
                    .col(ColumnDef::new(CatalogueEncryptionAlgorithms::Description).string())
                    .col(ColumnDef::new(CatalogueEncryptionAlgorithms::Standard).string())
                    .col(ColumnDef::new(CatalogueEncryptionAlgorithms::Metadata).json_binary())
                    .col(ColumnDef::new(CatalogueEncryptionAlgorithms::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(CatalogueEncryptionAlgorithms::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create catalogue_relations table
        manager
            .create_table(
                Table::create()
                    .table(CatalogueRelations::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CatalogueRelations::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(CatalogueRelations::SourceType).string().not_null())
                    .col(ColumnDef::new(CatalogueRelations::SourceId).uuid().not_null())
                    .col(ColumnDef::new(CatalogueRelations::TargetType).string().not_null())
                    .col(ColumnDef::new(CatalogueRelations::TargetId).uuid().not_null())
                    .col(ColumnDef::new(CatalogueRelations::RelationType).string().not_null())
                    .col(ColumnDef::new(CatalogueRelations::Description).string())
                    .col(ColumnDef::new(CatalogueRelations::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order (respecting foreign key constraints)
        manager
            .drop_table(Table::drop().table(CatalogueRelations::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(CatalogueEncryptionAlgorithms::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(CatalogueSoftwareVersions::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(CatalogueLicenseKeys::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(CatalogueEndpoints::Table).to_owned())
            .await?;

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

#[derive(DeriveIden)]
enum CatalogueEndpoints {
    Table,
    Id,
    Name,
    EndpointType,
    Description,
    Address,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum CatalogueLicenseKeys {
    Table,
    Id,
    Name,
    LicenseType,
    KeyValue,
    FilePath,
    FileName,
    FileSize,
    StorageType,
    Description,
    ExpiresAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum CatalogueSoftwareVersions {
    Table,
    Id,
    Name,
    Version,
    Description,
    ReleaseDate,
    EndOfLife,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum CatalogueEncryptionAlgorithms {
    Table,
    Id,
    Name,
    AlgorithmType,
    KeySize,
    Description,
    Standard,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum CatalogueRelations {
    Table,
    Id,
    SourceType,
    SourceId,
    TargetType,
    TargetId,
    RelationType,
    Description,
    CreatedAt,
}
