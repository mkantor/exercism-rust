#[derive(Debug, PartialEq)]
pub struct Clock {
    minute_of_day: u16
}

impl Clock {
    const MINUTES_IN_DAY: isize = 24 * 60;

    pub fn new(hours: isize, minutes: isize) -> Self {
        let total_minutes = (hours % 24) * 60 + minutes;
        if total_minutes < 0 {
            Self {
                minute_of_day: (Self::MINUTES_IN_DAY + total_minutes % Self::MINUTES_IN_DAY) as u16
            }
        } else {
            Self {
                minute_of_day: (total_minutes % Self::MINUTES_IN_DAY) as u16
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
