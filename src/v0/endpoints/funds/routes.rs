use super::handlers::*;
use crate::v0::context::SharedContext;
use warp::filters::BoxedFilter;
use warp::{Filter, Rejection, Reply};

pub async fn filter(
    root: BoxedFilter<()>,
    context: SharedContext,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let with_context = warp::any().map(move || context.clone());

    let fund = warp::path!("fund" / String)
        .and(warp::get())
        .and(with_context.clone())
        .and_then(get_fund)
        .boxed();

    root.and(fund).boxed()
}
