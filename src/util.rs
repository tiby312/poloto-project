use core::fmt;
use fmt::Write;



struct Result<X>{
    num_steps:usize,
    step:X,
    start_step:X,
    good_normalized_step:usize
}


#[test]
pub fn test_good_step_int(){
    let a=[100,1231,3,2,1,444444,23];
    
    for &a in a.iter(){
        let val=find_good_step_int(5,[0,a]);

        dbg!(a,val);

    }

    assert!(false);
   
}




pub trait PlotNumber:PartialOrd+Copy{
    fn find_good_step(ideal_num_steps:usize,range:[Self;2])->(Self,u8);
    fn get_range_info(step:Self,range:[Self;2])->(Self,usize);
    fn make_hole(&mut self);
    
    fn is_hole(&self)->bool;
    fn unit_range()->[Self;2];

    fn get_tick(&self,index:usize,step:Self)->Self;
    
    fn scale(&self,val:[Self;2],max:f64)->f64;

    fn scale2(&self,val:[Self;2],max:f64)->f64;

    fn zero()->Self;

    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter,
        step: Option<Self>,
    ) -> std::fmt::Result;

    fn display_with_offset(xstart_step:Self,step_num:usize,xstep:Self)->bool;
}





impl PlotNumber for f64{
    fn zero()->Self{
        0.0
    }

    //Returns true if we should display all plots relativ to a base number
    fn display_with_offset(xstart_step:Self,step_num:usize,xstep:Self)->bool{
        determine_if_should_use_strat(
            xstart_step,
            xstart_step + ((step_num - 1) as f64) * xstep,
            xstep,
        )
    }


    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter,
        step: Option<Self>,
    ) -> std::fmt::Result{
        write!(formatter, "{}", crate::util::interval_float(*self, step))
    }

    fn find_good_step(ideal_num_steps:usize,range:[Self;2])->(Self,u8){
        find_good_step_f64(ideal_num_steps,range)
    }
    fn get_range_info(step:Self,range:[Self;2])->(Self,usize){
        get_range_info_f64(step,range)
    }
    
    fn make_hole(&mut self){
        *self=f64::NAN;
    }
    fn is_hole(&self)->bool{
        self.is_nan()
    }

    fn unit_range()->[Self;2]{
        [-1.0,1.0]
    }

    fn get_tick(&self,index:usize,step:Self)->Self{
        self+step*(index as f64)
    }

    fn scale(&self,val:[Self;2],max:f64)->f64{
        let diff=val[1]-val[0];

        let scale=max/diff;

        (*self-val[0])*scale
    }

    fn scale2(&self,val:[Self;2],max:f64)->f64{
        let diff=val[1]-val[0];

        let scale=max/diff;

        (*self)*scale
    }
}

impl PlotNumber for i128{
    fn zero()->Self{
        0
    }

    //Returns true if we should display all plots relativ to a base number
    fn display_with_offset(xstart_step:Self,step_num:usize,xstep:Self)->bool{
        /*
        const MAX_LEN:i128=1000;

        if xstart_step.abs()>MAX_LEN{
            return true
        }

        if xstart_step + ((step_num as i128 - 1) ) * xstep > MAX_LEN{
            return true
        }

        false
        */
        false  
    }

    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter,
        step: Option<Self>,
    ) -> std::fmt::Result{
        write!(formatter, "{}", self)
    }

    fn find_good_step(ideal_num_steps:usize,range:[Self;2])->(Self,u8){
        find_good_step_int(ideal_num_steps,range)
    }
    fn get_range_info(step:Self,range:[Self;2])->(Self,usize){
        get_range_info_int(step,range)
    }
    
    fn make_hole(&mut self){
        *self=i128::MAX
    }
    fn is_hole(&self)->bool{
        *self==i128::MAX
    }

    fn unit_range()->[Self;2]{
        [0,1]
    }

    fn get_tick(&self,index:usize,step:Self)->Self{
        self+step*(index as i128)
    }

    fn scale(&self,val:[Self;2],max:f64)->f64{
        let diff=(val[1]-val[0]) as f64;

        let scale=max/diff;

        (*self-val[0]) as f64*scale
    }

    fn scale2(&self,val:[Self;2],max:f64)->f64{
        let diff=(val[1]-val[0]) as f64;

        let scale=max/diff;

        (*self) as f64*scale
    }
}



