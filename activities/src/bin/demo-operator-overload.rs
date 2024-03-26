use std::ops::Index;

enum Temp {
    Current,
    Max,
    Min,
}

struct Hvac {
    current_temp: i16,
    max_temp: i16,
    min_temp: i16,
}

impl Index<Temp> for Hvac {
    type Output = i16;

    fn index(&self, temp: Temp) -> &Self::Output {
        match temp {
            Temp::Current => &self.current_temp,
            Temp::Max => &self.max_temp,
            Temp::Min => &self.min_temp,
        }
    }
}

fn main() {
    let env = Hvac {
        current_temp: 78,
        max_temp: 95,
        min_temp: 30,
    };

    let current = env[Temp::Current];
    let max = env[Temp::Max];
    let min = env[Temp::Min];

    println!("current: {:?}, max: {:?}, min: {:?}", current, max, min);
}
