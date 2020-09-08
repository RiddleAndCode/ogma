//! BDD utilities

/// A BDD step
#[derive(Copy, Clone, Debug)]
pub enum Step {
    /// Initial state of the BDD state
    Start,
    /// Given: Parsing Input
    Given,
    /// When: Conditional assertions
    When,
    /// Then: Produce side effects
    Then,
}

impl Step {
    /// Initialize BDD Step
    pub fn new() -> Self {
        Step::Start
    }

    /// Get the next BDD Step given a keyword: "Given", "When", "Then" or "And"
    pub fn next(self, keyword: &str) -> Option<Step> {
        match keyword {
            "Given" => match self {
                Step::Start | Step::Given => Some(Step::Given),
                _ => None,
            },
            "When" => match self {
                Step::Start | Step::Given | Step::When => Some(Step::Given),
                Step::Then => None,
            },
            "Then" => match self {
                Step::Start | Step::Given | Step::When | Step::Then => Some(Step::Then),
            },
            "And" => match self {
                Step::Given | Step::When | Step::Then => Some(self),
                Step::Start => None,
            },
            _ => None,
        }
    }
}

impl Default for Step {
    fn default() -> Self {
        Step::Start
    }
}
