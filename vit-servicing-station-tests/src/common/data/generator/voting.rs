use super::{ArbitraryGenerator, Snapshot};
use chain_impl_mockchain::certificate::VotePlan;
use chain_impl_mockchain::testing::scenario::template::VotePlanDef;
use vit_servicing_station_lib::db::models::{
    challenges::Challenge, funds::Fund, proposals::Proposal, vote_options::VoteOptions,
    voteplans::Voteplan,
};

use fake::{
    faker::company::en::{Buzzword, CatchPhase},
    faker::lorem::en::*,
    Fake,
};

pub struct ValidVotePlanParameters {
    pub vote_plan: VotePlanDef,
    pub voting_power_threshold: Option<i64>,
    pub voting_start: Option<i64>,
    pub voting_tally_start: Option<i64>,
    pub voting_tally_end: Option<i64>,
    pub next_fund_start_time: Option<i64>,
}

impl ValidVotePlanParameters {
    pub fn new(vote_plan: VotePlanDef) -> Self {
        Self {
            vote_plan,
            voting_power_threshold: Some(8000),
            voting_start: None,
            voting_tally_start: None,
            voting_tally_end: None,
            next_fund_start_time: None,
        }
    }

    pub fn set_voting_power_threshold(&mut self, voting_power_threshold: i64) {
        self.voting_power_threshold = Some(voting_power_threshold);
    }

    pub fn set_voting_start(&mut self, voting_start: i64) {
        self.voting_start = Some(voting_start);
    }

    pub fn set_voting_tally_start(&mut self, voting_tally_start: i64) {
        self.voting_tally_start = Some(voting_tally_start);
    }

    pub fn set_voting_tally_end(&mut self, voting_tally_end: i64) {
        self.voting_tally_end = Some(voting_tally_end);
    }

    pub fn set_next_fund_start_time(&mut self, next_fund_start_time: i64) {
        self.next_fund_start_time = Some(next_fund_start_time);
    }
}

pub struct ValidVotePlanGenerator {
    parameters: ValidVotePlanParameters,
}

impl ValidVotePlanGenerator {
    pub fn new(parameters: ValidVotePlanParameters) -> Self {
        Self { parameters }
    }

    fn convert_to_vote_plan(vote_plan_def: &VotePlanDef) -> VotePlan {
        vote_plan_def.clone().into()
    }

    pub fn build(self) -> Snapshot {
        let mut generator = ArbitraryGenerator::new();
        let chain_vote_plan_id = Self::convert_to_vote_plan(&self.parameters.vote_plan)
            .to_id()
            .to_string();
        let threshold = self.parameters.voting_power_threshold.unwrap();
        let voting_start = self.parameters.voting_start.unwrap();
        let voting_tally_start = self.parameters.voting_tally_start.unwrap();
        let voting_tally_end = self.parameters.voting_tally_end.unwrap();
        let next_fund_start_time = self.parameters.next_fund_start_time.unwrap();
        let fund_id = 1;

        let vote_plan = Voteplan {
            id: generator.id(),
            chain_voteplan_id: chain_vote_plan_id.clone(),
            chain_vote_start_time: voting_start,
            chain_vote_end_time: voting_tally_start,
            chain_committee_end_time: voting_tally_end,
            chain_voteplan_payload: "public".to_string(),
            chain_vote_encryption_key: "".to_string(),
            fund_id,
        };

        let challenge = Challenge {
            id: generator.id(),
            title: "up for a challenge?".to_string(),
            description: "hey hey hey it's awesome".to_string(),
            rewards_total: 100500,
            fund_id,
        };

        let fund = Fund {
            id: fund_id,
            fund_name: self.parameters.vote_plan.alias(),
            fund_goal: "How will we encourage developers and entrepreneurs to build Dapps and businesses on top of Cardano in the next 6 months?".to_string(),
            voting_power_info: format!(">{}", threshold),
            voting_power_threshold: threshold,
            rewards_info: Sentence(3..5).fake::<String>(),
            fund_start_time: voting_start,
            fund_end_time: voting_tally_end,
            next_fund_start_time,
            chain_vote_plans: vec![vote_plan.clone()],
            challenges: vec![challenge],
        };

        let mut proposals = vec![];

        for (index, proposal) in self.parameters.vote_plan.proposals().iter().enumerate() {
            let proposal_url = generator.gen_http_address();
            let proposal = Proposal {
                internal_id: index as i32,
                proposal_id: proposal.id().to_string(),
                proposal_category: generator.proposal_category(),
                proposal_title: CatchPhase().fake::<String>(),
                proposal_summary: CatchPhase().fake::<String>(),
                proposal_problem: Buzzword().fake::<String>(),
                proposal_solution: CatchPhase().fake::<String>(),
                proposal_public_key: generator.hash(),
                proposal_funds: generator.proposal_fund(),
                proposal_url: proposal_url.to_string(),
                proposal_impact_score: generator.impact_score(),
                proposal_files_url: format!("{}/files", proposal_url),
                proposer: generator.proposer(),
                chain_proposal_id: proposal.id().to_string().as_bytes().to_vec(),
                chain_proposal_index: index as i64,
                chain_vote_options: VoteOptions::parse_coma_separated_value("blank,yes,no"),
                chain_voteplan_id: chain_vote_plan_id.clone(),
                chain_vote_start_time: vote_plan.chain_vote_start_time,
                chain_vote_end_time: vote_plan.chain_vote_end_time,
                chain_committee_end_time: vote_plan.chain_committee_end_time,
                chain_voteplan_payload: vote_plan.chain_voteplan_payload.clone(),
                chain_vote_encryption_key: vote_plan.chain_vote_encryption_key.clone(),
                fund_id: fund.id,
                challenge_id: fund.challenges.first().unwrap().id,
            };

            proposals.push(proposal);
        }

        Snapshot::new(vec![fund], proposals, generator.tokens(), vec![vote_plan])
    }
}
