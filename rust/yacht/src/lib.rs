pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    let mut dice = dice;
    dice.sort_unstable();

    match category {
        Category::Ones => dice.into_iter().filter(|d| *d == 1).sum(),
        Category::Twos => dice.into_iter().filter(|d| *d == 2).sum(),
        Category::Threes => dice.into_iter().filter(|d| *d == 3).sum(),
        Category::Fours => dice.into_iter().filter(|d| *d == 4).sum(),
        Category::Fives => dice.into_iter().filter(|d| *d == 5).sum(),
        Category::Sixes => dice.into_iter().filter(|d| *d == 6).sum(),
        Category::Choice => dice.into_iter().sum(),
        Category::LittleStraight => {
            if dice == [1, 2, 3, 4, 5] {
                30
            } else {
                0
            }
        }
        Category::BigStraight => {
            if dice == [2, 3, 4, 5, 6] {
                30
            } else {
                0
            }
        }
        Category::Yacht => {
            if dice[0] == dice[4] {
                50
            } else {
                0
            }
        }
        Category::FourOfAKind => {
            if dice[0] == dice[3] || dice[1] == dice[4] {
                4 * dice[1]
            } else {
                0
            }
        }
        Category::FullHouse => {
            if dice[0] != dice[4]
                && (dice[0] == dice[1] && dice[2] == dice[4]
                    || dice[0] == dice[2] && dice[3] == dice[4])
            {
                dice.into_iter().sum()
            } else {
                0
            }
        }
    }
}
