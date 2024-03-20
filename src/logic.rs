use std::collections::{HashMap, HashSet};

use crate::abc::{
    data::{Recipe, RecipeType, Rate},
    logic::{ProliferatorEffect, Default, GlobalConfig, RecipeConfig, RecipeSolution, RecipeStep},
};
use crate::data::{RECIPES, PRODUCTION_BUILDING, PROLIFERATORS};

// TODO: Fix recipe: 1 Coal + 2 Crude Oil => 3 Refined Oil
// TODO: Add export and load function
// TODO: Fix ray receiver lens consumption

// TODO: Add solution configuration:
// 1. Miner speed (mining machine, oil extractor, water pump, orbital collector)
// 2. Add tech level

impl Default for ProliferatorEffect {
    fn default() -> Self {
        ProliferatorEffect::None(0)
    }
}

impl Default for GlobalConfig {
    fn default() -> Self {
        GlobalConfig {
            fractionator_speed: 7200.0,
            proliferator_spray_level: u8::MAX,
            ignored_items: HashSet::new(),
            default_building: HashMap::new(),
            default_proliferator: ProliferatorEffect::default()
        }
    }
}

impl Default for RecipeConfig {
    fn default() -> Self {
        RecipeConfig {
            selected_recipe: 0,
            preferred_building: HashMap::new(),
            proliferator_usage: ProliferatorEffect::default()
        }
    }

}

impl ProliferatorEffect {
    pub fn effect(&self) -> f64 {
        match self {
            ProliferatorEffect::None(_) => 1.0,
            ProliferatorEffect::Accelerate(level) => PROLIFERATORS.get(&level).unwrap().accelerate_effect,
            ProliferatorEffect::Proliferate(level) => PROLIFERATORS.get(&level).unwrap().proliferate_effect,
            ProliferatorEffect::Lens(level) => PROLIFERATORS.get(&level).unwrap().lens_effect
        }
    }

    pub fn power(&self) -> f64 {
        match self {
            ProliferatorEffect::None(_) => 1.0,
            effect => PROLIFERATORS.get(&effect.level()).unwrap().power_consumption
        }
    }

    pub fn spray_num(&self) -> u8 {
        PROLIFERATORS.get(&self.level()).unwrap().spray_num
    }

    pub fn level(&self) -> u8 {
        match self {
            ProliferatorEffect::None(level) => *level,
            ProliferatorEffect::Accelerate(level) => *level,
            ProliferatorEffect::Proliferate(level) => *level,
            ProliferatorEffect::Lens(level) => *level
        }
    }

    pub fn set_level(&self, level: u8) -> Self {
        match self {
            ProliferatorEffect::None(_) => ProliferatorEffect::None(level),
            ProliferatorEffect::Accelerate(_) => ProliferatorEffect::Accelerate(level),
            ProliferatorEffect::Proliferate(_) => ProliferatorEffect::Proliferate(level),
            ProliferatorEffect::Lens(_) => ProliferatorEffect::Lens(level)
        }
    }
}

impl RecipeConfig {
    pub fn set_recipe(&self, idx: usize) -> Self {
        // Set preferred recipe
        let mut ret = self.clone();
        ret.selected_recipe = idx;
        ret
    }

    pub fn set_building(&self, recipe_type: RecipeType, idx: usize) -> Self {
        // Set preferred building for a recipe type
        let mut ret = self.clone();
        ret.preferred_building.insert(recipe_type, idx);
        ret
    }

    pub fn set_proliferator(&self, effect: ProliferatorEffect) -> Self {
        let mut ret = self.clone();
        ret.proliferator_usage = effect;
        ret
    }

    pub fn toggle_proliferator(&self, level: u8) -> Self {
        let mut ret = self.clone();
        ret.proliferator_usage = ret.proliferator_usage.set_level(level);
        ret
    }

    pub fn get_recipe(&self) -> usize {
        self.selected_recipe
    }

    pub fn get_building(&self, recipe_type: RecipeType) -> usize {
        *self.preferred_building.get(&recipe_type).unwrap_or(&0)
    }

    pub fn get_proliferator(&self) -> ProliferatorEffect {
        self.proliferator_usage
    }
}

