pub mod store;

use async_graphql::{Context, EmptySubscription, ID, Object, Result, Schema};

pub use store::{Store, {{ PrefixName }}};

pub type {{ PrefixName }}{{ SuffixName }}Schema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

fn store_err(err: anyhow::Error) -> async_graphql::Error {
    tracing::error!("store error: {err:#}");
    async_graphql::Error::new("internal error")
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Fetch a single {{ PrefixName }} by id.
    async fn {{ prefix_name }}(&self, ctx: &Context<'_>, id: ID) -> Result<Option<{{ PrefixName }}>> {
        ctx.data_unchecked::<Store>().get(&id).await.map_err(store_err)
    }

    /// List all {{ PrefixName }}s.
    async fn {{ prefix_name }}s(&self, ctx: &Context<'_>) -> Result<Vec<{{ PrefixName }}>> {
        ctx.data_unchecked::<Store>().list().await.map_err(store_err)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_{{ prefix_name }}(
        &self,
        ctx: &Context<'_>,
        display_name: String,
    ) -> Result<{{ PrefixName }}> {
        ctx.data_unchecked::<Store>()
            .create(&display_name)
            .await
            .map_err(store_err)
    }

    async fn update_{{ prefix_name }}(
        &self,
        ctx: &Context<'_>,
        id: ID,
        display_name: String,
    ) -> Result<Option<{{ PrefixName }}>> {
        ctx.data_unchecked::<Store>()
            .update(&id, &display_name)
            .await
            .map_err(store_err)
    }

    async fn delete_{{ prefix_name }}(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        ctx.data_unchecked::<Store>().delete(&id).await.map_err(store_err)
    }
}
