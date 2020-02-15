struct PredispatchLocalPrice { 
    datetime: chrono::NaiveDateTime,
    periodid: String, 
    local_price_adjustment: f64, 
    locally_constrained: i32, 
    lastchanged:  chrono::NaiveDateTime,
}