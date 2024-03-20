use std::ops::Range;

use iced::{
    alignment::Alignment,
    theme,
    widget::{
        button, container, row, column, scrollable, text, text_input, tooltip, checkbox, horizontal_rule,
        Column, Container, Row, Text, TextInput,
        image::{Handle, Image},
        scrollable::{Direction, Properties}
    }, Element, Length, Renderer
};
use phf::Map;

use crate::{
    abc::{
        data::Recipe,
        logic::{RecipeSolution, RecipeStep, ProliferatorEffect},
        ui::{
            style::*,
            Message, ToUI, UIObject, Visualize
        }
    },
    data::{DUMMY_ICON, ICON_MAP, PROLIFERATORS},
    style::*
};

// TODO: Add global recipe config toggle for all steps
// TODO: Add input panel to choose target item and its production rate

// Implement SetStyle trait for widgets

impl<'a> SetStyle<TextStyle> for Text<'a, Renderer> {
    fn set_style(self, style: &TextStyle) -> Self {
        let mut text = self;
        text = if let Some(size) = style.size { text.size(size) } else { text };
        text = if let Some(width) = style.width { text.width(width) } else { text };
        text = if let Some(height) = style.height { text.height(height) } else { text };
        text = if let Some(horizontal_alignment) = style.horizontal_alignment {
            text.horizontal_alignment(horizontal_alignment)
        } else { text };
        text = if let Some(vertical_alignment) = style.vertical_alignment {
            text.vertical_alignment(vertical_alignment)
        } else { text };
        text
    }
}

impl<'a> SetStyle<RowStyle> for Row<'a, Message, Renderer> {
    fn set_style(self, style: &RowStyle) -> Self {
        let mut row = self;
        row = if let Some(spacing) = style.spacing { row.spacing(spacing) } else { row };
        row = if let Some(padding) = style.padding { row.padding(padding) } else { row };
        row = if let Some(height) = style.height { row.height(height) } else { row };
        row = if let Some(align_items) = style.align_items { row.align_items(align_items) } else { row };
        row
    }
}

impl<'a> SetStyle<TextInputStyle> for TextInput<'a, Message, Renderer> {
    fn set_style(self, style: &TextInputStyle) -> Self {
        let mut input = self;
        input = if let Some(size) = style.size { input.size(size) } else { input };
        input = if let Some(width) = style.width { input.width(width) } else { input };
        input
    }
}

impl SetStyle<ImageStyle> for Image<Handle> {
    fn set_style(self, style: &ImageStyle) -> Self {
        let mut image = self;
        image = if let Some(width) = style.width { image.width(width) } else { image };
        image = if let Some(height) = style.height { image.height(height) } else { image };
        image
    }
}

impl SetStyle<ContainerStyle> for Container<'_, Message, Renderer> {
    fn set_style(self, style: &ContainerStyle) -> Self {
        let mut container = self;
        container = if let Some(width) = style.width { container.width(width) } else { container };
        container = if let Some(height) = style.height { container.height(height) } else { container };
        container = if let Some(padding) = style.padding { container.padding(padding) } else { container };
        container
    }
}

fn get_recipe_item_widget<'a>(id: &'a u16, amount: i16) -> Element<'a, Message, Renderer> {
    let icon = id.to_ui().visualize().into();
    let text = text(amount.to_string()).set_style(&ITEM_COUNT_TEXT_STYLE);
    row![icon, text]
        .set_style(&RECIPE_ITEM_STYLE)
        .into()
}

fn get_recipe_side_widget<'a>(map: &'a Map<u16, i16>) -> Element<'a, Message, Renderer> {
    let mut row = Row::new();
    let mut entries = Vec::from_iter(map.entries());
    entries.sort_by(|a: &(&u16, &i16), b| a.0.cmp(b.0));
    for (id, amount) in entries {
        let widget = get_recipe_item_widget(id, *amount);
        row = row.push(widget);
    }
    row.set_style(&RECIPE_SIDE_STYLE).into()
}

// Implement ToUI trait for data types

impl<'a> ToUI for u16 {
    fn to_ui(&self) -> UIObject<Self> {
        UIObject::new(self, vec![])
    }
}

