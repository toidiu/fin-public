use crate::api;
use crate::backend;
use crate::data;
use crate::portfolio::Portfolio;
use crate::portfolio::{Action, TickerActual, *};
use crate::std_ext::*;
use crate::ticker::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct BuyNext {
    pub init_state: Portfolio,
    pub evolved_actual: HashMap<TickerId, TickerActual>,
    pub actions: Vec<Action>,
    pub buy_value: f32,
    pub action_summary: HashMap<TickerId, Action>,
}

impl BuyNext {
    pub fn new(port: Portfolio) -> Self {
        let actual_tickers = port.get_actual_tickers();
        BuyNext {
            init_state: port,
            evolved_actual: actual_tickers,
            actions: Vec::new(),
            buy_value: 0.0,
            action_summary: HashMap::new(),
        }
    }

    // fn do_buy_next<T: data::PortfolioBackend>(
    //     db: &mut T,
    //     actual: &HashMap<TickerId, TickerActual>,
    //     buy_amount: f32,
    //     port_g_id: &i64,
    // ) -> Portfolio {
    //     let mut buy_next =
    //         Self::get_buy_next(db, actual, buy_amount, port_g_id);

    //     // get init values to store history
    //     let init = buy_next.init_state.get_actual_tickers();
    //     let init = init.values().collect();

    //     // get evolved values
    //     let evolved = buy_next.evolved_actual.values().collect();

    //     // update
    //     let inserted_tic_actual = db.update_actual(&init, &evolved);

    //     // get actions
    //     let actions = buy_next.actions.clone();
    //     let mut port = buy_next.init_state;
    //     for a in actions {
    //         port = port.evolve(&a);
    //     }
    //     port
    // }

    pub fn get_buy_next<T: backend::PortfolioBackend>(
        db: &T,
        actual: &HashMap<TickerId, TickerActual>,
        buy_amount: f32,
        port_g_id: &i64,
    ) -> Self {
        let goal_tickers = db.get_tic_goal(port_g_id);
        let port_goal = db
            .get_port_goal(port_g_id)
            .unwrap()
            .to_port_goal(goal_tickers);

        let keys = actual.keys().map(|x| x.0).collect();
        let tickers_map: HashMap<TickerId, Ticker> = db.get_tickers(&keys);

        let mut port = Portfolio::new(db, &actual, &tickers_map, &port_goal);
        let mut buy_next_resp = BuyNext::new(port);

        // todo do based on buy_value and the desired value
        while (buy_next_resp.buy_value < buy_amount) {
            if let None = Self::get_next_action(
                &mut buy_next_resp,
                buy_amount,
                db,
                &port_goal,
                &tickers_map,
            ) {
                break;
            }
        }
        buy_next_resp
    }

    fn get_next_action<T: backend::PortfolioBackend>(
        buy_next_resp: &mut BuyNext,
        buy_amount: f32,
        db: &T,
        port_goal: &PortfolioGoal,
        tickers_map: &HashMap<TickerId, Ticker>,
    ) -> Option<Action> {
        // get port from action actual
        let port = Portfolio::new(
            db,
            &buy_next_resp.evolved_actual,
            &tickers_map,
            port_goal,
        );

        // get action
        let action = port.get_buy_next_action();

        // buying more would put us above the buy value
        if (buy_next_resp.buy_value + action.get_price() > buy_amount) {
            return None;
        }

        // get evolved state
        let evolved_port = port.evolve(&action);

        // update buy_value
        buy_next_resp.buy_value += action.get_price();
        StdExt::round_two_digits(&mut buy_next_resp.buy_value);
        // update action
        buy_next_resp.actions.push(action.clone());

        // update final state
        buy_next_resp.evolved_actual = evolved_port.get_actual_tickers();

        Some(action)
    }
}
