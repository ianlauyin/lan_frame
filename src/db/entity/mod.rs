mod select;

pub use lan_be_frame_macros::Entity;
pub use lan_be_frame_macros::condition;

use sea_orm::EntityTrait as OrmEntityTrait;
use select::Select;

type SingleResult<T> = Result<Option<<T as sea_orm::EntityTrait>::Model>, sea_orm::DbErr>;
type MultipleResult<T> = Result<Vec<<T as sea_orm::EntityTrait>::Model>, sea_orm::DbErr>;
type PrimaryKeyValue<T> =
    <<T as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType;

#[async_trait::async_trait]
pub trait EntityTrait {
    type OrmEntity: OrmEntityTrait;
    fn select() -> Select<Self::OrmEntity> {
        Select {
            inner: <Self::OrmEntity as OrmEntityTrait>::find(),
        }
    }

    async fn select_by_id<PKV>(primary_key_value: PKV) -> SingleResult<Self::OrmEntity>
    where
        PKV: Into<PrimaryKeyValue<Self::OrmEntity>> + Send,
    {
        Self::OrmEntity::find_by_id(primary_key_value)
            .one(&Self::db().await)
            .await
    }

    async fn db() -> sea_orm::DatabaseConnection {
        crate::db::get_db().await
    }
}
