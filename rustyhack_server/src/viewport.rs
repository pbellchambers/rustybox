use crate::background_map::tiles::{Tile, TilePosition};
use crate::background_map::BackgroundMap;
use crate::ecs::components::{Character, EntityColour, IsPlayer, Position, VisibleState};
use console_engine::{pixel, ConsoleEngine};
use legion::{IntoQuery, World};

pub struct Viewport {
    pub width: u32,
    pub height: u32,
    pub target_fps: u32,
}

impl Viewport {
    pub fn new(width: u32, height: u32, target_fps: u32) -> Viewport {
        Viewport {
            width,
            height,
            target_fps,
        }
    }

    pub fn draw_viewport_contents(
        &self,
        console: &mut ConsoleEngine,
        world: &World,
        background_map: &BackgroundMap,
    ) {
        let viewable_map_coords: TilePosition = calculate_viewable_map_coords(&self, &world);
        draw_viewable_map(console, &background_map, &viewable_map_coords, &self);
        draw_viewport_frame(console, &self);
        draw_viewable_entities(console, &world, &self);
        console.draw();
    }
}

fn draw_viewable_map(
    console: &mut ConsoleEngine,
    world_map: &BackgroundMap,
    viewable_map_topleft: &TilePosition,
    viewport: &Viewport,
) {
    let mut viewport_print_y_loc: i32 = 0;
    while viewport_print_y_loc < viewport.height as i32 {
        let mut viewport_print_x_loc: i32 = 0;
        while viewport_print_x_loc < viewport.width as i32 {
            let current_map_print_loc = TilePosition {
                x: viewable_map_topleft.x + viewport_print_x_loc,
                y: viewable_map_topleft.y + viewport_print_y_loc,
            };
            if (current_map_print_loc.x >= 0)
                && (current_map_print_loc.y >= 0)
                && (current_map_print_loc.x
                    < world_map
                        .data()
                        .get(current_map_print_loc.y as usize)
                        .unwrap_or(&vec![Tile::EmptySpace])
                        .len() as i32)
                && (current_map_print_loc.y < world_map.data().len() as i32)
            {
                console.print(
                    viewport_print_x_loc,
                    viewport_print_y_loc,
                    &world_map
                        .data()
                        .get(current_map_print_loc.y as usize)
                        .unwrap_or(&vec![Tile::EmptySpace])
                        .get(current_map_print_loc.x as usize)
                        .unwrap_or(&Tile::EmptySpace)
                        .character()
                        .to_string(),
                );
            } else {
                console.print(viewport_print_x_loc, viewport_print_y_loc, " ");
            }
            viewport_print_x_loc += 1;
        }
        viewport_print_y_loc += 1;
    }
}

fn calculate_viewable_map_coords(viewport: &Viewport, world: &World) -> TilePosition {
    let x_view_distance = viewport.width / 2;
    let y_view_distance = viewport.height / 2;
    let mut viewable_map_coords: TilePosition = TilePosition { x: 0, y: 0 };

    let mut query = <(&Position, &IsPlayer)>::query();

    for (position, is_player) in query.iter(world) {
        if is_player.is_player {
            viewable_map_coords = TilePosition {
                x: position.x - x_view_distance as i32,
                y: position.y - y_view_distance as i32,
            };
            break;
        } else {
            // do nothing
        }
    }
    viewable_map_coords
}

fn draw_viewable_entities(console: &mut ConsoleEngine, world: &World, viewport: &Viewport) {
    let mut query = <(
        &Position,
        &Character,
        &EntityColour,
        &VisibleState,
        &IsPlayer,
    )>::query();

    for (position, character, entity_colour, visible_state, is_player) in query.iter(world) {
        if is_player.is_player {
            console.set_pxl(
                (viewport.width / 2) as i32,
                (viewport.height / 2) as i32,
                pixel::pxl_fg(character.icon, entity_colour.colour),
            )
        } else if visible_state.visible {
            console.set_pxl(
                position.x,
                position.y,
                pixel::pxl_fg(character.icon, entity_colour.colour),
            )
        }
    }
}

fn draw_viewport_frame(console: &mut ConsoleEngine, viewport: &Viewport) {
    console.rect(
        0,
        0,
        (viewport.width - 1) as i32,
        (viewport.height - 1) as i32,
        pixel::pxl('.'),
    );
}