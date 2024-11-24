use serde::{Deserialize, Serialize};

enum GameType {
    PreSeason,
    RegularSeason,
    PostSeason,
}

impl GameType {
    fn value(&self) -> i64 {
        match *self {
            GameType::PreSeason => 1,
            GameType::RegularSeason => 2,
            GameType::PostSeason => 3,
        }
    }
}

struct Schedule {
    game_week: Vec<GameDay>,
}

struct GameDay {
    date: String,
    day_of_week: String,
    number_of_games: i64,
    games: Vec<Game>,
}

struct Game<'a> {
    home_team: Team,
    away_team: Team,
    start_time_utc: datetime::ZonedDateTime<'a>,
    game_state: String,
    game_schedule_state: String,
    game_type: GameType,
}

struct Team {
    abbrev: String,
    dark_logo: String,
    logo: String,
    score: i64,
}