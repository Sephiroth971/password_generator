use clap::Parser;

#[derive(Parser)]
#[command(name ="pwgen",version ="1.0",author = "Sephiroth", about= "générateur de mot de passe")]
pub struct Args{
     /// Longueur du mot de passe
    #[arg(short, long, default_value_t = 16)]
    pub l: usize,
}