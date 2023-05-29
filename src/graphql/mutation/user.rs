use async_graphql::{Context, InputObject, Object, Result};

use crate::{
    graphql::types::User,
    prisma::{user, PrismaClient},
};

#[derive(InputObject)]
pub struct CreateUserInput {
    pub display_name: String,
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<User> {
        let db = ctx.data::<PrismaClient>().unwrap();
        let created = db.user().create(input.display_name, vec![]).exec().await?;
        Ok(created.into())
    }

    // write a function to delete a user
    pub async fn delete_user(&self, ctx: &Context<'_>, id: String) -> Result<User> {
        let db = ctx.data::<PrismaClient>().unwrap();
        let result = db.user().delete(user::id::equals(id)).exec().await?;
        Ok(result.into())
    }
}
