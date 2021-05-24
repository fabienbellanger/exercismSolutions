use std::collections::BTreeMap;

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
    played: usize,
    won: usize,
    drawn: usize,
    lost: usize,
    points: usize,
}

impl Team {
    fn new() -> Self {
        Self {
            played: 0,
            won: 0,
            drawn: 0,
            lost: 0,
            points: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Tournament {
    teams: BTreeMap<String, Team>,
}

impl Tournament {
    fn new() -> Self {
        Self {
            teams: BTreeMap::new(),
        }
    }

    fn update_team(&mut self, a_match: Match) {
        let team_name_1 = a_match.team_1;
        let team_name_2 = a_match.team_2;

        let team_1 = self.teams.entry(team_name_1).or_insert_with(Team::new);
        team_1.played += 1;
        match a_match.score {
            Score::Win => {
                team_1.won += 1;
                team_1.points += 3;
            }
            Score::Draw => {
                team_1.drawn += 1;
                team_1.points += 1;
            }
            Score::Loss => {
                team_1.lost += 1;
            }
        }

        let team_2 = self.teams.entry(team_name_2).or_insert_with(Team::new);
        team_2.played += 1;
        match a_match.score {
            Score::Win => {
                team_2.lost += 1;
            }
            Score::Draw => {
                team_2.drawn += 1;
                team_2.points += 1;
            }
            Score::Loss => {
                team_2.won += 1;
                team_2.points += 3;
            }
        }
    }

    fn display_team(&self, team_name: String) -> String {
        let team = self.teams.get(&team_name).unwrap();
        format!(
            "{: <30} |{: >3} |{: >3} |{: >3} |{: >3} |{: >3}",
            team_name, team.played, team.won, team.drawn, team.lost, team.points,
        )
    }

    fn display_header() -> String {
        format!(
            "{: <30} |{: >3} |{: >3} |{: >3} |{: >3} |{: >3}",
            "Team", "MP", "W", "D", "L", "P"
        )
    }

    fn display(&self) -> String {
        let mut ordered_teams: Vec<(&String, &usize)> = self
            .teams
            .iter()
            .map(|(team_name, team)| (team_name, &(team.points)))
            .collect();
        ordered_teams.sort_by(|a, b| b.1.cmp(a.1));

        let table_teams: Vec<String> = vec![Tournament::display_header()]
            .into_iter()
            .chain(
                ordered_teams
                    .into_iter()
                    .map(|item| self.display_team(item.0.clone())),
            )
            .collect();
        table_teams.join("\n")
    }
}

pub fn tally(match_results: &str) -> String {
    let mut tournament = Tournament::new();
    match_results
        .lines()
        .for_each(|line| tournament.update_team(Match::from_input(line)));
    tournament.display()
}
