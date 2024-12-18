mod mail_ui;
mod test_ui;

use bevy::{prelude::*, utils::HashMap};

fn main() {
    let localization_database = HashMap::from([
        (("en", "bevymail_logo_text"), "Bevy mail!"),
        (("ar", "bevymail_logo_text"), "بريد بَڤِي!"),
        (("en", "good_morning"), "Good morning "),
        (("ar", "good_morning"), "صباح الخير "),
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
        .add_systems(Startup, (setup, spawn_layout.after(setup)))
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

    test_banner: TextStyle,
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
    commands.spawn(Camera2d);
    ui_assets.font = asset_server.load("fonts/NotoKufiArabic-Regular.ttf");
    ui_assets.typographies.logo_text = TextStyle {
        font: ui_assets.font.clone(),
        color: Color::linear_rgb(0., 0., 0.),
        font_size: 36.,
        font_smoothing: bevy::text::FontSmoothing::AntiAliased,
    };
    ui_assets.typographies.user_text = TextStyle {
        font: ui_assets.font.clone(),
        color: Color::BLACK,
        font_size: 24.,
        font_smoothing: bevy::text::FontSmoothing::AntiAliased,
    };
    ui_assets.typographies.folder_text = TextStyle {
        font: ui_assets.font.clone(),
        color: Color::BLACK,
        font_size: 16.,
        font_smoothing: bevy::text::FontSmoothing::AntiAliased,
    };
    ui_assets.typographies.mail_subject_text = TextStyle {
        font: ui_assets.font.clone(),
        color: Color::BLACK,
        font_size: 16.,
        font_smoothing: bevy::text::FontSmoothing::AntiAliased,
    };
    ui_assets.typographies.test_banner = TextStyle {
        font: ui_assets.font.clone(),
        color: Color::WHITE,
        font_size: 36.,
        font_smoothing: bevy::text::FontSmoothing::AntiAliased,
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
struct LocalizedImageFlip;

#[derive(Component)]
struct SwitchLanguageButton;

fn change_language_system(
    current_language: Res<CurrentLanguage>,
    localization_database: Res<LocalizationDatabase>,
    mut text_query: Query<
        (&LocalizedText, Option<&mut Text>, Option<&mut TextSpan>),
        (Or<(With<Text>, With<TextSpan>)>,),
    >,
    mut image_query: Query<&mut UiImage, With<LocalizedImageFlip>>,
    mut direction_query: Query<&mut Style>,
) {
    let is_left_to_right = match current_language.0 {
        "ar" => false,
        "en" => true,
        _ => panic!("Unknown language"),
    };

    for (localized_text, text, text_span) in &mut text_query {
        if let Some(new_text) = localization_database
            .0
            .get(&(current_language.0, localized_text.0))
        {
            text.map(|mut t| t.0 = new_text.to_string());
            text_span.map(|mut t| t.0 = new_text.to_string());
        }
    }

    for mut ui_image in &mut image_query {
        ui_image.flip_x = !is_left_to_right;
    }

    for mut style in &mut direction_query {
        style.direction = if is_left_to_right {
            Direction::Ltr
        } else {
            Direction::Rtl
        };
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
            //mail_ui::container(builder, &ui_assets);
            test_ui::container(builder, &ui_assets);
        });
}
