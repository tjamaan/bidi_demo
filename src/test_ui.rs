use crate::{LocalizedText, SwitchLanguageButton, UiAssets};
use bevy::prelude::*;

pub fn container(builder: &mut ChildBuilder, ui_assets: &UiAssets) {
    let background_color = BackgroundColor(Color::hsl(0., 0., 0.));

    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                flex_wrap: FlexWrap::NoWrap,
                align_items: AlignItems::FlexStart,
                row_gap: Val::Px(50.),
                padding: UiRect {
                    left: Val::Px(10.),
                    right: Val::Px(20.),
                    top: Val::Px(10.),
                    bottom: Val::Px(20.),
                },
                ..default()
            },
            background_color,
            ..default()
        })
        .with_children(|builder| {
            banner(builder, &ui_assets);
            blue_box(builder, &ui_assets);
        });
}

pub fn banner(builder: &mut ChildBuilder, ui_assets: &UiAssets) {
    let background_color = BackgroundColor(Color::hsl(0., 0., 0.2));

    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::NoWrap,
                align_items: AlignItems::FlexStart,
                align_content: AlignContent::Stretch,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color,
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                Text::new("Bidi layout demo"),
                ui_assets.typographies.test_banner.clone(),
            ));

            builder
                .spawn((
                    ButtonBundle {
                        background_color: BackgroundColor(Color::linear_rgb(0.8, 0.8, 0.8)),
                        border_color: BorderColor(Color::linear_rgb(0.6, 0.6, 0.6)),
                        ..default()
                    },
                    SwitchLanguageButton,
                ))
                .with_children(|builder| {
                    builder.spawn((Text::new("E/ع"), ui_assets.typographies.test_banner.clone()));
                });
        });
}

pub fn blue_box(builder: &mut ChildBuilder, ui_assets: &UiAssets) {
    let background_color = BackgroundColor(Color::hsl(240., 0.33, 0.30));
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Grid,
                grid_template_rows: [RepeatedGridTrack::flex(5, 1.)].to_vec(),
                grid_template_columns: [RepeatedGridTrack::flex(4, 1.)].to_vec(),
                align_items: AlignItems::Start,
                align_content: AlignContent::Stretch,
                justify_items: JustifyItems::Start,
                justify_content: JustifyContent::Stretch,
                row_gap: Val::Px(5.),
                column_gap: Val::Px(10.),
                padding: UiRect {
                    left: Val::Px(10.),
                    right: Val::Px(20.),
                    top: Val::Px(10.),
                    bottom: Val::Px(20.),
                },
                ..default()
            },
            background_color,
            ..default()
        })
        .with_children(|builder| {
            green_box(builder, &ui_assets);
            red_box(builder, &ui_assets);
        });
}

pub fn green_box(builder: &mut ChildBuilder, ui_assets: &UiAssets) {
    let background_color = BackgroundColor(Color::hsl(120., 0.33, 0.30));
    let border_color = BorderColor(Color::hsl(120., 0.33, 0.60));
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                flex_wrap: FlexWrap::Wrap,
                align_content: AlignContent::FlexStart,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                grid_column: GridPlacement::start_span(1, 3),
                grid_row: GridPlacement::start_span(1, 4),
                border: UiRect {
                    left: Val::Px(2.),
                    right: Val::Px(5.),
                    top: Val::Px(2.),
                    bottom: Val::Px(5.),
                },
                ..default()
            },
            background_color,
            border_color,
            ..default()
        })
        .with_children(|builder| {
            for i in 1..=10 {
                pink_box(builder, &ui_assets, i);
            }
        });
}

pub fn pink_box(builder: &mut ChildBuilder, ui_assets: &UiAssets, index: usize) {
    let saturation = 0.5 + index as f32 / 20.;
    let background_color = BackgroundColor(Color::hsl(300., saturation, 0.30));
    let border_color = BorderColor(Color::hsl(300., saturation, 0.60));
    let width_extra = if index % 2 == 0 {
        index as f32 * 3.0
    } else {
        index as f32 * -5.0
    };
    let height_extra = if index % 2 == 0 {
        index as f32 * -1.0
    } else {
        index as f32 * 2.0
    };
    builder.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(100.0 + width_extra),
                height: Val::Px(50.0 + height_extra),
                border: UiRect {
                    left: Val::Px(2.),
                    right: Val::Px(5.),
                    top: Val::Px(2.),
                    bottom: Val::Px(5.),
                },
                ..default()
            },
            background_color,
            border_color,
            ..default()
        },
        //Text(format!("{}", index)),
    ));
}

pub fn red_box(builder: &mut ChildBuilder, ui_assets: &UiAssets) {
    let background_color = BackgroundColor(Color::hsl(0., 0.33, 0.30));
    let border_color = BorderColor(Color::hsl(0., 0.33, 0.60));
    builder.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            grid_column: GridPlacement::start(4),
            grid_row: GridPlacement::start_span(3, 3),
            border: UiRect {
                left: Val::Px(2.),
                right: Val::Px(5.),
                top: Val::Px(2.),
                bottom: Val::Px(5.),
            },
            ..default()
        },
        background_color,
        border_color,
        ..default()
    });
}
