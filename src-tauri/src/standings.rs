use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct NhlStandings {
    standings: Vec<NhlTeam>,
}

#[derive(Deserialize)]
struct NhlTeam {
    #[serde(rename = "teamName")]
    team_name: NhlTeamName,
    #[serde(rename = "leagueSequence")]
    league_sequence: i64,
    points: i64,
}

#[derive(Deserialize)]
struct NhlTeamName {
    default: String,
}

#[derive(Serialize)]
pub struct Standings {
    standings: Vec<Team>,
}

#[derive(Serialize)]
pub struct Team {
    #[serde(rename = "teamName")]
    team_name: String,
    #[serde(rename = "leaguePosition")]
    league_position: i64,
    points: i64,
}

impl Team {
    fn new(
        team_name: String,
        league_position: i64,
        points: i64,
    ) -> Self {
        Team {
            team_name,
            league_position,
            points,
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
pub fn get_standings() -> Standings {
    let response = reqwest::blocking::get("https://api-web.nhle.com/v1/standings/now")
        .expect("Could not retrieve NHL standings")
        .json::<NhlStandings>()
        .expect("Could not retrieve NHL standings");

    let mut standings: Standings = Standings { standings: vec![] };
    for team in response.standings {
        standings
            .standings
            .push(Team::new(team.team_name.default.clone(), team.league_sequence, team.points));
    }
    standings
}
