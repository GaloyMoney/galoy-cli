use clap::{Parser, Subcommand};

use crate::client::types::{ReceiveVia, Wallet};
use rust_decimal::Decimal;

#[derive(Parser)]
#[clap(
    version,
    author = "Galoy",
    about = "Galoy CLI",
    long_about = "CLI client to interact with Galoy's APIs"
)]
pub struct Cli {
    #[clap(
        long,
        env = "GALOY_API",
        default_value = "http://localhost:4455/graphql"
    )]
    pub api: String,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Get global values from the instance
    Globals,
    /// Get auth token of an account
    Login {
        #[clap(short, long, value_parser)]
        phone: Option<String>,
        #[clap(long, conflicts_with("phone"))]
        email: bool,
        #[clap(short, long)]
        code: String,
        #[clap(short = 't', long = "two-fa-code", value_parser)]
        two_fa_code: Option<String>,
    },
    /// Logout the current user by removing the auth token
    Logout,
    /// Execute Me query
    Me,
    /// Get WalletId for an account
    DefaultWallet {
        #[clap(value_parser)]
        username: String,
    },
    // Update the default wallet of an account
    SetDefaultWallet {
        #[clap(short, long, value_parser, conflicts_with("wallet_id"))]
        wallet: Option<Wallet>,
        #[clap(long)]
        wallet_id: Option<String>,
    },
    /// Set a username for a new account
    SetUsername {
        #[clap(short, long)]
        username: String,
    },
    /// Fetch the balance of a wallet
    Balance {
        #[clap(long)]
        btc: bool,
        #[clap(long)]
        usd: bool,
        #[clap(long, use_value_delimiter = true)]
        wallet_ids: Vec<String>,
    },
    /// Execute a Payment
    Pay {
        #[clap(short, long)]
        username: Option<String>,
        #[clap(short, long, conflicts_with("username"))]
        onchain_address: Option<String>,
        #[clap(short, long, value_parser)]
        wallet: Wallet,
        #[clap(short, long, required_if_eq("wallet", "usd"))]
        cents: Option<Decimal>,
        #[clap(short, long, required_if_eq("wallet", "btc"))]
        sats: Option<Decimal>,
        #[clap(short, long)]
        memo: Option<String>,
    },
    /// Receive a Payment
    Receive {
        #[clap(short, long, value_parser)]
        wallet: Wallet,
        #[clap(short, long, value_parser)]
        via: ReceiveVia,
    },
    ///
    ///
    /// Execute a batch payment
    ///
    /// This command allows you to perform multiple payments at once using a CSV file. Each line in the CSV file represents a single payment transaction.
    /// Information About Columns Required in the CSV File:
    ///
    /// - username:
    ///     This column determines the Blink username of the recipient.
    ///
    /// - amount:
    ///     Amount that will be sent.
    ///
    /// - currency:
    ///     Indicates the currency of the amount. Can be either USD or SATS. Note: SATS currency only works with BTC wallets.
    ///
    /// - wallet (optional values):
    ///     Specifies the wallet (USD or BTC) to be used for sending the payment. If not provided, the default wallet is used.
    ///
    /// - memo (optional values):
    ///     A message or note to attach with the payment.
    ///
    /// *--* ALL HEADERS NEEDS TO BE IN THIS ORDER, BUT VALUES ARE OPTIONAL FOR "wallet" AND "memo" *--*
    ///
    ///
    /// EXAMPLE :-
    ///
    /// | username  | amount | currency | wallet | memo   |
    ///
    /// |-----------|--------|----------|--------|--------|
    ///
    /// |   user a  |   12   |   USD    |   USD  |        |
    ///
    /// |   user b  |   10   |   SATS   |   BTC  |  memo  |
    ///
    /// |   user a  |   14   |   USD    |   BTC  |        |
    ///
    ///
    Batch {
        #[clap(short, long = "csv")]
        file: String,
        #[clap(action, long)]
        skip_confirmation: bool,
    },
    /// Request a code from a Phone number or Email
    RequestCode {
        #[clap(short, long, value_parser, conflicts_with("email"))]
        phone: Option<String>,
        #[clap(short, long, value_parser)]
        email: Option<String>,
    },
}
