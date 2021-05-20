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

#[derive(Debug)]
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

pub fn tally(match_results: &str) -> String {
    unimplemented!(
        "Given the result of the played matches '{}' return a properly formatted tally table string.",
        match_results
    );
}
