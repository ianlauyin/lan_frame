// use mysql::Params;

// use super::super::{PartialRow, Row, Table};

// pub struct InsertQuery<'a, T: Table, D: PartialRow<Row = T::Row>> {
//     table: &'a T,
//     updated_datas: Vec<D>,
// }

// impl<'a, T: Table, D: PartialRow<Row = T::Row>> InsertQuery<'a, T, D> {
//     pub fn single(table: &'a T, updated_data: D) -> Self {
//         Self {
//             table,
//             updated_datas: vec![updated_data],
//         }
//     }

//     pub fn stmt(&self) -> String {
//         let table_name = self.table.name();
//         let fields = D::Row::fields().join(", ");
//         format!("INSERT INTO {table_name} ({fields})")
//     }

//     pub fn params(self) -> impl FnMut() -> Params {
//         self.updated_datas.into_iter().map(D::params_mapper())
//     }
// }
