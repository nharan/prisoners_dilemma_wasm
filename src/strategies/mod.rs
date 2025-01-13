
pub mod registry;

pub mod AlwaysCooperate;
pub mod Extortionary;
pub mod AlwaysDefect;
pub mod TitForTat;
pub mod SuspiciousTitForTat;
pub mod NPavlov;
pub mod Pavlov;
pub mod GenerousTitForTat;
pub mod Random;
pub mod BetaTitForTat;

pub fn register_strategies() {
    registry::register_strategy(Box::new(AlwaysCooperate::AlwaysCooperate::new()));
    registry::register_strategy(Box::new(Extortionary::Extortionary::new()));
    registry::register_strategy(Box::new(AlwaysDefect::AlwaysDefect::new()));
    registry::register_strategy(Box::new(TitForTat::TitForTat::new()));
    registry::register_strategy(Box::new(SuspiciousTitForTat::SuspiciousTitForTat::new()));
    registry::register_strategy(Box::new(NPavlov::NPavlov::new()));
    registry::register_strategy(Box::new(Pavlov::Pavlov::new()));
    registry::register_strategy(Box::new(GenerousTitForTat::GenerousTitForTat::new()));
    registry::register_strategy(Box::new(Random::Random::new()));
    registry::register_strategy(Box::new(BetaTitForTat::BetaTitForTat::new()));
}
