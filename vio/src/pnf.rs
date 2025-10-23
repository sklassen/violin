use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum PnfDirection {
    Up,
    Down,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PnfColumn {
    pub direction: PnfDirection,
    pub from: f64,
    pub to: f64,
    pub start_time: usize,
    pub end_time: usize,
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
    let mut column_start_time = 0;
    let mut last_extreme = time_series[0]; // High for up-column, low for down-column
    let mut last_extreme_time = 0;

    for (i, &price) in time_series.iter().enumerate().skip(1) {
        if let Some(dir) = direction {
            match dir {
                PnfDirection::Up => {
                    if price > last_extreme {
                        last_extreme = price;
                        last_extreme_time = i;
                    } else if price <= last_extreme - reversal_distance {
                        columns.push(PnfColumn {
                            direction: PnfDirection::Up,
                            from: column_start_price,
                            to: last_extreme,
                            start_time: column_start_time,
                            end_time: last_extreme_time,
                        });
                        direction = Some(PnfDirection::Down);
                        column_start_price = last_extreme;
                        column_start_time = last_extreme_time;
                        last_extreme = price;
                        last_extreme_time = i;
                    }
                }
                PnfDirection::Down => {
                    if price < last_extreme {
                        last_extreme = price;
                        last_extreme_time = i;
                    } else if price >= last_extreme + reversal_distance {
                        columns.push(PnfColumn {
                            direction: PnfDirection::Down,
                            from: column_start_price,
                            to: last_extreme,
                            start_time: column_start_time,
                            end_time: last_extreme_time,
                        });
                        direction = Some(PnfDirection::Up);
                        column_start_price = last_extreme;
                        column_start_time = last_extreme_time;
                        last_extreme = price;
                        last_extreme_time = i;
                    }
                }
            }
        } else {
            // Establish initial direction
            if price >= column_start_price + box_size {
                direction = Some(PnfDirection::Up);
                last_extreme = price;
                last_extreme_time = i;
            } else if price <= column_start_price - box_size {
                direction = Some(PnfDirection::Down);
                last_extreme = price;
                last_extreme_time = i;
            }
        }
    }

    // Add the last, uncommitted column
    if let Some(dir) = direction {
        columns.push(PnfColumn {
            direction: dir,
            from: column_start_price,
            to: last_extreme,
            start_time: column_start_time,
            end_time: last_extreme_time,
        });
    }

    Ok(serde_wasm_bindgen::to_value(&columns).unwrap())
}

#[wasm_bindgen]
pub fn calculate_return_pnf(
    pnf_data: JsValue,
    box_size: f64,
    reversal_amount: f64,
    transaction_cost: Option<f64>,
    mode: i64,
) -> Result<JsValue, JsValue> {
    let mut input_columns: Vec<PnfColumn> = serde_wasm_bindgen::from_value(pnf_data)
        .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))?;

    let cost = transaction_cost.unwrap_or(3.0);
    let mut cumulative_return = 0.0;

    for col in &mut input_columns {
        let num_boxes = ((col.to - col.from).abs() / box_size).floor();
        let mut bar_return = num_boxes - reversal_amount * 2.0 - cost;

        bar_return = match mode {
            1 => {
                if col.direction == PnfDirection::Up {
                    bar_return
                } else {
                    0.0
                }
            }
            -1 => {
                if col.direction == PnfDirection::Down {
                    bar_return
                } else {
                    0.0
                }
            }
            _ => bar_return,
        };

        let previous_return = cumulative_return;
        cumulative_return += bar_return;

        col.from = previous_return;
        col.to = cumulative_return;
    }

    Ok(serde_wasm_bindgen::to_value(&input_columns).unwrap())
}

fn calculate_total_return(
    time_series: &Vec<f64>,
    box_size: f64,
    reversal_amount: usize,
    transaction_cost: f64,
    mode: i64,
) -> f64 {
    if time_series.len() < 2 || box_size <= 0.0 {
        return 0.0;
    }

    let mut columns: Vec<PnfColumn> = Vec::new();
    let reversal_distance = box_size * reversal_amount as f64;

    let mut direction: Option<PnfDirection> = None;
    let mut column_start_price = time_series[0];
    let mut last_extreme = time_series[0];

    for &price in time_series.iter().skip(1) {
        if let Some(dir) = direction {
            match dir {
                PnfDirection::Up => {
                    if price > last_extreme {
                        last_extreme = price;
                    } else if price <= last_extreme - reversal_distance {
                        columns.push(PnfColumn {
                            direction: PnfDirection::Up,
                            from: column_start_price,
                            to: last_extreme,
                            start_time: 0,
                            end_time: 0,
                        });
                        direction = Some(PnfDirection::Down);
                        column_start_price = last_extreme;
                        last_extreme = price;
                    }
                }
                PnfDirection::Down => {
                    if price < last_extreme {
                        last_extreme = price;
                    } else if price >= last_extreme + reversal_distance {
                        columns.push(PnfColumn {
                            direction: PnfDirection::Down,
                            from: column_start_price,
                            to: last_extreme,
                            start_time: 0,
                            end_time: 0,
                        });
                        direction = Some(PnfDirection::Up);
                        column_start_price = last_extreme;
                        last_extreme = price;
                    }
                }
            }
        } else {
            if price >= column_start_price + box_size {
                direction = Some(PnfDirection::Up);
                last_extreme = price;
            } else if price <= column_start_price - box_size {
                direction = Some(PnfDirection::Down);
                last_extreme = price;
            }
        }
    }

    if let Some(dir) = direction {
        columns.push(PnfColumn {
            direction: dir,
            from: column_start_price,
            to: last_extreme,
            start_time: 0,
            end_time: 0,
        });
    }

    let mut total_return = 0.0;
    for col in &columns {
        let num_boxes = ((col.to - col.from).abs() / box_size).floor();
        let mut bar_return = num_boxes - reversal_amount as f64 * 2.0 - transaction_cost;

        bar_return = match mode {
            1 => if col.direction == PnfDirection::Up { bar_return } else { 0.0 },
            -1 => if col.direction == PnfDirection::Down { bar_return } else { 0.0 },
            _ => bar_return,
        };
        total_return += bar_return;
    }

    total_return
}

#[wasm_bindgen]
pub fn optimize_box_size(
    time_series: Vec<f64>,
    reversal_amount: usize,
    transaction_cost: f64,
    direction: i64,
) -> Result<f64, JsValue> {
    let min_box_size = 0.1;
    let max_box_size = 5.0;
    let steps = 50;

    let mut best_box_size = min_box_size;
    let mut max_return = -1.0 / 0.0;

    for i in 0..=steps {
        let box_size = min_box_size + (max_box_size - min_box_size) * (i as f64 / steps as f64);
        let total_return = calculate_total_return(&time_series, box_size, reversal_amount, transaction_cost, direction);

        if total_return > max_return {
            max_return = total_return;
            best_box_size = box_size;
        }
    }

    Ok(best_box_size)
}
