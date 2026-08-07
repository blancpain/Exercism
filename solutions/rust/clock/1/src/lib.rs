use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hours = hours.rem_euclid(24);

        if minutes >= 60 {
            hours += (minutes / 60) % 24;
        } else if minutes < -60 {
            let temp = -(minutes / 60) % 24;
            hours -= temp + 1;
        } else if minutes < 0 {
            hours -= 1;
        }

        let hours = hours.rem_euclid(24);
        let minutes = minutes.rem_euclid(60);

        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_minutes = self.minutes + minutes;
        let mut new_hours = self.hours;

        if new_minutes > 60 {
            new_hours += (new_minutes / 60) % 24;
        } else if new_minutes < -60 {
            let temp = -(new_minutes / 60) % 24;
            new_hours -= temp + 1;
        } else if new_minutes < 0 {
            new_hours -= 1;
        }
        let new_hours = new_hours.rem_euclid(24);

        Self {
            hours: new_hours,
            minutes: new_minutes.rem_euclid(60),
        }
    }
}