pub fn round_up_to_nearest_multiple_int(val:i128,multiple:i128)->i128{
    let mut ss=multiple-1;
    
    let ss=if val>=0{
        multiple-1
    }else{
        0
    };
    
    ((val+ss)/multiple)*multiple
}

pub fn round_up_to_nearest_multiple_f64(val:f64,multiple:f64)->f64{
    ((val)/multiple).ceil()*multiple
}


pub fn get_range_info_int(step:i128,range_all:[i128;2])->(i128,usize){
    let start_step=round_up_to_nearest_multiple_int(range_all[0],step);
    
    let step_num={
        let mut counter=start_step;
        let mut res=0;
        for a in 0..{
            if counter>range_all[1]{
                res=a;
                break;
            }

            assert!(step+counter>counter,"{:?}",(step,range_all));
            counter+=step;
        }
        res
    };

    (start_step,step_num)
}

//TODO handle case zero steps are found
pub fn get_range_info_f64(step:f64,range_all:[f64;2])->(f64,usize){
    let start_step=round_up_to_nearest_multiple_f64(range_all[0],step);
    
    let step_num={
        let mut counter=start_step;
        let mut res=0;
        for a in 0..{
            if counter>range_all[1]{
                res=a;
                break;
            }

            assert!(step+counter>counter,"{:?}",(step,range_all));
            counter+=step;
        }
        res
    };

    (start_step,step_num)
}




pub fn find_good_step_int(num_steps: usize, range_all: [i128; 2])->(i128,u8){
    let range=range_all[1]-range_all[0];

    let rough_step = range / (num_steps - 1) as i128;

    let step_power = 10.0f64.powf((rough_step as f64).log10().floor()) as i128;
        
    let normalized_step=rough_step/step_power;
 

    let good_steps = [1, 2, 5, 10];
    let good_normalized_step = *good_steps
        .iter()
        .find(|a| **a > normalized_step )
        .unwrap();

    (good_normalized_step * step_power,good_normalized_step as u8)
}


pub fn find_good_step_f64(num_steps: usize, range_all: [f64; 2])->(f64,u8){
    let range=range_all[1]-range_all[0];

    let rough_step = range / (num_steps - 1) as f64;

    let step_power = 10.0f64.powf((rough_step as f64).log10().floor());
        
    let normalized_step=(rough_step/step_power) as usize;
 

    let good_steps = [1, 2, 5, 10];
    let good_normalized_step = *good_steps
        .iter()
        .find(|a| **a > normalized_step)
        .unwrap();

    (good_normalized_step as f64 * step_power,good_normalized_step as u8)
}





/// Specify ideal number of steps and range.
/// Returns:
/// number of intervals.
/// size of each interval
/// first interval location.
pub fn find_good_step(num_steps: usize, range_all: [f64; 2]) -> (usize, f64, f64, u8) {
    let range_all = [range_all[0] as f64, range_all[1] as f64];
    let range = range_all[1] - range_all[0];

    //https://stackoverflow.com/questions/237220/tickmark-algorithm-for-a-graph-axis

    let rough_step = range / (num_steps - 1) as f64;

    let step_power = 10.0f64.powf(-rough_step.abs().log10().floor()) as f64;
    let normalized_step = rough_step * step_power;

    let good_steps = [1u8, 2, 5, 10];
    let good_normalized_step = *good_steps
        .iter()
        .find(|a| **a as f64 > normalized_step)
        .unwrap();

    let step = good_normalized_step as f64 / step_power;

    let start_step = {
        //naively find starting point.
        let aa = (range_all[0] / step).floor() * step;
        let bb = (range_all[0] / step).ceil() * step;
        if aa < bb {
            if aa < range_all[0] {
                bb
            } else {
                aa
            }
        } else if bb < range_all[0] {
            aa
        } else {
            bb
        }
    };
    assert!(start_step >= range_all[0]);

    let num_step = {
        //naively find number of steps
        let mut counter = start_step;
        let mut num = 0;
        loop {
            if counter > range_all[1] {
                break;
            }
            counter += step;

            num += 1;
        }
        num
    };
    assert!(num_step >= 1);

    // Because of the requirement for the num step to be atleast one, this assertion isnt
    // necessarily true.
    // assert!(start_step + step * ((num_step - 1) as f64) <= range_all[1]);

    (
        num_step,
        step as f64,
        start_step as f64,
        good_normalized_step,
    )
}

