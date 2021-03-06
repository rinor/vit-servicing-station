use super::vote_options;
use crate::db::models::vote_options::VoteOptions;
use crate::db::{schema::proposals, views_schema::full_proposals_info, DB};
use diesel::{ExpressionMethods, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Category {
    #[serde(alias = "categoryId", default = "Default::default")]
    pub category_id: String,
    #[serde(alias = "categoryName")]
    pub category_name: String,
    #[serde(alias = "categoryDescription", default = "Default::default")]
    pub category_description: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Proposer {
    #[serde(alias = "proposerName")]
    pub proposer_name: String,
    #[serde(alias = "proposerEmail")]
    pub proposer_email: String,
    #[serde(alias = "proposerUrl")]
    pub proposer_url: String,
    #[serde(alias = "proposerRelevantExperience")]
    pub proposer_relevant_experience: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Proposal {
    #[serde(alias = "internalId")]
    pub internal_id: i32,
    #[serde(alias = "proposalId")]
    pub proposal_id: String,
    #[serde(alias = "category")]
    pub proposal_category: Category,
    #[serde(alias = "proposalTitle")]
    pub proposal_title: String,
    #[serde(alias = "proposalSummary")]
    pub proposal_summary: String,
    #[serde(alias = "proposalProblem")]
    pub proposal_problem: String,
    #[serde(alias = "proposalSolution")]
    pub proposal_solution: String,
    #[serde(alias = "proposalPublicKey")]
    pub proposal_public_key: String,
    #[serde(alias = "proposalFunds")]
    pub proposal_funds: i64,
    #[serde(alias = "proposalUrl")]
    pub proposal_url: String,
    #[serde(alias = "proposalFilesUrl")]
    pub proposal_files_url: String,
    #[serde(alias = "proposalImpactScore")]
    pub proposal_impact_score: i64,
    pub proposer: Proposer,
    #[serde(alias = "chainProposalId")]
    #[serde(serialize_with = "crate::utils::serde::serialize_bin_as_str")]
    #[serde(deserialize_with = "crate::utils::serde::deserialize_string_as_bytes")]
    pub chain_proposal_id: Vec<u8>,
    #[serde(alias = "chainProposalIndex")]
    pub chain_proposal_index: i64,
    #[serde(alias = "chainVoteOptions")]
    pub chain_vote_options: VoteOptions,
    #[serde(alias = "chainVoteplanId")]
    pub chain_voteplan_id: String,
    #[serde(alias = "chainVoteStartTime", default = "Default::default")]
    #[serde(serialize_with = "crate::utils::serde::serialize_unix_timestamp_as_rfc3339")]
    #[serde(deserialize_with = "crate::utils::serde::deserialize_unix_timestamp_from_rfc3339")]
    pub chain_vote_start_time: i64,
    #[serde(alias = "chainVoteEndTime", default = "Default::default")]
    #[serde(serialize_with = "crate::utils::serde::serialize_unix_timestamp_as_rfc3339")]
    #[serde(deserialize_with = "crate::utils::serde::deserialize_unix_timestamp_from_rfc3339")]
    pub chain_vote_end_time: i64,
    #[serde(alias = "chainCommitteeEndTime", default = "Default::default")]
    #[serde(serialize_with = "crate::utils::serde::serialize_unix_timestamp_as_rfc3339")]
    #[serde(deserialize_with = "crate::utils::serde::deserialize_unix_timestamp_from_rfc3339")]
    pub chain_committee_end_time: i64,
    #[serde(alias = "chainVoteplanPayload")]
    pub chain_voteplan_payload: String,
    #[serde(alias = "chainVoteEncryptionKey")]
    pub chain_vote_encryption_key: String,
    #[serde(alias = "fundId")]
    pub fund_id: i32,
}

impl Queryable<full_proposals_info::SqlType, DB> for Proposal {
    // The row is the row, for now it cannot be any other type, may change when the DB schema changes
    #[allow(clippy::type_complexity)]
    type Row = (
        // 0 ->id
        i32,
        // 1 -> proposal_id
        String,
        // 2-> category_name
        String,
        // 3 -> proposal_title
        String,
        // 4 -> proposal_summary
        String,
        // 5 -> proposal_problem
        String,
        // 6 -> proposal_solution
        String,
        // 7 -> proposal_public_key
        String,
        // 8 -> proposal_funds
        i64,
        // 9 -> proposal_url
        String,
        // 10 -> proposal_files_url,
        String,
        // 11 -> proposal_impact_score
        i64,
        // 12 -> proposer_name
        String,
        // 13 -> proposer_contact
        String,
        // 14 -> proposer_url
        String,
        // 15 -> proposer_relevant_experience
        String,
        // 16 -> chain_proposal_id
        Vec<u8>,
        // 17 -> chain_proposal_index
        i64,
        // 18 -> chain_vote_options
        String,
        // 19 -> chain_voteplan_id
        String,
        // 20 -> chain_vote_starttime
        i64,
        // 21 -> chain_vote_endtime
        i64,
        // 22 -> chain_committee_end_time
        i64,
        // 23 -> chain_voteplan_payload
        String,
        // 24 -> chain_vote_encryption_key
        String,
        // 25 -> fund_id
        i32,
    );

    fn build(row: Self::Row) -> Self {
        Proposal {
            internal_id: row.0,
            proposal_id: row.1,
            proposal_category: Category {
                category_id: "".to_string(),
                category_name: row.2,
                category_description: "".to_string(),
            },
            proposal_title: row.3,
            proposal_summary: row.4,
            proposal_problem: row.5,
            proposal_solution: row.6,
            proposal_public_key: row.7,
            proposal_funds: row.8,
            proposal_url: row.9,
            proposal_files_url: row.10,
            proposal_impact_score: row.11,
            proposer: Proposer {
                proposer_name: row.12,
                proposer_email: row.13,
                proposer_url: row.14,
                proposer_relevant_experience: row.15,
            },
            chain_proposal_id: row.16,
            chain_proposal_index: row.17,
            chain_vote_options: vote_options::VoteOptions::parse_coma_separated_value(&row.18),
            chain_voteplan_id: row.19,
            chain_vote_start_time: row.20,
            chain_vote_end_time: row.21,
            chain_committee_end_time: row.22,
            chain_voteplan_payload: row.23,
            chain_vote_encryption_key: row.24,
            fund_id: row.25,
        }
    }
}

// This warning is disabled here. Values is only referenced as a type here. It should be ok not to
// split the types definitions.
#[allow(clippy::type_complexity)]
impl Insertable<proposals::table> for Proposal {
    type Values = (
        diesel::dsl::Eq<proposals::proposal_id, String>,
        diesel::dsl::Eq<proposals::proposal_category, String>,
        diesel::dsl::Eq<proposals::proposal_title, String>,
        diesel::dsl::Eq<proposals::proposal_summary, String>,
        diesel::dsl::Eq<proposals::proposal_problem, String>,
        diesel::dsl::Eq<proposals::proposal_solution, String>,
        diesel::dsl::Eq<proposals::proposal_public_key, String>,
        diesel::dsl::Eq<proposals::proposal_funds, i64>,
        diesel::dsl::Eq<proposals::proposal_url, String>,
        diesel::dsl::Eq<proposals::proposal_files_url, String>,
        diesel::dsl::Eq<proposals::proposal_impact_score, i64>,
        diesel::dsl::Eq<proposals::proposer_name, String>,
        diesel::dsl::Eq<proposals::proposer_contact, String>,
        diesel::dsl::Eq<proposals::proposer_url, String>,
        diesel::dsl::Eq<proposals::proposer_relevant_experience, String>,
        diesel::dsl::Eq<proposals::chain_proposal_id, Vec<u8>>,
        diesel::dsl::Eq<proposals::chain_proposal_index, i64>,
        diesel::dsl::Eq<proposals::chain_vote_options, String>,
        diesel::dsl::Eq<proposals::chain_voteplan_id, String>,
    );

    fn values(self) -> Self::Values {
        (
            proposals::proposal_id.eq(self.proposal_id),
            proposals::proposal_category.eq(self.proposal_category.category_name),
            proposals::proposal_title.eq(self.proposal_title),
            proposals::proposal_summary.eq(self.proposal_summary),
            proposals::proposal_problem.eq(self.proposal_problem),
            proposals::proposal_solution.eq(self.proposal_solution),
            proposals::proposal_public_key.eq(self.proposal_public_key),
            proposals::proposal_funds.eq(self.proposal_funds),
            proposals::proposal_url.eq(self.proposal_url),
            proposals::proposal_files_url.eq(self.proposal_files_url),
            proposals::proposal_impact_score.eq(self.proposal_impact_score),
            proposals::proposer_name.eq(self.proposer.proposer_name),
            proposals::proposer_contact.eq(self.proposer.proposer_email),
            proposals::proposer_url.eq(self.proposer.proposer_url),
            proposals::proposer_relevant_experience.eq(self.proposer.proposer_relevant_experience),
            proposals::chain_proposal_id.eq(self.chain_proposal_id),
            proposals::chain_proposal_index.eq(self.chain_proposal_index),
            proposals::chain_vote_options.eq(self.chain_vote_options.as_csv_string()),
            proposals::chain_voteplan_id.eq(self.chain_voteplan_id),
        )
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::db::{
        models::vote_options::VoteOptions,
        schema::{proposals, voteplans},
        DBConnectionPool,
    };
    use chrono::Utc;
    use diesel::{ExpressionMethods, RunQueryDsl};

    pub fn get_test_proposal() -> Proposal {
        Proposal {
            internal_id: 1,
            proposal_id: "1".to_string(),
            proposal_category: Category {
                category_id: "".to_string(),
                category_name: "foo_category_name".to_string(),
                category_description: "".to_string(),
            },
            proposal_title: "the proposal".to_string(),
            proposal_summary: "the proposal summary".to_string(),
            proposal_problem: "the proposal problem".to_string(),
            proposal_solution: "the proposal solution".to_string(),
            proposal_public_key: "pubkey".to_string(),
            proposal_funds: 10000,
            proposal_url: "http://foo.bar".to_string(),
            proposal_files_url: "http://foo.bar/files".to_string(),
            proposal_impact_score: 100,
            proposer: Proposer {
                proposer_name: "tester".to_string(),
                proposer_email: "tester@tester.tester".to_string(),
                proposer_url: "http://tester.tester".to_string(),
                proposer_relevant_experience: "ilumination".to_string(),
            },
            chain_proposal_id: b"foobar".to_vec(),
            chain_proposal_index: 0,
            chain_vote_options: VoteOptions::parse_coma_separated_value("b,a,r"),
            chain_voteplan_id: "voteplain_id".to_string(),
            chain_vote_start_time: Utc::now().timestamp(),
            chain_vote_end_time: Utc::now().timestamp(),
            chain_committee_end_time: Utc::now().timestamp(),
            chain_voteplan_payload: "none".to_string(),
            chain_vote_encryption_key: "none".to_string(),
            fund_id: 1,
        }
    }

    pub fn populate_db_with_proposal(proposal: &Proposal, pool: &DBConnectionPool) {
        let connection = pool.get().unwrap();

        // insert the proposal information
        let values = (
            proposals::proposal_id.eq(proposal.proposal_id.clone()),
            proposals::proposal_category.eq(proposal.proposal_category.category_name.clone()),
            proposals::proposal_title.eq(proposal.proposal_title.clone()),
            proposals::proposal_summary.eq(proposal.proposal_summary.clone()),
            proposals::proposal_problem.eq(proposal.proposal_problem.clone()),
            proposals::proposal_solution.eq(proposal.proposal_solution.clone()),
            proposals::proposal_public_key.eq(proposal.proposal_public_key.clone()),
            proposals::proposal_funds.eq(proposal.proposal_funds.clone()),
            proposals::proposal_url.eq(proposal.proposal_url.clone()),
            proposals::proposal_files_url.eq(proposal.proposal_files_url.clone()),
            proposals::proposal_impact_score.eq(proposal.proposal_impact_score),
            proposals::proposer_name.eq(proposal.proposer.proposer_name.clone()),
            proposals::proposer_contact.eq(proposal.proposer.proposer_email.clone()),
            proposals::proposer_url.eq(proposal.proposer.proposer_url.clone()),
            proposals::proposer_relevant_experience
                .eq(proposal.proposer.proposer_relevant_experience.clone()),
            proposals::chain_proposal_id.eq(proposal.chain_proposal_id.clone()),
            proposals::chain_proposal_index.eq(proposal.chain_proposal_index),
            proposals::chain_vote_options.eq(proposal.chain_vote_options.as_csv_string()),
            proposals::chain_voteplan_id.eq(proposal.chain_voteplan_id.clone()),
        );
        diesel::insert_into(proposals::table)
            .values(values)
            .execute(&connection)
            .unwrap();

        // insert the related fund voteplan information
        let voteplan_values = (
            voteplans::chain_voteplan_id.eq(proposal.chain_voteplan_id.clone()),
            voteplans::chain_vote_start_time.eq(proposal.chain_vote_start_time),
            voteplans::chain_vote_end_time.eq(proposal.chain_vote_end_time),
            voteplans::chain_committee_end_time.eq(proposal.chain_committee_end_time),
            voteplans::chain_voteplan_payload.eq(proposal.chain_voteplan_payload.clone()),
            voteplans::chain_vote_encryption_key.eq(proposal.chain_vote_encryption_key.clone()),
            voteplans::fund_id.eq(proposal.fund_id),
        );

        diesel::insert_into(voteplans::table)
            .values(voteplan_values)
            .execute(&connection)
            .unwrap();
    }
}
