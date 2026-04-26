use packrat_application::{create_item, get_item};
use packrat_domain::entity::EntityName;
use packrat_infrastructure::{
    PostgresItemCommand, PostgresItemQuery, connect_pool, run_migrations,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let database_url = std::env::var("DATABASE_URL")?;

    let pool = connect_pool(&database_url).await?;
    run_migrations(&pool).await?;

    let item_command = PostgresItemCommand::new(pool.clone());
    let created = create_item(&item_command, EntityName::from("from use case"), None).await;
    println!("#{:?}: {:?}", created.id, created.name);

    let item_query = PostgresItemQuery::new(pool);
    if let Some(item) = get_item(&item_query, created.id).await {
        println!("#{:?}: {:?}", item.id, item.name)
    }

    Ok(())
}
