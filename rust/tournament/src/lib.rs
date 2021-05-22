use std::collections::HashMap;

// Input: 
// Team1;Team2;{win|loss|draw}
// Devastating Donkeys;Courageous Californians;draw
// Devastating Donkeys;Allegoric Alaskans;win
// Courageous Californians;Blithering Badgers;loss
//
// Output table:
// {Team: 30} | MP |  W |  D |  L |  P
// {Team: 30} |  4 |  2 |  1 |  1 |  7

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
                _ => panic!("Invalid match result {}", result)
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Team {
    name: String,
    matchs_played: usize,
    matchs_won: usize,
    matchs_drawn: usize,
    matchs_lost: usize,
}

impl Team {
    fn new(name: String) -> Self {
        Self {
            name,
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
    fn update_team(self, a_match: Match) -> Self {
        let mut tournament = self.clone();

        let team_name_1 = a_match.team_1;
        let team_name_2 = a_match.team_2;

        let team_1 = self.teams.get(&team_name_1);
        let mut team_1 = match team_1 {
            Some(team) => {
                team
            },
            _ => &Team::new(team_name_1),
        };
        let team_2 = self.teams.get(&team_name_2);
        let mut team_2 = match team_2 {
            Some(team) => {
                team
            },
            _ => &Team::new(team_name_2),
        };

        match a_match.score {
            Score::Win => {
                team_1.matchs_played += 1;
                team_1.matchs_won += 1;
                team_2.matchs_played += 1;
                team_2.matchs_lost += 1;
            },
            Score::Draw => {
                team_1.matchs_played += 1;
                team_1.matchs_drawn += 1;
                team_2.matchs_played += 1;
                team_2.matchs_drawn += 1;
            },
            Score::Loss => {
                team_1.matchs_played += 1;
                team_1.matchs_lost += 1;
                team_2.matchs_played += 1;
                team_2.matchs_won += 1;
            },
        }

        tournament.teams.insert(team_name_1.clone(), team_1.clone()).unwrap();
        tournament.teams.insert(team_name_2.clone(), team_2.clone()).unwrap();

        tournament
    }
}

pub fn tally(match_results: &str) -> String {
    let mut table = String::from("Team                           | MP |  W |  D |  L |  P");
    let matches: Vec<Match> = match_results
        .lines()
        .map(|line| Match::from_input(line))
        .collect();
    

    table
}
