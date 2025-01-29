use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Recur {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    Todo,
    Done,
    Postpone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    None,
    A,
    B,
    C,
    X,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::None => write!(f, ""),
            Priority::A => write!(f, "#A"),
            Priority::B => write!(f, "#B"),
            Priority::C => write!(f, "#C"),
            Priority::X => write!(f, "#X"),
        }
    }
}

impl std::fmt::Display for Recur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Recur::Daily => write!(f, "|Daily|"),
            Recur::Weekly => write!(f, "|Weekly|"),
            Recur::Monthly => write!(f, "|Monthly|"),
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Todo => write!(f, "Todo"),
            Status::Done => write!(f, "Done"),
            Status::Postpone => write!(f, "Postpone"),
        }
    }
}