impl RecipeStep {
    pub fn new(target_item: u16, num_per_min: f64, config: RecipeConfig) -> Self {
        let recipe_candidate = query_recipe(target_item);
        let selected_recipe = recipe_candidate[config.get_recipe()];
        let mut building_candidate = query_building(selected_recipe.recipe_type.clone());
        building_candidate.sort();

        // Check whether proliferator usage is allowed
        let tuple = (
            selected_recipe.allow_accelerate,
            selected_recipe.allow_proliferate,
            selected_recipe.is_lens
        );
        let mut new_config = config.clone();
        new_config = new_config.set_proliferator(
            match (config.clone().proliferator_usage, tuple) {
                (ProliferatorEffect::None(_), _)
                    |  (ProliferatorEffect::Accelerate(_), (false, _, _))
                    | (ProliferatorEffect::Proliferate(_), (_, false, _))
                    | (ProliferatorEffect::Lens(_), (_, _, false))
                => None,
                (effect, _) => Some(effect)
            }.unwrap_or(ProliferatorEffect::None(0))
        );

        RecipeStep {
            target_item,
            num_per_min, // Target item per minute but not recipe per minute
            recipe_candidates: recipe_candidate,
            building_candidates: building_candidate,
            config: new_config,
            speed_factor: 1.0
        }
    }

    // Interfaces
    pub fn get_recipe(&self) -> &Recipe {
        self.recipe_candidates[self.config.get_recipe()]
    }

    pub fn set_speed_factor(&mut self, factor: f64) {
        self.speed_factor = factor;
    }

    pub fn get_building(&self) -> &u16 {
        self.building_candidates.get(self.config.get_building(self.get_recipe_type())).unwrap()
    }

    fn get_recipe_type(&self) -> RecipeType {
        self.get_recipe().recipe_type.clone()
    }

    fn get_target_rate(&self) -> f64 {
        self.num_per_min
    }

    fn _set_target_rate(&mut self, rate: f64) {
        // Merge two recipe steps
        self.num_per_min = rate;
    }

    fn get_recipe_rate(&self) -> f64 {
        // Recipe execution per minute
        let target_coef = self.get_item_coef(self.target_item);
        let effect = self.config.get_proliferator().effect();
        self.get_target_rate().max(0.0) / target_coef * match self.config.proliferator_usage {
            ProliferatorEffect::Proliferate(_) => 1.0 / effect,
            ProliferatorEffect::Accelerate(_) => 1.0 / effect,
            ProliferatorEffect::Lens(_) => effect,
            ProliferatorEffect::None(_) => effect
        }
    }

    fn get_proliferator_rate(&self) -> Option<(u8, u16, f64)> {
        let num_inputs: i16 = self.get_recipe().input
            .values()
            .sum();
        let level = self.config.get_proliferator().level();
        if level == 0 { return None; }
        let id = PROLIFERATORS.get(&level).unwrap().id;
        match self.config.get_proliferator() {
            ProliferatorEffect::None(_) => None,
            ProliferatorEffect::Lens(_) => None, // TODO
            others => {
                let amount = if let ProliferatorEffect::Proliferate(_) = others {
                    self.get_recipe_rate()
                } else {
                    self.get_recipe_rate() * others.effect()
                } / others.spray_num() as f64;
                Some((level, id, amount))
            }
        }.and_then(|(level, id, amount)| {
            Some((level, id, amount * num_inputs as f64))
        })
    }

    fn get_item_coef(&self, id: u16) -> f64 {
        let recipe = self.get_recipe();
        let prod_in_num = *recipe.input.get(&id).unwrap_or(&0) as f64;
        let prod_out_num = *recipe.output.get(&id).unwrap_or(&0) as f64;
        prod_out_num - prod_in_num
    }

    pub fn get_by_product_rate(&self) -> Vec<Rate> {
        let recipe = self.get_recipe();
        let mut result: Vec<Rate> = Vec::new();
        for id in recipe.output.keys() {
            if *id == self.target_item { continue; }
            let by_coef = self.get_item_coef(*id);
            let main_coef = self.get_item_coef(self.target_item);
            result.push((*id, self.get_target_rate().max(0.0) as f64 * by_coef / main_coef));
        }
        result
    }

