mod args;

use args::Args;
use clap::Parser;
use rand::{thread_rng, Rng};


fn main() {
    let mut rng = thread_rng();
    let args = Args::parse();
    let password: String= (0..args.l).map(|_|{
        let n = rng.gen_range(0..=3);
        let c= match n {
            0 => {
                let c = rng.gen_range('a'..='z') as char;//genere le charactere
                c // le retourne la minuscule
            }
            1 => {
                let c = rng.gen_range('A'..='Z') as char;
            c // le retourne la majuscule
            }
            2 => {
                let c = rng.gen_range('0'..='9') as char;
                c
            }
            _ => {
                let specials = "!@#$%&*()-_=+[]?/<>";
                specials.chars().nth(rng.gen_range(0..specials.len())).unwrap() //choix du cararteres dans la liste 
            }
        };
        c
    })
    .collect::<String>();
    println!("Géneration du mot passe: {}",password);
}
