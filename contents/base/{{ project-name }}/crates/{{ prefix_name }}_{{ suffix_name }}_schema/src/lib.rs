use async_graphql::{EmptySubscription, Object, Schema, SimpleObject};

pub type {{ PrefixName }}{{ SuffixName }}Schema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(SimpleObject, Clone)]
pub struct {{ PrefixName }} {
    pub id: String,
    pub display_name: String,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn {{ prefix_name }}(&self, _id: String) -> Option<{{ PrefixName }}> {
        // TODO: implement using ctx.data_unchecked::<T>() to access resources
        None
    }

    async fn {{ prefix_name }}s(&self) -> Vec<{{ PrefixName }}> {
        // TODO: implement
        vec![]
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_{{ prefix_name }}(&self, display_name: String) -> {{ PrefixName }} {
        // TODO: implement
        {{ PrefixName }} {
            id: String::new(),
            display_name,
        }
    }
}