    pub fn get_power_usage(&self) -> f64 {
        let building_info = PRODUCTION_BUILDING.get(&self.get_building()).unwrap();
        self.get_building_count() as f64
            * building_info.work_power as f64
            * self.config.proliferator_usage.power()
            / 1000.0 // Unit: MW
    }

    pub fn get_building_count(&self) -> f64 {
        let recipe = self.get_recipe();
        let building_info = PRODUCTION_BUILDING.get(&self.get_building()).unwrap();
        self.get_recipe_rate() / 60.0 * recipe.time / building_info.speed / self.speed_factor
    }

    fn get_input_items(&self) -> Vec<u16> {
        let recipe = self.get_recipe();
        let mut result = Vec::new();
        for id in recipe.input.keys() {
            if self.get_item_coef(*id) < 0.0 { result.push(*id); }
        }
        result
    }

    fn get_input_rate(&self) -> Vec<Rate> {
        // Source input per minute
        self.get_input_items().iter()
            .map(|x| {
                let coef = -self.get_item_coef(*x);
                let recipe_per_min = self.get_recipe_rate();
                let effect = self.config.proliferator_usage.effect();
                (*x, coef * recipe_per_min * match self.config.proliferator_usage {
                    ProliferatorEffect::Accelerate(_) => effect,
                    _ => 1.0
                })
            })
            .collect()
    }

    // Interfaces used by external ui
    pub fn set_recipe(&self, idx: usize) -> (u16, RecipeConfig) {
        let mut ret = self.clone();
        ret.config = ret.config.set_recipe(idx);
        let mut building_candidate = query_building(ret.get_recipe_type());
        building_candidate.sort();
        ret.building_candidates = building_candidate;

        (ret.target_item, ret.config.clone())
    }
    pub fn set_building(&self, idx: usize) -> (u16, RecipeConfig) {
        let mut ret = self.clone();
        ret.config = ret.config.set_building(self.get_recipe_type(), idx);
        (ret.target_item, ret.config.clone())
    }
    pub fn set_proliferator(&self, effect: ProliferatorEffect) -> (u16, RecipeConfig) {
        let mut ret = self.clone();
        ret.config = ret.config.set_proliferator(effect);
        (ret.target_item, ret.config.clone())
    }
    pub fn set_proliferator_level(&self, level: u8) -> (u16, RecipeConfig) {
        let mut ret = self.clone();
        ret.config = ret.config.toggle_proliferator(level);
        (ret.target_item, ret.config.clone())
    }
}

impl Default for RecipeSolution {
    fn default() -> Self {
        RecipeSolution {
            targets: HashMap::<u16, f64>::new(),
            recipe_steps: Vec::new(),
            recipe_configs: HashMap::new(),

            global_config: GlobalConfig::default()
        }
    }
}

impl RecipeSolution {
    pub fn reset(mut self) -> Self {
        self.targets.clear();
        self.recipe_steps.clear();
        self.recipe_configs.clear();
        self.global_config.ignored_items.clear();
        self
    }

    pub fn add_target(mut self, id: u16, num: f64) -> Self {
        match self.targets.get_mut(&id) {
            Some(x) => *x += num,
            None => { self.targets.insert(id, num); }
        }
        self.calculate()
    }

    pub fn remove_target(mut self, id: u16) -> Self {
        self.targets.remove(&id);
        self.calculate()
    }

    pub fn set_building(mut self, recipe_type: RecipeType, idx: usize) -> Self {
        match self.global_config.default_building.get_mut(&recipe_type) {
            Some(x) => *x = idx,
            None => { self.global_config.default_building.insert(recipe_type, idx); }
        }
        for recipe in self.recipe_configs.values_mut() {
            match recipe.preferred_building.get_mut(&recipe_type) {
                Some(x) => *x = idx,
                None => { recipe.preferred_building.insert(recipe_type, idx); }
            }
        }
        self.calculate()
    }

    pub fn set_proliferator(mut self, effect: ProliferatorEffect) -> Self {
        self.global_config.default_proliferator = effect;
        for recipe in self.recipe_configs.values_mut() {
            recipe.proliferator_usage = effect;
        }
        self.calculate()
    }

    pub fn ignore_item(mut self, id: u16) -> Self {
        self.global_config.ignored_items.insert(id);
        self.calculate()
    }

    pub fn unignore_item(mut self, id: u16) -> Self {
        self.global_config.ignored_items.remove(&id);
        self.calculate()
    }

