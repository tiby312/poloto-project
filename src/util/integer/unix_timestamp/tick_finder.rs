use super::*;

macro_rules! make_consider {
    ($fn_name1:ident,$fn_name2:ident,$ee:expr) => {
        pub fn $fn_name1(&mut self, step_sizes: &[i64]) {
            for &a in step_sizes {
                if let Some(range) = self.gen_tick(self.start.$fn_name2(a)) {
                    self.consider_set(range, $ee);
                }
            }
        }
    };
}

pub struct BestTickFinder {
    ideal_num_steps: u32,
    start: UnixTime,
    end: UnixTime,
    //The number of ticks at which to give up on this candidate.
    max_tick_num: u32,
    best: Vec<UnixTime>,
    typ: TimestampType,
}
impl BestTickFinder {
    pub fn new(range: [UnixTime; 2], ideal_num_steps: u32) -> Self {
        let [start, end] = range;
        BestTickFinder {
            ideal_num_steps,
            start,
            end,
            max_tick_num: ideal_num_steps * 3,
            best: Vec::new(),
            typ: TimestampType::YR,
        }
    }
    pub fn into_best(self) -> Option<(Vec<UnixTime>, TimestampType)> {
        if self.best.len() >= 2 {
            Some((self.best, self.typ))
        } else {
            None
        }
    }
    fn gen_tick<I: Iterator<Item = UnixTime>>(&self, it: I) -> Option<Vec<UnixTime>> {
        let mut set = Vec::new();
        for b in it {
            if set.len() > self.max_tick_num as usize {
                return None;
            }

            if b > self.end {
                break;
            }

            set.push(b);
        }
        Some(set)
    }

    fn consider_set(&mut self, range: Vec<UnixTime>, ee: TimestampType) {
        let new_closeness = (self.ideal_num_steps as i64 - range.len() as i64).abs();
        let old_closeness = (self.ideal_num_steps as i64 - self.best.len() as i64).abs();
        dbg!(old_closeness, new_closeness);
        if new_closeness < old_closeness {
            self.best = range;
            self.typ = ee;

            dbg!(&self.best);
            //Keep improving upper bound
            if self.best.len() > self.ideal_num_steps as usize {
                self.max_tick_num = self.best.len() as u32;
            }
        }
    }

    make_consider!(consider_yr, years, TimestampType::YR);
    make_consider!(consider_mo, months, TimestampType::MO);
    make_consider!(consider_dy, days, TimestampType::DY);
    make_consider!(consider_hr, hours, TimestampType::HR);
    make_consider!(consider_mi, minutes, TimestampType::MI);
    make_consider!(consider_se, seconds, TimestampType::SE);
}
