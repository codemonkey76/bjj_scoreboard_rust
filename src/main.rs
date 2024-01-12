#![allow(unused)]

use std::thread::sleep;
use std::time::Duration;
use fake::{Dummy, Faker};
use bjj_lib::{Competitor, CompetitorNumber, Match, Points, Team};

fn main() {
    let c1 = Competitor::dummy(&Faker);
    let c2 = Competitor::dummy(&Faker);

    let mut m = Match::new(c1, c2);

    println!("{m}");

    m.start();
    while !(m.is_complete()) {
        sleep(Duration::from_secs(1));
        println!("{m}");
        m.stop();
        m.add_score(Points::Points(2), CompetitorNumber::One);
    }
}
