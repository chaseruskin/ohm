mod ohm;

use clif::{Cli, ErrorKind};
use ohm::Ohm;
use clif::cmd::{Runner, Command, FromCli};
use crayon::Color;

pub fn go() -> u8 {
    // set-up the command-line interface
    let mut cli = Cli::new()
        .emphasize_help()
        .threshold(2)
        .tokenize(std::env::args());

    // parse the command-line interface
    let ohm = match Ohm::from_cli(&mut cli) {
        Ok(app) => {
            std::mem::drop(cli);
            app
        },
        Err(err) => {
            match err.kind() {
                ErrorKind::Help => println!("{}", err),
                _ => eprintln!("{}: {}", "error".red().bold(), err),
            }
            return err.code()
        }
    };
    // execute the backend program
    ohm.exec(&())
}

impl Runner<()> for Ohm {}

impl Command<()> for Ohm {
    type Status = u8;
    fn exec(&self, _: &()) -> <Self as clif::cmd::Command<()>>::Status { 

        let resistance = self.compute();
        println!("resistance: {}", resistance);
        0
    }
}


