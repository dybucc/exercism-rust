use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hours: String = self.hours.to_string();
        let mut minutes: String = self.minutes.to_string();

        if let ..10 = self.hours {
            hours = format!("0{}", hours);
        }
        if let ..10 = self.minutes {
            minutes = format!("0{}", minutes);
        }

        write!(f, "{}:{}", hours, minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut output = Self { hours, minutes };
        output.check();

        output
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut output = Self::new(self.hours, self.minutes + minutes);
        output.check();

        output
    }

    fn check(&mut self) {
        loop {
            match self.hours {
                24.. => self.hours -= 24,
                ..0 => self.hours += 24,
                _ => (),
            }

            match self.minutes {
                0..60 if self.hours < 24 && self.hours >= 0 => break,
                60.. => {
                    self.minutes -= 60;
                    self.hours += 1;
                }
                ..0 => {
                    self.minutes += 60;
                    self.hours -= 1;
                }
                _ => (),
            }
        }
    }
}
