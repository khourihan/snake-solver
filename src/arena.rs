use bevy::{prelude::*, utils::HashMap};
use rand::seq::IteratorRandom;
use smallvec::SmallVec;

use crate::{cell::{DrawCell, DrawCellTransform, ForegroundCell}, game::GameState, settings::Settings, snake::Snake};

#[derive(Resource)]
pub struct Arena {
    pub size: UVec2,
    cells: Vec<Cell>,
    contains_food: bool,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Cell {
    None,
    SnakeTail {
        /// The distance from this segment to the head, with the closest tail segment having a
        /// distance of 1.
        distance: usize,
    },
    SnakeHead,
    Food,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct CellPositions {
    pos: IVec2,
    size: IVec2,
}

impl Iterator for CellPositions {
    type Item = UVec2;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos.x += 1;

        if self.pos.x >= self.size.x {
            self.pos.x = 0;
            self.pos.y += 1;
        }

        if self.pos.y >= self.size.y {
            return None;
        }

        Some(self.pos.as_uvec2())
    }
}

pub struct Cells<I: Iterator> {
    pos: IVec2,
    width: i32,
    cells: I
}

impl<I: Iterator> Iterator for Cells<I> {
    type Item = (UVec2, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.pos.x += 1;

        if self.pos.x >= self.width {
            self.pos.x = 0;
            self.pos.y += 1;
        }
        
        Some((self.pos.as_uvec2(), self.cells.next()?))
    }
}

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct Directions: u8 {
        const NONE = 0;
        const UP = 1 << 0;
        const DOWN = 1 << 1;
        const LEFT = 1 << 2;
        const RIGHT = 1 << 3;
        const ALL = Self::UP.bits() | Self::DOWN.bits() | Self::LEFT.bits() | Self::RIGHT.bits();
    }
}

impl Direction {
    pub const UP_OFFSET: IVec2 = IVec2::new(0, 1);
    pub const DOWN_OFFSET: IVec2 = IVec2::new(0, -1);
    pub const LEFT_OFFSET: IVec2 = IVec2::new(-1, 0);
    pub const RIGHT_OFFSET: IVec2 = IVec2::new(1, 0);

    #[inline]
    pub const fn offset(&self) -> IVec2 {
        match self {
            Direction::Up => Self::UP_OFFSET,
            Direction::Down => Self::DOWN_OFFSET,
            Direction::Left => Self::LEFT_OFFSET,
            Direction::Right => Self::RIGHT_OFFSET,
        }
    }

    #[inline]
    pub const fn rotate_clockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    #[inline]
    pub const fn rotate_counterclockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    #[inline]
    pub const fn flip(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Directions {
    #[inline]
    pub fn up(&self) -> bool {
        self.contains(Directions::UP)
    }

    #[inline]
    pub fn down(&self) -> bool {
        self.contains(Directions::DOWN)
    }

    #[inline]
    pub fn left(&self) -> bool {
        self.contains(Directions::LEFT)
    }

    #[inline]
    pub fn right(&self) -> bool {
        self.contains(Directions::RIGHT)
    }
}

impl From<Direction> for Directions {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Directions::UP,
            Direction::Down => Directions::DOWN,
            Direction::Left => Directions::LEFT,
            Direction::Right => Directions::RIGHT,
        }
    }
}

impl Arena {
    pub fn get_cell(&self, pos: IVec2) -> Option<&Cell> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size.x as i32 || pos.y >= self.size.y as i32 {
            return None;
        }

