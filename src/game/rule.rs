use game::EntityTable;
use game::UpdateSummary;

pub enum RuleResult {
    After(Vec<UpdateSummary>),
    Instead(Vec<UpdateSummary>),
}

pub fn pass() -> RuleResult { RuleResult::After(vec![]) }
pub fn fail() -> RuleResult { RuleResult::Instead(vec![]) }

pub trait Rule {
    fn check(&self,
             summary: &UpdateSummary,
             entities: &EntityTable)
        -> RuleResult;
}

impl<F: Fn(&UpdateSummary, &EntityTable) -> RuleResult> Rule for F {
    fn check(&self,
             summary: &UpdateSummary,
             entities: &EntityTable)
        -> RuleResult
    {
        self(summary, entities)
    }
}
