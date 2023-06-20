use bevy::prelude::*;
use bevy::ecs::component::Component;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct Health
{
    health: i32,
}

#[derive(Clone, PartialEq)]
pub enum Season
{
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Clone, PartialEq)]
pub struct Plant
{
    name: &'static str,
    sow: Season,
    harvest: Season
}

#[derive(Component)]
pub struct PlantBed
{
    pub plant: Option<Plant>,
    pub row: i32,
    pub column: i32
}

const KUMARA: Plant = Plant { name: "Kumara", sow: Season::Autumn, harvest: Season::Spring };
const MANUKA: Plant = Plant { name: "Manuka", sow: Season::Autumn, harvest: Season::Spring };
const PUHA: Plant = Plant { name: "Puha", sow: Season::Autumn, harvest: Season::Spring };

#[derive(Component)]
pub struct SeedBag
{
    pub seeds: Vec<Plant>,
    pub selected: usize,
}

const ROWS: i32 = 5;
const COLUMNS: i32 = 4;

pub(crate) fn spawn_beds(mut commands: Commands,
                         asset_server: Res<AssetServer>)
{
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SeedBag { seeds: vec![ KUMARA.clone(), MANUKA.clone(), PUHA.clone()], selected:0});

    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("shelf.png"),
            transform: Transform::from_translation(Vec3::new(0., 110., 0.)),
            ..default() });

    let bed_sprite =  asset_server.load("bed.png");

    for x in 0..ROWS
    {
        for y in 0..COLUMNS
        {
            commands.spawn((
                SpriteBundle {
                    texture: bed_sprite.clone(),
                    transform: Transform::from_translation(Vec3::new(32. * (x as f32 - 2.), 32. * (y as f32 - 3.), 0.)),
                    ..default() },
                PlantBed{ plant: None, row: x, column: y }));
        }
    }
}

// will hover over beds and interact with them
pub(crate) fn bed_interact(mut bed_query: Query<(&mut PlantBed, &mut Transform)>,
                           mouse_pos: Query<&Window, With<PrimaryWindow>>,
                           buttons: Res<Input<MouseButton>>,
                           camera_q: Query<(&Camera, &GlobalTransform)>,
                           mut seed_bag_q: Query<&mut SeedBag>)
{
    // https://bevy-cheatbook.github.io/cookbook/cursor2world.html
    let (camera, camera_transform) = camera_q.single();
    let mut seed_bag: &mut SeedBag = &mut seed_bag_q.single_mut();

    let m_pos = match mouse_pos.single().cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        Some(a) => a,
        None => return,
    };

    for mut bed in bed_query.iter_mut()
    {
        let mut bed_trans: &mut Transform = &mut bed.1;
        let bed_pos = Vec2::new(bed_trans.translation.x, bed_trans.translation.y);
        let rect = Rect::from_center_size(bed_pos, Vec2::ONE * 32.);

        bed_trans.scale = Vec3::new(1., 1., 1.);

        if !rect.contains(m_pos) { continue; }

        bed_trans.scale = Vec3::new(1.1, 1.1, 1.1);

        if buttons.just_pressed(MouseButton::Left)
        {
            // planting example -> put in func?
            let mut plant_bed: &mut PlantBed = &mut bed.0;
            if plant_bed.plant == None
            {

            }
        }
    }
}

pub(crate) fn show_seeds(seed_bag: Query<&SeedBag>,
                        )
{

}