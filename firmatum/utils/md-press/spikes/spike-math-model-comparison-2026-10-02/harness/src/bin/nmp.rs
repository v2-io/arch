use std::io::BufRead;
fn main(){ for l in std::io::stdin().lock().lines(){ let l=l.unwrap(); println!("{}\t{:?}\t{}", md_press::math::needs_math_pass(&l), md_press::math::protected_ranges(&l), l.chars().take(90).collect::<String>()); } }
