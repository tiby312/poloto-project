


pub fn find_good_step(num_steps: usize, range_all: [f32;2]) -> (usize, f32,f32) {
    let range_all = [range_all[0] as f64,range_all[1] as f64];
    let range=range_all[1]-range_all[0];


    //https://stackoverflow.com/questions/237220/tickmark-algorithm-for-a-graph-axis

    let rough_step = range / (num_steps - 1) as f64;

    let step_power = 10.0f64.powf(-rough_step.abs().log10().floor()) as f64;
    let normalized_step = rough_step * step_power;

    let good_steps = [1.0, 2.0, 5.0, 10.0];
    let good_normalized_step = good_steps.iter().find(|a| **a > normalized_step).unwrap();

    let step = good_normalized_step / step_power;


    let start_step={
        //naively find starting point.
        let aa=(range_all[0]/step).floor()*step;
        let bb=(range_all[0]/step).ceil()*step;
        if aa<bb{
            if aa<range_all[0]{
                bb
            }else{
                aa
            }
        }else{
            if bb<range_all[0]{
                aa
            }else{
                aa
            }
        }
    };
    assert!(start_step>=range_all[0]);
    

    let num_step={
        //naively find number of steps
        let mut counter=start_step;
        let mut num=0;
        loop{
            
            if counter>range_all[1]{
                break;
            }
            counter+=step;
            
            num+=1;
            
        }    
        num
        
    };
    assert!(num_step>=1);
    assert!(start_step+step*((num_step-1) as f64)<=range_all[1]);
    
    (num_step, step as f32,start_step as f32)
}

pub fn print_interval_float(a: f32,precision:f32) -> String {
    //scientific notation: m x 10n
    //let n = a.log10().floor();
    //let m = a / 10.0f32.powf(n);

    let k=(-precision.log10()).ceil();
    let k=k.max(0.0);
    

    format!("{0:.1$}",a,k as usize)
}

pub fn find_bounds(it: impl IntoIterator<Item = [f32; 2]>) -> Option<[f32; 4]> {
    let mut ii = it.into_iter();
    if let Some([x, y]) = ii.next() {
        let mut val = [x, x, y, y];
        ii.fold(&mut val, |val, [x, y]| {
            if x < val[0] {
                val[0] = x;
            } else if x > val[1] {
                val[1] = x;
            }
            if y < val[2] {
                val[2] = y;
            } else if y > val[3] {
                val[3] = y;
            }
            val
        });
        Some(val)
    } else {
        None
    }
}
