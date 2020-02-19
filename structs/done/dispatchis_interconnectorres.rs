struct Interonnectorres { 
    settlementdate: chrono::NaiveDateTime,
    runno: i64,
    interconnectorid: String, 
    dispatchinterval:i64,
    intervention: i64,
    meteredmwflow: f64,
    mwflow: i64,
    mwlosses: i64, 
    marginalvalue: i64,
    violationdegree: i64,
    lastchanged: chrono::NaiveDateTime,
    exportlimit: f64,
    importlimit: f64,
    marginalloss: f64, 
    exportgenconid: String, 
    importgenconid: String, 
    fcasexportlimit: f64,
    fcasimportlimit: f64,
    local_price_adjustment_export: i64,
    locally_constrained_export: i64,
    local_price_adjustment_import: i64,
    locally_constrained_import: i64, 
} 				
				
				
				
				
				