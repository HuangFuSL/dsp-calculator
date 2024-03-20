use crate::abc::data::{Proliferator, Recipe, RecipeType, ProductionBuildingProperties};
use phf::{Map, phf_map};

// Icons

pub const DUMMY_ID: u16 = 0;
pub const DUMMY_ICON: &'static [u8] = include_bytes!("../assets/icons/dummy.png");

pub const ICON_MAP: Map<u16, &'static [u8]> = phf_map!{
    0u16 => DUMMY_ICON,
    // Row 1
    1u16 => include_bytes!("../assets/icons/iron-ore.png"), // Iron Ore
    2u16 => include_bytes!("../assets/icons/copper-ore.png"), // Copper Ore
    3u16 => include_bytes!("../assets/icons/coal.png"), // Coal
    4u16 => include_bytes!("../assets/icons/stone.png"), // Stone
    5u16 => include_bytes!("../assets/icons/silicon-ore.png"), // Silicon Ore
    6u16 => include_bytes!("../assets/icons/titanium-ore.png"), // Titanium Ore
    7u16 => include_bytes!("../assets/icons/water.png"), // Water
    8u16 => include_bytes!("../assets/icons/crude-oil.png"), // Crude Oil
    9u16 => include_bytes!("../assets/icons/hydrogen.png"), // Hydrogen
    10u16 => include_bytes!("../assets/icons/deuterium.png"), // Deuterium
    11u16 => include_bytes!("../assets/icons/antimatter.png"), // Antimatter
    12u16 => include_bytes!("../assets/icons/core-element.png"), // Core Element
    13u16 => include_bytes!("../assets/icons/critical-photon.png"), // Critical Photon
    14u16 => include_bytes!("../assets/icons/kimberlite-ore.png"), // Kimberlite Ore
    // Row 2
    15u16 => include_bytes!("../assets/icons/iron-ingot.png"), // Iron Ingot
    16u16 => include_bytes!("../assets/icons/copper-ingot.png"), // Copper Ingot
    17u16 => include_bytes!("../assets/icons/stone-brick.png"), // Stone Brick
    18u16 => include_bytes!("../assets/icons/energetic-graphite.png"), // Energetic Graphite
    19u16 => include_bytes!("../assets/icons/high-purity-silicon.png"), // High-Purity Silicon
    20u16 => include_bytes!("../assets/icons/titanium-ingot.png"), // Titanium Ingot
    21u16 => include_bytes!("../assets/icons/sulfuric-acid.png"), // Sulfuric Acid
    22u16 => include_bytes!("../assets/icons/refined-oil.png"), // Refined Oil
    23u16 => include_bytes!("../assets/icons/hydrogen-fuel-rod.png"), // Hydrogen Fuel Rod
    24u16 => include_bytes!("../assets/icons/deuterium-fuel-rod.png"), // Deuterium Fuel Rod
    25u16 => include_bytes!("../assets/icons/antimatter-fuel-rod.png"), // Antimatter Fuel Rod
    26u16 => include_bytes!("../assets/icons/strange-annihilation-fuel-rod.png"), // Strange Annihilation Fuel Rod
    27u16 => include_bytes!("../assets/icons/missile-set.png"), // Missile Set
    28u16 => include_bytes!("../assets/icons/fractal-silicon.png"), // Fractal Silicon
    // Row 3
    29u16 => include_bytes!("../assets/icons/magnet.png"), // Magnet
    30u16 => include_bytes!("../assets/icons/magnetic-coil.png"), // Magnetic Coil
    31u16 => include_bytes!("../assets/icons/glass.png"), // Glass
    32u16 => include_bytes!("../assets/icons/diamond.png"), // Diamond
    33u16 => include_bytes!("../assets/icons/crystal-silicon.png"), // Crystal Silicon
    34u16 => include_bytes!("../assets/icons/titanium-alloy.png"), // Titanium Alloy
    35u16 => include_bytes!("../assets/icons/combustible-unit.png"), // Combustible Unit
    36u16 => include_bytes!("../assets/icons/plastic.png"), // Plastic
    37u16 => include_bytes!("../assets/icons/organic-crystal.png"), // Organic Crystal
    38u16 => include_bytes!("../assets/icons/graphene.png"), // Graphene
    39u16 => include_bytes!("../assets/icons/annihilation-constraint-sphere.png"), // Annihilation Constraint Sphere
    40u16 => include_bytes!("../assets/icons/magnum-ammo-box.png"), // Magnum Ammo Box
    41u16 => include_bytes!("../assets/icons/supersonic-missile-set.png"), // Supersonic Missile Set
    42u16 => include_bytes!("../assets/icons/grating-crystal.png"), // Grating Crystal
    // Row 4
    43u16 => include_bytes!("../assets/icons/steel.png"), // Steel
    44u16 => include_bytes!("../assets/icons/circuit-board.png"), // Circuit Board
    45u16 => include_bytes!("../assets/icons/prism.png"), // Prism
    46u16 => include_bytes!("../assets/icons/electric-motor.png"), // Electric Motor
    47u16 => include_bytes!("../assets/icons/microcrystalline-component.png"), // Microcrystalline Component
    48u16 => include_bytes!("../assets/icons/proliferator-mk.i.png"), // Proliferator Mk.I
    49u16 => include_bytes!("../assets/icons/explosive-unit.png"), // Explosive Unit
    50u16 => include_bytes!("../assets/icons/strange-matter.png"), // Strange Matter
    51u16 => include_bytes!("../assets/icons/titanium-crystal.png"), // Titanium Crystal
    52u16 => include_bytes!("../assets/icons/carbon-nanotube.png"), // Carbon Nanotube
    53u16 => include_bytes!("../assets/icons/particle-broadband.png"), // Particle Broadband
    54u16 => include_bytes!("../assets/icons/titanium-ammo-box.png"), // Titanium Ammo Box
    55u16 => include_bytes!("../assets/icons/gravity-missile-set.png"), // Gravity Missile Set
    56u16 => include_bytes!("../assets/icons/stalagmite-crystal.png"), // Stalagmite Crystal
    // Row 5
    57u16 => include_bytes!("../assets/icons/gear.png"), // Gear
    58u16 => include_bytes!("../assets/icons/plasma-exciter.png"), // Plasma Exciter
    59u16 => include_bytes!("../assets/icons/photon-combiner.png"), // Photon Combiner
    60u16 => include_bytes!("../assets/icons/electromagnetic-turbine.png"), // Electromagnetic Turbine
    61u16 => include_bytes!("../assets/icons/processor.png"), // Processor
    62u16 => include_bytes!("../assets/icons/proliferator-mk.ii.png"), // Proliferator Mk.II
    63u16 => include_bytes!("../assets/icons/crystal-explosive-unit.png"), // Crystal Explosive Unit
    64u16 => include_bytes!("../assets/icons/casimir-crystal.png"), // Casimir Crystal
    65u16 => include_bytes!("../assets/icons/titanium-glass.png"), // Titanium Glass
    66u16 => include_bytes!("../assets/icons/plane-filter.png"), // Plane Filter
    67u16 => include_bytes!("../assets/icons/quantum-chip.png"), // Quantum Chip
    68u16 => include_bytes!("../assets/icons/superalloy-ammo-box.png"), // Superalloy Ammo Box
    69u16 => include_bytes!("../assets/icons/shell-set.png"), // Shell Set
    70u16 => include_bytes!("../assets/icons/unipolar-magnet.png"), // Unipolar Magnet
    // Row 6
    71u16 => include_bytes!("../assets/icons/engine.png"), // Engine
    72u16 => include_bytes!("../assets/icons/thruster.png"), // Thruster
    73u16 => include_bytes!("../assets/icons/reinforced-thruster.png"), // Reinforced Thruster
    74u16 => include_bytes!("../assets/icons/super-magnetic-ring.png"), // Super Magnetic Ring
    75u16 => include_bytes!("../assets/icons/particle-container.png"), // Particle Container
    76u16 => include_bytes!("../assets/icons/proliferator-mk.iii.png"), // Proliferator Mk.III
    77u16 => include_bytes!("../assets/icons/prototype.png"), // Prototype
    78u16 => include_bytes!("../assets/icons/precision-drone.png"), // Precision Drone
    79u16 => include_bytes!("../assets/icons/attack-drone.png"), // Attack Drone
    80u16 => include_bytes!("../assets/icons/corvette.png"), // Corvette
    81u16 => include_bytes!("../assets/icons/destroyer.png"), // Destroyer
    82u16 => include_bytes!("../assets/icons/plasma-capsule.png"), // Plasma Capsule
    83u16 => include_bytes!("../assets/icons/high-explosive-shell-set.png"), // High-Explosive Shell Set
    84u16 => include_bytes!("../assets/icons/fire-ice.png"), // Fire Ice
    // Row 7
    85u16 => include_bytes!("../assets/icons/logistics-bot.png"), // Logistics Bot
    86u16 => include_bytes!("../assets/icons/logistics-drone.png"), // Logistics Drone
    87u16 => include_bytes!("../assets/icons/interstellar-logistics-vessel.png"), // Interstellar Logistics Vessel
    88u16 => include_bytes!("../assets/icons/space-warper.png"), // Space Warper
    89u16 => include_bytes!("../assets/icons/gravity-lens.png"), // Gravity Lens
    90u16 => include_bytes!("../assets/icons/foundation.png"), // Foundation
    // Empty slot
    92u16 => include_bytes!("../assets/icons/solar-sail.png"), // Solar Sail
    93u16 => include_bytes!("../assets/icons/frame-material.png"), // Frame Material
    94u16 => include_bytes!("../assets/icons/dyson-sphere-component.png"), // Dyson Sphere Component
    95u16 => include_bytes!("../assets/icons/small-carrier-rocket.png"), // Small Carrier Rocket
    96u16 => include_bytes!("../assets/icons/antimatter-capsule.png"), // Antimatter Capsule
    97u16 => include_bytes!("../assets/icons/crystal-shell-set.png"), // Crystal Shell Set
    98u16 => include_bytes!("../assets/icons/log.png"), // Log
    // Row 8
    99u16 => include_bytes!("../assets/icons/electromagnetic-matrix.png"), // Electromagnetic Matrix
    100u16 => include_bytes!("../assets/icons/energy-matrix.png"), // Energy Matrix
    101u16 => include_bytes!("../assets/icons/structure-matrix.png"), // Structure Matrix
    102u16 => include_bytes!("../assets/icons/information-matrix.png"), // Information Matrix
    103u16 => include_bytes!("../assets/icons/gravity-matrix.png"), // Gravity Matrix
    104u16 => include_bytes!("../assets/icons/universe-matrix.png"), // Universe Matrix
    105u16 => include_bytes!("../assets/icons/dark-fog-matrix.png"), // Dark Fog Matrix
    106u16 => include_bytes!("../assets/icons/energy-shard.png"), // Energy Shard
    107u16 => include_bytes!("../assets/icons/silicon-based-neuron.png"), // Silicon-Based Neuron
    108u16 => include_bytes!("../assets/icons/negentropy-singularity.png"), // Negentropy Singularity
    109u16 => include_bytes!("../assets/icons/matter-recombinator.png"), // Matter Recombinator
    110u16 => include_bytes!("../assets/icons/jamming-capsule.png"), // Jamming Capsule
    111u16 => include_bytes!("../assets/icons/suppressing-capsule.png"), // Suppressing Capsule
    112u16 => include_bytes!("../assets/icons/plant-fuel.png"), // Plant Fuel
    // Buildings
    1001u16 => include_bytes!("../assets/icons/tesla-tower.png"), // Tesla Tower
    1002u16 => include_bytes!("../assets/icons/wireless-power-tower.png"), // Wireless Power Tower
    1003u16 => include_bytes!("../assets/icons/satellite-substation.png"), // Satellite Substation
    1004u16 => include_bytes!("../assets/icons/wind-turbine.png"), // Wind Turbine
    1005u16 => include_bytes!("../assets/icons/thermal-power-station.png"), // Thermal Power Station
    1006u16 => include_bytes!("../assets/icons/solar-panel.png"), // Solar Panel
    1007u16 => include_bytes!("../assets/icons/geothermal-power-station.png"), // Geothermal Power Station
    1008u16 => include_bytes!("../assets/icons/mini-fusion-power-station.png"), // Mini Fusion Power Station
    1009u16 => include_bytes!("../assets/icons/energy-exchanger.png"), // Energy Exchanger
    1010u16 => include_bytes!("../assets/icons/accumulator.png"), // Accumulator
    1011u16 => include_bytes!("../assets/icons/accumulator-full.png"), // Accumulator (Full)
    1012u16 => include_bytes!("../assets/icons/ray-receiver.png"), // Ray Receiver
    1013u16 => include_bytes!("../assets/icons/artificial-star.png"), // Artificial Star
    // Row 2
    1015u16 => include_bytes!("../assets/icons/conveyor-belt-mk.i.png"), // Conveyor Belt Mk.I
    1016u16 => include_bytes!("../assets/icons/conveyor-belt-mk.ii.png"), // Conveyor Belt Mk.II
    1017u16 => include_bytes!("../assets/icons/conveyor-belt-mk.iii.png"), // Conveyor Belt Mk.III
    1018u16 => include_bytes!("../assets/icons/splitter.png"), // Splitter
    1019u16 => include_bytes!("../assets/icons/automatic-piler.png"), // Automatic Piler
    1020u16 => include_bytes!("../assets/icons/traffic-monitor.png"), // Traffic Monitor
    1021u16 => include_bytes!("../assets/icons/spray-coater.png"), // Spray Coater
    1022u16 => include_bytes!("../assets/icons/depot-mk.i.png"), // Depot Mk.I
    1023u16 => include_bytes!("../assets/icons/depot-mk.ii.png"), // Depot Mk.II
    1024u16 => include_bytes!("../assets/icons/storage-tank.png"), // Storage Tank
    1025u16 => include_bytes!("../assets/icons/logistics-distributor.png"), // Logistics Distributor
    1026u16 => include_bytes!("../assets/icons/planetary-logistics-station.png"), // Planetary Logistics Station
    1027u16 => include_bytes!("../assets/icons/interstellar-logistics-station.png"), // Interstellar Logistics Station
    1028u16 => include_bytes!("../assets/icons/orbital-collector.png"), // Orbital Collector
    // Row 3
    1029u16 => include_bytes!("../assets/icons/sorter-mk.i.png"), // Sorter Mk.I
    1030u16 => include_bytes!("../assets/icons/sorter-mk.ii.png"), // Sorter Mk.II
    1031u16 => include_bytes!("../assets/icons/sorter-mk.iii.png"), // Sorter Mk.III
    1032u16 => include_bytes!("../assets/icons/pile-sorter.png"), // Pile Sorter
    1033u16 => include_bytes!("../assets/icons/mining-machine.png"), // Mining Machine
    1034u16 => include_bytes!("../assets/icons/advanced-mining-machine.png"), // Advanced Mining Machine
    1035u16 => include_bytes!("../assets/icons/water-pump.png"), // Water Pump
    1036u16 => include_bytes!("../assets/icons/oil-extractor.png"), // Oil Extractor
    1037u16 => include_bytes!("../assets/icons/oil-refinery.png"), // Oil Refinery
    1038u16 => include_bytes!("../assets/icons/fractionator.png"), // Fractionator
    1039u16 => include_bytes!("../assets/icons/chemical-plant.png"), // Chemical Plant
    1040u16 => include_bytes!("../assets/icons/quantum-chemical-plant.png"), // Quantum Chemical Plant
    1041u16 => include_bytes!("../assets/icons/miniature-particle-collider.png"), // Miniature Particle Collider
    // Row 4
    1043u16 => include_bytes!("../assets/icons/arc-smelter.png"), // Arc Smelter
    1044u16 => include_bytes!("../assets/icons/plane-smelter.png"), // Plane Smelter
    1045u16 => include_bytes!("../assets/icons/negentropy-smelter.png"), // Negentropy Smelter
    1046u16 => include_bytes!("../assets/icons/assembling-machine-mk.i.png"), // Assembling Machine Mk.I
    1047u16 => include_bytes!("../assets/icons/assembling-machine-mk.ii.png"), // Assembling Machine Mk.II
    1048u16 => include_bytes!("../assets/icons/assembling-machine-mk.iii.png"), // Assembling Machine Mk.III
    1049u16 => include_bytes!("../assets/icons/re-composing-assembler.png"), // Re-Composing Assembler
    1050u16 => include_bytes!("../assets/icons/matrix-lab.png"), // Matrix Lab
    1051u16 => include_bytes!("../assets/icons/self-evolution-lab.png"), // Self-Evolution Lab
    1052u16 => include_bytes!("../assets/icons/em-rail-ejector.png"), // EM Rail Ejector
    1053u16 => include_bytes!("../assets/icons/vertical-launch-silo.png"), // Vertical Launch Silo
    // Row 5
    1057u16 => include_bytes!("../assets/icons/gauss-turret.png"), // Gauss Turret
    1058u16 => include_bytes!("../assets/icons/missile-turret.png"), // Missile Turret
    1059u16 => include_bytes!("../assets/icons/implosion-cannon.png"), // Implosion Cannon
    1060u16 => include_bytes!("../assets/icons/laser-turret.png"), // Laser Turret
    1061u16 => include_bytes!("../assets/icons/plasma-turret.png"), // Plasma Turret
    1062u16 => include_bytes!("../assets/icons/sr-plasma-turret.png"), // SR Plasma Turret
    1063u16 => include_bytes!("../assets/icons/battlefield-analysis-base.png"), // Battlefield Analysis Base
    1064u16 => include_bytes!("../assets/icons/jammer-tower.png"), // Jammer Tower
    1065u16 => include_bytes!("../assets/icons/signal-tower.png"), // Signal Tower
    1066u16 => include_bytes!("../assets/icons/planetary-shield-generator.png"), // Planetary Shield Generator
    // Dummy buildings
    2001u16 => include_bytes!("../assets/icons/icarus.png"), // Icarus
    2002u16 => include_bytes!("../assets/icons/planetary-base.png"), // Planetary Base
};

