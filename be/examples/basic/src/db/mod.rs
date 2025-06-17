mod book;
mod user;

use lan_be_frame::db::DBConnectInfo;

pub const DB_CONNECT_INFO: DBConnectInfo = DBConnectInfo {
    url: "localhost",
    user: "postgres",
    password: "root",
    db_name: "basic_example",
};
