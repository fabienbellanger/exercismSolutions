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
        Self {
            frames: Vec::with_capacity(10),
        }
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
            self.frames.push(vec![pins]);
        } else {
            let frame = &self.frames[frames_length - 1];
            let frame_length = frame.len();

            if frames_length < 10 {
                if frame_length == 0 {
                    self.frames[frames_length - 1].push(pins);
                } else if frame_length == 1 {
                    if frame[0] == 10 {
                        self.frames.push(vec![pins]);
                    } else if frame[0] + pins <= 10 {
                        self.frames[frames_length - 1].push(pins);
                    } else {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                } else {
                    self.frames.push(vec![pins]);
                }
            } else if frame_length == 0 {
                self.frames[frames_length - 1].push(pins);
            } else if frame_length == 1 {
                if frame[0] == 10 || frame[0] + pins <= 10 {
                    self.frames[frames_length - 1].push(pins);
                } else {
                    return Err(Error::NotEnoughPinsLeft);
                }
            } else if (frame[1] + pins <= 10 || frame[1] == 10 || frame[0] != 10)
                && (frame[0] + frame[1] == 10 || frame[0] == 10)
            {
                self.frames[frames_length - 1].push(pins);
            } else {
                return Err(Error::NotEnoughPinsLeft);
            }
        }

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

            if index < 9 {
                if frame[0] == 10 {
                    score += self.frames[index + 1][0];
                    if self.frames[index + 1].len() >= 2 {
                        score += self.frames[index + 1][1];
                    } else {
                        score += self.frames[index + 2][0];
                    }
                } else if sum == 10 {
                    score += self.frames[index + 1][0];
                }
            }
        });

        Some(score)
    }

    fn is_finished(&self) -> bool {
        self.frames.len() == 10
            && (self.frames[9].len() == 3
                || self.frames[9].len() == 2 && self.frames[9][0] + self.frames[9][1] < 10)
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}
