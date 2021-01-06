use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    collect_table(parse_matches(match_results))
}

#[derive(Default)]
struct Stats {
    won: u32,
    lost: u32,
    drawn: u32,
}

impl Stats {
    const PTS_FOR_VICTORY: u32 = 3;
    const PTS_FOR_DRAW: u32 = 1;
    const PTS_FOR_LOSS: u32 = 0;

    fn points(&self) -> u32 {
        self.won * Self::PTS_FOR_VICTORY
            + self.lost * Self::PTS_FOR_LOSS
            + self.drawn * Self::PTS_FOR_DRAW
    }

    fn matches(&self) -> u32 {
        self.won + self.lost + self.drawn
    }

    fn win(&mut self) {
        self.won += 1
    }

    fn loss(&mut self) {
        self.lost += 1
    }

    fn draw(&mut self) {
        self.drawn += 1
    }
}

fn parse_matches(match_results: &str) -> HashMap<String, Stats> {
    let mut teams: HashMap<String, Stats> = HashMap::new();
    for line in match_results.lines() {
        let info: Vec<_> = line.split(';').collect();
        match info[2] {
            "win" => {
                teams.entry(info[0].to_string()).or_default().win();
                teams.entry(info[1].to_string()).or_default().loss();
            }
            "loss" => {
                teams.entry(info[0].to_string()).or_default().loss();
                teams.entry(info[1].to_string()).or_default().win();
            }
            _ => {
                teams.entry(info[0].to_string()).or_default().draw();
                teams.entry(info[1].to_string()).or_default().draw();
            }
        }
    }
    teams
}

macro_rules! row {
    ($col1:expr, $col2:expr, $col3:expr, $col4:expr, $col5:expr, $col6:expr) => {
        format!(
            "{0: <30} |{1: >3} |{2: >3} |{3: >3} |{4: >3} |{5: >3}",
            $col1, $col2, $col3, $col4, $col5, $col6
        )
    };
}

fn collect_table(teams: HashMap<String, Stats>) -> String {
    let mut table = row!("Team", "MP", "W", "D", "L", "P");
    let mut teams: Vec<_> = teams.iter().collect();
    teams.sort_by(|(team_a, a), (team_b, b)| {
        b.points()
            .partial_cmp(&a.points())
            .unwrap()
            .then(team_a.partial_cmp(&team_b).unwrap())
    });
    for (name, stats) in teams {
        table += "\n";
        table += &row!(
            name,
            stats.matches(),
            stats.won,
            stats.drawn,
            stats.lost,
            stats.points()
        )[..]
    }
    table
}
