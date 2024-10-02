use crate::{LocalizedImageFlip, LocalizedText, SwitchLanguageButton, UiAssets};
use bevy::prelude::*;

pub fn container(builder: &mut ChildBuilder, ui_assets: &UiAssets) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                align_content: AlignContent::Stretch,
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        })
        .with_children(|builder| {
            banner(builder, &ui_assets);
            body(builder, &ui_assets);
        });
}

pub fn banner(builder: &mut ChildBuilder, ui_assets: &UiAssets) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                align_content: AlignContent::Stretch,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::axes(Val::Px(20.), Val::Px(10.)),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            logo(builder, &ui_assets);
            builder
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        align_content: AlignContent::Stretch,
                        justify_content: JustifyContent::FlexEnd,
                        column_gap: Val::Px(30.),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    language_switcher(builder, &ui_assets);
                    user(builder, &ui_assets);
                });
        });
}

pub fn logo(builder: &mut ChildBuilder, ui_assets: &UiAssets) {
    builder
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                align_content: AlignContent::Stretch,
                justify_content: JustifyContent::FlexStart,
                column_gap: Val::Px(20.),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(50.),
                    height: Val::Px(50.),
                    ..default()
                },
                image: UiImage::new(ui_assets.images.logo.clone()),
                ..default()
            });
            builder.spawn((
                TextBundle {
                    text: Text::from_section(
                        "bevymail_logo_text",
                        ui_assets.typographies.logo_text.clone(),
                    ),
                    ..default()
                },
                LocalizedText("bevymail_logo_text"),
            ));
        });
}
pub fn language_switcher(builder: &mut ChildBuilder, ui_assets: &UiAssets) {
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
            builder.spawn(TextBundle::from_section(
                "E/ع",
                ui_assets.typographies.user_text.clone(),
            ));
        });
}
pub fn user(builder: &mut ChildBuilder, ui_assets: &UiAssets) {
    builder
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                align_content: AlignContent::Stretch,
                justify_content: JustifyContent::FlexStart,
                column_gap: Val::Px(10.),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(35.),
                    height: Val::Px(35.),
                    ..default()
                },
                image: UiImage::new(ui_assets.images.avatars.bear.clone()),
                ..default()
            });
            builder.spawn(TextBundle::from_section(
                "$USER",
                ui_assets.typographies.user_text.clone(),
            ));
        });
}

pub fn body(builder: &mut ChildBuilder, ui_assets: &UiAssets) {
    builder
        .spawn((
            Name::new("Body"),
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::FlexStart,
                    align_content: AlignContent::Stretch,
                    justify_content: JustifyContent::FlexStart,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|builder| {
            left_navigation(builder, &ui_assets);
            content(builder, &ui_assets);
        });
}

pub fn left_navigation(builder: &mut ChildBuilder, ui_assets: &UiAssets) {
    builder
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                align_content: AlignContent::Stretch,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::horizontal(Val::Px(20.)),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section("folders", ui_assets.typographies.folder_text.clone()),
                LocalizedText("folders"),
            ));
            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Start,
                        align_content: AlignContent::Stretch,
                        justify_content: JustifyContent::FlexStart,
                        margin: UiRect::horizontal(Val::Px(20.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    folder_item(
                        builder,
                        &ui_assets,
                        "inbox",
                        ui_assets.images.icons.inbox.clone(),
                    );
                    folder_item(
                        builder,
                        &ui_assets,
                        "sent",
                        ui_assets.images.icons.sent.clone(),
                    );
                    folder_item(
                        builder,
                        &ui_assets,
                        "deleted",
                        ui_assets.images.icons.deleted.clone(),
                    );
                });
        });
}

