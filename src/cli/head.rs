use super::{ReplCommand, ReplResult};
use crate::ReplContext;
use clap::{ArgMatches, Parser};

#[derive(Debug, Parser)]
pub struct HeadOpts {
    #[arg(short, long, help = "The name of the dataset")]
    pub name: String,

    #[arg(short, long, help = "The number of rows to show")]
    pub n: Option<usize>,
}

pub fn head(args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let name = args
        .get_one::<String>("name")
        .expect("expect name")
        .to_string();

    let n = args.get_one::<usize>("n").copied();

    let cmd = HeadOpts::new(name, n).into();
    ctx.send(cmd);

    Ok(None)
}

impl From<HeadOpts> for ReplCommand {
    fn from(opts: HeadOpts) -> Self {
        ReplCommand::Head(opts)
    }
}

impl HeadOpts {
    pub fn new(name: String, n: Option<usize>) -> Self {
        Self { name, n }
    }
}
