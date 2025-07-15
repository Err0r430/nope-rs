use crate::db::postgres_service::PostgresService;
use entity::nopes::{self, Entity as Nopes};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, Set, DbErr, ActiveModelTrait, IntoActiveModel};

impl PostgresService {
    pub async fn create_nope(&self, nope: nopes::Model) -> Result<nopes::Model, DbErr> {
        let nope = nope.into_active_model();
        nope.insert(&self.db).await
    }

    pub async fn get_nope_by_id(&self, id: String) -> Result<Option<nopes::Model>, DbErr> {
        Nopes::find_by_id(id).one(&self.db).await
    }

    pub async fn get_all_nopes(&self) -> Result<Vec<nopes::Model>, DbErr> {
        Nopes::find().all(&self.db).await
    }

    pub async fn update_nope(
        &self,
        id: String,
        nope: nopes::Model,
    ) -> Result<nopes::Model, DbErr> {
        let mut nope = nope.into_active_model();
        nope.id = sea_orm::ActiveValue::Unchanged(id);
        nope.update(&self.db).await
    }

    pub async fn delete_nope(&self, id: String) -> Result<(), DbErr> {
        let nope = Nopes::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(DbErr::Custom("Nope not found".to_string()))?;
        nope.into_active_model().delete(&self.db).await?;
        Ok(())
    }
}