use rand::{thread_rng, Rng};


fn main() {
    let mut rng = thread_rng();
    let password: String= (0..16).map(|_|{
        let n = rng.gen_range(0..=2);
        let c= match n {
            0 => {
                let c = rng.gen_range('a'..='z') as char;//genere le charactere
                c // le retourne la minuscule
            }
            1 => {
                let c = rng.gen_range('A'..='Z') as char;
            c // le retourne la majuscule
            }
            _ => {
                let c = rng.gen_range('0'..='9') as char;
                c
            }
        };
        c
    })
    .collect::<String>();
    println!("Géneration du mot passe: {}",password);
}
