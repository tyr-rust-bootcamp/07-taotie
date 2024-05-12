use super::ReplResult;
use crate::{Backend, CmdExector, ReplContext, ReplDisplay, ReplMsg};
use clap::{ArgMatches, Parser};

#[derive(Debug, Parser)]
pub struct SchemaOpts {
    #[arg(help = "The name of the dataset")]
    pub name: String,
}

pub fn schema(args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let name = args
        .get_one::<String>("name")
        .expect("expect name")
        .to_string();

    let (msg, rx) = ReplMsg::new(SchemaOpts::new(name));
    Ok(ctx.send(msg, rx))
}

impl SchemaOpts {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl CmdExector for SchemaOpts {
    async fn execute<T: Backend>(self, backend: &mut T) -> anyhow::Result<String> {
        let df = backend.schema(&self.name).await?;
        df.display().await
    }
}
