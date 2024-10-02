mod ui;

use bevy::{prelude::*, utils::HashMap};

fn main() {
    let localization_database = HashMap::from([
        (("en", "bevymail_logo_text"), "Bevy mail!"),
        (("ar", "bevymail_logo_text"), "بريد بَڤِي!"),
        (("en", "folders"), "Folders"),
        (("ar", "folders"), "مجلدات"),
        (("en", "inbox"), "Inbox"),
        (("ar", "inbox"), "وارد"),
        (("en", "sent"), "Sent"),
        (("ar", "sent"), "صادر"),
        (("en", "deleted"), "Deleted"),
        (("ar", "deleted"), "محذوف"),
    ]);

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(UiAssets::default())
        .insert_resource(LocalizationDatabase(localization_database))
        .insert_resource(CurrentLanguage("en"))
        .add_systems(
            Startup,
            (
                setup,
                spawn_layout.after(setup),
                add_layout_localization.after(spawn_layout),
            ),
        )
        .add_systems(
            Update,
            (
                switch_language_button_system,
                change_language_system.run_if(resource_changed::<CurrentLanguage>),
            ),
        )
        .run();
}

#[derive(Resource, Default)]
struct UiAssets {
    font: Handle<Font>,
    typographies: Typographies,
    images: Images,
}

#[derive(Default)]
struct Typographies {
    logo_text: TextStyle,
    user_text: TextStyle,
    folder_text: TextStyle,
    mail_subject_text: TextStyle,
}

#[derive(Default)]
struct Images {
    logo: Handle<Image>,
    avatars: Avatars,
    icons: Icons,
}

#[derive(Default)]
struct Avatars {
    bear: Handle<Image>,
}

#[derive(Default)]
struct Icons {
    inbox: Handle<Image>,
    sent: Handle<Image>,
    deleted: Handle<Image>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut ui_assets: ResMut<UiAssets>) {
    commands.spawn(Camera2dBundle::default());
    ui_assets.font = asset_server.load("fonts/NotoKufiArabic-Regular.ttf");
    ui_assets.typographies.logo_text = TextStyle {
        font: ui_assets.font.clone(),
        color: Color::BLACK,
        font_size: 36.,
    };
    ui_assets.typographies.user_text = TextStyle {
        font: ui_assets.font.clone(),
        color: Color::BLACK,
        font_size: 24.,
    };
    ui_assets.typographies.folder_text = TextStyle {
        font: ui_assets.font.clone(),
        color: Color::BLACK,
        font_size: 16.,
    };
    ui_assets.typographies.mail_subject_text = TextStyle {
        font: ui_assets.font.clone(),
        color: Color::BLACK,
        font_size: 16.,
    };
    ui_assets.images.logo = asset_server.load("images/logo.png");
    ui_assets.images.avatars.bear = asset_server.load("images/avatars/bear.png");
    ui_assets.images.icons.inbox = asset_server.load("images/icons/inbox.png");
    ui_assets.images.icons.sent = asset_server.load("images/icons/sent.png");
    ui_assets.images.icons.deleted = asset_server.load("images/icons/deleted.png");
}

