use tokio::runtime::Builder;
use color_eyre::eyre::Result;

pub fn serve() ->  Result<()> {
    let rt = Builder::new_multi_thread()
        .enable_all()
        .build()?;

    Ok(())
}
