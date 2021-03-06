use crate::db::{models::proposals::Proposal, queries::proposals as proposals_queries};
use crate::v0::{context::SharedContext, errors::HandleError};

pub async fn get_all_proposals(context: SharedContext) -> Result<Vec<Proposal>, HandleError> {
    let pool = &context.read().await.db_connection_pool;
    proposals_queries::query_all_proposals(&pool).await
}

pub async fn get_proposal(id: i32, context: SharedContext) -> Result<Proposal, HandleError> {
    let pool = &context.read().await.db_connection_pool;
    proposals_queries::query_proposal_by_id(id, &pool).await
}
