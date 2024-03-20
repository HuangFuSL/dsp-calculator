pub mod abc;
pub mod data;
pub mod logic;
pub mod ui;
pub mod style;

use abc::logic::{Default, ProliferatorEffect, RecipeSolution};
use abc::ui::{Message, ToUI, Visualize};
use iced::widget::Row;
use iced::{widget::column, Application, Theme};

fn main() -> Result<(), iced::Error> {
    MainInterface::run(iced::Settings::default())
}


struct MainInterface {
    index: u16,
    targets: Vec<(u16, f64)>,
    steps: RecipeSolution
}

impl Application for MainInterface {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (MainInterface, iced::Command<Self::Message>) {
        (
            MainInterface {
                index: 0, steps: RecipeSolution::default(), targets: Vec::new()
            },
            iced::Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Dyson sphere program calculator")
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        match _message {
            Message::ItemSelected(id) => {
                self.index = id;
                self.targets.push((id, 60.0));
                let mut ret = self.steps.clone();
                ret = ret.set_proliferator(ProliferatorEffect::Proliferate(3));
                ret = ret.add_target(id, 60.0);
                self.steps = ret;
            },
            Message::RecipeConfigUpdated((id, config)) => {
                let mut ret = self.steps.clone();
                ret = ret.set_recipe_config(id, config);
                self.steps = ret;
            },
            Message::RecipeRateUpdated((id, rate)) => {
                let mut ret = self.steps.clone();
                ret = ret.set_recipe_rate(id, rate);
                self.steps = ret;
            },
            Message::RecipeBuildingCountUpdated((id, count)) => {
                let mut ret = self.steps.clone();
                ret = ret.set_recipe_building_count(id, count);
                self.steps = ret;
            },
            Message::RecipeItemIgnored((id, ignore)) => {
                let mut ret = self.steps.clone();
                ret = if ignore {
                    ret.ignore_item(id)
                } else {
                    ret.unignore_item(id)
                };
                self.steps = ret;
            },
            Message::TargetRemoved(id) => {
                let mut ret = self.steps.clone();
                ret = ret.remove_target(id);
                self.steps = ret;
            },
            _ => {}
        }
        println!("Power usage: {:.2} MW", self.steps.get_power_usage());
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        // let recipe = &data::RECIPES[self.index..self.index+3];
        let row = if self.index != 0 {
            self.steps.to_ui().visualize().into()
        } else {
            Row::new().into()
        };
        column(Vec::new())
            .push(row)
            .push(ui::item_selector())
            .push(ui::building_selector())
            .spacing(10)
            .into()
    }
}
