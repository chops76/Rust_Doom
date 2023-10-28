//build.rs
use std::io::{Result, Write};
use std::path::Path;
use std::fs::File;
use std::env;

const MAX: usize = 360;

fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap() ;
    let dest_path = Path::new(&out_dir).join("sin_cos.rs") ;
    let mut f = File::create(&dest_path).unwrap() ;

    write!(f, "const SIN: [f64; {}] = [\n",MAX)? ;
    for i in 0..MAX {
        let rad = i as f64 / 180.0 * std::f64::consts::PI;
        write!(f, "  {}f64,\n", rad.sin())?;
    }
    write!(f, "];\n\n\n")? ;

    write!(f, "const COS: [f64; {}] = [\n",MAX)? ;
    for i in 0..MAX {
        let rad = i as f64 / 180.0 * std::f64::consts::PI;
        write!(f, "  {}f64,\n", rad.cos())?;
    }
    write!(f, "];\n\n\n")? ;

    Ok(())
}