impl<'a> Visualize<'a> for UIObject<'a, u16> {
    fn visualize(&self) -> impl Into<Element<'a, Message, Renderer>> where Self: Sized {
        let id = *self.parent;

        let memory = ICON_MAP.get(&id).unwrap_or(&DUMMY_ICON);
        let handle = Handle::from_memory(memory);
        let image = Image::<Handle>::new(handle).set_style(&ICON_STYLE);
        image
    }
}

impl ToUI for Recipe {
    fn to_ui(&self) -> UIObject<Self> where Self: Sized {
        UIObject::new(self, {
            let (in_num, out_num) = (self.input.len(), self.output.len());
            [
                if in_num == 0 {
                    50 // out_num should equal to 1
                } else {
                    // 50 if for each input count < 10 otherwise 60
                    50 + out_num * 50 + self.input
                        .values()
                        .map(|x| if *x < 10 { 50 } else { 60 })
                        .sum::<usize>()
                } as u16
            ].to_vec()
        })
    }
}

impl<'a> Visualize<'a> for UIObject<'a, Recipe> {
    fn visualize(&self) -> impl Into<Element<'a, Message, Renderer>> where Self: Sized {
        let mut row = Row::new();
        if self.parent.input.len() > 0 {
            row = row.push(get_recipe_side_widget(&self.parent.input));
            row = row.push(
                text(format!("---({}s)--->", self.parent.time))
                    .set_style(&ARROW_TEXT_STYLE)
            );
        }
        row = row.push(get_recipe_side_widget(&self.parent.output));
        row.set_style(&RECIPE_ROW_STYLE).width(self.widths[0])
    }
}

impl<'a> Visualize<'a> for UIObject<'a, RecipeStep> {
    fn visualize(&self) -> impl Into<Element<'a, Message, Renderer>> where Self: Sized {
        // Column 1: Target item and production rate
        let target_item = self.parent.target_item.clone();
        let target_item_col = row![
            self.parent.target_item.to_ui().visualize().into(),
            text_input("", &format!("{:.2}", self.parent.num_per_min))
                .set_style(&ITEM_RATE_INPUT_STYLE)
                .on_input(move |x| {
                    Message::RecipeRateUpdated((target_item, x.parse().unwrap_or(0.0)))
                })
        ].set_style(&ITEM_COUNT_STYLE);
        // Column 2: Building icon and building count
        let building_count_col = row![
            self.parent.get_building().to_ui().visualize().into(),
            text_input("", &format!("{:.2}", self.parent.get_building_count()))
                .set_style(&ITEM_RATE_INPUT_STYLE)
                .on_input(move |x| {
                    Message::RecipeBuildingCountUpdated((target_item, x.parse().unwrap_or(0.0)))
                })
        ].set_style(&ITEM_COUNT_STYLE);

        row![
            target_item_col, building_count_col,
            recipe_selector(self), // Column 3: Recipe selector
            recipe_building_selector(self), // Column 4: Building candidate for the selected recipe
            recipe_proliferator_selector(self) // Column 5: Proliferator settings
        ]
            .set_style(&RECIPE_STEP_STYLE)
    }
}

impl ToUI for RecipeStep {
    fn to_ui(&self) -> UIObject<Self> {
        UIObject::new(self, vec![
            150, 150,
            self.recipe_candidates
                .iter()
                .map(|x| x.to_ui().get_display_width())
                .max()
                .unwrap()[0],
            self.building_candidates.len() as u16 * 44,
            {
                let current_recipe = self.get_recipe();
                52 * 4 + 25
                + if current_recipe.allow_proliferate { 52 } else { 0 }
                + if current_recipe.allow_accelerate { 52 } else { 0 }
                + if current_recipe.is_lens { 52 } else { 0 }
            }
        ])
    }
}