        Some(unsafe { self.cells.get_unchecked((pos.y * self.size.x as i32 + pos.x) as usize) })
    }

    pub fn get_cell_mut(&mut self, pos: IVec2) -> Option<&mut Cell> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size.x as i32 || pos.y >= self.size.y as i32 {
            return None;
        }

        Some(unsafe { self.cells.get_unchecked_mut((pos.y * self.size.x as i32 + pos.x) as usize) })
    }

    pub fn get_cell_unchecked(&self, pos: UVec2) -> &Cell {
        unsafe { self.cells.get_unchecked((pos.y * self.size.x + pos.x) as usize) }
    }

    pub fn get_cell_unchecked_mut(&mut self, pos: UVec2) -> &mut Cell {
        unsafe { self.cells.get_unchecked_mut((pos.y * self.size.x + pos.x) as usize) }
    }

    pub fn positions(&self) -> CellPositions {
        CellPositions {
            pos: IVec2::new(-1, 0),
            size: self.size.as_ivec2(),
        }
    }

    pub fn cells(&self) -> Cells<impl Iterator<Item = &Cell>> {
        Cells {
            pos: IVec2::new(-1, 0),
            width: self.size.x as i32,
            cells: self.cells.iter(),
        }
    }

    pub fn cells_mut(&mut self) -> Cells<impl Iterator<Item = &mut Cell>> {
        Cells {
            pos: IVec2::new(-1, 0),
            width: self.size.x as i32,
            cells: self.cells.iter_mut(),
        }
    }

    /// Compute the [`Directions`] in which the neighbors of the given `pos` matching the given
    /// `cell`.
    pub fn neighbors_matching(&self, pos: UVec2, cell: Cell) -> Directions {
        let pos = pos.as_ivec2();
        let mut dirs = Directions::NONE;

        if let Some(&c) = self.get_cell(pos + Direction::UP_OFFSET) {
            if cell == c {
                dirs |= Directions::UP;
            }
        }

        if let Some(&c) = self.get_cell(pos + Direction::DOWN_OFFSET) {
            if cell == c {
                dirs |= Directions::DOWN;
            }
        }

        if let Some(&c) = self.get_cell(pos + Direction::LEFT_OFFSET) {
            if cell == c {
                dirs |= Directions::LEFT;
            }
        }

        if let Some(&c) = self.get_cell(pos + Direction::RIGHT_OFFSET) {
            if cell == c {
                dirs |= Directions::RIGHT;
            }
        }

        dirs
    }
}

pub fn setup_arena(
    mut commands: Commands,
    settings: Res<Settings>,
) {
    commands.insert_resource(Arena {
        size: settings.arena_size,
        cells: vec![Cell::None; (settings.arena_size.x * settings.arena_size.y) as usize],
        contains_food: false,
    })
}

