use iced::{alignment::{Horizontal, Vertical}, Alignment, Length, Padding, Pixels};

use crate::abc::ui::style::*;

const fn padding_1(padding: f32) -> Padding {
    Padding {
        top: padding,
        right: padding,
        bottom: padding,
        left: padding
    }
}

const fn padding_2(top: f32, left: f32) -> Padding {
    Padding {
        top,
        right: left,
        bottom: top,
        left
    }
}

const fn padding_4(top: f32, right: f32, bottom: f32, left: f32) -> Padding {
    Padding {
        top,
        right,
        bottom,
        left
    }
}

// Row Styles
// 1. Row containing target item and its production rate
// The style also applies to the row containing the building count
pub static ITEM_COUNT_STYLE: RowStyle = RowStyle {
    spacing: Some(Pixels(5.0)),
    padding: Some(padding_2(5.0, 0.0)),
    height: None,
    align_items: Some(Alignment::Center)
};

// 2. RecipeStep row style
pub static RECIPE_STEP_STYLE: RowStyle = RowStyle {
    spacing: Some(Pixels(20.0)),
    padding: Some(padding_1(5.0)),
    height: None,
    align_items: Some(Alignment::Start)
};

// 3. ButtonSelector row style
pub static BUTTON_SELECTOR_STYLE: RowStyle = RowStyle {
    spacing: Some(Pixels(2.0)),
    padding: None,
    height: None,
    align_items: None
};

pub static PROLIFERATOR_CONFIG_STYLE: RowStyle = RowStyle {
    spacing: Some(Pixels(10.0)),
    padding: None,
    height: None,
    align_items: Some(Alignment::Start)
};

// 4. Left or right side of the recipe row
pub static RECIPE_SIDE_STYLE: RowStyle = RowStyle {
    spacing: Some(Pixels(3.0)),
    padding: None,
    height: None,
    align_items: Some(Alignment::Center)
};

// 5. Recipe item and its amount
pub static RECIPE_ITEM_STYLE: RowStyle = RowStyle {
    spacing: Some(Pixels(2.0)),
    padding: Some(padding_1(1.0)),
    height: None,
    align_items: Some(Alignment::End)
};

// 6. A recipe row
pub static RECIPE_ROW_STYLE: RowStyle = RowStyle {
    spacing: Some(Pixels(3.0)),
    padding: None,
    height: None,
    align_items: Some(Alignment::Center)
};

// Text Styles
// 1. Recipe item count
pub static ITEM_COUNT_TEXT_STYLE: TextStyle = TextStyle {
    size: Some(Pixels(10.0)),
    width: None,
    height: None,
    horizontal_alignment: None,
    vertical_alignment: None
};

// 2. Recipe arrow style
pub static ARROW_TEXT_STYLE: TextStyle = TextStyle {
    size: Some(Pixels(10.0)),
    width: Some(Length::Fill),
    height: None,
    horizontal_alignment: Some(Horizontal::Center),
    vertical_alignment: None
};

// 3. Default text style
pub static SELECTOR_TEXT_STYLE: TextStyle = TextStyle {
    size: Some(Pixels(14.0)),
    width: Some(Length::Fixed(32.0)),
    height: Some(Length::Fixed(32.0)),
    horizontal_alignment: Some(Horizontal::Center),
    vertical_alignment: Some(Vertical::Center)
};

// 4. Delete text style
pub static DELETE_TEXT_STYLE: TextStyle = TextStyle {
    size: Some(Pixels(14.0)),
    width: Some(Length::Fixed(19.0)),
    height: Some(Length::Fixed(19.0)),
    horizontal_alignment: Some(Horizontal::Center),
    vertical_alignment: Some(Vertical::Center)
};

// 4. Item and building count text input style
pub static ITEM_RATE_INPUT_STYLE: TextInputStyle = TextInputStyle {
    size: Some(Pixels(14.0)),
    width: Some(Length::Fixed(50.0))
};

// Image Styles
// 1. Item icon style
pub static ICON_STYLE: ImageStyle = ImageStyle {
    width: Some(Length::Fixed(32.0)),
    height: Some(Length::Fixed(32.0))
};

// Container Styles
pub static CHECKBOX_PADDING: ContainerStyle = ContainerStyle {
    width: None,
    height: None,
    padding: Some(padding_2(16.0, 0.0))
};