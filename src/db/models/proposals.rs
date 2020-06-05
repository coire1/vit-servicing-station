use super::vote_options;
use crate::db::models::vote_options::VoteOptions;
use crate::db::{views_schema::full_proposals_info, DB};
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub category_id: String,
    pub category_name: String,
    pub category_description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Proposer {
    pub proposer_name: String,
    pub proposer_email: String,
    pub proposer_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Proposal {
    pub internal_id: i32,
    pub proposal_id: String,
    pub proposal_category: Category,
    pub proposal_title: String,
    pub proposal_summary: String,
    pub proposal_problem: String,
    pub proposal_solution: String,
    pub proposal_public_key: String,
    pub proposal_funds: i64,
    pub proposal_url: String,
    pub proposal_files_url: String,
    pub proposer: Proposer,
    pub chain_proposal_id: String,
    pub chain_proposal_index: i64,
    pub chain_vote_options: VoteOptions,
    pub chain_voteplan_id: String,
    pub chain_voteplan_payload: String,
    pub chain_vote_start_time: String,
    pub chain_vote_end_time: String,
    pub chain_committee_end_time: String,
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
        // 11 -> proposer_name
        String,
        // 12 -> proposer_contact
        String,
        // 13 -> proposer_url
        String,
        // 14 -> chain_proposal_id
        Vec<u8>,
        // 15 -> chain_proposal_index
        i64,
        // 16 -> chain_vote_options
        String,
        // 17 -> chain_voteplan_id
        String,
        // 18 -> chain_vote_starttime
        String,
        // 29 -> chain_vote_endtime
        String,
        // 20 -> chain_committee_end_time
        String,
        // 21 -> chain_voteplan_payload
        String,
        // 22 -> fund_id
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
            proposer: Proposer {
                proposer_name: row.11,
                proposer_email: row.12,
                proposer_url: row.13,
            },
            // TODO: check, would this be invalid in any case?
            chain_proposal_id: String::from_utf8(row.14).unwrap(),
            chain_proposal_index: row.15,
            chain_vote_options: vote_options::VoteOptions::parse_coma_separated_value(&row.16),
            chain_voteplan_id: row.17,
            chain_vote_start_time: row.18,
            chain_vote_end_time: row.19,
            chain_committee_end_time: row.20,
            chain_voteplan_payload: row.21,
            fund_id: row.22,
        }
    }
}
