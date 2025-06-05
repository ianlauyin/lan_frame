use mysql::prelude::AsStatement;

pub struct InsertQuery<'a> {
    table_name: &'a str,
}

impl<'a> InsertQuery<'a> {
    pub fn new(table_name: &'a str) -> Self {
        Self { table_name }
    }
}

impl<'a> AsStatement for InsertQuery<'a> {
    fn as_statement<Q: mysql::prelude::Queryable>(
        &self,
        queryable: &mut Q,
    ) -> mysql::Result<std::borrow::Cow<'_, mysql::Statement>> {
        todo!()
    }
}
