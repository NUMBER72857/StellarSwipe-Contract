use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum AutoTradeError {
    InvalidAmount = 1,
    Unauthorized = 2,
    SignalNotFound = 3,
    SignalExpired = 4,
    InsufficientBalance = 5,
    InsufficientLiquidity = 6,
    DailyTradeLimitExceeded = 7,
    PositionLimitExceeded = 8,
    StopLossTriggered = 9,
    TradingPaused = 10,
    StrategyNotFound = 11,
    PositionAlreadyExists = 12,
<<<<<<< Updated upstream
    RankingDisabled = 13,
    InvalidBasketSize = 14,
    InsufficientPriceHistory = 15,
=======
    InsufficientPriceHistory = 13,
    RankingDisabled = 14,
    InvalidBasketSize = 15,
>>>>>>> Stashed changes
    InvalidPriceData = 16,
    NonCointegratedBasket = 17,
    ActivePortfolioExists = 18,
    NoActivePortfolio = 19,
    NoTradeSignal = 20,
    InvalidStatArbConfig = 21,
    ExitStrategyNotFound = 22,
<<<<<<< Updated upstream
    InvalidExitConfig = 23,
    InsuranceNotConfigured = 24,
    InvalidInsuranceConfig = 25,
=======
>>>>>>> Stashed changes
}
