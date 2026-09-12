use macroquad::rand::gen_range;

pub const GRID_SIZE: i32 = 20;
pub const CELL_SIZE: f32 = 20.0;
pub const MOVE_INTERVAL: f64 = 0.12;

pub type Cell = (i32, i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn offset(self) -> Cell {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }

    fn opposes(self, other: Self) -> bool {
        let (horizontal, vertical) = self.offset();
        let (other_horizontal, other_vertical) = other.offset();
        horizontal + other_horizontal == 0 && vertical + other_vertical == 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Ready,
    Playing,
    Paused,
    GameOver,
    Won,
}

pub struct Game {
    pub snake: Vec<Cell>,
    pub previous_snake: Vec<Cell>,
    pub apple: Cell,
    pub direction: Direction,
    pub score: u32,
    pub best_score: u32,
    pub phase: Phase,
    turn_pending: bool,
    last_move: f64,
}

impl Game {
    pub fn new(now: f64) -> Self {
        let snake = vec![(10, 10), (9, 10), (8, 10)];
        let mut game = Self {
            previous_snake: snake.clone(),
            snake,
            apple: (0, 0),
            direction: Direction::Right,
            score: 0,
            best_score: 0,
            phase: Phase::Ready,
            turn_pending: false,
            last_move: now,
        };
        game.spawn_apple();
        game
    }

    pub fn restart(&mut self, now: f64) {
        let best_score = self.best_score;
        *self = Self::new(now);
        self.best_score = best_score;
        self.phase = Phase::Playing;
    }

    pub fn toggle_pause(&mut self, now: f64) {
        self.phase = match self.phase {
            Phase::Playing => Phase::Paused,
            Phase::Paused => Phase::Playing,
            _ => return,
        };
        self.previous_snake.clone_from(&self.snake);
        self.last_move = now;
    }

    pub fn turn(&mut self, direction: Direction) {
        if self.phase == Phase::Playing
            && !self.turn_pending
            && direction != self.direction
            && !direction.opposes(self.direction)
        {
            self.direction = direction;
            self.turn_pending = true;
        }
    }

    pub fn animation_progress(&self, now: f64) -> f32 {
        if self.phase != Phase::Playing {
            return 1.0;
        }
        let progress = ((now - self.last_move) / 0.075).clamp(0.0, 1.0) as f32;
        progress * progress * (3.0 - 2.0 * progress)
    }

    pub fn update(&mut self, now: f64) {
        if self.phase != Phase::Playing || now - self.last_move < MOVE_INTERVAL {
            return;
        }
        self.last_move = now;
        self.turn_pending = false;

        let (head_x, head_y) = self.snake[0];
        let (horizontal, vertical) = self.direction.offset();
        let next_head = (head_x + horizontal, head_y + vertical);
        let eating = next_head == self.apple;
        let occupied_length = self.snake.len() - usize::from(!eating);
        let hit_wall =
            !(0..GRID_SIZE).contains(&next_head.0) || !(0..GRID_SIZE).contains(&next_head.1);
        let hit_body = self.snake[..occupied_length].contains(&next_head);

        if hit_wall || hit_body {
            self.phase = Phase::GameOver;
            return;
        }

        self.previous_snake.clone_from(&self.snake);
        self.snake.insert(0, next_head);
        if eating {
            self.score += 1;
            self.best_score = self.best_score.max(self.score);
            if self.snake.len() == (GRID_SIZE * GRID_SIZE) as usize {
                self.phase = Phase::Won;
            } else {
                self.spawn_apple();
            }
        } else {
            self.snake.pop();
        }
    }

    fn spawn_apple(&mut self) {
        loop {
            let apple = (gen_range(0, 20), gen_range(0, 20));
            if !self.snake.contains(&apple) {
                self.apple = apple;
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn playing() -> Game {
        let mut game = Game::new(0.0);
        game.restart(0.0);
        game.apple = (0, 0);
        game
    }

    #[test]
    fn waits_for_start_and_movement_interval() {
        let mut game = Game::new(0.0);
        game.update(10.0);
        assert_eq!(game.snake[0], (10, 10));
        game.restart(10.0);
        game.update(10.1);
        assert_eq!(game.snake[0], (10, 10));
        game.update(10.13);
        assert_eq!(game.snake[0], (11, 10));
    }

    #[test]
    fn rejects_reversals_and_second_turn_in_same_tick() {
        let mut game = playing();
        game.turn(Direction::Left);
        assert_eq!(game.direction, Direction::Right);
        game.turn(Direction::Up);
        game.turn(Direction::Left);
        game.update(0.13);
        assert_eq!(game.snake[0], (10, 9));
        game.turn(Direction::Left);
        game.update(0.26);
        assert_eq!(game.snake[0], (9, 9));
    }

    #[test]
    fn eating_grows_scores_and_respawns_off_body() {
        let mut game = playing();
        game.apple = (11, 10);
        game.update(0.13);
        assert_eq!(game.snake.len(), 4);
        assert_eq!((game.score, game.best_score), (1, 1));
        assert!(!game.snake.contains(&game.apple));
    }

    #[test]
    fn wall_collision_ends_round_without_mutating_body() {
        let mut game = playing();
        game.snake = vec![(19, 10), (18, 10), (17, 10)];
        game.update(0.13);
        assert_eq!(game.phase, Phase::GameOver);
        assert_eq!(game.snake[0], (19, 10));
    }

    #[test]
    fn self_collision_ends_round() {
        let mut game = playing();
        game.snake = vec![(10, 10), (10, 11), (11, 11), (11, 10), (12, 10)];
        game.update(0.13);
        assert_eq!(game.phase, Phase::GameOver);
    }

    #[test]
    fn can_move_into_vacating_tail() {
        let mut game = playing();
        game.snake = vec![(10, 10), (10, 11), (11, 11), (11, 10)];
        game.update(0.13);
        assert_eq!(game.phase, Phase::Playing);
        assert_eq!(game.snake[0], (11, 10));
    }

    #[test]
    fn pause_freezes_and_resume_resets_timer() {
        let mut game = playing();
        game.toggle_pause(0.05);
        game.update(20.0);
        assert_eq!(game.snake[0], (10, 10));
        game.toggle_pause(20.0);
        game.update(20.05);
        assert_eq!(game.snake[0], (10, 10));
        game.update(20.13);
        assert_eq!(game.snake[0], (11, 10));
    }

    #[test]
    fn animation_eases_between_ticks_without_changing_grid_state() {
        let mut game = playing();
        game.update(0.13);
        assert_eq!(game.previous_snake[0], (10, 10));
        assert_eq!(game.snake[0], (11, 10));
        assert_eq!(game.animation_progress(0.13), 0.0);
        assert!((game.animation_progress(0.1675) - 0.5).abs() < 0.001);
        assert_eq!(game.animation_progress(0.21), 1.0);
        game.toggle_pause(0.22);
        assert_eq!(game.animation_progress(0.22), 1.0);
    }

    #[test]
    fn restart_resets_round_but_preserves_best() {
        let mut game = playing();
        game.score = 5;
        game.best_score = 9;
        game.phase = Phase::GameOver;
        game.turn_pending = true;
        game.restart(7.0);
        assert_eq!(game.phase, Phase::Playing);
        assert_eq!((game.score, game.best_score), (0, 9));
        assert_eq!(game.direction, Direction::Right);
        assert!(!game.turn_pending);
        assert_eq!(game.snake, vec![(10, 10), (9, 10), (8, 10)]);
        assert_eq!(game.previous_snake, game.snake);
        assert!(!game.snake.contains(&game.apple));
    }

    #[test]
    fn full_board_wins_without_spawning_another_apple() {
        let mut game = playing();
        game.snake = (0..GRID_SIZE)
            .flat_map(|row| (0..GRID_SIZE).map(move |column| (column, row)))
            .filter(|&cell| cell != (0, 0))
            .collect();
        game.direction = Direction::Left;
        game.apple = (0, 0);
        game.update(0.13);
        assert_eq!(game.phase, Phase::Won);
        assert_eq!(game.snake.len(), 400);
    }
}
