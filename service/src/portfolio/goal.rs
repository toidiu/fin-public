use crate::std_ext::*;
use crate::ticker::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PortfolioGoal {
    pub id: i64,
    pub name: String,
    description: Option<String>,
    pub tickers_goal: HashMap<TickerId, GoalTicker>,
}

impl PortfolioGoal {
    pub fn new(
        id: i64,
        name: String,
        description: Option<String>,
        tickers_goal: &HashMap<TickerId, GoalTicker>,
        tickers_map: &HashMap<TickerId, Ticker>,
        actual_stock_percent: &f32,
    ) -> PortfolioGoal {
        // TODO break into separate fn
        // calculate the goal_percent based on actual_stock_percent
        let tickers_goal = tickers_goal
            .clone()
            .into_iter()
            .map(|(tic_id, mut gt)| {
                let is_stock = tickers_map
                    .get(&tic_id)
                    .expect(&format!(
                        "{} ticker map should contain tic_id: {:?}",
                        line!(),
                        tic_id
                    ))
                    .is_stock();

                gt.goal_percent = match is_stock {
                    true => gt.goal_percent * (actual_stock_percent / 100.0),
                    false => {
                        gt.goal_percent
                            * ((100.0 - actual_stock_percent) / 100.0)
                    }
                };
                StdExt::round_two_digits(&mut gt.goal_percent);

                (tic_id, gt)
            })
            .collect();
        PortfolioGoal {
            id: id,
            name: name,
            description: description,
            tickers_goal: tickers_goal,
        }
    }

    pub(super) fn get_ticker_g(&self, id: &TickerId) -> &GoalTicker {
        self.tickers_goal.get(id).expect(&format!(
            "{} add ticker to db: {:?}",
            line!(),
            id
        ))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct GoalTicker {
    pub id: i64,
    pub port_goal_id: i64,
    pub ticker_id: i64,
    pub goal_percent: f32,
    order: i32,
}

impl GoalTicker {
    pub fn new(
        id: i64,
        port_goal_id: i64,
        ticker_id: i64,
        goal_percent: f32,
        order: i32,
    ) -> Self {
        GoalTicker {
            id: id,
            port_goal_id: port_goal_id,
            ticker_id: ticker_id,
            goal_percent: goal_percent,
            order: order,
        }
    }

    pub(super) fn get_order(&self) -> i32 {
        self.order
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerGoalDetailed {
    pub port_goal_id: i64,
    pub ticker_id: i64,
    pub goal_percent: f32,
    pub order: i32,
    pub symbol: TickerSymbol,
}
