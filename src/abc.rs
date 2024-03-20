pub mod data {
    use phf::Map;

    pub type Rate = (u16, f64); // Item id -> rate

    #[derive(Debug, Eq, PartialEq, Clone, Hash, Copy)]
    pub enum RecipeType {
        Smelt,
        Assemble,
        Refine,
        Chemical,
        Research,
        Particle,
        Fractionate, // Fractionator -> Deuterium
        Mine,
        Pump, // Water Pump -> Water, Sulfuric Acid
        Extract, // Oil Extractor -> Crude Oil
        Charge, // Energy Exchanger -> Accumulator (Full)
        Orbital, // Orbital Collector -> Hydrogen, Deuterium, Fire Ice
        Ray, // Ray Receiver -> Proton
        Darkfog, // Dark fog items
        Nature // Nature items
    }

    #[derive(Debug)]
    pub struct ProductionBuildingProperties {
        pub recipe_type: RecipeType,
        pub work_power: i32,
        pub stand_by_power: i32,
        pub speed: f64
    }

    #[derive(Debug)]
    pub struct Item {
        pub id: u16
    }
    impl Item {
        pub fn new(id: u16) -> Self {
            Item {id}
        }
    }

    #[derive(Debug)]
    pub struct Recipe {
        // Basic properties
        pub recipe_type: RecipeType,
        pub time: f64,

        // Recipe input and output
        pub input: Map<u16, i16>, // Item id -> count
        pub output: Map<u16, i16>, // Item id -> count

        // Proliferator effect
        pub allow_accelerate: bool,
        pub allow_proliferate: bool,

        // Modifiers
        pub is_lens: bool
    }

    #[derive(Debug, Clone)]
    pub struct Proliferator {
        pub level: u8,
        pub id: u16,
        pub spray_num: u8,
        pub accelerate_effect: f64,
        pub proliferate_effect: f64,
        pub lens_effect: f64,
        pub power_consumption: f64
    }
}

pub mod logic {
    use std::collections::{HashMap, HashSet};

    use super::data::{Recipe, RecipeType};

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum ProliferatorEffect {
        None(u8),
        Accelerate(u8),
        Proliferate(u8),
        Lens(u8)
    }

    #[derive(Debug, Clone)]
    pub struct RecipeConfig {
        pub selected_recipe: usize,
        pub preferred_building: HashMap<RecipeType, usize>,
        pub proliferator_usage: ProliferatorEffect
    }

    #[derive(Debug, Clone)]
    pub struct RecipeStep {
        pub target_item: u16,
        pub num_per_min: f64,

        // Recipe properties
        pub recipe_candidates: Vec<&'static Recipe>,
        // Production building properties
        pub building_candidates: Vec<u16>,
        // Proliferator effect
        pub config: RecipeConfig,
        pub speed_factor: f64
    }

    #[derive(Debug, Clone)]
    pub struct GlobalConfig {
        pub fractionator_speed: f64,
        pub proliferator_spray_level: u8,
        pub ignored_items: HashSet<u16>,
        pub default_building: HashMap<RecipeType, usize>,
        pub default_proliferator: ProliferatorEffect
    }

    pub trait Default {
        fn default() -> Self;
    }

    #[derive(Debug, Clone)]
    pub struct RecipeSolution {
        pub targets: HashMap<u16, f64>,
        pub recipe_steps: Vec<RecipeStep>,
        pub recipe_configs: HashMap<u16, RecipeConfig>,

        // Global solution configuration
        pub global_config: GlobalConfig,
    }
}

pub mod ui {
    use iced::{Element, Renderer};
    use super::logic::{GlobalConfig, RecipeConfig};

    pub type WidgetWidth = u16;

    #[derive(Debug, Clone)]
    pub enum Message {
        ItemSelected(u16),
        RecipeConfigUpdated((u16, RecipeConfig)), // (item id, altered recipe config)
        GlobalConfigUpdated(GlobalConfig),
        RecipeRateUpdated((u16, f64)), // (item id, new rate)
        RecipeBuildingCountUpdated((u16, f64)), // (item id, new building count)
        RecipeItemIgnored((u16, bool)), // (item id, ignore or not)
        TargetRemoved(u16)
    }

    // Wrapper for rendering data structure
    pub struct UIObject<'a, T> {
        pub parent: &'a T, // Store the reference to data structure
        pub widths: Vec<WidgetWidth> // Widths of different rendered widgets may interfere
    }

    pub trait ToUI {
        fn to_ui(&self) -> UIObject<Self> where Self: Sized;
    }

    pub trait Visualize<'a> {
        fn visualize(&self) -> impl Into<Element<'a, Message, Renderer>> where Self: Sized;
    }

    impl<'a, T> UIObject<'a, T> where T: ToUI {
        pub fn new(parent: &'a T, default_widths: Vec<WidgetWidth>) -> Self {
            UIObject {parent, widths: default_widths}
        }

        pub fn get_display_width(&self) -> Vec<WidgetWidth> {
            self.widths.clone()
        }

        pub fn set_display_width(&mut self, widths: &Vec<WidgetWidth>) {
            self.widths = widths.clone();
        }
    }

    pub mod style {
        use iced::{
            alignment::{Horizontal, Vertical, Alignment},
            Length, Padding, Pixels
        };

        pub struct ContainerStyle {
            pub width: Option<Length>,
            pub height: Option<Length>,
            pub padding: Option<Padding>
        }

        pub struct ImageStyle {
            pub width: Option<Length>,
            pub height: Option<Length>
        }

        pub struct TextStyle {
            pub size: Option<Pixels>,
            pub width: Option<Length>,
            pub height: Option<Length>,
            pub horizontal_alignment: Option<Horizontal>,
            pub vertical_alignment: Option<Vertical>
        }

        pub struct TextInputStyle {
            pub size: Option<Pixels>,
            pub width: Option<Length>
        }

        pub struct RowStyle {
            pub spacing: Option<Pixels>,
            pub padding: Option<Padding>,
            pub height: Option<Length>,
            pub align_items: Option<Alignment>
        }

        pub trait SetStyle<Style> {
            fn set_style(self, style: &Style) -> Self;
        }
    }
}

pub mod error {
    pub enum Error {
        MergeError
    }
}
