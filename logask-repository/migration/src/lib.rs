pub use sea_orm_migration::prelude::*;

mod m20221219_151218_add_table_task;
mod m20221219_151220_add_table_project;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221219_151220_add_table_project::Migration),
            Box::new(m20221219_151218_add_table_task::Migration),
        ]
    }
}
