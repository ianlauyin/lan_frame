#[macro_export]
macro_rules! migrator {
    ($migration_folder:literal) => {
        use lan_be_frame::sea_orm_migration::*;

        pub struct Migrator;
        #[async_trait::async_trait]
        impl MigratorTrait for Migrator {
            fn migrations() -> Vec<Box<dyn MigrationTrait>> {
                vec![Box::new(Migration::Up)]
            }
        }
    };
}