#[derive(Resource)]
struct LocalizationDatabase(HashMap<(&'static str, &'static str), &'static str>);

#[derive(Resource)]
struct CurrentLanguage(&'static str);

#[derive(Component)]
struct LocalizedText(&'static str);

#[derive(Component)]
enum LocalizedFlexRowDirection {
    Forward,
    Backward,
}

#[derive(Component)]
enum LocalizedAlignItems {
    Forward,
    Backward,
}

#[derive(Component)]
enum LocalizedJustifyText {
    Start,
    End,
}

#[derive(Component)]
struct LocalizedImageFlip;

#[derive(Component)]
struct SwitchLanguageButton;

fn change_language_system(
    current_language: Res<CurrentLanguage>,
    localization_database: Res<LocalizationDatabase>,
    mut text_query: Query<
        (
            Option<&LocalizedText>,
            Option<&LocalizedJustifyText>,
            &mut Text,
        ),
        Or<(With<LocalizedText>, With<LocalizedJustifyText>)>,
    >,
    mut image_query: Query<&mut UiImage, With<LocalizedImageFlip>>,
    mut style_query: Query<(
        Option<&LocalizedFlexRowDirection>,
        Option<&LocalizedAlignItems>,
        &mut Style,
    )>,
) {
    let is_left_to_right = match current_language.0 {
        "ar" => false,
        "en" => true,
        _ => panic!("Unknown language"),
    };

    for (localized_text, localized_justify_text, mut text) in &mut text_query {
        if let Some(localized_text) = localized_text {
            if let Some(new_text) = localization_database
                .0
                .get(&(current_language.0, localized_text.0))
            {
                text.sections[0].value = new_text.to_string();
            }
        }

        if let Some(localized_justify_text) = localized_justify_text {
            text.justify = match localized_justify_text {
                LocalizedJustifyText::Start => {
                    if is_left_to_right {
                        JustifyText::Left
                    } else {
                        JustifyText::Right
                    }
                }
                LocalizedJustifyText::End => {
                    if !is_left_to_right {
                        JustifyText::Left
                    } else {
                        JustifyText::Right
                    }
                }
            }
        }
    }

    for mut ui_image in &mut image_query {
        ui_image.flip_x = !is_left_to_right;
    }

    for (localized_flex_row_direction, localized_align_items, mut style) in &mut style_query {
        if let Some(localized_flex_row_direction) = localized_flex_row_direction {
            style.flex_direction = match localized_flex_row_direction {
                LocalizedFlexRowDirection::Forward => {
                    if is_left_to_right {
                        FlexDirection::Row
                    } else {
                        FlexDirection::RowReverse
                    }
                }
                LocalizedFlexRowDirection::Backward => {
                    if !is_left_to_right {
                        FlexDirection::Row
                    } else {
                        FlexDirection::RowReverse
                    }
                }
            }
        }

        if let Some(localized_align_items) = localized_align_items {
            style.align_items = match localized_align_items {
                LocalizedAlignItems::Forward => {
                    if is_left_to_right {
                        AlignItems::Start
                    } else {
                        AlignItems::End
                    }
                }
                LocalizedAlignItems::Backward => {
                    if !is_left_to_right {
                        AlignItems::Start
                    } else {
                        AlignItems::End
                    }
                }
            }
        }
    }
}

fn switch_language_button_system(
    mut current_language: ResMut<CurrentLanguage>,
    mut q: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (With<SwitchLanguageButton>, Changed<Interaction>),
    >,
) {
    for (interaction, mut background_color, mut border_color) in &mut q {
        match interaction {
            Interaction::Pressed => {
                if current_language.0 == "en" {
                    current_language.0 = "ar"
                } else {
                    current_language.0 = "en"
                }
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::linear_rgb(0.6, 0.6, 0.6));
                *border_color = BorderColor(Color::linear_rgb(0., 0., 0.));
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::linear_rgb(0.8, 0.8, 0.8));
                *border_color = BorderColor(Color::linear_rgb(0., 0., 0.));
            }
        }
    }
}

fn spawn_layout(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            ui::container(builder, &ui_assets);
        });
}

fn add_layout_localization(
    mut commands: Commands,
    style_query: Query<(Entity, &Style)>,
    text_query: Query<(Entity, &Text)>,
) {
    for (entity, style) in &style_query {
        match (style.display, style.flex_direction, style.align_items) {
            (Display::Flex, FlexDirection::Row, _) => {
                commands
                    .entity(entity)
                    .insert(LocalizedFlexRowDirection::Forward);
            }
            (Display::Flex, FlexDirection::RowReverse, _) => {
                commands
                    .entity(entity)
                    .insert(LocalizedFlexRowDirection::Backward);
            }
            (Display::Flex, _, AlignItems::Start) => {
                commands.entity(entity).insert(LocalizedAlignItems::Forward);
            }
            (Display::Flex, _, AlignItems::End) => {
                commands
                    .entity(entity)
                    .insert(LocalizedAlignItems::Backward);
            }
            _ => (),
        }
    }

    for (entity, text) in &text_query {
        match text.justify {
            JustifyText::Left => {
                commands.entity(entity).insert(LocalizedJustifyText::Start);
            }
            JustifyText::Right => {
                commands.entity(entity).insert(LocalizedJustifyText::End);
            }
            _ => (),
        }
    }
}
