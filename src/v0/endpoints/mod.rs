mod chain_data;
mod genesis;
mod graphql;

use crate::v0::context::SharedContext;

use warp::filters::BoxedFilter;
use warp::{Filter, Rejection, Reply};

pub fn filter(
    root: BoxedFilter<()>,
    context: SharedContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // mount chain-data endpoint
    let chain_data_root = warp::path!("chain-data" / ..);
    let chain_data_filter = chain_data::filter(chain_data_root.boxed(), context.clone());

    // mount genesis endpoint
    let genesis_root = warp::path!("genesis" / ..);
    let genesis_filter = genesis::filter(genesis_root.boxed(), context.clone());

    // mount graphql endpoint
    let graphql_root = warp::path!("graphql" / ..);
    // TODO: Use proper db url
    let graphql_filter = graphql::filter(graphql_root.boxed(), context, "");

    root.and(genesis_filter.or(chain_data_filter).or(graphql_filter))
        .boxed()
}