impl<'a> Visualize<'a> for UIObject<'a, RecipeSolution> {
    fn visualize(&self) -> impl Into<Element<'a, Message, Renderer>> where Self: Sized {
        let parent = self.parent;

        let mut targets = Row::new();
        if parent.targets.len() > 0 {
            for (id, rate) in parent.targets.iter() {
                targets = targets.push(
                    row![
                        id.to_ui().visualize().into(),
                        text_input("", &format!("{:.2}", rate))
                            .set_style(&ITEM_RATE_INPUT_STYLE)
                            .on_input(move |x| {
                                Message::RecipeRateUpdated((*id, x.parse().unwrap_or(0.0)))
                            }),
                        button(text("Del").set_style(&DELETE_TEXT_STYLE))
                            .padding(4.5)
                            .on_press(Message::TargetRemoved(*id))
                            .style(theme::Button::Destructive)
                    ].set_style(&ITEM_COUNT_STYLE)
                );
            }
        }
        targets = targets.spacing(10);

        let mut solution = Column::new();
        if parent.recipe_steps.len() > 0 {
            let mut ui_objs = parent.recipe_steps
                .iter()
                .map(|x| (x.target_item, x.to_ui()))
                .collect::<Vec<_>>();
            let mut max_widths: Vec<u16> = ui_objs[0].1.get_display_width();
            for (_, obj) in ui_objs.iter() {
                for (idx, width) in obj.get_display_width().iter().enumerate() {
                    if width > &max_widths[idx] { max_widths[idx] = *width; }
                }
            }
            for (id, obj) in ui_objs.iter_mut() {
                let new_id = id.clone();
                obj.set_display_width(&max_widths);
                solution = solution.push(row![
                    container(checkbox(
                        "", parent.is_ignored(*id),
                        move |x| Message::RecipeItemIgnored((new_id, x))
                    ).size(15).style(theme::Checkbox::Success)).set_style(&CHECKBOX_PADDING),
                    obj.visualize().into()
                ]);
            }
        }

        let container_ = column![
            scrollable(targets)
                .direction(Direction::Horizontal(Properties::default())),
            solution
        ].spacing(10);

        scrollable(container_.padding(15))
            .width(Length::Fill)
            .height(Length::Fill)
            .direction(Direction::Both {
                vertical: Properties::default(),
                horizontal: Properties::default()
            })
    }
}

impl ToUI for RecipeSolution {
    fn to_ui(&self) -> UIObject<Self> {
        UIObject::new(self, vec![])
    }
}

impl ToUI for Vec<(u16, f64)> {
    fn to_ui(&self) -> UIObject<Self> {
        UIObject::new(self, vec![])
    }
}

fn button_theme(selected: bool) -> theme::Button {
    if selected {
        theme::Button::Positive
    } else {
        theme::Button::Secondary
    }
}

fn recipe_proliferator_selector<'a>(
    recipe_step: &UIObject<RecipeStep>,
) -> Element<'a, Message, Renderer> {
    let parent = recipe_step.parent;
    let recipe = parent.recipe_candidates[parent.config.selected_recipe];
    let current_effect = parent.config.proliferator_usage;
    let current_level = current_effect.level();

    let mut values = PROLIFERATORS.values().collect::<Vec<_>>();
    values.sort_by(|a: &&crate::abc::data::Proliferator, b| a.level.cmp(&b.level));

    let levels = row(values
        .iter()
        .map(|v| {
            if v.level == 0 {
                button(text("N/A").set_style(&SELECTOR_TEXT_STYLE))
            } else {
                button(v.id.to_ui().visualize())
            }
            .style(button_theme(v.level == current_level))
            .on_press(Message::RecipeConfigUpdated(
                parent.set_proliferator_level(v.level)
            ))
            .into()
        }).collect::<Vec<_>>()).spacing(2);

    // Evaluate available effects
    let mut effect_container = Row::new();
    {
        let effect = ProliferatorEffect::None(current_level);
        effect_container = effect_container.push(
        button(text("N/A").set_style(&SELECTOR_TEXT_STYLE))
            .style(button_theme(current_effect == effect))
            .on_press(Message::RecipeConfigUpdated(
                parent.set_proliferator(effect)
            ))
        );
    }
    if recipe.allow_accelerate {
        let effect = ProliferatorEffect::Accelerate(current_level);
        effect_container = effect_container.push(
        button(text("Acc").set_style(&SELECTOR_TEXT_STYLE))
            .style(button_theme(current_effect == effect))
            .on_press(Message::RecipeConfigUpdated(
                parent.set_proliferator(effect)
            ))
        );
    }
    if recipe.allow_proliferate {
        let effect = ProliferatorEffect::Proliferate(current_level);
        effect_container = effect_container.push(
        button(text("Pro").set_style(&SELECTOR_TEXT_STYLE))
            .style(button_theme(current_effect == effect))
            .on_press(Message::RecipeConfigUpdated(
                parent.set_proliferator(effect)
            ))
        );
    }
    if recipe.is_lens {
        let effect = ProliferatorEffect::Lens(current_level);
        effect_container = effect_container.push(
        button(text("Lens").set_style(&SELECTOR_TEXT_STYLE))
            .style(button_theme(current_effect == effect))
            .on_press(Message::RecipeConfigUpdated(
                parent.set_proliferator(effect)
            ))
        );
    }

    row![levels, effect_container.set_style(&BUTTON_SELECTOR_STYLE)]
        .width(recipe_step.get_display_width()[4])
        .set_style(&PROLIFERATOR_CONFIG_STYLE)
        .into()
}

