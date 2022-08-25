use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Exam {
    pub year: i32,
    pub term: Term,
    pub kind: Kind,
    pub source: Source,
    pub exam_url: Option<String>,
    pub solution_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Term {
    Winter,
    Spring,
    Fall,
}

impl FromStr for Term {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('w' | 'W') => Ok(Term::Winter),
            Some('s' | 'S') => Ok(Term::Spring),
            Some('f' | 'F') => Ok(Term::Fall),
            _ => Err(format!("invalid term: {s:?}")),
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Quiz,
    Test,
    Midterm,
    Final,
}

impl FromStr for Kind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Quiz" => Ok(Kind::Quiz),
            "Test" => Ok(Kind::Test),
            "Midterm" => Ok(Kind::Midterm),
            "Final" => Ok(Kind::Final),
            _ => Err(format!("invalid kind: {s:?}")),
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Source {
    // lowest to highest priority
    MathSoc,
    MathSocServices,
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::MathSoc => f.pad("MathSoc"),
            Source::MathSocServices => f.pad("MathSoc Services"),
        }
    }
}
