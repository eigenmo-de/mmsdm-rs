struct LocalPrice {
    settlementdate: chrono::NaiveDateTime,
    duid: String,
    local_price_adjustment:	f64,
    locally_constrained: i32,
}
