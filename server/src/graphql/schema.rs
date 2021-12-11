use juniper::{EmptyMutation, EmptySubscription, RootNode, FieldResult, GraphQLObject};
use crate::context::GraphQLContext;

#[derive(GraphQLObject)]
//#[graphql(description = "User preferences for content", name = "UserPreferences")]
struct Person {
    name: String,
    age: i32,
}

pub struct QueryRoot;

#[juniper::graphql_object(Context = GraphQLContext)]
impl QueryRoot {
    async fn person(_context: &GraphQLContext) -> FieldResult<Person> {
        Ok(Person {
            name: "Reidar".to_string(),
            age: 51,
        })
    }
}

type Schema = RootNode<'static, QueryRoot, EmptyMutation<GraphQLContext>, EmptySubscription<GraphQLContext>>;

pub fn schema() -> Schema {
    Schema::new(
        QueryRoot,
        EmptyMutation::<GraphQLContext>::new(),
        EmptySubscription::<GraphQLContext>::new(),
    )
}