// Translations

pub const TRANSLATION_EN: Map<u16, &'static str> = phf_map!{
    // Row 1
    1u16 => "Iron Ore",
    2u16 => "Copper Ore",
    3u16 => "Coal",
    4u16 => "Stone",
    5u16 => "Silicon Ore",
    6u16 => "Titanium Ore",
    7u16 => "Water",
    8u16 => "Crude Oil",
    9u16 => "Hydrogen",
    10u16 => "Deuterium",
    11u16 => "Antimatter",
    12u16 => "Core Element",
    13u16 => "Critical Photon",
    14u16 => "Kimberlite Ore",
    // Row 2
    15u16 => "Iron Ingot",
    16u16 => "Copper Ingot",
    17u16 => "Stone Brick",
    18u16 => "Energetic Graphite",
    19u16 => "High-Purity Silicon",
    20u16 => "Titanium Ingot",
    21u16 => "Sulfuric Acid",
    22u16 => "Refined Oil",
    23u16 => "Hydrogen Fuel Rod",
    24u16 => "Deuterium Fuel Rod",
    25u16 => "Antimatter Fuel Rod",
    26u16 => "Strange Annihilation Fuel Rod",
    27u16 => "Missile Set",
    28u16 => "Fractal Silicon",
    // Row 3
    29u16 => "Magnet",
    30u16 => "Magnetic Coil",
    31u16 => "Glass",
    32u16 => "Diamond",
    33u16 => "Crystal Silicon",
    34u16 => "Titanium Alloy",
    35u16 => "Combustible Unit",
    36u16 => "Plastic",
    37u16 => "Organic Crystal",
    38u16 => "Graphene",
    39u16 => "Annihilation Constraint Sphere",
    40u16 => "Magnum Ammo Box",
    41u16 => "Supersonic Missile Set",
    42u16 => "Grating Crystal",
    // Row 4
    43u16 => "Steel",
    44u16 => "Circuit Board",
    45u16 => "Prism",
    46u16 => "Electric Motor",
    47u16 => "Microcrystalline Component",
    48u16 => "Proliferator Mk.I",
    49u16 => "Explosive Unit",
    50u16 => "Strange Matter",
    51u16 => "Titanium Crystal",
    52u16 => "Carbon Nanotube",
    53u16 => "Particle Broadband",
    54u16 => "Titanium Ammo Box",
    55u16 => "Gravity Missile Set",
    56u16 => "Stalagmite Crystal",
    // Row 5
    57u16 => "Gear",
    58u16 => "Plasma Exciter",
    59u16 => "Photon Combiner",
    60u16 => "Electromagnetic Turbine",
    61u16 => "Processor",
    62u16 => "Proliferator Mk.II",
    63u16 => "Crystal Explosive Unit",
    64u16 => "Casimir Crystal",
    65u16 => "Titanium Glass",
    66u16 => "Plane Filter",
    67u16 => "Quantum Chip",
    68u16 => "Superalloy Ammo Box",
    69u16 => "Shell Set",
    70u16 => "Unipolar Magnet",
    // Row 6
    71u16 => "Engine",
    72u16 => "Thruster",
    73u16 => "Reinforced Thruster",
    74u16 => "Super Magnetic Ring",
    75u16 => "Particle Container",
    76u16 => "Proliferator Mk.III",
    77u16 => "Prototype",
    78u16 => "Precision Drone",
    79u16 => "Attack Drone",
    80u16 => "Corvette",
    81u16 => "Destroyer",
    82u16 => "Plasma Capsule",
    83u16 => "High-Explosive Shell Set",
    84u16 => "Fire Ice",
    // Row 7
    85u16 => "Logistics Bot",
    86u16 => "Logistics Drone",
    87u16 => "Interstellar Logistics Vessel",
    88u16 => "Space Warper",
    89u16 => "Gravity Lens",
    90u16 => "Foundation",
    // Empty slot
    92u16 => "Solar Sail",
    93u16 => "Frame Material",
    94u16 => "Dyson Sphere Component",
    95u16 => "Small Carrier Rocket",
    96u16 => "Antimatter Capsule",
    97u16 => "Crystal Shell Set",
    98u16 => "Log",
    // Row 8
    99u16 => "Electromagnetic Matrix",
    100u16 => "Energy Matrix",
    101u16 => "Structure Matrix",
    102u16 => "Information Matrix",
    103u16 => "Gravity Matrix",
    104u16 => "Universe Matrix",
    105u16 => "Dark Fog Matrix",
    106u16 => "Energy Shard",
    107u16 => "Silicon-Based Neuron",
    108u16 => "Negentropy Singularity",
    109u16 => "Matter Recombinator",
    110u16 => "Jamming Capsule",
    111u16 => "Suppressing Capsule",
    112u16 => "Plant Fuel",
    // Buildings
    // Row 1
    1001u16 => "Tesla Tower",
    1002u16 => "Wireless Power Tower",
    1003u16 => "Satellite Substation",
    1004u16 => "Wind Turbine",
    1005u16 => "Thermal Power Station",
    1006u16 => "Solar Panel",
    1007u16 => "Geothermal Power Station",
    1008u16 => "Mini Fusion Power Station",
    1009u16 => "Energy Exchanger",
    1010u16 => "Accumulator",
    1011u16 => "Accumulator (Full)",
    1012u16 => "Ray Receiver",
    1013u16 => "Artificial Star",
    // Row 2
    1015u16 => "Conveyor Belt Mk.I",
    1016u16 => "Conveyor Belt Mk.II",
    1017u16 => "Conveyor Belt Mk.III",
    1018u16 => "Splitter",
    1019u16 => "Automatic Piler",
    1020u16 => "Traffic Monitor",
    1021u16 => "Spray Coater",
    1022u16 => "Depot Mk.I",
    1023u16 => "Depot Mk.II",
    1024u16 => "Storage Tank",
    1025u16 => "Logistics Distributor",
    1026u16 => "Planetary Logistics Station",
    1027u16 => "Interstellar Logistics Station",
    1028u16 => "Orbital Collector",
    // Row 3
    1029u16 => "Sorter Mk.I",
    1030u16 => "Sorter Mk.II",
    1031u16 => "Sorter Mk.III",
    1032u16 => "Pile Sorter",
    1033u16 => "Mining Machine",
    1034u16 => "Advanced Mining Machine",
    1035u16 => "Water Pump",
    1036u16 => "Oil Extractor",
    1037u16 => "Oil Refinery",
    1038u16 => "Fractionator",
    1039u16 => "Chemical Plant",
    1040u16 => "Quantum Chemical Plant",
    1041u16 => "Miniature Particle Collider",
    // Row 4
    1043u16 => "Arc Smelter",
    1044u16 => "Plane Smelter",
    1045u16 => "Negentropy Smelter",
    1046u16 => "Assembling Machine Mk.I",
    1047u16 => "Assembling Machine Mk.II",
    1048u16 => "Assembling Machine Mk.III",
    1049u16 => "Re-Composing Assembler",
    1050u16 => "Matrix Lab",
    1051u16 => "Self-Evolution Lab",
    1052u16 => "EM Rail Ejector",
    1053u16 => "Vertical Launch Silo",
    // Row 5
    1057u16 => "Gauss Turret",
    1058u16 => "Missile Turret",
    1059u16 => "Implosion Cannon",
    1060u16 => "Laser Turret",
    1061u16 => "Plasma Turret",
    1062u16 => "SR Plasma Turret",
    1063u16 => "Battlefield Analysis Base",
    1064u16 => "Jammer Tower",
    1065u16 => "Signal Tower",
    1066u16 => "Planetary Shield Generator",
    // Dummy buildings
    2001u16 => "Icarus",
    2002u16 => "Planetary Base"
};

