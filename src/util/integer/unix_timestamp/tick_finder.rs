use super::*;

macro_rules! make_consider {
    ($fn_name1:ident,$fn_name2:ident,$ee:expr) => {
        pub fn $fn_name1(&mut self, step_sizes: &[i64]) {
            for &a in step_sizes.iter().rev() {
                if let Some(range) = self.gen_tick(self.start.$fn_name2(a)) {
                    self.consider_set(range, $ee);
                } else {
                    // Since we are hansling smaller and smaller step sizes,
                    // If gen_tick fails, thats means it has too many ticks,
                    // so we can safely exist the loop because the intervals
                    // are just going to get smaller and smaller.
                    break;
                }
            }
        }
    };
}

#[derive(Debug)]
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
            max_tick_num: ideal_num_steps * 2,
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

    fn consider_set(&mut self, range: Vec<UnixTime>, ee: TimestampType) -> bool {
        let new_closeness = (self.ideal_num_steps as i64 - range.len() as i64).abs();
        let old_closeness = (self.ideal_num_steps as i64 - self.best.len() as i64).abs();

        let is_better = if new_closeness < old_closeness {
            true
        } else if new_closeness == old_closeness {
            if range.len() > self.best.len() {
                true
            } else {
                false
            }
        } else {
            false
        };

        if is_better {
            self.best = range;
            self.typ = ee;
            true
        } else {
            false
        }
    }

    make_consider!(consider_yr, years, TimestampType::YR);
    make_consider!(consider_mo, months, TimestampType::MO);
    make_consider!(consider_dy, days, TimestampType::DY);
    make_consider!(consider_hr, hours, TimestampType::HR);
    make_consider!(consider_mi, minutes, TimestampType::MI);
    make_consider!(consider_se, seconds, TimestampType::SE);
}
