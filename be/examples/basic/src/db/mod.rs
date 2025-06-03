mod table;

pub use table::users::UserTable;

use lan_be_frame::db::DBConnectInfo;
pub const DB_CONNECT_INFO: DBConnectInfo = DBConnectInfo {
    url: "localhost",
    user: "root",
    password: "root",
    schema_name: "basic_example",
};
