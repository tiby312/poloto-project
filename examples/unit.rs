

#[derive(Debug)]
struct Res{
    use_scientific:bool,
    precision:usize
}
fn write_data(low:f32,heigh:f32,step:f32)->Res{
    //const SCIENCE: usize = 4;
    //if a != 0.0 && a.abs().log10().floor().abs() > SCIENCE as f32 {
    //    write!(fm, "{0:.1$e}", a, 2)?
    //} else {
    let k = (step.log10()).ceil();
    dbg!(k);
    let k = k.max(0.0);

    Res{
        use_scientific:false,
        precision:k as usize
    }
}

fn main(){
    dbg!(write_data(1000.0,200.0,1000.0));
}