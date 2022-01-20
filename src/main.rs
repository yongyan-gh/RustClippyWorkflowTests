use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    if 100 > i32::MAX 
    {
        println!("should be wrong");
    }
    
    let x :f32 = 1.0;
    if x == f32::NAN 
    {
        println!("do not compare to NaN");
    }

    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
