use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {

        let mut tot = if hours >= 0 {hours*60+minutes} else {(24 - hours.abs()%24)*60+minutes};
        tot = if tot > 0 {tot} else {60*24-tot.abs()%(60*24)};

        Self {
            hours: tot/60%24,
            minutes: tot%60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {

        let mut mtot = self.hours*60 + self.minutes + minutes;
        mtot = if mtot > 0 {mtot} else {60*24-mtot.abs()%(60*24)};
        Self {
            hours: mtot/60%24,
            minutes: mtot%60,
        }
    }
    fn format(&self, num: i32) -> String {
        if num < 10 {
            return String::from("0") + &num.to_string();
        }
        return num.to_string();
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            self.format(self.hours),
            self.format(self.minutes)
        )
    }
}
impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            self.format(self.hours),
            self.format(self.minutes)
        )
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        return self.hours == other.hours && self.minutes == other.minutes;
    }
}
