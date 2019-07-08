#[derive(Debug, PartialEq)]
pub struct Clock {
    minute_of_day: u16
}

impl Clock {
    pub fn new(hours: isize, minutes: isize) -> Self {
        let total_minutes = hours % 24 * 60 + minutes;
        if total_minutes < 0 {
            Self {
                minute_of_day: (1440 + total_minutes % 1440) as u16
            }
        } else {
            Self {
                minute_of_day: (total_minutes % 1440) as u16
            }
        }
    }

    pub fn add_minutes(&self, minutes: isize) -> Self {
        Self::new(0, self.minute_of_day as isize + minutes)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = self.minute_of_day / 60;
        let minutes = self.minute_of_day - hours * 60;
        write!(formatter, "{:02}:{:02}", hours, minutes)
    }
}
