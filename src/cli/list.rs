use super::ReplResult;
use crate::{Backend, CmdExector, ReplContext, ReplDisplay, ReplMsg};
use clap::{ArgMatches, Parser};

#[derive(Debug, Parser)]
pub struct ListOpts;

pub fn list(_args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let (msg, rx) = ReplMsg::new(ListOpts);
    Ok(ctx.send(msg, rx))
}

impl CmdExector for ListOpts {
    async fn execute<T: Backend>(self, backend: &mut T) -> anyhow::Result<String> {
        let df = backend.list().await?;
        df.display().await
    }
}
