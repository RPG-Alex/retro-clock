// sauce: https://www.youtube.com/watch?v=jhI69fodWUY&ab_channel=timClicks

use bevy::prelude::*;
use chrono::{NaiveDateTime,Timelike};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, clock_face)
    .run()
}

fn setup(
    mut commands: Commands,
    mut gizmo_conf: ResMut<GizmoConfig>
) {
    commands.spawn(Camera2dBundle::default());

    // set the line thickness
    gizmo_conf.line_width = 10.0;
    commands.spawn(TextBundle::from_section("PLACEHOLDER", TextStyle { 
        font: default() , font_size: 14.0, color: Color::WHITE })
            .with_style(Style {
                position_type: PositionType::Absolute,
                //bottom: Val::Percent(50.0),
                right: Val::Percent(47.5),
                //left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                ..default()
            }))
            ;

}

fn clock_face(
    mut gizmos: Gizmos,
    mut query: Query<&mut Text>,
    mut window: Query<&mut Window>,
) {
    let now = chrono::Local::now();
    let hour =  now.hour() as f32;
    let minute = now.minute() as f32;
    let second = now.second() as f32;

    let hour_angle = ((360.0/ 24.0) * hour).to_radians();
    let minute_angle = ((360.0/60.0) * minute).to_radians();
    let second_angle = ((360.0/60.0) * second).to_radians();

    let disp_time = now.time().format("%H:%M:%S");
    let disp_date = now.date_naive().format("%Y-%B-%d");

    //set Seconds location
    let sec_loc = window.single_mut();
    let (top_left_x, top_left_y) = (sec_loc.width()/-3.5, sec_loc.height()/3.5);
    let (top_right_x, top_right_y) = (sec_loc.width()/3.5, sec_loc.height()/3.5);
    let (bottom_left_x, bottom_left_y) = (sec_loc.width()/-3.5, sec_loc.height()/-3.5);
    let (bottom_right_x, bottom_right_y) = (sec_loc.width()/3.5, sec_loc.height()/-3.5);
    
    // hours
    gizmos.arc_2d(Vec2::ZERO, hour_angle / 2.0, hour_angle, 100., Color::BISQUE).segments(360*3);
    // minutes
    gizmos.arc_2d(Vec2::ZERO, minute_angle / 2.0, minute_angle, 120., Color::TEAL).segments(360*3);
    // seconds
    gizmos.arc_2d(Vec2::ZERO, second_angle / 2.0, second_angle, 140., Color::ORANGE).segments(360*3);
    
    
	// gizmos.arc_2d(Vec2::ZERO, second_angle / 2.0, second_angle, 200., Color::SILVER).segments(360*3);
    
    // gizmos.arc_2d(Vec2::ZERO, second_angle / 2.0, second_angle, 240., Color::YELLOW).segments(360*3);
	// gizmos.arc_2d(Vec2::ZERO, second_angle / 2.0, second_angle, 260., Color::CRIMSON).segments(360*3);
    // gizmos.arc_2d(Vec2::ZERO, second_angle / 2.0, second_angle, 280., Color::AQUAMARINE).segments(360*3);

    gizmos.arc_2d(Vec2::new(top_left_x, top_left_y), second_angle / 2.0, second_angle, 60., Color::LIME_GREEN).segments(360*3);
    gizmos.arc_2d(Vec2::new(top_right_x, top_right_y), second_angle / 2.0, second_angle, 60., Color::BLUE).segments(360*3);
    gizmos.arc_2d(Vec2::new(bottom_left_x,bottom_left_y), second_angle / 2.0, second_angle, 60., Color::VIOLET).segments(360*3);
    gizmos.arc_2d(Vec2::new(bottom_right_x,bottom_right_y), second_angle / 2.0, second_angle, 60., Color::CYAN).segments(360*3);
    for mut text in &mut query {
        text.sections[0].value = format!("{disp_time}");
        //text.sections[0].value = format!("{disp_date}");
        
    }

}