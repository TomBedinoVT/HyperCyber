use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// User entity
pub mod user {
    use super::*;
    
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub email: String,
        pub password_hash: String,
        pub first_name: Option<String>,
        pub last_name: Option<String>,
        pub is_active: bool,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "super::user_entity::Entity")]
        UserEntities,
    }

    impl Related<super::user_entity::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::UserEntities.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

// Entity entity
pub mod entity {
    use super::*;
    
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "entities")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub name: String,
        pub description: Option<String>,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "super::user_entity::Entity")]
        UserEntities,
    }

    impl Related<super::user_entity::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::UserEntities.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

// UserEntity (junction table)
pub mod user_entity {
    use super::*;
    
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "user_entities")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub user_id: Uuid,
        pub entity_id: Uuid,
        pub role: String,
        pub created_at: DateTime<Utc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::entity::Entity",
            from = "Column::EntityId",
            to = "super::entity::Column::Id"
        )]
        Entity,
        #[sea_orm(
            belongs_to = "super::user::Entity",
            from = "Column::UserId",
            to = "super::user::Column::Id"
        )]
        User,
    }

    impl Related<super::entity::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Entity.def()
        }
    }

    impl Related<super::user::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::User.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

// RegisterEntry entity
pub mod register_entry {
    use super::*;
    
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "rgpd_register")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub entity_id: Uuid,
        pub processing_name: String,
        pub purpose: String,
        pub legal_basis: String,
        pub data_categories: Vec<String>,
        pub data_subjects: Vec<String>,
        pub recipients: Vec<String>,
        pub retention_period: Option<String>,
        pub security_measures: Option<String>,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::entity::Entity",
            from = "Column::EntityId",
            to = "super::entity::Column::Id"
        )]
        Entity,
    }

    impl Related<super::entity::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Entity.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

// AccessRequest entity
pub mod access_request {
    use super::*;
    
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "rgpd_access_requests")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub entity_id: Uuid,
        pub requester_name: String,
        pub requester_email: String,
        pub request_type: String,
        pub description: Option<String>,
        pub status: String,
        pub response: Option<String>,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
        pub completed_at: Option<DateTime<Utc>>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::entity::Entity",
            from = "Column::EntityId",
            to = "super::entity::Column::Id"
        )]
        Entity,
    }

    impl Related<super::entity::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Entity.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

// Breach entity
pub mod breach {
    use super::*;
    
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "rgpd_breaches")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub entity_id: Uuid,
        pub breach_date: DateTime<Utc>,
        pub discovery_date: DateTime<Utc>,
        pub description: String,
        pub data_categories_affected: Vec<String>,
        pub number_of_subjects: Option<i32>,
        pub severity: String,
        pub status: String,
        pub containment_measures: Option<String>,
        pub notification_date: Option<DateTime<Utc>>,
        pub authority_notified: bool,
        pub subjects_notified: bool,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::entity::Entity",
            from = "Column::EntityId",
            to = "super::entity::Column::Id"
        )]
        Entity,
    }

    impl Related<super::entity::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Entity.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

// Type aliases for easier use
pub use user::Entity as User;
pub use entity::Entity as EntityModel;
pub use user_entity::Entity as UserEntity;
pub use register_entry::Entity as RegisterEntry;
pub use access_request::Entity as AccessRequest;
pub use breach::Entity as Breach;