pub fn folder_item(
    builder: &mut ChildBuilder,
    ui_assets: &UiAssets,
    label: &'static str,
    icon: Handle<Image>,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                align_content: AlignContent::Stretch,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(24.),
                        height: Val::Px(24.),
                        ..default()
                    },
                    image: UiImage::new(icon),
                    ..default()
                },
                LocalizedImageFlip,
            ));
            builder.spawn((
                TextBundle::from_section(label, ui_assets.typographies.folder_text.clone()),
                LocalizedText(label),
            ));
        });
}

pub fn content(builder: &mut ChildBuilder, ui_assets: &UiAssets) {
    builder
        .spawn((
            Name::new("Content"),
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                    display: Display::Grid,
                    grid_template_rows: [RepeatedGridTrack::flex(2, 1.)].to_vec(),
                    grid_auto_columns: [GridTrack::flex(1.)].to_vec(),
                    align_items: AlignItems::FlexStart,
                    align_content: AlignContent::Stretch,
                    justify_content: JustifyContent::Stretch,
                    row_gap: Val::Px(5.),
                    padding: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|builder| {
            mail_list(builder, &ui_assets);
            mail_message(builder, &ui_assets);
        });
}

pub fn mail_list(builder: &mut ChildBuilder, ui_assets: &UiAssets) {
    builder
        // mail list container
        .spawn((
            Name::new("Mail list container"),
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::FlexStart,
                    align_content: AlignContent::Stretch,
                    justify_content: JustifyContent::FlexStart,
                    grid_column: GridPlacement::start(1),
                    grid_row: GridPlacement::start(1),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                border_radius: BorderRadius::all(Val::Px(2.)),
                ..default()
            },
        ))
        .with_children(|builder| {
            builder
                // mail list
                .spawn((
                    Name::new("Mail list"),
                    NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            grid_template_rows: [RepeatedGridTrack::flex(4, 1.)].to_vec(),
                            grid_template_columns: [GridTrack::auto(), GridTrack::flex(1.)]
                                .to_vec(),
                            align_items: AlignItems::Center,
                            align_content: AlignContent::FlexStart,
                            justify_items: JustifyItems::Start,
                            justify_content: JustifyContent::Stretch,
                            column_gap: Val::Px(20.),
                            padding: UiRect::all(Val::Px(5.)),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|builder| {
                    for (row, sender) in ["Gitpup", "Amashop", "Legitauth", "Bevymail"]
                        .into_iter()
                        .enumerate()
                    {
                        builder.spawn(TextBundle {
                            style: Style {
                                grid_column: GridPlacement::start(1),
                                grid_row: GridPlacement::start(row as i16 + 1),
                                ..default()
                            },
                            text: Text::from_section(
                                sender,
                                ui_assets.typographies.mail_subject_text.clone(),
                            ),
                            ..default()
                        });
                    }
                    for (row, subject) in [
                        "Your PR #513 has been merged",
                        "Order receipt #5164134",
                        "Here is your OTP",
                        "Welcome to Bevymail!",
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        builder.spawn(TextBundle {
                            style: Style {
                                grid_column: GridPlacement::start(2),
                                grid_row: GridPlacement::start(row as i16 + 1),
                                ..default()
                            },
                            text: Text::from_section(
                                subject,
                                ui_assets.typographies.mail_subject_text.clone(),
                            ),
                            ..default()
                        });
                    }
                });
        });
}

pub fn mail_message(builder: &mut ChildBuilder, ui_assets: &UiAssets) {
    let message = "Welcome to Bevymail, $USER!

Enjoy the fastest and easiest email experience ever!
Check out the user guide for the latest tips and tricks.

Best regards,
Bevymail team";

    builder
        .spawn((
            Name::new("Mail message"),
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::FlexStart,
                    align_content: AlignContent::Stretch,
                    justify_content: JustifyContent::FlexStart,
                    grid_column: GridPlacement::start(1),
                    grid_row: GridPlacement::start(2),
                    padding: UiRect::all(Val::Px(5.)),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                border_radius: BorderRadius::all(Val::Px(2.)),
                ..default()
            },
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                message,
                ui_assets.typographies.folder_text.clone(),
            ));
        });
}