    pub fn is_ignored(&self, id: u16) -> bool {
        self.global_config.ignored_items.contains(&id)
    }

    pub fn set_recipe_config(mut self, id: u16, config: RecipeConfig) -> Self {
        match self.recipe_configs.get_mut(&id) {
            Some(x) => *x = config,
            None => { self.recipe_configs.insert(id, config); }
        }
        self.calculate()
    }

    pub fn set_recipe_building_count(mut self, id: u16, count: f64) -> Self {
        let med = self.set_recipe_rate(id, 1.0); // Prevent divide by zero
        let building_count = med.recipe_steps
            .iter()
            .find(|x| x.target_item == id)
            .unwrap()
            .get_building_count();
        med.set_recipe_rate(id, count / building_count * 1.0)
    }

    pub fn set_recipe_rate(mut self, id: u16, rate: f64) -> Self {
        if let Some(x) = self.targets.get_mut(&id) {
            *x = rate;
        } else {
            let old_rate = self.recipe_steps.iter()
                .find(|x| x.target_item == id)
                .unwrap()
                .get_target_rate();
            self.targets.iter_mut()
                .for_each(|(_, num)| *num = *num / old_rate * rate);
        }
        self.calculate()
    }

    fn propagate(
        &mut self,
        display_seq: &mut Vec<u16>,
    ) -> HashMap<u16, RecipeStep> {
        let mut result: HashMap<u16, RecipeStep> = HashMap::new();
        let mut queue = display_seq.clone();

        while !queue.is_empty() {
            let item = queue.pop().unwrap();
            if !display_seq.contains(&item) { display_seq.push(item); }
            if result.contains_key(&item) { continue; }
            let step = self.create_step(item, 0.0);
            if let Some((_, id, _)) = step.get_proliferator_rate() {
                queue.push(id);
                match self.global_config.proliferator_spray_level {
                    0 | u8::MAX => {},
                    level => queue.push(PROLIFERATORS.get(&level).unwrap().id)
                }
            }
            if !self.global_config.ignored_items.contains(&item) {
                queue.extend(
                    step.get_input_rate()
                    .iter()
                    .chain(step.get_by_product_rate().iter())
                    .map(|(id, _)| id)
                );
            }
            result.insert(item, step);
        }

        result
    }

    fn create_step(&mut self, id: u16, num: f64) -> RecipeStep {
        if !self.recipe_configs.contains_key(&id) {
            let mut default_config = RecipeConfig::default();
            let _ = self.global_config.default_building
                .iter()
                .for_each(|(k, v)| {
                    default_config = default_config.set_building(*k, *v);
                });
            default_config = default_config.set_proliferator(self.global_config.default_proliferator);
            self.recipe_configs.insert(id, default_config);
        }
        let config = self.recipe_configs.get(&id).unwrap().clone();
        RecipeStep::new(id, num, config)
    }

    pub fn get_building_summary(&self) -> HashMap<u16, i32> {
        let mut result: HashMap<u16, i32> = HashMap::new();
        for step in &self.recipe_steps {
            let building = step.get_building();
            match result.get_mut(&building) {
                Some(x) => *x += step.get_building_count().ceil() as i32,
                None => {
                    result.insert(*building, 1);
                }
            }
        }
        result
    }

    fn iterate(
        &mut self,
        dirty_items: &mut Vec<Rate>,
        item_list: &mut HashMap<u16, RecipeStep>
    ) -> f64 {
        // This function returns delta of the rate
        let mut delta = 0.0;
        while !dirty_items.is_empty() {
            let (id, mut num) = dirty_items.pop().unwrap();

            num += dirty_items.iter().filter(|(x, _)| *x == id).map(|(_, x)| *x).sum::<f64>();
            println!("Dirty items: {:?}", dirty_items);
            println!("{}: {}", id, num);
            dirty_items.retain(|(x, _)| *x != id);

            let step = item_list.get_mut(&id).unwrap();
            if (step.get_target_rate() - num).abs() < 1e-3 { continue; }
            delta += (step.get_target_rate() - num).abs();

            if !self.global_config.ignored_items.contains(&id) {
                let current_src_rate = step.get_input_rate();
                let current_out_rate = step.get_by_product_rate();
                step._set_target_rate(num);
                let new_src_rate = step.get_input_rate();
                let new_out_rate = step.get_by_product_rate();

                for (id, step_rate) in current_src_rate {
                    let total_rate = item_list.get(&id).unwrap().get_target_rate();
                    let new_rate = new_src_rate.iter().find(|(x, _)| *x == id).unwrap().1;
                    dirty_items.push((id, total_rate - step_rate + new_rate));
                }
                for (id, step_rate) in current_out_rate {
                    let total_rate = item_list.get(&id).unwrap().get_target_rate();
                    let new_rate = new_out_rate.iter().find(|(x, _)| *x == id).unwrap().1;
                    dirty_items.push((id, total_rate + step_rate - new_rate));
                }
            } else { step._set_target_rate(num); }
        }
        delta
    }