fn recipe_building_selector<'a>(recipe_step: &UIObject<'a, RecipeStep>) -> Element<'a, Message, Renderer> {
    let parent = recipe_step.parent;
    let mut container_ = Row::new();
    let building = parent.get_building();

    for (index, building_id) in parent.building_candidates.iter().enumerate() {
        let row = button(building_id.to_ui().visualize())
            .style(button_theme(*building_id == *building))
            .on_press(Message::RecipeConfigUpdated(parent.set_building(index)));
        container_ = container_.push(row);
    }
    container_
        .set_style(&BUTTON_SELECTOR_STYLE)
        .width(recipe_step.get_display_width()[3])
        .align_items(Alignment::Start)
        .into()
}

pub fn recipe_selector<'a>(recipe_step: &UIObject<'a, RecipeStep>) -> Element<'a, Message, Renderer> {
    let parent = recipe_step.parent;
    let recipes = &parent.recipe_candidates;
    let selected = parent.config.selected_recipe;

    // Initialize components, set width to the maximum width
    let mut container_ = Column::new();
    let mut ui_objs = recipes.iter().map(|x| x.to_ui()).collect::<Vec<_>>();
    let max_width = recipe_step.get_display_width()[2];
    for obj in ui_objs.iter_mut() {
        obj.set_display_width(&vec![max_width]);
    }

    for (idx, obj) in ui_objs.iter().enumerate() {
        let item_button = button(obj.visualize())
            .on_press(Message::RecipeConfigUpdated(parent.set_recipe(idx)))
            .width(max_width)
            .style(button_theme(idx == selected));
        container_ = container_.push(item_button);
    }
    container_
        .spacing(2)
        .into()
}

fn grid_selector<'a>(
    range: Range<u16>,
    icons: &'a Map<u16, &'static [u8]>,
) -> Element<'a, Message, Renderer> {
    let mut container_ = Column::new();
    let max_id = icons.keys().filter(|x| range.contains(x)).max().unwrap();
    let (item_count, modifier) = (max_id % 1000, max_id / 1000);
    let (columns, rows) = (14u16, (item_count - 1) / 14 + 1);

    for i in 0..rows {
        let mut row = Row::new();
        for j in 0..columns {
            let id = (i * columns + j + 1) + modifier * 1000;
            let (ui_obj, success) = icons
                .get_entry(&id)
                .map(|(k, _)| (k.to_ui(), true))
                .unwrap_or((0u16.to_ui(), false));
            row = row.push(button(ui_obj.visualize())
                .on_press_maybe(
                    if success { Some(Message::ItemSelected(id)) } else { None }
                )
                .style(theme::Button::Secondary)
                .width(40)
                .height(40)
            );
        }
        container_ = container_.push(row.spacing(2));
    }
    container_.spacing(2).into()
}

pub fn item_selector<'a>() -> Element<'a, Message, Renderer> {
    grid_selector(0..1000u16, &ICON_MAP)
}

pub fn building_selector<'a>() -> Element<'a, Message, Renderer> {
    grid_selector(1000..2000u16, &ICON_MAP)
}