pub const PRODUCTION_BUILDING: Map<u16, ProductionBuildingProperties> = phf_map!{
    1009u16 => ProductionBuildingProperties {
        // Energy Exchanger
        recipe_type: RecipeType::Charge,
        work_power: 54000,
        stand_by_power: 0,
        speed: 1.0,
    },
    1012u16 => ProductionBuildingProperties {
        // Ray Receiver
        recipe_type: RecipeType::Ray,
        work_power: 0,
        stand_by_power: 0,
        speed: 1.0, // TBD by external factors
    },
    1028u16 => ProductionBuildingProperties {
        // Orbital Collector
        recipe_type: RecipeType::Orbital,
        work_power: 0,
        stand_by_power: 0,
        speed: 1.0 // TBD by external factors
    },
    1033u16 => ProductionBuildingProperties {
        // Mining Machine
        recipe_type: RecipeType::Mine,
        work_power: 420,
        stand_by_power: 24,
        speed: 0.5 // Each ore per minute
    },
    1034u16 => ProductionBuildingProperties {
        // Advanced Mining Machine
        recipe_type: RecipeType::Mine,
        work_power: 2940,
        stand_by_power: 168,
        speed: 1.0 // Each ore per minute
    },
    1035u16 => ProductionBuildingProperties {
        // Water Pump
        recipe_type: RecipeType::Pump,
        work_power: 300,
        stand_by_power: 12,
        speed: 5.0 / 6.0 // Each ore per minute
    },
    1036u16 => ProductionBuildingProperties {
        // Oil Extractor
        recipe_type: RecipeType::Extract,
        work_power: 840,
        stand_by_power: 24,
        speed: 1.0 // TBD by external factors
    },
    1037u16 => ProductionBuildingProperties {
        // Oil Refinery
        recipe_type: RecipeType::Refine,
        work_power: 960,
        stand_by_power: 24,
        speed: 1.0
    },
    1038u16 => ProductionBuildingProperties {
        // Fractionator
        recipe_type: RecipeType::Fractionate,
        work_power: 720,
        stand_by_power: 18,
        speed: 1.0 // TBD by external factors
    },
    1039u16 => ProductionBuildingProperties {
        // Chemical Plant
        recipe_type: RecipeType::Chemical,
        work_power: 720,
        stand_by_power: 24,
        speed: 1.0
    },
    1040u16 => ProductionBuildingProperties {
        // Quantum Chemical Plant
        recipe_type: RecipeType::Chemical,
        work_power: 2160,
        stand_by_power: 36,
        speed: 2.0
    },
    1041u16 => ProductionBuildingProperties {
        // Miniature Particle Collider
        recipe_type: RecipeType::Particle,
        work_power: 12000,
        stand_by_power: 120,
        speed: 1.0
    },
    1043u16 => ProductionBuildingProperties {
        // Arc Smelter
        recipe_type: RecipeType::Smelt,
        work_power: 360,
        stand_by_power: 12,
        speed: 1.0
    },
    1044u16 => ProductionBuildingProperties {
        // Plane Smelter
        recipe_type: RecipeType::Smelt,
        work_power: 1440,
        stand_by_power: 48,
        speed: 2.0
    },
    1045u16 => ProductionBuildingProperties {
        // Negentropy Smelter
        recipe_type: RecipeType::Smelt,
        work_power: 2880,
        stand_by_power: 96,
        speed: 3.0
    },
    1046u16 => ProductionBuildingProperties {
        // Assembling Machine Mk.I
        recipe_type: RecipeType::Assemble,
        work_power: 270,
        stand_by_power: 12,
        speed: 0.75
    },
    1047u16 => ProductionBuildingProperties {
        // Assembling Machine Mk.II
        recipe_type: RecipeType::Assemble,
        work_power: 540,
        stand_by_power: 18,
        speed: 1.0
    },
    1048u16 => ProductionBuildingProperties {
        // Assembling Machine Mk.III
        recipe_type: RecipeType::Assemble,
        work_power: 1080,
        stand_by_power: 24,
        speed: 1.5
    },
    1049u16 => ProductionBuildingProperties {
        // Re-Composing Assembler
        recipe_type: RecipeType::Assemble,
        work_power: 2700,
        stand_by_power: 54,
        speed: 3.0
    },
    1050u16 => ProductionBuildingProperties {
        // Matrix Lab
        recipe_type: RecipeType::Research,
        work_power: 480,
        stand_by_power: 12,
        speed: 1.0
    },
    1051u16 => ProductionBuildingProperties {
        // Self-Evolution Lab
        recipe_type: RecipeType::Research,
        work_power: 1920,
        stand_by_power: 48,
        speed: 3.0
    },
    2001u16 => ProductionBuildingProperties {
        // Energy Exchanger
        recipe_type: RecipeType::Nature,
        work_power: 0,
        stand_by_power: 0,
        speed: 1.0,
    },
    2002u16 => ProductionBuildingProperties {
        // Energy Exchanger
        recipe_type: RecipeType::Darkfog,
        work_power: 0,
        stand_by_power: 0,
        speed: 1.0,
    },
};

