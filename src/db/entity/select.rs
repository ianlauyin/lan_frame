use super::{MultipleResult, SingleResult};

pub struct Select<T: sea_orm::EntityTrait> {
    pub(in crate::db) inner: sea_orm::Select<T>,
}

impl<T: sea_orm::EntityTrait> Select<T> {
    pub async fn one(self) -> SingleResult<T> {
        self.inner.one(&Self::db().await).await
    }

    pub async fn all(self) -> MultipleResult<T> {
        self.inner.all(&Self::db().await).await
    }

    pub async fn sql_one(self, stmt: sea_orm::Statement) -> SingleResult<T> {
        self.raw_selector(stmt).one(&Self::db().await).await
    }

    pub async fn sql_all(self, stmt: sea_orm::Statement) -> MultipleResult<T> {
        self.raw_selector(stmt).all(&Self::db().await).await
    }

    fn raw_selector(
        self,
        stmt: sea_orm::Statement,
    ) -> sea_orm::SelectorRaw<sea_orm::SelectModel<T::Model>> {
        self.inner.from_raw_sql(stmt)
    }

    async fn db() -> sea_orm::DatabaseConnection {
        crate::db::get_db().await
    }
}
