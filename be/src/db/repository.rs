use super::Table;

pub struct Repository<T: Table> {
    table: T,
}

impl<T: Table> Repository<T> {
    pub fn new(table: T) -> Self {
        Self { table }
    }
}
