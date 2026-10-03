fn main(){ let a=std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap(); let r=md_press::format_plain(&a);
 let (x,y)=(md_press::render_fingerprint(&a), md_press::render_fingerprint(&r.gate_output));
 let i = x.chars().zip(y.chars()).take_while(|(p,q)| p==q).count();
 let xs:String=x.chars().skip(i.saturating_sub(200)).take(400).collect(); let ys:String=y.chars().skip(i.saturating_sub(200)).take(400).collect();
 println!("first diff at char {i}\nIN : {xs}\n\nOUT: {ys}");
 // source-level diff lines
 for (n,(l1,l2)) in a.lines().zip(r.gate_output.lines()).enumerate() { if l1!=l2 { println!("src line {}: {:?}\n         -> {:?}", n+1, l1, l2); break; } }
}
