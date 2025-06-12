macro_rules! migration {
    ($migration_folder:literal) => {
        pub struct Migrator;
        #[async_trait::async_trait]
        impl MigratorTrait for Migrator {
            fn migrations() -> Vec<Box<dyn MigrationTrait>> {}
        }
    };
}
