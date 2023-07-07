use clap::Parser;

use crate::app::App;
use crate::cli::commands::{Cli, Command};

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let app = App::new(cli.api)?;

    match cli.command {
        Command::Globals => {
            app.globals().await?;
        }
        Command::Login { phone, code } => {
            app.user_login(phone, code).await?;
        }
        Command::Logout => {
            app.user_logout().await?;
        }
        Command::Me => {
            app.me().await?;
        }
        Command::DefaultWallet { username } => {
            app.default_wallet(username).await?;
        }
        Command::Balance {
            btc,
            usd,
            wallet_ids,
        } => {
            app.wallet_balance(btc, usd, wallet_ids).await?;
        }
        Command::SetUsername { username } => {
            app.set_username(username).await?;
        }
        Command::Pay {
            username,
            wallet,
            cents,
            sats,
            memo,
        } => {
            app.intraledger_payment(username, wallet, cents, sats, memo)
                .await?;
        }
        Command::Batch { file } => {
            app.batch_payment(file).await?;
        }
    }

    Ok(())
}