// Proliferators

pub const PROLIFERATORS: Map<u8, Proliferator> = phf_map!{
    0u8 => Proliferator {
        level: 0,
        id: 0u16,
        spray_num: 12,
        accelerate_effect: 1.0,
        proliferate_effect: 1.0,
        lens_effect: 1.0,
        power_consumption: 1.0
    },
    1u8 => Proliferator {
        level: 1,
        id: 48u16,
        spray_num: 12,
        accelerate_effect: 1.25,
        proliferate_effect: 1.125,
        lens_effect: 1.0,
        power_consumption: 1.3
    },
    2u8 => Proliferator {
        level: 2,
        id: 62u16,
        spray_num: 24,
        accelerate_effect: 1.5,
        proliferate_effect: 1.2,
        lens_effect: 1.0,
        power_consumption: 1.7
    },
    3u8 => Proliferator {
        level: 3,
        id: 76u16,
        spray_num: 60,
        accelerate_effect: 2.0,
        proliferate_effect: 1.25,
        lens_effect: 1.0,
        power_consumption: 2.5
    }
};

// Recipes

pub const RECIPES: [Recipe; 189] = [
    // Item recipes
    // Hydrogen is not considered as the main product
    // Iron Ore
    Recipe {
        recipe_type: RecipeType::Mine,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{1u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // Copper Ore
    Recipe {
        recipe_type: RecipeType::Mine,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{2u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // Coal
    Recipe {
        recipe_type: RecipeType::Mine,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{3u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // Stone
    Recipe {
        recipe_type: RecipeType::Mine,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{4u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // Silicon Ore
    Recipe {
        recipe_type: RecipeType::Mine,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{5u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // Titanium Ore
    Recipe {
        recipe_type: RecipeType::Mine,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{6u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // Water
    Recipe {
        recipe_type: RecipeType::Pump,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{7u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // Crude Oil
    Recipe {
        recipe_type: RecipeType::Extract,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{8u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // Hydrogen (Orbital)
    Recipe {
        recipe_type: RecipeType::Orbital,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{9u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // Deuterium (Orbital)
    Recipe {
        recipe_type: RecipeType::Orbital,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{10u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // 2 Hydrogen -> 1 Deuterium
    Recipe {
        recipe_type: RecipeType::Particle,
        time: 2.5,
        input: phf_map!{9u16 => 2},
        output: phf_map!{10u16 => 1},
        allow_accelerate: true,
        allow_proliferate: false,
        is_lens: false
    },
    // 1 Hydrogen -> 1 Deuterium
    Recipe {
        recipe_type: RecipeType::Fractionate,
        time: 100.0,
        input: phf_map!{9u16 => 1},
        output: phf_map!{10u16 => 1},
        allow_accelerate: true,
        allow_proliferate: false,
        is_lens: false
    },
    // 2 Proton -> 2 Hydrogen + 2 Antimatter
    Recipe {
        recipe_type: RecipeType::Particle,
        time: 2.0,
        input: phf_map!{13u16 => 2},
        output: phf_map!{9u16 => 2, 11u16 => 2},
        allow_accelerate: true,
        allow_proliferate: false,
        is_lens: false
    },
    // Core Element
    Recipe {
        recipe_type: RecipeType::Darkfog,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{12u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // Proton
    Recipe {
        recipe_type: RecipeType::Ray,
        time: 10.0,
        input: phf_map!{},
        output: phf_map!{13u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // Proton with gravity lens
    Recipe {
        recipe_type: RecipeType::Ray,
        time: 5.0,
        input: phf_map!{89u16 => 1},
        output: phf_map!{13u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: true // Here 1 stands for 0.0083333, and does not grow when accelerated
    },
    // Kimberlite Ore
    Recipe {
        recipe_type: RecipeType::Mine,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{14u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // 1 Iron Ore -> 1 Iron Ingot
    Recipe {
        recipe_type: RecipeType::Smelt,
        time: 1.0,
        input: phf_map!{1u16 => 1},
        output: phf_map!{15u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Copper Ore -> 1 Copper Ingot
    Recipe {
        recipe_type: RecipeType::Smelt,
        time: 1.0,
        input: phf_map!{2u16 => 1},
        output: phf_map!{16u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Stone -> 1 Stone Brick
    Recipe {
        recipe_type: RecipeType::Smelt,
        time: 1.0,
        input: phf_map!{4u16 => 1},
        output: phf_map!{17u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Coal -> 1 Energetic Graphite
    Recipe {
        recipe_type: RecipeType::Smelt,
        time: 2.0,
        input: phf_map!{3u16 => 2},
        output: phf_map!{18u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Silicon Ore -> 1 High-Purity Silicon
    Recipe {
        recipe_type: RecipeType::Smelt,
        time: 2.0,
        input: phf_map!{5u16 => 2},
        output: phf_map!{19u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Titanium Ore -> 1 Titanium Ingot
    Recipe {
        recipe_type: RecipeType::Smelt,
        time: 2.0,
        input: phf_map!{6u16 => 2},
        output: phf_map!{20u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // Sulfuric Acid (Pump)
    Recipe {
        recipe_type: RecipeType::Pump,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{21u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // 6 Refined Oil + 8 Stone + 4 Water -> 4 Sulfuric Acid
    Recipe {
        recipe_type: RecipeType::Chemical,
        time: 6.0,
        input: phf_map!{22u16 => 6, 4u16 => 8, 7u16 => 4},
        output: phf_map!{21u16 => 4},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Refined Oil + 2 Hydrogen -> 3 Hydrogen + 1 Energetic Graphite
    Recipe {
        recipe_type: RecipeType::Refine,
        time: 2.0,
        input: phf_map!{22u16 => 1, 9u16 => 2},
        output: phf_map!{9u16 => 3, 18u16 => 1},
        allow_accelerate: true,
        allow_proliferate: false,
        is_lens: false
    },
    // 2 Crude Oil -> 2 Refined Oil + 1 Hydrogen
    Recipe {
        recipe_type: RecipeType::Refine,
        time: 4.0,
        input: phf_map!{8u16 => 2},
        output: phf_map!{22u16 => 2, 9u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Refine Oil + 1 Hydrogen + 1 Coal -> 3 Refined Oil
    Recipe {
        recipe_type: RecipeType::Refine,
        time: 4.0,
        input: phf_map!{22u16 => 2, 9u16 => 1, 3u16 => 1},
        output: phf_map!{22u16 => 3},
        allow_accelerate: true,
        allow_proliferate: false,
        is_lens: false
    },
    // 1 Titanium Ingot + 10 Hydrogen -> 2 Hydrogen Fuel Rod
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{20u16 => 1, 9u16 => 10},
        output: phf_map!{23u16 => 2},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Titanium Alloy + 20 Deuterium + 1 Super Magnetic Ring -> 2 Deuterium Fuel Rod
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 12.0,
        input: phf_map!{34u16 => 1, 10u16 => 20, 74u16 => 1},
        output: phf_map!{24u16 => 2},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 12 Antimatter + 12 Hydrogen + 1 Annihilation Constraint Sphere + 1 Titanium Alloy -> 2 Antimatter Fuel Rod
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 24.0,
        input: phf_map!{11u16 => 12, 9u16 => 12, 39u16 => 1, 34u16 => 1},
        output: phf_map!{25u16 => 2},
        allow_accelerate: true,
        allow_proliferate: false,
        is_lens: false
    },
    // 8 Antimatter Fuel Rod + 1 Core Element + 2 Strange Matter + 1 Frame Material -> 1 Strange Annihilation Fuel Rod
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 48.0,
        input: phf_map!{25u16 => 8, 12u16 => 1, 50u16 => 2, 93u16 => 1},
        output: phf_map!{26u16 => 1},
        allow_accelerate: true,
        allow_proliferate: false,
        is_lens: false
    },
    // 6 Copper Ingot + 3 Circuit Board + 2 Combustible Unit + 1 Engine -> 1 Missile Set
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 12.0,
        input: phf_map!{16u16 => 6, 44u16 => 3, 35u16 => 2, 71u16 => 1},
        output: phf_map!{27u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // Fractal Silicon
    Recipe {
        recipe_type: RecipeType::Mine,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{28u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // 1 Iron Ore -> 1 Magnet
    Recipe {
        recipe_type: RecipeType::Smelt,
        time: 1.5,
        input: phf_map!{1u16 => 1},
        output: phf_map!{29u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Magnet + 1 Copper Ingot -> 2 Magnetic Coil
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 1.0,
        input: phf_map!{29u16 => 2, 16u16 => 1},
        output: phf_map!{30u16 => 2},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Stone -> 1 Glass
    Recipe {
        recipe_type: RecipeType::Smelt,
        time: 2.0,
        input: phf_map!{4u16 => 2},
        output: phf_map!{31u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Energetic Graphite -> 1 Diamond
    Recipe {
        recipe_type: RecipeType::Smelt,
        time: 2.0,
        input: phf_map!{18u16 => 1},
        output: phf_map!{32u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Kimblerite Ore -> 2 Diamond
    Recipe {
        recipe_type: RecipeType::Smelt,
        time: 1.5,
        input: phf_map!{14u16 => 1},
        output: phf_map!{32u16 => 2},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 High-Purity Silicon -> 1 Crystal Silicon
    Recipe {
        recipe_type: RecipeType::Smelt,
        time: 2.0,
        input: phf_map!{19u16 => 1},
        output: phf_map!{33u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Fractal Silicon -> 2 Crystal Silicon
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 1.5,
        input: phf_map!{28u16 => 1},
        output: phf_map!{33u16 => 2},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 4 Titanium Ingot + 4 Steel + 8 Sulfuric Acid -> 4 Titanium Alloy
    Recipe {
        recipe_type: RecipeType::Smelt,
        time: 12.0,
        input: phf_map!{20u16 => 4, 43u16 => 4, 21u16 => 8},
        output: phf_map!{34u16 => 4},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 3 Coal -> 1 Combustible Unit
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 3.0,
        input: phf_map!{3u16 => 3},
        output: phf_map!{35u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Refine Oil + 1 Energy Graphite -> 1 Plastic
    Recipe {
        recipe_type: RecipeType::Chemical,
        time: 3.0,
        input: phf_map!{22u16 => 2, 18u16 => 1},
        output: phf_map!{36u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // Organic Crystal
    Recipe {
        recipe_type: RecipeType::Mine,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{37u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // 2 Plastic + 1 Refined Oil + 1 Water -> 1 Organic Crystal
    Recipe {
        recipe_type: RecipeType::Chemical,
        time: 6.0,
        input: phf_map!{36u16 => 2, 22u16 => 1, 7u16 => 1},
        output: phf_map!{37u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 20 Log + 30 Plant Fuel + 10 Water -> 1 Organic Crystal
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{98u16 => 20, 112u16 => 30, 7u16 => 10},
        output: phf_map!{37u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 3 Energy Graphite + 1 Sulfuric Acid -> 2 Graphene
    Recipe {
        recipe_type: RecipeType::Chemical,
        time: 3.0,
        input: phf_map!{18u16 => 3, 21u16 => 1},
        output: phf_map!{38u16 => 2},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Fire Ice -> 2 Graphene + 1 Hydrogen
    Recipe {
        recipe_type: RecipeType::Chemical,
        time: 2.0,
        input: phf_map!{84u16 => 2},
        output: phf_map!{38u16 => 2, 9u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Particle Container + 1 Processor -> 1 Annihilation Constraint Sphere
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 20.0,
        input: phf_map!{75u16 => 1, 61u16 => 1},
        output: phf_map!{39u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 3 Copper Ingot -> 1 Magnum Ammo Box
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 1.0,
        input: phf_map!{16u16 => 3},
        output: phf_map!{40u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Missile Set + 4 Processor + 4 Explosive Unit + 2 Thruster -> 1 Supersonic Missile Set
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 4.0,
        input: phf_map!{27u16 => 2, 61u16 => 4, 43u16 => 4, 45u16 => 2},
        output: phf_map!{41u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // Grating Crystal
    Recipe {
        recipe_type: RecipeType::Mine,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{42u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // 3 Iron Ingot -> 1 Steel
    Recipe {
        recipe_type: RecipeType::Smelt,
        time: 3.0,
        input: phf_map!{15u16 => 3},
        output: phf_map!{43u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Iron Ingot + 1 Copper Ingot -> 2 Circuit Board
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 1.0,
        input: phf_map!{15u16 => 2, 16u16 => 1},
        output: phf_map!{44u16 => 2},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 3 Glass -> 2 Prism
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 2.0,
        input: phf_map!{31u16 => 3},
        output: phf_map!{45u16 => 2},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Iron Ingot + 1 Gear + 1 Magnetic Coil -> 1 Electric Motor
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 2.0,
        input: phf_map!{15u16 => 2, 57u16 => 1, 30u16 => 1},
        output: phf_map!{46u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 High-Purity Silicon + 1 Copper Ingot -> 1 Microcrystalline Component
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 2.0,
        input: phf_map!{19u16 => 2, 16u16 => 1},
        output: phf_map!{47u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Coal -> 1 Proliferator Mk.I
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 0.5,
        input: phf_map!{3u16 => 1},
        output: phf_map!{48u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Combustible Unit + 2 Plastic + 1 Sulfuric Acid -> 2 Explosive Unit
    Recipe {
        recipe_type: RecipeType::Chemical,
        time: 6.0,
        input: phf_map!{35u16 => 2, 36u16 => 2, 21u16 => 1},
        output: phf_map!{49u16 => 2},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Particle Container + 2 Iron Ingot + 10 Deuterium -> 1 Strange Matter
    Recipe {
        recipe_type: RecipeType::Particle,
        time: 8.0,
        input: phf_map!{75u16 => 2, 15u16 => 2, 10u16 => 10},
        output: phf_map!{50u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Organic Crystal + 3 Titanium Ingot -> 1 Titanium Crystal
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 4.0,
        input: phf_map!{37u16 => 1, 20u16 => 3},
        output: phf_map!{51u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 3 Graphene + 1 Titanium Ingot -> 2 Carbon Nanotube
    Recipe {
        recipe_type: RecipeType::Chemical,
        time: 4.0,
        input: phf_map!{38u16 => 3, 20u16 => 1},
        output: phf_map!{52u16 => 2},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 6 Stalagmite Crystal -> 2 Carbon Nanotube
    Recipe {
        recipe_type: RecipeType::Chemical,
        time: 4.0,
        input: phf_map!{56u16 => 6},
        output: phf_map!{52u16 => 2},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Carbon Nanotube + 2 Plastic + 1 Crystal Silicon -> 1 Particle Broadband
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 8.0,
        input: phf_map!{52u16 => 2, 36u16 => 2, 33u16 => 1},
        output: phf_map!{53u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Magnum Ammo Box + 2 Titanium Ingot -> 1 Titanium Ammo Box
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 2.0,
        input: phf_map!{40u16 => 1, 20u16 => 2},
        output: phf_map!{54u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 3 Supersonic Missile Set + 6 Crystal Explosive Unit + 3 Strange Matter -> 3 Gravity Missile Set
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{41u16 => 3, 63u16 => 6, 50u16 => 3},
        output: phf_map!{55u16 => 3},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // Stalagmite Crystal
    Recipe {
        recipe_type: RecipeType::Mine,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{56u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // 1 Iron Ingot -> 1 Gear
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 1.0,
        input: phf_map!{15u16 => 1},
        output: phf_map!{57u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 4 Magnetic Coil + 2 Prism -> 1 Plasma Exciter
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 2.0,
        input: phf_map!{30u16 => 4, 45u16 => 2},
        output: phf_map!{58u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Prism + 1 Circuit Board -> 1 Proton Combiner
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 3.0,
        input: phf_map!{45u16 => 2, 44u16 => 1},
        output: phf_map!{59u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Grating Crystal + 1 Circuit Board -> 1 Proton Combiner
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 3.0,
        input: phf_map!{42u16 => 1, 44u16 => 1},
        output: phf_map!{59u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Electric Motor + 2 Magnetic Coil -> 1 Electromagnetic Turbine
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 2.0,
        input: phf_map!{46u16 => 2, 30u16 => 2},
        output: phf_map!{60u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Circuit Board + 2 Microcrystalline Component -> 1 Processor
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 2.0,
        input: phf_map!{44u16 => 2, 47u16 => 2},
        output: phf_map!{61u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Proliferator Mk.I + 1 Diamond -> 1 Proliferator Mk.II
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 1.0,
        input: phf_map!{48u16 => 2, 32u16 => 1},
        output: phf_map!{62u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 8 Explosive Unit + 1 Casmir Crystal + 8 Crystal Silicon -> 8 Crystal Explosive Unit
    Recipe {
        recipe_type: RecipeType::Chemical,
        time: 24.0,
        input: phf_map!{49u16 => 8, 64u16 => 1, 33u16 => 8},
        output: phf_map!{63u16 => 8},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Titanium Crystal + 2 Graphene + 12 Hydrogen -> 1 Casmir Crystal
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 4.0,
        input: phf_map!{51u16 => 1, 38u16 => 2, 9u16 => 12},
        output: phf_map!{64u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 8 Grating Crystal + 2 Graphene + 12 Hydrogen -> 1 Casmir Crystal
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 4.0,
        input: phf_map!{42u16 => 8, 38u16 => 2, 9u16 => 12},
        output: phf_map!{64u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Glass + 2 Titanium Ingot + 2 Water -> 2 Titanium Glass
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 5.0,
        input: phf_map!{31u16 => 2, 20u16 => 2, 7u16 => 2},
        output: phf_map!{65u16 => 2},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Casimir Crystal + 2 Titanium Glass -> 1 Plane Filter
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 12.0,
        input: phf_map!{64u16 => 1, 65u16 => 2},
        output: phf_map!{66u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Processor + 2 Plane Filter -> 1 Quantum Chip
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{61u16 => 2, 66u16 => 2},
        output: phf_map!{67u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Titanium Ammo Box + 1 Titanium Alloy -> 1 Superalloy Ammo Box
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 3.0,
        input: phf_map!{54u16 => 1, 34u16 => 1},
        output: phf_map!{68u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 9 Copper Ingot + 2 Combustible Unit -> 1 Shell Set
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 1.5,
        input: phf_map!{16u16 => 9, 35u16 => 2},
        output: phf_map!{69u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // Unipolar Magnet
    Recipe {
        recipe_type: RecipeType::Mine,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{70u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // 1 Magnetic Coil + 2 Copper Ingot -> 1 Engine
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 3.0,
        input: phf_map!{30u16 => 1, 16u16 => 2},
        output: phf_map!{71u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Steel + 3 Copper Ingot -> 1 Thruster
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 4.0,
        input: phf_map!{43u16 => 2, 16u16 => 3},
        output: phf_map!{72u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 5 Titanium Alloy + 5 Electromagnetic Turbine -> 1 Reinforced Thruster
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{34u16 => 5, 60u16 => 5},
        output: phf_map!{73u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Electromagnetic Turbine + 3 Magnet + 1 Energy Graphite -> 1 Super Magnetic Ring
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 3.0,
        input: phf_map!{60u16 => 2, 29u16 => 3, 18u16 => 1},
        output: phf_map!{74u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Electromagnetic Turbine + 2 Copper Ingot + 2 Graphene -> 1 Particle Container
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 4.0,
        input: phf_map!{60u16 => 2, 16u16 => 2, 38u16 => 2},
        output: phf_map!{75u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 10 Unipoplar Magnet + 2 Copper Ingot -> 1 Particle Container
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 4.0,
        input: phf_map!{70u16 => 10, 16u16 => 2},
        output: phf_map!{75u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Proliferator Mk.II + 1 Carbon Nanotube -> 1 Proliferator Mk.III
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 2.0,
        input: phf_map!{62u16 => 2, 52u16 => 1},
        output: phf_map!{76u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 3 Iron Ingot + 1 Engine + 2 Circuit Board + 1 Plasma Exciter -> 1 Prototype
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 2.0,
        input: phf_map!{15u16 => 3, 71u16 => 1, 44u16 => 2, 58u16 => 1},
        output: phf_map!{77u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Prototype + 1 Electromagnetic Turbine + 2 Circuit Board + 2 Proton Combiner -> 1 Precision Drone
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 4.0,
        input: phf_map!{77u16 => 1, 60u16 => 1, 44u16 => 2, 59u16 => 2},
        output: phf_map!{78u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Prototype + 1 Electromagnetic Turbine + 1 Processor + 1 Particle Container -> 1 Attack Drone
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 4.0,
        input: phf_map!{77u16 => 1, 60u16 => 1, 61u16 => 1, 75u16 => 1},
        output: phf_map!{79u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 5 Titanium Alloy + 1 Reinforced Thruster + 2 Processor + 3 Particle Container -> 1 Corvette
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 5.0,
        input: phf_map!{34u16 => 5, 73u16 => 1, 61u16 => 2, 75u16 => 3},
        output: phf_map!{80u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 20 Frame Material + 4 Reinforced Thruster + 4 Processor + 1 Strange Matter -> 1 Destroyer
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 8.0,
        input: phf_map!{93u16 => 20, 73u16 => 4, 61u16 => 4, 50u16 => 1},
        output: phf_map!{81u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Graphene + 2 Magnet + 10 Deuterium -> 1 Plasma Capsule
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 2.0,
        input: phf_map!{38u16 => 1, 29u16 => 2, 10u16 => 10},
        output: phf_map!{82u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Shell Set + 6 Titanium Ingot + 2 Explosive Unit -> 1 High-Explosive Shell Set
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 3.0,
        input: phf_map!{69u16 => 1, 20u16 => 6, 49u16 => 2},
        output: phf_map!{83u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // Fire Ice (Mine)
    Recipe {
        recipe_type: RecipeType::Mine,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{84u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // Fire Ice (Orbital)
    Recipe {
        recipe_type: RecipeType::Orbital,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{84u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // 2 Iron Ingot + 1 Engine + 1 Processor -> 1 Logistic Bot
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 2.0,
        input: phf_map!{15u16 => 2, 71u16 => 1, 61u16 => 1},
        output: phf_map!{85u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 5 Iron Ingot + 2 Thruster + 2 Processor -> 1 Logistic Drone
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 4.0,
        input: phf_map!{15u16 => 5, 72u16 => 2, 61u16 => 2},
        output: phf_map!{86u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 10 Titanium Alloy + 2 Reinforced Thruster + 10 Processor -> Interstellar Logistics Vessel
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{34u16 => 10, 73u16 => 2, 61u16 => 10},
        output: phf_map!{87u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Gravity Lens -> 1 Space Warper
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 10.0,
        input: phf_map!{89u16 => 1},
        output: phf_map!{88u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Gravity Matrix -> 8 Space Warper
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 10.0,
        input: phf_map!{103u16 => 1},
        output: phf_map!{88u16 => 8},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 4 Diamond + 1 Strange Matter -> 1 Gravity Lens
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{32u16 => 4, 50u16 => 1},
        output: phf_map!{89u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 3 Stone Brick + 1 Steel -> 1 Foundation
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 1.0,
        input: phf_map!{17u16 => 3, 43u16 => 1},
        output: phf_map!{90u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Graphene + 1 Proton Combiner -> 2 Solar Sail
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 4.0,
        input: phf_map!{38u16 => 1, 59u16 => 1},
        output: phf_map!{92u16 => 2},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 4 Carbon Nanotube + 1 Titanium Alloy + 1 High-Purity Silicon -> 1 Frame Material
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{52u16 => 4, 34u16 => 1, 19u16 => 1},
        output: phf_map!{93u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 3 Frame Material + 3 Solar Sail + 3 Processor -> 1 Dyson Sphere Component
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 8.0,
        input: phf_map!{93u16 => 3, 92u16 => 3, 61u16 => 3},
        output: phf_map!{94u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Dyson Sphere Component + 4 Deuterium Fuel Rod + 2 Quantum Chip -> 1 Small Carrier Rocket
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{94u16 => 2, 24u16 => 4, 67u16 => 2},
        output: phf_map!{95u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Plasma Capsule + 1 Particle Container + 10 Hydrogen + 10 Antimatter -> 1 Antimatter Capsule
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 2.0,
        input: phf_map!{82u16 => 1, 75u16 => 1, 9u16 => 10, 12u16 => 10},
        output: phf_map!{96u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 High-Explosive Shell Set + 3 Titanium Alloy + 2 Crystal Explosive Unit -> 1 Crystal Shell Set
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{83u16 => 1, 34u16 => 3, 63u16 => 2},
        output: phf_map!{97u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // Log
    Recipe {
        recipe_type: RecipeType::Nature,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{98u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // 1 Magnetic Coil + 1 Circuit Board -> 1 Electromagnetic Matrix
    Recipe {
        recipe_type: RecipeType::Research,
        time: 3.0,
        input: phf_map!{30u16 => 1, 44u16 => 1},
        output: phf_map!{99u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Energetic Graphite + 2 Hydrogen -> 1 Energy Matrix
    Recipe {
        recipe_type: RecipeType::Research,
        time: 6.0,
        input: phf_map!{18u16 => 2, 9u16 => 2},
        output: phf_map!{100u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Diamond + 1 Titanium Crystal -> 1 Structure Matrix
    Recipe {
        recipe_type: RecipeType::Research,
        time: 8.0,
        input: phf_map!{32u16 => 1, 51u16 => 1},
        output: phf_map!{101u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Processor + 1 Particle Broadband -> 1 Information Matrix
    Recipe {
        recipe_type: RecipeType::Research,
        time: 10.0,
        input: phf_map!{61u16 => 2, 53u16 => 1},
        output: phf_map!{102u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Gravity Lens + 1 Quantum Chip -> 2 Gravity Matrix
    Recipe {
        recipe_type: RecipeType::Research,
        time: 24.0,
        input: phf_map!{89u16 => 1, 67u16 => 1},
        output: phf_map!{103u16 => 2},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Electromagnetic Matrix + 1 Energy Matrix + 1 Structure Matrix + 1 Information Matrix + 1 Gravity Matrix + 1 Antimatter -> 1 Universe Matrix
    Recipe {
        recipe_type: RecipeType::Research,
        time: 15.0,
        input: phf_map!{99u16 => 1, 100u16 => 1, 101u16 => 1, 102u16 => 1, 103u16 => 1, 11u16 => 1},
        output: phf_map!{104u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // Dark Fog Matrix
    Recipe {
        recipe_type: RecipeType::Darkfog,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{105u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // Energy Shard
    Recipe {
        recipe_type: RecipeType::Darkfog,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{106u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // Silicon-Based Neuron
    Recipe {
        recipe_type: RecipeType::Darkfog,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{107u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // Negentropy Singularity
    Recipe {
        recipe_type: RecipeType::Darkfog,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{108u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // Matter Recombinator
    Recipe {
        recipe_type: RecipeType::Darkfog,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{109u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // 1 Electromagnetic Turbine + 1 Plasma Exciter + 3 Hydrogen -> 1 Jamming Capsule
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 2.0,
        input: phf_map!{60u16 => 1, 58u16 => 1, 9u16 => 3},
        output: phf_map!{110u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Jamming Capsule + 1 Super Magnetic Ring + 2 Titanium Glass -> 2 Suppressing Capsule
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 8.0,
        input: phf_map!{110u16 => 2, 74u16 => 1, 65u16 => 2},
        output: phf_map!{111u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // Plant Fuel
    Recipe {
        recipe_type: RecipeType::Nature,
        time: 1.0,
        input: phf_map!{},
        output: phf_map!{112u16 => 1},
        allow_accelerate: false,
        allow_proliferate: false,
        is_lens: false
    },
    // Building recipes
    // 2 Iron Ingot + 1 Magnetic Coil -> 1 Tesla Tower
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 1.0,
        input: phf_map!{15u16 => 2, 30u16 => 1},
        output: phf_map!{1001u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Tesla Tower + 3 Plasma Exciter -> 1 Wireless Power Tower
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 3.0,
        input: phf_map!{1001u16 => 1, 58u16 => 3},
        output: phf_map!{1002u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Wireless Power Tower + 10 Super Magnetic Ring + 2 Frame Material -> 1 Satellite Substation
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 5.0,
        input: phf_map!{1002u16 => 1, 74u16 => 10, 93u16 => 2},
        output: phf_map!{1003u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 6 Iron Ingot + 1 Gear + 3 Magnetic Coil -> 1 Wind Turbine
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 4.0,
        input: phf_map!{15u16 => 6, 57u16 => 1, 30u16 => 3},
        output: phf_map!{1004u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 10 Iron Ingot + 4 Stone Brick + 4 Gear + 4 Magnetic Coil -> 1 Thermal Power Station
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 5.0,
        input: phf_map!{15u16 => 10, 17u16 => 4, 57u16 => 4, 30u16 => 4},
        output: phf_map!{1005u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 10 Copper Ingot + 10 High-Purity Silicon + 5 Circuit Board -> 1 Solar Panel
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{16u16 => 10, 19u16 => 10, 44u16 => 5},
        output: phf_map!{1006u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 15 Steel + 20 Copper Ingot + 4 Proton Combiner + 1 Super Magnetic Ring -> 1 Geothermal Power Station
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{43u16 => 15, 16u16 => 20, 59u16 => 4, 74u16 => 1},
        output: phf_map!{1007u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 12 Titanium Alloy + 10 Super Magnetic Ring + 8 Carbon Nanotube + 4 Processor -> Mini Fusion Power Station
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 10.0,
        input: phf_map!{34u16 => 12, 74u16 => 10, 52u16 => 8, 61u16 => 4},
        output: phf_map!{1008u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 40 Titanium Alloy + 40 Steel + 40 Processor + 8 Particle Container -> 1 Energy Exchanger
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 15.0,
        input: phf_map!{34u16 => 40, 43u16 => 40, 61u16 => 40, 75u16 => 8},
        output: phf_map!{1009u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 6 Iron Ingot + 1 Super Magnetic Ring + 3 Crystal Silicon -> 1 Accumulator
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 3.0,
        input: phf_map!{15u16 => 6, 74u16 => 1, 33u16 => 3},
        output: phf_map!{1010u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Accumulator -> 1 Accumulator (Full)
    Recipe {
        recipe_type: RecipeType::Charge,
        time: 10.0,
        input: phf_map!{1010u16 => 1},
        output: phf_map!{1011u16 => 1},
        allow_accelerate: true,
        allow_proliferate: false,
        is_lens: false
    },
    // 20 Steel + 20 High-Purity Silicon + 10 Proton Combiner + 5 Processor + 20 Super Magnetic Ring -> 1 Ray Receiver
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 8.0,
        input: phf_map!{43u16 => 20, 19u16 => 20, 59u16 => 10, 61u16 => 5, 74u16 => 20},
        output: phf_map!{1012u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 20 Titanium Alloy + 20 Frame Material + 10 Annihilation Constraint Sphere + 10 Quantum Chip -> 1 Artificial Star
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 30.0,
        input: phf_map!{34u16 => 20, 93u16 => 20, 55u16 => 10, 67u16 => 10},
        output: phf_map!{1013u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Iron Ingot + 1 Gear -> 3 Conveyor Belt Mk.I
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 1.0,
        input: phf_map!{15u16 => 2, 57u16 => 1},
        output: phf_map!{1015u16 => 3},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 3 Conveyor Belt Mk.I + 1 Electromagnetic Turbine -> 3 Conveyor Belt Mk.II
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 1.0,
        input: phf_map!{1015u16 => 3, 60u16 => 1},
        output: phf_map!{1016u16 => 3},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 3 Conveyor Belt Mk.II + 1 Super Magnetic Ring + 1 Graphene -> 3 Conveyor Belt Mk.III
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 1.0,
        input: phf_map!{1016u16 => 3, 74u16 => 1, 38u16 => 1},
        output: phf_map!{1017u16 => 3},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 3 Iron Ingot + 2 Gear + 1 Circuit Board -> 1 Splitter
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 2.0,
        input: phf_map!{15u16 => 3, 57u16 => 2, 44u16 => 1},
        output: phf_map!{1018u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 3 Steel + 4 Gear + 1 Super Magnetic Ring + 2 Processor -> 1 Automatic Piler
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 4.0,
        input: phf_map!{43u16 => 3, 57u16 => 4, 74u16 => 1, 61u16 => 2},
        output: phf_map!{1019u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 3 Iron Ingot + 2 Gear + 1 Glass + 2 Circuit Board -> 1 Traffic Monitor
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 2.0,
        input: phf_map!{15u16 => 3, 57u16 => 2, 31u16 => 1, 44u16 => 2},
        output: phf_map!{1020u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 4 Steel + 2 Plasma Exciter + 2 Circuit Board + 2 Microcrystalline Component -> 1 Spray Coater
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 3.0,
        input: phf_map!{43u16 => 4, 58u16 => 2, 44u16 => 2, 56u16 => 2},
        output: phf_map!{1021u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 4 Iron Ingot + 4 Stone Brick -> 1 Depot Mk.I
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 2.0,
        input: phf_map!{15u16 => 4, 17u16 => 4},
        output: phf_map!{1022u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 8 Steel + 8 Stone Brick -> 1 Depot Mk.II
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 4.0,
        input: phf_map!{43u16 => 8, 17u16 => 8},
        output: phf_map!{1023u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 8 Steel + 4 Stone Brick + 4 Glass -> 1 Storage Tank
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 2.0,
        input: phf_map!{43u16 => 8, 17u16 => 4, 31u16 => 4},
        output: phf_map!{1024u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 8 Steel + 4 Plasma Exciter + 4 Processor -> 1 Logistic Distributor
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 8.0,
        input: phf_map!{43u16 => 8, 58u16 => 4, 61u16 => 4},
        output: phf_map!{1025u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 40 Steel + 40 Titanium Ingot + 40 Processor + 20 Particle Container -> 1 Planetary Logistics Station
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 20.0,
        input: phf_map!{43u16 => 40, 20u16 => 40, 61u16 => 40, 75u16 => 20},
        output: phf_map!{1026u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Planetary Logistics Station + 40 Titanium Alloy + 20 Particle Container -> 1 Interstellar Logistics Station
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 30.0,
        input: phf_map!{1026u16 => 1, 34u16 => 40, 75u16 => 20},
        output: phf_map!{1027u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Interstellar Logistics Station + 50 Super Magnetic Ring + 20 Reinforced Thruster + 20 Accumulator (Full) -> 1 Orbital Collector
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 30.0,
        input: phf_map!{1027u16 => 1, 74u16 => 50, 73u16 => 20, 1011u16 => 20},
        output: phf_map!{1028u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Iron Ingot + 1 Circuit Board -> 1 Sorter Mk.I
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 1.0,
        input: phf_map!{15u16 => 1, 44u16 => 1},
        output: phf_map!{1029u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Sorter Mk.I + 1 Electric Motor -> 2 Sorter Mk.II
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 1.0,
        input: phf_map!{1029u16 => 2, 46u16 => 1},
        output: phf_map!{1030u16 => 2},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Sorter Mk.II + 1 Electromagnetic Turbine -> 2 Sorter Mk.III
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 1.0,
        input: phf_map!{1030u16 => 2, 60u16 => 1},
        output: phf_map!{1031u16 => 2},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Sorter Mk.III + 1 Super Magnetic Ring + 1 Processor -> 1 Pile Sorter
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 1.0,
        input: phf_map!{1031u16 => 2, 74u16 => 1, 61u16 => 1},
        output: phf_map!{1032u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 4 Iron Ingot + 2 Gear + 2 Magnetic Coil + 2 Circuit Board -> 1 Mining Machine
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 3.0,
        input: phf_map!{15u16 => 4, 57u16 => 2, 30u16 => 2, 44u16 => 2},
        output: phf_map!{1033u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 20 Titanium Alloy + 10 Frame Material + 10 Super Magnetic Ring + 4 Quantum Chip + 40 Grating Crystal -> 1 Advanced Mining Machine
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 20.0,
        input: phf_map!{34u16 => 20, 93u16 => 10, 74u16 => 10, 67u16 => 4, 42u16 => 40},
        output: phf_map!{1034u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 8 Iron Ingot + 4 Stone Brick + 4 Electric Motor + 2 Circuit Board -> 1 Water Pump
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 4.0,
        input: phf_map!{15u16 => 8, 17u16 => 4, 55u16 => 4, 44u16 => 2},
        output: phf_map!{1035u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 12 Steel + 12 Stone Brick + 6 Circuit Board + 4 Plasma Exciter -> 1 Oil Extractor
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 8.0,
        input: phf_map!{43u16 => 12, 17u16 => 12, 44u16 => 6, 58u16 => 4},
        output: phf_map!{1036u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 10 Steel + 10 Stone Brick + 6 Circuit Board + 6 Plasma Exciter -> 1 Oil Refinery
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{43u16 => 10, 17u16 => 10, 44u16 => 6, 58u16 => 6},
        output: phf_map!{1037u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 8 Steel + 4 Stone Brick + 4 Glass + 1 Processor -> 1 Fractionator
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 3.0,
        input: phf_map!{43u16 => 8, 17u16 => 4, 31u16 => 4, 61u16 => 1},
        output: phf_map!{1038u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 8 Steel + 8 Stone Brick + 8 Glass + 2 Circuit Board -> 1 Chemical Plant
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 5.0,
        input: phf_map!{43u16 => 8, 17u16 => 8, 31u16 => 8, 44u16 => 2},
        output: phf_map!{1039u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Chemical Plant + 10 Titanium Glass + 3 Strange Matter + 3 Quantum Chip -> 1 Quantum Chemical Plant
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 10.0,
        input: phf_map!{1039u16 => 1, 65u16 => 10, 50u16 => 3, 67u16 => 3},
        output: phf_map!{1040u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 20 Titanium Alloy + 20 Frame Material + 25 Super Magnetic Ring + 10 Graphene + 8 Processor -> 1 Miniature Particle Collider
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 15.0,
        input: phf_map!{34u16 => 20, 93u16 => 20, 74u16 => 25, 38u16 => 10, 61u16 => 8},
        output: phf_map!{1041u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 4 Iron Ingot + 2 Stone Brick + 4 Magnetic Coil + 2 Circuit Board -> 1 Arc Smelter
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 3.0,
        input: phf_map!{15u16 => 4, 17u16 => 2, 30u16 => 4, 44u16 => 2},
        output: phf_map!{1043u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Arc Smelter + 5 Frame Material + 4 Plane Filter + 15 Unipolar Magnet -> 1 Plane Smelter
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 5.0,
        input: phf_map!{1043u16 => 1, 93u16 => 5, 66u16 => 4, 70u16 => 15},
        output: phf_map!{1044u16 => 1},
        allow_accelerate: true,
        allow_proliferate: false,
        is_lens: false
    },
    // 1 Plane Smelter + 10 Negentropy Singularity + 30 Energy Shard + 4 Quantum Chip -> 1 Negentropy Smelter
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{1044u16 => 1, 108u16 => 10, 106u16 => 30, 67u16 => 4},
        output: phf_map!{1045u16 => 1},
        allow_accelerate: true,
        allow_proliferate: false,
        is_lens: false
    },
    // 4 Iron Ingot + 8 Gear + 4 Circuit Board -> 1 Assembling Machine Mk.I
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 2.0,
        input: phf_map!{15u16 => 4, 57u16 => 8, 44u16 => 4},
        output: phf_map!{1046u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Assembling Machine Mk.I + 4 Processor + 8 Graphene -> 1 Assembling Machine Mk.II
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 3.0,
        input: phf_map!{1046u16 => 1, 61u16 => 4, 38u16 => 8},
        output: phf_map!{1047u16 => 1},
        allow_accelerate: true,
        allow_proliferate: false,
        is_lens: false
    },
    // 1 Assembling Machine Mk.II + 8 Particle Broadband + 2 Quantum Chip -> 1 Assembling Machine Mk.III
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 4.0,
        input: phf_map!{1047u16 => 1, 53u16 => 8, 67u16 => 2},
        output: phf_map!{1048u16 => 1},
        allow_accelerate: true,
        allow_proliferate: false,
        is_lens: false
    },
    // 1 Assembling Machine Mk.III + 10 Matter Recombinator + 30 Energy Shard + 4 Quantum Chip -> 1 Re-Composing Assembler
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{1048u16 => 1, 109u16 => 10, 106u16 => 30, 67u16 => 4},
        output: phf_map!{1049u16 => 1},
        allow_accelerate: true,
        allow_proliferate: false,
        is_lens: false
    },
    // 8 Iron Ingot + 4 Glass + 4 Circuit Board + 4 Magnetic Coil -> 1 Matrix Lab
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 3.0,
        input: phf_map!{15u16 => 8, 31u16 => 4, 44u16 => 4, 30u16 => 4},
        output: phf_map!{1050u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 1 Matrix Lab + 10 Silicon-Based Neuron + 20 Dark Fog Matrix + 4 Quantum Chip -> 1 Self-Evolution Lab
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 4.0,
        input: phf_map!{1050u16 => 1, 107u16 => 10, 105u16 => 20, 67u16 => 4},
        output: phf_map!{1051u16 => 1},
        allow_accelerate: true,
        allow_proliferate: false,
        is_lens: false
    },
    // 20 Steel + 20 Gear + 5 Processor + 10 Super Magnetic Ring -> 1 EM Rail Ejector
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{43u16 => 20, 57u16 => 20, 61u16 => 5, 74u16 => 10},
        output: phf_map!{1052u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 80 Titanium Alloy + 30 Frame Material + 20 Gravity Lens + 10 Quantum Chip -> 1 Vertical Launch Silo
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 30.0,
        input: phf_map!{34u16 => 80, 93u16 => 30, 89u16 => 20, 67u16 => 10},
        output: phf_map!{1053u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 8 Iron Ingot + 8 Gear + 2 Circuit Board + 4 Magnetic Coil -> 1 Gauss Turret
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 4.0,
        input: phf_map!{15u16 => 8, 57u16 => 8, 44u16 => 2, 30u16 => 4},
        output: phf_map!{1057u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 8 Steel + 6 Electric Motor + 12 Circuit Board + 6 Engine -> 1 Missile Turret
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{43u16 => 8, 46u16 => 6, 44u16 => 12, 71u16 => 6},
        output: phf_map!{1058u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 10 Steel + 8 Electric Motor + 10 Circuit Board + 2 Super Magnetic Ring -> 1 Implosion Cannon
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 5.0,
        input: phf_map!{43u16 => 10, 46u16 => 8, 44u16 => 10, 74u16 => 2},
        output: phf_map!{1059u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 9 Steel + 6 Plasma Exciter + 6 Circuit Board + 9 Proton Combiner -> 1 Laser Turret
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{43u16 => 9, 58u16 => 6, 44u16 => 6, 59u16 => 9},
        output: phf_map!{1060u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 20 Titanium Alloy + 10 Titanium Glass + 10 Super Magnetic Ring + 5 Processor + 5 Plasma Exciter -> 1 Plasma Turret
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 10.0,
        input: phf_map!{34u16 => 20, 65u16 => 10, 74u16 => 10, 61u16 => 5, 58u16 => 5},
        output: phf_map!{1061u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 15 Steel + 5 Super Magnetic Ring + 5 Plasma Exciter + 5 Processor -> 1 SR Plasma Turret
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 8.0,
        input: phf_map!{43u16 => 15, 74u16 => 5, 58u16 => 5, 61u16 => 5},
        output: phf_map!{1062u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 12 Steel + 18 Circuit Board + 6 Microcrystalline Component + 12 Engine -> 1 Battlefield Analysis Base
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{43u16 => 12, 44u16 => 18, 47u16 => 6, 71u16 => 12},
        output: phf_map!{1063u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 12 Copper + 9 Plasma Exciter + 6 Diamond + 3 Processor -> 1 Jamming Tower
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 5.0,
        input: phf_map!{16u16 => 12, 58u16 => 9, 32u16 => 6, 61u16 => 3},
        output: phf_map!{1064u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 2 Wireless Power Tower + 12 Steel + 6 Crystal Silicon -> 1 Signal Tower
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 6.0,
        input: phf_map!{1002u16 => 2, 43u16 => 12, 33u16 => 6},
        output: phf_map!{1065u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
    // 20 Steel + 20 Electormagnetic Turbine + 5 Super Magnetic Ring + 5 Particle Container -> 1 Planetary Shield Generator
    Recipe {
        recipe_type: RecipeType::Assemble,
        time: 10.0,
        input: phf_map!{43u16 => 20, 60u16 => 20, 74u16 => 5, 75u16 => 5},
        output: phf_map!{1066u16 => 1},
        allow_accelerate: true,
        allow_proliferate: true,
        is_lens: false
    },
];
