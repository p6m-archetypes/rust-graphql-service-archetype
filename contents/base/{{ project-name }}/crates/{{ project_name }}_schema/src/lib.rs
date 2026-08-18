pub mod store;

use async_graphql::{Context, EmptySubscription, Object, Result, Schema, ID};

pub use store::{{ "{" }}{{ EntityName }}, Store};

pub type {{ ProjectName }}Schema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

fn store_err(err: anyhow::Error) -> async_graphql::Error {
    tracing::error!("store error: {err:#}");
    async_graphql::Error::new("internal error")
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Fetch a single {{ EntityName }} by id.
    async fn {{ entity_name }}(&self, ctx: &Context<'_>, id: ID) -> Result<Option<{{ EntityName }}>> {
        ctx.data_unchecked::<Store>().get(&id).await.map_err(store_err)
    }

    /// List all {{ EntityName }}s.
    async fn {{ entity_name }}s(&self, ctx: &Context<'_>) -> Result<Vec<{{ EntityName }}>> {
        ctx.data_unchecked::<Store>().list().await.map_err(store_err)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_{{ entity_name }}(&self, ctx: &Context<'_>, display_name: String) -> Result<{{ EntityName }}> {
        ctx.data_unchecked::<Store>()
            .create(&display_name)
            .await
            .map_err(store_err)
    }

    async fn update_{{ entity_name }}(&self, ctx: &Context<'_>, id: ID, display_name: String) -> Result<Option<{{ EntityName }}>> {
        ctx.data_unchecked::<Store>()
            .update(&id, &display_name)
            .await
            .map_err(store_err)
    }

    async fn delete_{{ entity_name }}(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        ctx.data_unchecked::<Store>().delete(&id).await.map_err(store_err)
    }
}