fn make_normal(a: f64, step: Option<f64>) -> impl fmt::Display {
    crate::DisplayableClosure::new(move |fm| {
        if let Some(step) = step {
            let k = (-step.log10()).ceil();
            let k = k.max(0.0);
            write!(fm, "{0:.1$}", a, k as usize)
        } else {
            write!(fm, "{0:e}", a)
        }
    })
}

fn make_science(a: f64, step: Option<f64>) -> impl fmt::Display {
    crate::DisplayableClosure::new(move |fm| {
        if let Some(step) = step {
            let precision = if a == 0.0 {
                0
            } else {
                let k1 = -step.log10().ceil();
                let k2 = -a.abs().log10().ceil();
                let k1 = k1 as isize;
                let k2 = k2 as isize;

                (k1 - k2).max(0) as usize
            };

            write!(fm, "{0:.1$e}", a, precision)
        } else {
            write!(fm, "{}", a)
        }
    })
}

pub fn determine_if_should_use_strat(start: f64, end: f64, step: f64) -> bool {
    let mut start_s = String::new();
    let mut end_s = String::new();

    write!(&mut start_s, "{}", interval_float(start, Some(step))).unwrap();
    write!(&mut end_s, "{}", interval_float(end, Some(step))).unwrap();

    start_s.len() > 7 || end_s.len() > 7
}

const SCIENCE: usize = 4;

/// The step amount dictates the precision we need to show at each interval
/// in order to capture the changes from each step
pub fn interval_float(a: f64, step: Option<f64>) -> impl fmt::Display {
    //TODO handle zero???
    //want to display zero with a formatting that is cosistent with others
    crate::DisplayableClosure::new(move |fm| {
        if a.abs().log10().floor().abs() > SCIENCE as f64 {
            let mut k = String::new();
            write!(&mut k, "{}", make_science(a, step))?;

            let mut j = String::new();
            write!(&mut j, "{}", make_normal(a, step))?;

            //Even if we use scientific notation,
            //it could end up as more characters
            //because of the needed precision.
            let ans = if k.len() < j.len() { k } else { j };
            write!(fm, "{}", ans)?;
        } else {
            write!(fm, "{}", make_normal(a, step))?;
        }
        Ok(())
    })
}



pub fn find_bounds2<X:PlotNumber,Y:PlotNumber>(
    it: impl IntoIterator<Item = (X,Y)>,
    xmarkers: impl IntoIterator<Item = X>,
    ymarkers: impl IntoIterator<Item = Y>,
) -> ([X;2],[Y;2]) {
    let mut ii = it
        .into_iter()
        .filter(|(x, y)| !x.is_hole() && !y.is_hole());

    if let Some((x, y)) = ii.next() {
      
        let mut val=([x,x],[y,y]);
        let mut xmoved=false;
        let mut ymoved=false;  
        let ii = ii
            .chain(
                xmarkers
                    .into_iter()
                    .filter(|a| !a.is_hole())
                    .map(|xx| (xx, y)),
            )
            .chain(
                ymarkers
                    .into_iter()
                    .filter(|a| !a.is_hole())
                    .map(|yy| (x, yy)),
            );

        ii.fold(&mut val, |val, (x, y)| {
            if x < val.0[0] {
                val.0[0] = x;
                xmoved=true;
            } else if x > val.0[1] {
                val.0[1] = x;
                xmoved=true;
            }
            if y < val.1[0] {
                val.1[0] = y;
                ymoved=true;
            } else if y > val.1[1] {
                val.1[1] = y;
                ymoved=true;
            }
            val
        });

        if !xmoved {
            val.0=X::unit_range();
        }

        if !ymoved {
            val.1=Y::unit_range();
        }

        val
    } else {
        (X::unit_range(),Y::unit_range())
    }
}


pub struct WriteCounter<T> {
    counter: usize,
    writer: T,
}
impl<T: fmt::Write> WriteCounter<T> {
    pub fn new(writer: T) -> WriteCounter<T> {
        WriteCounter { writer, counter: 0 }
    }
    pub fn get_counter(&self) -> usize {
        self.counter
    }
}
impl<T: fmt::Write> fmt::Write for WriteCounter<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.counter += s.len();
        self.writer.write_str(s)
    }
}
