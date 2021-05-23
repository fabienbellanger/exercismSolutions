use std::collections::HashMap;

#[derive(Debug)]
enum Score {
    Win,
    Draw,
    Loss,
}

#[derive(Debug)]
struct Match {
    team_1: String,
    team_2: String,
    score: Score,
}

impl Match {
    fn from_input(line: &str) -> Self {
        let mut parts = line.split(';');
        let team_1 = parts.next().unwrap().to_owned();
        let team_2 = parts.next().unwrap().to_owned();
        let result = parts.next().unwrap();
        Self {
            team_1,
            team_2,
            score: match result {
                "win" => Score::Win,
                "loss" => Score::Loss,
                "draw" => Score::Draw,
                _ => panic!("Invalid match result {}", result),
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Team {
    matchs_played: usize,
    matchs_won: usize,
    matchs_drawn: usize,
    matchs_lost: usize,
}

impl Team {
    fn new() -> Self {
        Self {
            matchs_played: 0,
            matchs_won: 0,
            matchs_drawn: 0,
            matchs_lost: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Tournament {
    teams: HashMap<String, Team>,
}

impl Tournament {
    fn new() -> Self {
        Self {
            teams: HashMap::new(),
        }
    }

    fn update_team(&mut self, a_match: Match) {
        let team_name_1 = a_match.team_1;
        let team_name_2 = a_match.team_2;

        let team_1 = self.teams.entry(team_name_1).or_insert_with(Team::new);
        team_1.matchs_played += 1;
        match a_match.score {
            Score::Win => {
                team_1.matchs_won += 1;
            }
            Score::Draw => {
                team_1.matchs_drawn += 1;
            }
            Score::Loss => {
                team_1.matchs_lost += 1;
            }
        }

        let team_2 = self.teams.entry(team_name_2).or_insert_with(Team::new);
        team_2.matchs_played += 1;
        match a_match.score {
            Score::Win => {
                team_2.matchs_lost += 1;
            }
            Score::Draw => {
                team_2.matchs_drawn += 1;
            }
            Score::Loss => {
                team_2.matchs_won += 1;
            }
        }
    }

    fn display_team(&self, team_name: String) -> String {
        let team = self.teams.get(&team_name).unwrap();
        format!(
            "{: <30} |{: >3} |{: >3} |{: >3} |{: >3} |{: >3}",
            team_name,
            team.matchs_played,
            team.matchs_won,
            team.matchs_drawn,
            team.matchs_lost,
            team.matchs_won * 3 + team.matchs_drawn
        )
    }

    fn display_header() -> String {
        format!(
            "{: <30} |{: >3} |{: >3} |{: >3} |{: >3} |{: >3}",
            "Team", "MP", "W", "D", "L", "P"
        )
    }

    fn display(&self) -> String {
        let mut table = Tournament::display_header();

        // TODO: Order teams by points (desc) then alphabetically.
        let table_teams: String = self
            .teams
            .iter()
            .map(|(name, _)| {
                let mut line = String::from("\n");
                line += &self.display_team(name.clone());
                line
            })
            .collect();

        table.push_str(&table_teams);
        table
    }
}

impl std::fmt::Display for Tournament {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

pub fn tally(match_results: &str) -> String {
    let mut tournament = Tournament::new();
    match_results
        .lines()
        .for_each(|line| tournament.update_team(Match::from_input(line)));

    format!("{}", tournament)
}
