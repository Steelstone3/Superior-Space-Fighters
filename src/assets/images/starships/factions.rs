use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen)]
pub enum SpaceShipSprite {
    AstralSovereignAllRounder,
    AstralSovereignTrader,
    AstralSovereignExplorer,
    AstralSovereignFighter,
    CosmicCommerceAssemblyShip1,
    CosmicCommerceAssemblyShip2,
    CosmicCommerceAssemblyShip3,
    CosmicCommerceAssemblyShip4,
    DarkstarFaelithDominionShip1,
    DarkstarFaelithDominionShip2,
    DarkstarFaelithDominionShip3,
    DarkstarFaelithDominionShip4,
    FreeStarConfederacyShip1,
    FreeStarConfederacyShip2,
    FreeStarConfederacyShip3,
    FreeStarConfederacyShip4,
    LumithrinShip1,
    MoonhoofClanShip1,
    MoonhoofClanShip2,
    MoonhoofClanShip3,
    MoonhoofClanShip4,
    OuterReachMiningGuildShip1,
    OuterReachMiningGuildShip2,
    OuterReachMiningGuildShip3,
    OuterReachMiningGuildShip4,
    SiliconFangTechnocracyShip1,
    SiliconFangTechnocracyShip2,
    SiliconFangTechnocracyShip3,
    SiliconFangTechnocracyShip4,
    SolarisUnionShip1,
    SolarisUnionShip2,
    SolarisUnionShip3,
    SolarisUnionShip4,
    ZephyrarianShip1,
}
impl Display for SpaceShipSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceShipSprite::AstralSovereignAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/astral_sovereign/all_rounder.png"
                )
            }
            SpaceShipSprite::AstralSovereignExplorer => {
                write!(
                    formatter,
                    "images/starships/factions/astral_sovereign/explorer.png"
                )
            }
            SpaceShipSprite::AstralSovereignFighter => {
                write!(
                    formatter,
                    "images/starships/factions/astral_sovereign/fighter.png"
                )
            }
            SpaceShipSprite::AstralSovereignTrader => {
                write!(
                    formatter,
                    "images/starships/factions/astral_sovereign/trader.png"
                )
            }
            SpaceShipSprite::CosmicCommerceAssemblyShip1 => {
                write!(
                    formatter,
                    "images/starships/factions/cosmic_commerce_assembly/ship_1.png"
                )
            }
            SpaceShipSprite::CosmicCommerceAssemblyShip2 => {
                write!(
                    formatter,
                    "images/starships/factions/cosmic_commerce_assembly/ship_2.png"
                )
            }
            SpaceShipSprite::CosmicCommerceAssemblyShip3 => {
                write!(
                    formatter,
                    "images/starships/factions/cosmic_commerce_assembly/ship_3.png"
                )
            }
            SpaceShipSprite::CosmicCommerceAssemblyShip4 => {
                write!(
                    formatter,
                    "images/starships/factions/cosmic_commerce_assembly/ship_4.png"
                )
            }
            SpaceShipSprite::DarkstarFaelithDominionShip1 => {
                write!(
                    formatter,
                    "images/starships/factions/darkstar_faelith_dominion/ship_1.png"
                )
            }
            SpaceShipSprite::DarkstarFaelithDominionShip2 => {
                write!(
                    formatter,
                    "images/starships/factions/darkstar_faelith_dominion/ship_2.png"
                )
            }
            SpaceShipSprite::DarkstarFaelithDominionShip3 => {
                write!(
                    formatter,
                    "images/starships/factions/darkstar_faelith_dominion/ship_3.png"
                )
            }
            SpaceShipSprite::DarkstarFaelithDominionShip4 => {
                write!(
                    formatter,
                    "images/starships/factions/darkstar_faelith_dominion/ship_4.png"
                )
            }
            SpaceShipSprite::FreeStarConfederacyShip1 => {
                write!(
                    formatter,
                    "images/starships/factions/free_star_confederacy/ship_1.png"
                )
            }
            SpaceShipSprite::FreeStarConfederacyShip2 => {
                write!(
                    formatter,
                    "images/starships/factions/free_star_confederacy/ship_2.png"
                )
            }
            SpaceShipSprite::FreeStarConfederacyShip3 => {
                write!(
                    formatter,
                    "images/starships/factions/free_star_confederacy/ship_3.png"
                )
            }
            SpaceShipSprite::FreeStarConfederacyShip4 => {
                write!(
                    formatter,
                    "images/starships/factions/free_star_confederacy/ship_4.png"
                )
            }
            SpaceShipSprite::LumithrinShip1 => {
                write!(
                    formatter,
                    "images/starships/factions/aliens/lumithrin_ship_1.png"
                )
            }
            SpaceShipSprite::MoonhoofClanShip1 => {
                write!(
                    formatter,
                    "images/starships/factions/moonhoof_clan/ship_1.png"
                )
            }
            SpaceShipSprite::MoonhoofClanShip2 => {
                write!(
                    formatter,
                    "images/starships/factions/moonhoof_clan/ship_2.png"
                )
            }
            SpaceShipSprite::MoonhoofClanShip3 => {
                write!(
                    formatter,
                    "images/starships/factions/moonhoof_clan/ship_3.png"
                )
            }
            SpaceShipSprite::MoonhoofClanShip4 => {
                write!(
                    formatter,
                    "images/starships/factions/moonhoof_clan/ship_4.png"
                )
            }
            SpaceShipSprite::OuterReachMiningGuildShip1 => {
                write!(
                    formatter,
                    "images/starships/factions/outer_reach_mining_guild/ship_1.png"
                )
            }
            SpaceShipSprite::OuterReachMiningGuildShip2 => {
                write!(
                    formatter,
                    "images/starships/factions/outer_reach_mining_guild/ship_2.png"
                )
            }
            SpaceShipSprite::OuterReachMiningGuildShip3 => {
                write!(
                    formatter,
                    "images/starships/factions/outer_reach_mining_guild/ship_3.png"
                )
            }
            SpaceShipSprite::OuterReachMiningGuildShip4 => {
                write!(
                    formatter,
                    "images/starships/factions/outer_reach_mining_guild/ship_4.png"
                )
            }
            SpaceShipSprite::SiliconFangTechnocracyShip1 => {
                write!(
                    formatter,
                    "images/starships/factions/silicon_fang_technocracy/ship_1.png"
                )
            }
            SpaceShipSprite::SiliconFangTechnocracyShip2 => {
                write!(
                    formatter,
                    "images/starships/factions/silicon_fang_technocracy/ship_2.png"
                )
            }
            SpaceShipSprite::SiliconFangTechnocracyShip3 => {
                write!(
                    formatter,
                    "images/starships/factions/silicon_fang_technocracy/ship_3.png"
                )
            }
            SpaceShipSprite::SiliconFangTechnocracyShip4 => {
                write!(
                    formatter,
                    "images/starships/factions/silicon_fang_technocracy/ship_4.png"
                )
            }
            SpaceShipSprite::SolarisUnionShip1 => {
                write!(
                    formatter,
                    "images/starships/factions/solaris_union/ship_1.png"
                )
            }
            SpaceShipSprite::SolarisUnionShip2 => {
                write!(
                    formatter,
                    "images/starships/factions/solaris_union/ship_2.png"
                )
            }
            SpaceShipSprite::SolarisUnionShip3 => {
                write!(
                    formatter,
                    "images/starships/factions/solaris_union/ship_3.png"
                )
            }
            SpaceShipSprite::SolarisUnionShip4 => {
                write!(
                    formatter,
                    "images/starships/factions/solaris_union/ship_4.png"
                )
            }
            SpaceShipSprite::ZephyrarianShip1 => {
                write!(
                    formatter,
                    "images/starships/factions/aliens/zephyrarian_ship_1.png"
                )
            }
        }
    }
}
