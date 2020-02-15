struct Interconnecttion { 
    settlementdate:  chrono::NaiveDateTime,
    runno: i64,
    intervention: i64,
    from_regionid: String, 
    to_regionid: String, 
    dispatchinterval: i64, 
    irlf: f64, 
    mwflow: f64, 
    meteredmwflow: f64, 
    from_region_mw_losses: f64, 
    to_region_mw_losses: f64, 
    lastchanged: chrono::NaiveDateTime,
}