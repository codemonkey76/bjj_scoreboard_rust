#![allow(unused)]

use std::fmt::{Display, Formatter};
use std::time::Instant;
use crate::prelude::*;

mod error;
mod prelude;
mod utils;


use fake::Dummy;
use fake::faker::company::raw::CompanyName;
use fake::faker::name::raw::{Name, FirstName, LastName};
use fake::locales::EN;

#[derive(Debug, Clone, Dummy)]
pub struct Team {
    #[dummy(faker = "CompanyName(EN)")]
    name: String,

    #[dummy(default)]
    logo: String,
}

impl Team {
    fn new(name: &str) -> Self {
        Team {
            name: name.to_string(),
            logo: String::new()
        }
    }
}


#[derive(Debug, Dummy)]
pub struct Competitor {
    #[dummy(faker = "FirstName(EN)")]
    pub first_name: String,

    #[dummy(faker = "LastName(EN)")]
    pub last_name: String,

    pub team: Team,

    #[dummy(default)]
    pub flag: String
}

impl Competitor {
    fn new(first_name: &str, last_name: &str, team: &Team) -> Self {
        Competitor {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            team: team.clone(),
            flag: String::new()
        }
    }
    fn name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}


pub struct MatchTimer {
    running: bool,
    last_running_at: Option<Instant>,
    duration_seconds: u32,
    elapsed_milliseconds: u128
}

impl Default for MatchTimer {
    fn default() -> Self {
        MatchTimer {
            running: false,
            last_running_at: None,
            duration_seconds: 300,
            elapsed_milliseconds: 0
        }
    }
}

impl Display for MatchTimer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.remaining())
    }
}

impl MatchTimer {
    fn running_for(&self) -> u128 {
        if let Some(last_running) = self.last_running_at {
            return last_running.elapsed().as_millis()
        }
        0
    }
    fn remaining(&self) -> u128 {
        (self.duration_seconds as u128 * 1000).saturating_sub(self.elapsed_milliseconds / 1000).saturating_sub(self.running_for())
    }

    fn start(&mut self) {
        self.running = true;
        self.last_running_at = Some(Instant::now());
    }

    fn stop(&mut self) {
        self.running = false;

        if let Some(last_running) = self.last_running_at {
            self.elapsed_milliseconds += last_running.elapsed().as_millis();
            self.last_running_at = None;
        }
    }

    fn is_complete(&self) -> bool {
        self.remaining() == 0
    }
}

#[derive(Default)]
pub struct CompetitorScore {
    points: u8,
    advantages: u8,
    penalties: u8,
    medical: u8
}

impl Display for CompetitorScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pts: {} - Adv: {} - Pen: {}", self.points, self.advantages, self.penalties)
    }
}
enum WinMethod {
    Submission,
    Points,
    RefDecision,
    Disqualification,
    WalkOver,
    DoctorStoppage
}

#[derive(Default)]
pub struct MatchScore {
    competitor_one: CompetitorScore,
    competitor_two: CompetitorScore,
    winner: Option<(CompetitorNumber, WinMethod)>
}
impl MatchScore {

    fn add(&mut self, points: Points, competitor: CompetitorNumber) {
        let competitor = match competitor {
            CompetitorNumber::One => &mut self.competitor_one,
            CompetitorNumber::Two => &mut self.competitor_two
        };
        match points {
            Points::Points(amount) => competitor.points += amount,
            Points::Advantages(amount) => competitor.advantages += amount,
            Points::Penalties(amount) => competitor.penalties += amount,
            Points::Medical(amount) => competitor.medical += amount
        }
    }

    fn subtract(&mut self, points: Points, competitor: CompetitorNumber) {
        let competitor = match competitor {
            CompetitorNumber::One => &mut self.competitor_one,
            CompetitorNumber::Two => &mut self.competitor_two
        };
        match points {
            Points::Points(amount) => competitor.points.saturating_sub(amount),
            Points::Advantages(amount) => competitor.advantages.saturating_sub(amount),
            Points::Penalties(amount) => competitor.penalties.saturating_sub(amount),
            Points::Medical(amount) => competitor.medical.saturating_sub(amount)
        };
    }

    fn is_winner(&self) -> bool {
        self.winner.is_some()
    }

    fn set_winner(&mut self, competitor: CompetitorNumber, method: WinMethod) {
        self.winner = Some((competitor, method))
    }

    fn clear_winner(&mut self) {
        self.winner = None
    }
}

pub enum Points {
    Points(u8),
    Advantages(u8),
    Penalties(u8),
    Medical(u8)
}

pub enum CompetitorNumber {
    One,
    Two
}

pub struct Match {
    competitor_one: Competitor,
    competitor_two: Competitor,
    score: MatchScore,
    timer: MatchTimer
}

impl Match {
    pub fn new(competitor_one: Competitor, competitor_two: Competitor) -> Self {
        Match {
            competitor_one,
            competitor_two,
            score: MatchScore::default(),
            timer: MatchTimer::default()
        }
    }

    pub fn start(&mut self) {
        self.timer.start();
    }

    pub fn stop(&mut self) {
        self.timer.stop();
    }

    pub fn is_complete(&self) -> bool {
        self.timer.is_complete() || self.score.is_winner()
    }

    pub fn add_score(&mut self, points: Points, competitor: CompetitorNumber) {
        self.score.add(points, competitor);
    }

    pub fn remove_score(&mut self, points: Points, competitor: CompetitorNumber) {
        self.score.subtract(points, competitor);
    }
}

impl Display for Match {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {}", self.competitor_one.name(), self.score.competitor_one);
        writeln!(f, "{}: {}", self.competitor_two.name(), self.score.competitor_two);
        writeln!(f, "{}", self.timer)
    }
}