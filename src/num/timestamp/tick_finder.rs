use super::*;

/*
macro_rules! make_consider {
    ($fn_name1:ident,$fn_name2:ident,$ee:expr) => {
        pub fn $fn_name1(&mut self, step_sizes: &[i64], dash_nums: &'a [i64]) {
            for &a in step_sizes.iter().rev() {
                if let Some(ticks) = self.gen_tick(self.start.$fn_name2(&self.timezone, a)) {
                    self.consider_set(Candidate {
                        ticks,
                        unit_data: $ee,
                        dash_nums,
                        chosen_tick: a,
                    });
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
*/

#[derive(Debug)]
pub struct Candidate {
    pub ticks: Vec<UnixTime>,
    pub unit_data: StepUnit,
    //pub dash_nums: &'a [i64],
    pub chosen_tick: i64,
}

#[derive(Debug)]
pub struct BestTickFinder {
    ideal_num_steps: u32,
    end: UnixTime,
    //The number of ticks at which to give up on this candidate.
    max_tick_num: u32,
    best: Candidate,
}
impl BestTickFinder {
    pub fn new(end: UnixTime, ideal_num_steps: u32) -> Self {
        BestTickFinder {
            ideal_num_steps,
            end,
            max_tick_num: ideal_num_steps * 2,
            best: Candidate {
                ticks: Vec::new(),
                unit_data: StepUnit::YR,
                //dash_nums: &[],
                chosen_tick: 0,
            },
        }
    }
    pub fn into_best(self) -> Option<Candidate> {
        if self.best.ticks.len() >= 2 {
            Some(self.best)
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

    ///Returns true if the candidate was chosen
    fn consider_set(&mut self, candidate: Candidate) -> bool {
        let new_closeness = (self.ideal_num_steps as i64 - candidate.ticks.len() as i64).abs();
        let old_closeness = (self.ideal_num_steps as i64 - self.best.ticks.len() as i64).abs();

        use std::cmp::Ordering;
        let is_better = match new_closeness.cmp(&old_closeness) {
            Ordering::Less => true,
            //If there is a tie, choose the one with less ticks.
            Ordering::Greater => candidate.ticks.len() < self.best.ticks.len(),
            Ordering::Equal => false,
        };

        if is_better {
            self.best = candidate;
            true
        } else {
            false
        }
    }

    pub(crate) fn consider_meta<I: UnixTimeGenerator>(
        &mut self,
        unit_data: StepUnit,
        gen: I,
        step_sizes: &[i64],
    ) {
        for &a in step_sizes.iter().rev() {
            if let Some(ticks) = self.gen_tick(gen.generate(a)) {
                self.consider_set(Candidate {
                    ticks,
                    unit_data,
                    chosen_tick: a,
                });
            } else {
                // Since we are hansling smaller and smaller step sizes,
                // If gen_tick fails, thats means it has too many ticks,
                // so we can safely exist the loop because the intervals
                // are just going to get smaller and smaller.
                break;
            }
        }
    }

    /*
    make_consider!(consider_yr, years, TimestampType::YR);
    make_consider!(consider_mo, months, TimestampType::MO);
    make_consider!(consider_dy, days, TimestampType::DY);
    make_consider!(consider_hr, hours, TimestampType::HR);
    make_consider!(consider_mi, minutes, TimestampType::MI);
    make_consider!(consider_se, seconds, TimestampType::SE);
    */
}
