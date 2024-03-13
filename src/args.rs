use clap::Parser;
use rust_decimal::Decimal;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    base_code: String,
    #[arg(short, long)]
    target_code: String,
    #[arg(short, long)]
    amount: Decimal,
    #[arg(short, long, default_value_t = false)]
    list: bool,
}

impl Args {
    pub fn parse() -> Self {
        Parser::parse()
    }
    pub fn base_code(&self) -> &String {
        &self.base_code
    }
    pub fn target_code(&self) -> &String {
        &self.target_code
    }
    pub fn amount(&self) -> Decimal {
        self.amount
    }
    pub fn is_list_enabled(&self) -> bool {
        self.list
    }
}