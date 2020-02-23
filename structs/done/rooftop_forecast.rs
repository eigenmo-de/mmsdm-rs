struct RooftopForecast {
    version_datetime: chrono::NaiveDateTime,
    regionid: String, 
    interval_datetime: chrono::NaiveDateTime,
    powermean: f64, 
    powerpoe50: f64, 
    powerpoelow: f64, 
    powerpoehigh: f64, 
    lastchanged: chrono::NaiveDateTime,	
} 