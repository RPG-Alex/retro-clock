// sauce: https://www.youtube.com/watch?v=jhI69fodWUY&ab_channel=timClicks

use bevy::prelude::*;
use chrono::Timelike;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, clock_face)
    .run()
}


#[derive(Component, PartialEq)]
enum TextField {
    Time,
    Date,
    Hour,
    Minute,
    Second,
    TwelveHour,
}

fn setup(
    mut commands: Commands,
    mut gizmo_conf: ResMut<GizmoConfig>
) {
    commands.spawn(Camera2dBundle::default());

    // set the line thickness
    gizmo_conf.line_width = 10.0;
    gizmo_conf.line_perspective = false;
    // Generate Text field for Time
    commands.spawn((TextBundle::from_section("TIME", TextStyle { 
        font: default() , font_size: 14.0, color: Color::WHITE })
            .with_style(Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(47.5),
                top: Val::Percent(50.0),
                ..default() 
            }),
        )).insert(TextField::Time);


    // Generate Text field for Date
    commands.spawn((TextBundle::from_section("DATE", TextStyle { 
        font: default() , font_size: 14.0, color: Color::WHITE })
            .with_style(Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(45.0),
                bottom: Val::Percent(5.0),
                ..default()
            }),
        )).insert(TextField::Date);

        // Generate Text field for Hour
    commands.spawn((TextBundle::from_section("HO", TextStyle { 
        font: default() , font_size: 22.0, color: Color::WHITE })
            .with_style(Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(77.5),
                bottom: Val::Percent(78.5),
                ..default()
            }),
        )).insert(TextField::Hour);

    // Generate Text field for Minute
    commands.spawn((TextBundle::from_section("MN", TextStyle { 
        font: default() , font_size: 22.0, color: Color::WHITE })
            .with_style(Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(21.5),
                bottom: Val::Percent(78.5),
                ..default()
            }),
        )).insert(TextField::Minute);

    // Generate Text field for Date
    commands.spawn((TextBundle::from_section("SC", TextStyle { 
        font: default() , font_size: 22.0, color: Color::WHITE })
            .with_style(Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(77.5),
                bottom: Val::Percent(21.5),
                ..default()
            }),
        )).insert(TextField::Second);

    // Generate Text field for Date
    commands.spawn((TextBundle::from_section("PM", TextStyle { 
        font: default() , font_size: 22.0, color: Color::WHITE })
            .with_style(Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(21.5),
                bottom: Val::Percent(21.5),
                ..default()
            }),
        )).insert(TextField::TwelveHour);

}

// Passes in the gizmos, queries text, and queries window size
fn clock_face(
    mut gizmos: Gizmos,
    mut time: Query<(&mut Text, &TextField)>,
    mut window: Query<&mut Window>,
) {
    // Get time and setup time
    let now = chrono::Local::now();
    let hour =  now.hour() as f32;
    let minute = now.minute() as f32;
    let second = now.second() as f32;
    // Create our circle arc
    let hour_angle = ((360.0/ 24.0) * hour).to_radians();
    let minute_angle = ((360.0/60.0) * minute).to_radians();
    let second_angle = ((360.0/60.0) * second).to_radians();

    // Share time in readable manner
    let disp_time = now.time().format("%H:%M:%S");
    let disp_date = now.date_naive().format("%Y-%B-%d");

    // Get the display info for relative locations
    let sec_loc = window.single_mut();
    //set Seconds location for 4 corners
    let (top_left_x, top_left_y) = (sec_loc.width()/-3.5, sec_loc.height()/3.5);
    let (top_right_x, top_right_y) = (sec_loc.width()/3.5, sec_loc.height()/3.5);
    let (bottom_left_x, bottom_left_y) = (sec_loc.width()/-3.5, sec_loc.height()/-3.5);
    let (bottom_right_x, bottom_right_y) = (sec_loc.width()/3.5, sec_loc.height()/-3.5);
    
    // hours
    gizmos.arc_2d(Vec2::ZERO, hour_angle / 2.0, hour_angle, 100., Color::BISQUE).segments(360*3);
    // minutes
    gizmos.arc_2d(Vec2::ZERO, minute_angle / 2.0, minute_angle, 120., Color::TEAL).segments(360*3);
    // seconds (main output)
    gizmos.arc_2d(Vec2::ZERO, second_angle / 2.0, second_angle, 140., Color::ORANGE).segments(360*3);
    
    // Instantiate the gizmos for the 4 corners second counters
    gizmos.arc_2d(Vec2::new(top_left_x, top_left_y), second_angle / 2.0, second_angle, 60., Color::LIME_GREEN).segments(360*3);
    gizmos.arc_2d(Vec2::new(top_right_x, top_right_y), second_angle / 2.0, second_angle, 60., Color::BLUE).segments(360*3);
    gizmos.arc_2d(Vec2::new(bottom_left_x,bottom_left_y), second_angle / 2.0, second_angle, 60., Color::VIOLET).segments(360*3);
    gizmos.arc_2d(Vec2::new(bottom_right_x,bottom_right_y), second_angle / 2.0, second_angle, 60., Color::CYAN).segments(360*3);

    for (mut text, text_field) in time.iter_mut() {
        match text_field {
            TextField::Hour => {
                text.sections[0].value = format!("{hour}");
            },
            TextField::Minute => {
                text.sections[0].value = format!("{minute}");
            },
            TextField::Second => {
                text.sections[0].value = format!("{second}");
            },
            TextField::TwelveHour => {
                if hour < 12.0 {
                    text.sections[0].value = format!("AM");
                } else {
                    text.sections[0].value = format!("PM");
                }
            },
            TextField::Time => {
                text.sections[0].value = format!("{disp_time}");
            },
            TextField::Date => {
                text.sections[0].value = format!("{disp_date}");
            },
        }
    }
}
