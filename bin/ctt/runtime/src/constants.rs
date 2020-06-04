/// Money

pub mod currency {
    pub type Balance = u128;

    pub const KPTS: Balance = 1_000_000_000_000;
    pub const DOLLARS: Balance = KPTS;
    pub const CENTS: Balance = DOLLARS / 100;
    pub const MILLICENTS: Balance = CENTS / 1_000;
}