pub fn update_cell(
    mut commands: Commands,
    arena: Res<Arena>,
    settings: Res<Settings>,
    mut cells: Query<(&mut ForegroundCell, &mut Sprite, &mut DrawCellTransform)>,
    mut positions: Local<HashMap<UVec2, [Entity; 2]>>,
) {
    let get_sizes_offsets = |dirs: Directions| -> ((Vec2, Vec2), Option<(Vec2, Vec2)>) {
        let mut sizes_offsets = SmallVec::<[(Vec2, Vec2); 2]>::new();

        if dirs.up() {
            sizes_offsets.push((Vec2::new(0.5, 0.75), Vec2::new(0.0, 0.125)));
        }

        if dirs.down() {
            sizes_offsets.push((Vec2::new(0.5, 0.75), Vec2::new(0.0, -0.125)));
        }

        if dirs.left() {
            sizes_offsets.push((Vec2::new(0.75, 0.5), Vec2::new(-0.125, 0.0)));
        }

        if dirs.right() {
            sizes_offsets.push((Vec2::new(0.75, 0.5), Vec2::new(0.125, 0.0)));
        }

        (sizes_offsets.pop().unwrap(), sizes_offsets.pop())
    };

    for (pos, ty) in arena.cells() {
        if let Some(entities) = positions.get_mut(&pos) {
            let (mut contents, mut sprite, mut transform) = cells.get_mut(entities[0]).unwrap();

            if *ty == contents.contents {
                continue;
            }

            contents.contents = *ty;

            match ty {
                Cell::None => {
                    for &mut entity in entities {
                        if let Some(mut e) = commands.get_entity(entity) { e.despawn() }
                    }
                    positions.remove(&pos);
                },
                Cell::SnakeTail { distance } => {
                    let dirs = arena.neighbors_matching(pos, Cell::SnakeTail { distance: distance + 1 })
                        | arena.neighbors_matching(pos, if *distance == 1 { Cell::SnakeHead } else { Cell::SnakeTail { distance: distance - 1 } });

                    let sizes_offsets = get_sizes_offsets(dirs);

                    sprite.color = settings.colors.snake_tail;
                    transform.size = sizes_offsets.0.0;
                    transform.offset = sizes_offsets.0.1;

                    if let Some(size_offset) = sizes_offsets.1 {
                        if let Ok((_contents, mut sprite, mut transform)) = cells.get_mut(entities[1]) {
                            sprite.color = settings.colors.snake_tail;
                            transform.size = size_offset.0;
                            transform.offset = size_offset.1;
                        } else {
                            entities[1] = commands.spawn((
                                DrawCell { pos },
                                Sprite::from_color(settings.colors.snake_tail, Vec2::ONE),
                                ForegroundCell { contents: Cell::None },
                                DrawCellTransform {
                                    size: size_offset.0,
                                    offset: size_offset.1,
                                }
                            )).id();
                        }
                    } else {
                        if let Some(mut e) = commands.get_entity(entities[1]) { e.despawn() };
                        entities[1] = Entity::PLACEHOLDER;
                    }
                },
                Cell::SnakeHead => {
                    let dirs = arena.neighbors_matching(pos, Cell::SnakeTail { distance: 1 });

                    let sizes_offsets = get_sizes_offsets(dirs);

                    sprite.color = settings.colors.snake_head;
                    transform.size = sizes_offsets.0.0;
                    transform.offset = sizes_offsets.0.1;

                    if let Some(size_offset) = sizes_offsets.1 {
                        if let Ok((_contents, mut sprite, mut transform)) = cells.get_mut(entities[1]) {
                            sprite.color = settings.colors.snake_tail;
                            transform.size = size_offset.0;
                            transform.offset = size_offset.1;
                        } else {
                            entities[1] = commands.spawn((
                                DrawCell { pos },
                                Sprite::from_color(settings.colors.snake_head, Vec2::ONE),
                                ForegroundCell { contents: Cell::None },
                                DrawCellTransform {
                                    size: size_offset.0,
                                    offset: size_offset.1,
                                },
                            )).id();
                        }
                    } else {
                        if let Some(mut e) = commands.get_entity(entities[1]) { e.despawn() };
                        entities[1] = Entity::PLACEHOLDER;
                    }
                },
                Cell::Food => {
                    sprite.color = settings.colors.food;
                    transform.size = Vec2::splat(0.5);
                    transform.offset = Vec2::ZERO;

                    if let Some(mut e) = commands.get_entity(entities[1]) { e.despawn() };
                    entities[1] = Entity::PLACEHOLDER;
                },
            }
        } else {
            if *ty == Cell::None {
                continue;
            }

            match ty {
                Cell::SnakeTail { distance } => {
                    let dirs = arena.neighbors_matching(pos, Cell::SnakeTail { distance: distance + 1 })
                        | arena.neighbors_matching(pos, if *distance == 1 { Cell::SnakeHead } else { Cell::SnakeTail { distance: distance - 1 } });

                    let sizes_offsets = get_sizes_offsets(dirs);

                    positions.insert(
                        pos,
                        [
                            commands.spawn((
                                DrawCell { pos },
                                Sprite::from_color(settings.colors.snake_tail, Vec2::ONE),
                                ForegroundCell { contents: Cell::SnakeTail { distance: *distance } },
                                DrawCellTransform {
                                    size: sizes_offsets.0.0,
                                    offset: sizes_offsets.0.1,
                                }
                            )).id(),
                            if let Some(size_offset) = sizes_offsets.1 {
                                commands.spawn((
                                    DrawCell { pos },
                                    Sprite::from_color(settings.colors.snake_tail, Vec2::ONE),
                                    ForegroundCell { contents: Cell::None },
                                    DrawCellTransform {
                                        size: size_offset.0,
                                        offset: size_offset.1,
                                    }
                                )).id()
                            } else {
                                Entity::PLACEHOLDER
                            }
                        ]
                    );
                },
                Cell::SnakeHead => {
                    let dirs = arena.neighbors_matching(pos, Cell::SnakeTail { distance: 1 });
                    
                    let sizes_offsets = get_sizes_offsets(dirs);

                    positions.insert(
                        pos,
                        [
                            commands.spawn((
                                DrawCell { pos },
                                Sprite::from_color(settings.colors.snake_head, Vec2::ONE),
                                ForegroundCell { contents: Cell::SnakeHead },
                                DrawCellTransform {
                                    size: sizes_offsets.0.0,
                                    offset: sizes_offsets.0.1,
                                }
                            )).id(),
                            if let Some(size_offset) = sizes_offsets.1 {
                                commands.spawn((
                                    DrawCell { pos },
                                    Sprite::from_color(settings.colors.snake_head, Vec2::ONE),
                                    ForegroundCell { contents: Cell::None },
                                    DrawCellTransform {
                                        size: size_offset.0,
                                        offset: size_offset.1,
                                    },
                                )).id()
                            } else {
                                Entity::PLACEHOLDER
                            }
                        ]
                    );
                },
                Cell::Food => {
                    positions.insert(
                        pos,
                        [
                            commands.spawn((
                                DrawCell { pos },
                                Sprite::from_color(settings.colors.food, Vec2::ONE),
                                ForegroundCell { contents: Cell::Food },
                                DrawCellTransform {
                                    size: Vec2::splat(0.5),
                                    offset: Vec2::ZERO,
                                }
                            )).id(),
                            Entity::PLACEHOLDER,
                        ]
                    );
                },
                _ => unreachable!()
            }
        }
    }
}

