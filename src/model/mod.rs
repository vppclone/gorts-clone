#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Scoreboard {
    description: Option<String>,
    subtitle: Option<String>,
    p1name: Option<String>,
    p1country: Option<String>,
    p1score: Option<u32>,
    p1team: Option<String>,
    p2name: Option<String>,
    p2country: Option<String>,
    p2score: Option<u32>,
    p2team: Option<String>,
}
