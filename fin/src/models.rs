#[derive(Debug)]
pub struct UserData {
    pub id: i64,
    pub username: String,
}

#[derive(Debug)]
pub struct TickerData {
    pub id: i64,
    pub symbol: String,
    pub fk_exchange: String,
    pub fee: f32,
    pub kind: String,
}

#[derive(Debug)]
pub struct PortGoalData {
    pub id: i64,
    pub stock_per: f32,
    pub deviation: f32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct TickerGoalData {
    pub fk_port_g_id: i64,
    pub fk_tic_id: i64,
    pub goal_per: f32,
    pub ord: i32,
}

#[derive(Debug)]
pub struct TickerActualData {
    pub id: i64,
    pub fk_user_id: i64,
    pub fk_port_g_id: i64,
    pub fk_tic_id: i64,
    pub actual_shares: f64,
}
