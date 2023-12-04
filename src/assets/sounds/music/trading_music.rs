use core::fmt::Display;
use rand_derive2::RandGen;

#[derive(Default, RandGen)]
pub enum TradingMusicSound {
    #[default]
    Trading1,
    Trading2,
    Trading3,
    Trading4,
    Trading5,
}

impl Display for TradingMusicSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradingMusicSound::Trading1 => {
                write!(formatter, "sounds/music/trading/trading_1.ogg")
            }
            TradingMusicSound::Trading2 => {
                write!(formatter, "sounds/music/trading/trading_2.ogg")
            }
            TradingMusicSound::Trading3 => {
                write!(formatter, "sounds/music/trading/trading_3.ogg")
            }
            TradingMusicSound::Trading4 => {
                write!(formatter, "sounds/music/trading/trading_4.ogg")
            }
            TradingMusicSound::Trading5 => {
                write!(formatter, "sounds/music/trading/trading_5.ogg")
            }
        }
    }
}
