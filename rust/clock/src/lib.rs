use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    pub hours: u8,
    pub minutes: u8,
}

impl Clock {
    fn from_minutes(minutes: i32) -> Self {
        if minutes < 0 && minutes > -60 {
            return Self {
                hours: 23,
                minutes: (60 + minutes) as u8,
            };
        }

        let minites_remaining = minutes % 60;
        let hours_from_minutes = minutes / 60;
        
        // Minutes convertion
        let final_minutes = match minites_remaining.is_positive() {
            true => (minites_remaining % 60) as u8,
            false => ((60 + minites_remaining % 60) % 60) as u8,
        };
        
        // Hours convertion
        let mut final_hours = match hours_from_minutes.is_positive() {
            true => (hours_from_minutes % 24) as u8,
            false => ((24 + hours_from_minutes % 24) % 24) as u8,
        };
        if minites_remaining.is_negative() && hours_from_minutes.is_negative() {
            final_hours = (final_hours - 1) % 24;
        }
        
        Self {
            hours: final_hours,
            minutes: final_minutes,
        }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::from_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = (self.hours) as i32 * 60 + (self.minutes) as i32 + minutes;
        Self::from_minutes(minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
