use clap::Args;

#[derive(Args, Debug)]
pub struct Chpwd;

impl Chpwd {
    pub fn chpwd(&self) {
        println!("export RV_CHECK=1")
    }
}
