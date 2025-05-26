use mysql::{Error, Pool, PooledConn, Row, prelude::Queryable};

// TODO: Create a marco for Table (take name and Data field)
pub trait Table {
    type Data;
    fn name(&self) -> &'static str;
    fn row_mapper(&self) -> impl FnMut(Row) -> Self::Data;
}

pub struct Repository<'a, T: Table> {
    table: &'a T,
    conn: PooledConn,
}

impl<'a, T: Table> Repository<'a, T> {
    /// Get the repository for the given table.
    pub fn get(pool: Pool, table: &'a T) -> Result<Self, Error> {
        let conn = pool.get_conn()?;
        Ok(Self { table, conn })
    }

    /// Query the database with a raw SQL query.
    pub fn raw_query(&mut self, query: &str) -> Result<Vec<T::Data>, Error> {
        let result = self.conn.query_map(query, self.table.row_mapper())?;
        Ok(result)
    }
}