pub fn update_snake_position(
    mut arena: ResMut<Arena>,
    mut snake: ResMut<Snake>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let mut next_head: Option<IVec2> = None;
    let mut remove: Option<UVec2> = None;

    for pos in arena.positions() {
        let cell = arena.get_cell_unchecked_mut(pos);

        match cell {
            Cell::SnakeTail { distance } => {
                if *distance >= snake.length {
                    remove = Some(pos);
                }

                *distance += 1;
            },
            Cell::SnakeHead => {
                *cell = Cell::SnakeTail { distance: 1 };

                next_head = Some(pos.as_ivec2() + snake.direction.offset());
            },
            Cell::None | Cell::Food => (),
        }
    }

    if let Some(next_head) = next_head {
        if let Some(next) = arena.get_cell_mut(next_head) {
            match next {
                Cell::None => {
                    *next = Cell::SnakeHead;
                },
                Cell::SnakeTail { .. } => {
                    for (_pos, cell) in arena.cells_mut() {
                        if let Cell::SnakeTail { distance } = cell {
                            *distance -= 1;

                            if *distance == 0 {
                                *cell = Cell::SnakeHead;
                            }
                        }
                    }

                    game_state.set(GameState::Stopped);
                    return;
                },
                Cell::Food => {
                    snake.length += 1;
                    *next = Cell::SnakeHead;
                    arena.contains_food = false;
                },
                Cell::SnakeHead => unreachable!(),
            }
        } else {
            for (_pos, cell) in arena.cells_mut() {
                if let Cell::SnakeTail { distance } = cell {
                    *distance -= 1;

                    if *distance == 0 {
                        *cell = Cell::SnakeHead;
                    }
                }
            }

            game_state.set(GameState::Stopped);
            return;
        }
    }

    if let Some(pos) = remove {
        let cell = arena.get_cell_unchecked_mut(pos);
        *cell = Cell::None;
    }
}

pub fn check_win(
    arena: Res<Arena>,
    snake: Res<Snake>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if snake.length >= arena.cells.len() - 1 {
        game_state.set(GameState::Stopped);
    }
}

pub fn spawn_food(
    mut arena: ResMut<Arena>,
) {
    if arena.contains_food {
        return;
    }

    arena.contains_food = true;

    let mut rng = rand::thread_rng();
    
    let cell = arena.cells_mut()
        .filter(|(_, cell)| **cell == Cell::None)
        .choose(&mut rng)
        .unwrap().1;

    *cell = Cell::Food;
}

pub fn respawn_food(
    mut arena: ResMut<Arena>,
) {
    arena.contains_food = false;
}

impl std::fmt::Debug for Arena {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let cell = self.get_cell_unchecked(UVec2::new(x, y));
                match cell {
                    Cell::None => write!(f, ".")?,
                    Cell::SnakeTail { distance } => {
                        if *distance < 10 {
                            write!(f, "{}", distance)?
                        } else {
                            write!(f, "+")?
                        }
                    },
                    Cell::SnakeHead => write!(f, "@")?,
                    Cell::Food => write!(f, "*")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
