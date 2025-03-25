pub struct LevelExperience {
    experience_table: [i32; 99],
}

impl LevelExperience {
    pub fn new() -> Self {
        let mut experience_table: [i32; 99] = [0; 99];
        let mut acc = 0;

        for i in 0..99 {
            let level = i as f64 + 1.0;
            let delta = (level + 2.0_f64.powf(level / 7.0) * 300.0).floor() as i32;
            acc += delta;
            experience_table[i] = (acc / 4) * 10;
        }

        Self { experience_table }
    }

    pub fn get_level_by_experience(&self, experience: i32) -> i32 {
        for i in (0..99).rev() {
            if experience >= self.experience_table[i] {
                return (i + 2).min(99) as i32;
            }
        }
        1
    }

    pub fn get_experience_by_level(&self, level: i32) -> i32 {
        if level < 2 || level > 100 {
            panic!("Level must be between 2 and 100");
        }
        self.experience_table[(level - 2) as usize]
    }
}