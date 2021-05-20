// Score a bowling game.

// Bowling is a game where players roll a heavy ball to knock down pins arranged in a triangle. Write code to keep track of the score of a game of bowling.
// Scoring Bowling
// The game consists of 10 frames. A frame is composed of one or two ball throws with 10 pins standing at frame initialization. There are three cases for the tabulation of a frame.
//     An open frame is where a score of less than 10 is recorded for the frame. In this case the score for the frame is the number of pins knocked down.
//     A spare is where all ten pins are knocked down by the second throw. The total value of a spare is 10 plus the number of pins knocked down in their next throw.
//     A strike is where all ten pins are knocked down by the first throw. The total value of a strike is 10 plus the number of pins knocked down in the next two throws. If a strike is immediately followed by a second strike, then the value of the first strike cannot be determined until the ball is thrown one more time.

// Here is a three frame example:
// Frame 1 	Frame 2 	Frame 3
// X (strike) 	5/ (spare) 	9 0 (open frame)
// Frame 1 is (10 + 5 + 5) = 20
// Frame 2 is (5 + 5 + 9) = 19
// Frame 3 is (9 + 0) = 9
// This means the current running total is 48.

// The tenth frame in the game is a special case. If someone throws a strike or a spare then they get a fill ball. Fill balls exist to calculate the total of the 10th frame. Scoring a strike or spare on the fill ball does not give the player more fill balls. The total value of the 10th frame is the total number of pins knocked down.

// For a tenth frame of X1/ (strike and a spare), the total value is 20.
// For a tenth frame of XXX (three strikes), the total value is 30.


// Write code to keep track of the score of a game of bowling. It should support two operations:
//     roll(pins : int) is called each time the player rolls a ball. The argument is the number of pins knocked down.
//     score() : int is called only at the very end of the game. It returns the total score for that game.


#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
pub struct BowlingGame {
    frames: Vec<Vec<u16>>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { frames: Vec::with_capacity(10) }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_finished() {
            return Err(Error::GameComplete);
        }
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        let frames_length = self.frames.len();
        if frames_length == 0 {
            // First time
            // ----------
            self.frames.push(vec![pins]);
        } else {
            let frame = &self.frames[frames_length - 1];
            let frame_length = frame.len();

            if frames_length < 10 {
                // 9 premières frames
                if frame_length == 0 {
                    // 1er lancer
                    self.frames[frames_length - 1].push(pins);
                } else if frame_length == 1 {
                    // 2ème lancer
                    if frame[0] == 10 {
                        // Strike => next frame
                        self.frames.push(vec![pins]);
                    } else if frame[0] + pins <= 10 {
                        // Add frame
                        self.frames[frames_length - 1].push(pins);
                    } else {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                } else {
                    self.frames.push(vec![pins]);
                }
            } else {
                // Dernière frame
                if frame_length == 0 {
                    // 1er lancer
                    self.frames[frames_length - 1].push(pins);
                } else if frame_length == 1 {
                    if frame[0] == 10 || frame[0] + pins <= 10 {
                        self.frames[frames_length - 1].push(pins);
                    } else {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                } else {
                    if frame[1] == 10 || frame[0] + frame[1] == 10 {
                        // Strike ou spare
                        self.frames[frames_length - 1].push(pins);
                    }
                }
            }
        }

        // dbg!(&self);

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_finished() {
            return None;
        }
        
        let mut score: u16 = 0;
        self.frames.iter().enumerate().for_each(|(index, frame)| {
            let sum: u16 = frame.iter().sum();
            score += sum;
            
            // 9 premières frames
            if index < 9 {
                if frame[0] == 10 {
                    // Strike
                    score += self.frames[index + 1][0];
                    println!("Strike 1 : {} = {}", self.frames[index + 1][0], score);
                    if self.frames[index + 1].len() >= 2 {
                        score += self.frames[index + 1][1];
                        println!("Strike 2 : {} = {}", self.frames[index + 1][1], score);
                    } else {
                        score += self.frames[index + 2][0];
                        println!("Strike 3 : {} = {}", self.frames[index + 2][0], score);
                    }
                } else if sum == 10 {
                    // Spare
                    score += self.frames[index + 1][0]; 
                }
            }
        });
        Some(score)
    }

    fn is_finished(&self) -> bool {
        self.frames.len() == 10 && 
            (self.frames[9].len() == 3 || 
                self.frames[9].len() == 2 && self.frames[9][0] + self.frames[9][1] < 10)
    }
}
