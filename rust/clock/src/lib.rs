use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    total_minutes: i32
}

const MIN_IN_HR: i32 = 60;
const HR_IN_DAY: i32 = 24;


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { total_minutes: 0 }.add_minutes(hours * MIN_IN_HR + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut total_minutes = (self.total_minutes + minutes) %
            (HR_IN_DAY * MIN_IN_HR); 
        if total_minutes < 0 {
            total_minutes = HR_IN_DAY * MIN_IN_HR + total_minutes
        }
        Clock { total_minutes }
    }

    fn get_minutes(&self) -> i32 {
        self.total_minutes.rem_euclid(MIN_IN_HR)
    }

    fn get_hours(&self) -> i32 {
        self.total_minutes / MIN_IN_HR
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.get_hours(), self.get_minutes())
    }
}
