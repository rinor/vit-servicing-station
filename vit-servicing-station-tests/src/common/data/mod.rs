use vit_servicing_station_lib::db::models::{
    api_tokens::APITokenData, funds::Fund, proposals::Proposal, voteplans::Voteplan,
};

mod generator;
pub use generator::{Generator, Snapshot};

pub fn token() -> (APITokenData, String) {
    Generator::new().token()
}

pub fn proposals() -> Vec<Proposal> {
    let mut gen = Generator::new();
    let funds = gen.funds();
    gen.proposals(&funds)
}

pub fn funds() -> Vec<Fund> {
    Generator::new().funds()
}

pub fn voteplans() -> Vec<Voteplan> {
    let mut gen = Generator::new();
    let funds = gen.funds();
    gen.voteplans(&funds)
}
