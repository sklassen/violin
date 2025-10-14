use wasm_bindgen::prelude::*;
use serde::Serialize;

#[derive(Serialize, Clone, Copy, PartialEq, Debug)]
pub enum PnfDirection {
    Up,
    Down,
}

#[derive(Serialize, Debug)]
pub struct PnfColumn {
    pub direction: PnfDirection,
    pub from: f64,
    pub to: f64,
}

#[wasm_bindgen]
pub fn calculate_pnf_data(
    time_series: Vec<f64>,
    box_size: f64,
    reversal_amount: usize,
) -> Result<JsValue, JsValue> {
    if time_series.len() < 2 || box_size <= 0.0 {
        return Ok(serde_wasm_bindgen::to_value(&Vec::<PnfColumn>::new()).unwrap());
    }

    let mut columns: Vec<PnfColumn> = Vec::new();
    let reversal_distance = box_size * reversal_amount as f64;

    let mut direction: Option<PnfDirection> = None;
    let mut column_start_price = time_series[0];
    let mut last_extreme = time_series[0]; // High for up-column, low for down-column

    for &price in time_series.iter().skip(1) {
        if let Some(dir) = direction {
            match dir {
                PnfDirection::Up => {
                    if price > last_extreme {
                        last_extreme = price;
                    } else if price <= last_extreme - reversal_distance {
                        columns.push(PnfColumn { direction: PnfDirection::Up, from: column_start_price, to: last_extreme });
                        direction = Some(PnfDirection::Down);
                        column_start_price = last_extreme;
                        last_extreme = price;
                    }
                }
                PnfDirection::Down => {
                    if price < last_extreme {
                        last_extreme = price;
                    } else if price >= last_extreme + reversal_distance {
                        columns.push(PnfColumn { direction: PnfDirection::Down, from: column_start_price, to: last_extreme });
                        direction = Some(PnfDirection::Up);
                        column_start_price = last_extreme;
                        last_extreme = price;
                    }
                }
            }
        } else {
            // Establish initial direction
            if price >= column_start_price + box_size {
                direction = Some(PnfDirection::Up);
                last_extreme = price;
            } else if price <= column_start_price - box_size {
                direction = Some(PnfDirection::Down);
                last_extreme = price;
            }
        }
    }

    // Add the last, uncommitted column
    if let Some(dir) = direction {
        columns.push(PnfColumn {
            direction: dir,
            from: column_start_price,
            to: last_extreme,
        });
    }

    Ok(serde_wasm_bindgen::to_value(&columns).unwrap())
}