    pub fn get_power_usage(&self) -> f64 {
        self.recipe_steps
            .iter()
            .filter(|step| {
                match step.get_recipe_type() {
                    RecipeType::Mine |
                    RecipeType::Pump |
                    RecipeType::Extract |
                    RecipeType::Orbital |
                    RecipeType::Darkfog |
                    RecipeType::Nature |
                    RecipeType::Ray => false,
                    _ => true
                }
            })
            .map(|x| x.get_power_usage())
            .sum()
    }

    pub fn calculate(mut self) -> Self {
        self.recipe_steps.clear();
        if self.targets.len() == 0 { return self; }
        let mut display_seq: Vec<u16> = self.targets.keys().cloned().collect();
        let mut dirty_items: Vec<Rate> = Vec::new();

        // Propagate through the tree to get items required
        let mut item_list = self.propagate(&mut display_seq);
        for (id, num) in &self.targets {
            dirty_items.push((*id, *num));
        }

        // Update until convergence
        while self.iterate(&mut dirty_items, &mut item_list) > 1e-3 {
            let mut summed_usage: HashMap<(u8, u16), f64> = HashMap::new();
            let _: Vec<()> = item_list.iter()
                // Filter out ignored items
                .filter(|(id, _)| !self.global_config.ignored_items.contains(id))
                .map(|(_, step)| step.get_proliferator_rate())
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .map(|(level, id, num)| {
                    summed_usage.entry((level, id))
                        .and_modify(|x| *x += num)
                        .or_insert(num);
                })
                .collect();
            for ((level, id), num) in summed_usage.iter_mut() {
                let spray_level = match self.global_config.proliferator_spray_level {
                    u8::MAX => *level,
                    _ => self.global_config.proliferator_spray_level
                };
                if spray_level != 0 {
                    let spray_id: u16 = PROLIFERATORS.get(&spray_level).unwrap().id;
                    let sprayer = PROLIFERATORS.get(&spray_level).unwrap();
                    let spray_count = (sprayer.spray_num as f64 * sprayer.proliferate_effect) - 1.0;

                    let sprayed = PROLIFERATORS.get(&level).unwrap();
                    let spray_ratio = sprayed.spray_num as f64 / (sprayed.spray_num as f64 * sprayer.proliferate_effect).floor();
                    *num *= spray_ratio;
                    dirty_items.push((spray_id, *num / spray_count));
                }
                dirty_items.push((*id, *num));
                dirty_items.push((*id, *self.targets.get(id).unwrap_or(&0.0)));
            }
        }

        self.recipe_steps = display_seq.iter().map(|x| item_list.get(x).unwrap().clone()).collect();
        self
    }
}

fn get_direction(recipe: &Recipe, target_item: u16) -> bool {
    match (recipe.input.get(&target_item), recipe.output.get(&target_item)) {
        (None, Some(_)) => true,
        (_, None) => false,
        (Some(input), Some(output)) => input < output
    }
}

fn query_building(recipe_type: RecipeType) -> Vec<u16> {
    let mut result = Vec::new();
    for (id, building) in PRODUCTION_BUILDING.entries() {
        if building.recipe_type == recipe_type {
            result.push(*id);
        }
    }
    result
}

fn query_recipe(target_item: u16) -> Vec<&'static Recipe> {
    let mut result = Vec::new();
    for recipe in RECIPES.iter() {
        if get_direction(&recipe, target_item) {
            result.push(recipe);
        }
    }
    result
}
