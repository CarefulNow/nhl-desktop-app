use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use time::{OffsetDateTime, serde::iso8601};

#[derive(Deserialize)]
struct NhlSchedule {
    #[serde(rename = "gameWeek")]
    game_week: Vec<NhlGameDay>,
}

#[derive(Deserialize)]
struct NhlGameDay {
    date: String,
    #[serde(rename = "dayAbbrev")]
    day_of_week_abbrev: String,
    #[serde(rename = "numberOfGames")]
    number_of_games: i64,
    games: Vec<NhlGame>,
}

#[derive(Deserialize)]
struct NhlGame {
    #[serde(rename = "homeTeam")]
    home_team: NhlTeam,
    #[serde(rename = "awayTeam")]
    away_team: NhlTeam,
    #[serde(rename = "startTimeUTC", with = "iso8601")]
    start_time_utc: OffsetDateTime,
    #[serde(rename = "gameState")]
    game_state: String,
    #[serde(rename = "gameScheduleState")]
    game_schedule_state: String,
    #[serde(rename = "gameType")]
    game_type: GameType,
    #[serde(rename = "periodDescriptor")]
    period_descriptor: NhlPeriodDescriptor,
}

#[derive(Deserialize)]
struct NhlPeriodDescriptor {
    number: i64,
    #[serde(rename = "periodType")]
    period_type: String,
}

#[derive(Deserialize)]
struct NhlTeam {
    abbrev: String,
    #[serde(rename = "darkLogo")]
    dark_logo: String,
    logo: String,
    score: Option<i64>,
}



#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum GameType {
    PreSeason = 1,
    RegularSeason = 2,
    PostSeason = 3,
}

#[tauri::command]
pub fn get_schedule() {
    let response = reqwest::blocking::get("https://api-web.nhle.com/v1/schedule/2024-11-24")
        .expect("Could not retrieve NHL standings")
        .json::<NhlSchedule>()
        .expect("Could not retrieve NHL standings");

    for day in response.game_week {
        println!("{} {} {}", day.date, day.day_of_week_abbrev, day.number_of_games);
        for game in day.games {
            println!("{} {} {} - {} {} Period:{}",
                    game.game_state,
                    game.home_team.abbrev,
                    game.home_team.score.unwrap_or(0),
                    game.away_team.score.unwrap_or(0),
                    game.away_team.abbrev,
                    game.period_descriptor.number
            )
        }
    }
}
