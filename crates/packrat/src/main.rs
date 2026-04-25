//! Composition root: the only place `infrastructure` and the `packrat_application` layer are wired.

use packrat_application::{create_item, get_item};
use packrat_domain::item::{ItemId, ItemName};
use packrat_infrastructure::{
    connect_pool, run_migrations, PostgresItemCommand, StubItemQuery,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let database_url = std::env::var("DATABASE_URL")?;

    let pool = connect_pool(&database_url).await?;
    run_migrations(&pool).await?;

    let item_command = PostgresItemCommand::new(pool);
    let created = create_item(&item_command, ItemName::from("from use case")).await;
    println!("#{:?}: {:?}", created.id, created.name);

    let item_query = StubItemQuery;
    if let Some(item) = get_item(&item_query, ItemId::new(1)) {
        println!("#{:?}: {:?}", item.id, item.name)
    }

    Ok(())
}
