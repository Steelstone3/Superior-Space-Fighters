use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen)]
pub enum SpaceShipSprite {
    AstralSovereignAllRounder,
    AstralSovereignExplorer,
    AstralSovereignFighter,
    AstralSovereignTrader,
    CosmicCommerceAssemblyAllRounder,
    CosmicCommerceAssemblyExplorer,
    CosmicCommerceAssemblyFighter,
    CosmicCommerceAssemblyTrader,
    DarkstarFaelithDominionAllRounder,
    DarkstarFaelithDominionExplorer,
    DarkstarFaelithDominionFighter,
    DarkstarFaelithDominionTrader,
    FreeStarConfederacyAllRounder,
    FreeStarConfederacyExplorer,
    FreeStarConfederacyFighter,
    FreeStarConfederacyTrader,
    LumithrinAllRounder,
    MoonhoofClanAllRounder,
    MoonhoofClanExplorer,
    MoonhoofClanFighter,
    MoonhoofClanTrader,
    OuterReachMiningGuildAllRounder,
    OuterReachMiningGuildExplorer,
    OuterReachMiningGuildFighter,
    OuterReachMiningGuildTrader,
    SiliconFangTechnocracyAllRounder,
    SiliconFangTechnocracyExplorer,
    SiliconFangTechnocracyFighter,
    SiliconFangTechnocracyTrader,
    SolarisUnionAllRounder,
    SolarisUnionExplorer,
    SolarisUnionFighter,
    SolarisUnionTrader,
    ZephyrarianAllRounder,
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
            SpaceShipSprite::CosmicCommerceAssemblyAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/cosmic_commerce_assembly/all_rounder.png"
                )
            }
            SpaceShipSprite::CosmicCommerceAssemblyExplorer => {
                write!(
                    formatter,
                    "images/starships/factions/cosmic_commerce_assembly/explorer.png"
                )
            }
            SpaceShipSprite::CosmicCommerceAssemblyFighter => {
                write!(
                    formatter,
                    "images/starships/factions/cosmic_commerce_assembly/fighter.png"
                )
            }
            SpaceShipSprite::CosmicCommerceAssemblyTrader => {
                write!(
                    formatter,
                    "images/starships/factions/cosmic_commerce_assembly/trader.png"
                )
            }
            SpaceShipSprite::DarkstarFaelithDominionAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/darkstar_faelith_dominion/all_rounder.png"
                )
            }
            SpaceShipSprite::DarkstarFaelithDominionExplorer => {
                write!(
                    formatter,
                    "images/starships/factions/darkstar_faelith_dominion/ship_2.png"
                )
            }
            SpaceShipSprite::DarkstarFaelithDominionFighter => {
                write!(
                    formatter,
                    "images/starships/factions/darkstar_faelith_dominion/ship_3.png"
                )
            }
            SpaceShipSprite::DarkstarFaelithDominionTrader => {
                write!(
                    formatter,
                    "images/starships/factions/darkstar_faelith_dominion/ship_4.png"
                )
            }
            SpaceShipSprite::FreeStarConfederacyAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/free_star_confederacy/all_rounder.png"
                )
            }
            SpaceShipSprite::FreeStarConfederacyExplorer => {
                write!(
                    formatter,
                    "images/starships/factions/free_star_confederacy/ship_2.png"
                )
            }
            SpaceShipSprite::FreeStarConfederacyFighter => {
                write!(
                    formatter,
                    "images/starships/factions/free_star_confederacy/ship_3.png"
                )
            }
            SpaceShipSprite::FreeStarConfederacyTrader => {
                write!(
                    formatter,
                    "images/starships/factions/free_star_confederacy/ship_4.png"
                )
            }
            SpaceShipSprite::LumithrinAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/aliens/lumithrin_all_rounder.png"
                )
            }
            SpaceShipSprite::MoonhoofClanAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/moonhoof_clan/all_rounder.png"
                )
            }
            SpaceShipSprite::MoonhoofClanExplorer => {
                write!(
                    formatter,
                    "images/starships/factions/moonhoof_clan/ship_2.png"
                )
            }
            SpaceShipSprite::MoonhoofClanFighter => {
                write!(
                    formatter,
                    "images/starships/factions/moonhoof_clan/ship_3.png"
                )
            }
            SpaceShipSprite::MoonhoofClanTrader => {
                write!(
                    formatter,
                    "images/starships/factions/moonhoof_clan/ship_4.png"
                )
            }
            SpaceShipSprite::OuterReachMiningGuildAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/outer_reach_mining_guild/all_rounder.png"
                )
            }
            SpaceShipSprite::OuterReachMiningGuildExplorer => {
                write!(
                    formatter,
                    "images/starships/factions/outer_reach_mining_guild/ship_2.png"
                )
            }
            SpaceShipSprite::OuterReachMiningGuildFighter => {
                write!(
                    formatter,
                    "images/starships/factions/outer_reach_mining_guild/ship_3.png"
                )
            }
            SpaceShipSprite::OuterReachMiningGuildTrader => {
                write!(
                    formatter,
                    "images/starships/factions/outer_reach_mining_guild/ship_4.png"
                )
            }
            SpaceShipSprite::SiliconFangTechnocracyAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/silicon_fang_technocracy/all_rounder.png"
                )
            }
            SpaceShipSprite::SiliconFangTechnocracyExplorer => {
                write!(
                    formatter,
                    "images/starships/factions/silicon_fang_technocracy/ship_2.png"
                )
            }
            SpaceShipSprite::SiliconFangTechnocracyFighter => {
                write!(
                    formatter,
                    "images/starships/factions/silicon_fang_technocracy/ship_3.png"
                )
            }
            SpaceShipSprite::SiliconFangTechnocracyTrader => {
                write!(
                    formatter,
                    "images/starships/factions/silicon_fang_technocracy/ship_4.png"
                )
            }
            SpaceShipSprite::SolarisUnionAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/solaris_union/all_rounder.png"
                )
            }
            SpaceShipSprite::SolarisUnionExplorer => {
                write!(
                    formatter,
                    "images/starships/factions/solaris_union/ship_2.png"
                )
            }
            SpaceShipSprite::SolarisUnionFighter => {
                write!(
                    formatter,
                    "images/starships/factions/solaris_union/ship_3.png"
                )
            }
            SpaceShipSprite::SolarisUnionTrader => {
                write!(
                    formatter,
                    "images/starships/factions/solaris_union/ship_4.png"
                )
            }
            SpaceShipSprite::ZephyrarianAllRounder => {
                write!(
                    formatter,
                    "images/starships/factions/aliens/zephyrarian_all_rounder.png"
                )
            }
        }
    }
}
