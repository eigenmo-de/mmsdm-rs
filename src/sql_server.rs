
use std::collections;
use crate::data_model;
use futures_util::{AsyncRead, AsyncWrite};

/// This function is meant to be used in conjunction with the iterator over
/// the data contained within the AemoFile struct
#[cfg(feature = "sql_server")]
pub async fn save_all<'a, S>(file: impl Into<mmsdm_core::MmsFile<'a>>, skip_keys: Option<&std::collections::HashSet<mmsdm_core::FileKey>>, client: &mut tiberius::Client<S>, chunk_size: Option<usize>) -> mmsdm_core::Result<()>
where S: futures_util::AsyncRead + futures_util::AsyncWrite + Unpin + Send,
{
    let mut mms_file = file.into();
    for file_key in mms_file.file_keys() {
        if skip_keys.map(|set| set.contains(file_key)).unwrap_or(false) {
            log::info!("Skippping file key {} as it is in the list of keys to skip", file_key);                            
            continue;                                                                                                      
        }   
        match (
            file_key.data_set_name.as_str(),
        ) {
            "ASOFFER" => {
                #[cfg(feature = "asoffer")]
                {
                    data_model::asoffer::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "asoffer"))]
                {
                    log::warn!("File key {:?} is not handled as the feature asoffer is not activated", file_key);
                }
            }
            "BID" | "BIDS" | "OFFER" => {
                #[cfg(feature = "bids")]
                {
                    data_model::bids::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "bids"))]
                {
                    log::warn!("File key {:?} is not handled as the feature bids is not activated", file_key);
                }
            }
            "BILLING_CONFIG" => {
                #[cfg(feature = "billing_config")]
                {
                    data_model::billing_config::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "billing_config"))]
                {
                    log::warn!("File key {:?} is not handled as the feature billing_config is not activated", file_key);
                }
            }
            "BILLING" => {
                #[cfg(feature = "billing_run")]
                {
                    data_model::billing_run::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("File key {:?} is not handled as the feature billing_run is not activated", file_key);
                }
            }
            "DEMAND" | "FORECAST" | "OPERATIONAL_DEMAND" | "ROOFTOP" => {
                #[cfg(feature = "demand_forecasts")]
                {
                    data_model::demand_forecasts::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "demand_forecasts"))]
                {
                    log::warn!("File key {:?} is not handled as the feature demand_forecasts is not activated", file_key);
                }
            }
            "DISPATCH" | "PRICELOAD" => {
                #[cfg(feature = "dispatch")]
                {
                    data_model::dispatch::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("File key {:?} is not handled as the feature dispatch is not activated", file_key);
                }
            }
            "AP" | "FORCE_MAJEURE" => {
                #[cfg(feature = "force_majeure")]
                {
                    data_model::force_majeure::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "force_majeure"))]
                {
                    log::warn!("File key {:?} is not handled as the feature force_majeure is not activated", file_key);
                }
            }
            "GD_INSTRUCT" => {
                #[cfg(feature = "gd_instruct")]
                {
                    data_model::gd_instruct::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "gd_instruct"))]
                {
                    log::warn!("File key {:?} is not handled as the feature gd_instruct is not activated", file_key);
                }
            }
            "GCRHS" | "GENCONDATA" | "GENCONSET" | "GENCONSETTRK" | "GENERIC_CONSTRAINT" | "GEQDESC" | "GEQRHS" | "SPDCPC" | "SPDICC" | "SPDRC" => {
                #[cfg(feature = "generic_constraint")]
                {
                    data_model::generic_constraint::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "generic_constraint"))]
                {
                    log::warn!("File key {:?} is not handled as the feature generic_constraint is not activated", file_key);
                }
            }
            "IRAUCTION" | "IRAUCTION_BIDS" | "IRAUCTION_CONFIG" | "SETTLEMENT_CONFIG" => {
                #[cfg(feature = "irauction")]
                {
                    data_model::irauction::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("File key {:?} is not handled as the feature irauction is not activated", file_key);
                }
            }
            "MARKET_CONFIG" => {
                #[cfg(feature = "market_config")]
                {
                    data_model::market_config::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "market_config"))]
                {
                    log::warn!("File key {:?} is not handled as the feature market_config is not activated", file_key);
                }
            }
            "MARKET_NOTICE" => {
                #[cfg(feature = "market_notice")]
                {
                    data_model::market_notice::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "market_notice"))]
                {
                    log::warn!("File key {:?} is not handled as the feature market_notice is not activated", file_key);
                }
            }
            "MCC" => {
                #[cfg(feature = "mcc_dispatch")]
                {
                    data_model::mcc_dispatch::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "mcc_dispatch"))]
                {
                    log::warn!("File key {:?} is not handled as the feature mcc_dispatch is not activated", file_key);
                }
            }
            "METERDATA" => {
                #[cfg(feature = "meter_data")]
                {
                    data_model::meter_data::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "meter_data"))]
                {
                    log::warn!("File key {:?} is not handled as the feature meter_data is not activated", file_key);
                }
            }
            "NETWORK" => {
                #[cfg(feature = "network")]
                {
                    data_model::network::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "network"))]
                {
                    log::warn!("File key {:?} is not handled as the feature network is not activated", file_key);
                }
            }
            "P5MIN" => {
                #[cfg(feature = "p5min")]
                {
                    data_model::p5min::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "p5min"))]
                {
                    log::warn!("File key {:?} is not handled as the feature p5min is not activated", file_key);
                }
            }
            "PARTICIPANT_REGISTRATION" => {
                #[cfg(feature = "participant_registration")]
                {
                    data_model::participant_registration::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("File key {:?} is not handled as the feature participant_registration is not activated", file_key);
                }
            }
            "PDPASA" => {
                #[cfg(feature = "pdpasa")]
                {
                    data_model::pdpasa::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "pdpasa"))]
                {
                    log::warn!("File key {:?} is not handled as the feature pdpasa is not activated", file_key);
                }
            }
            "PREDISPATCH" => {
                #[cfg(feature = "pre_dispatch")]
                {
                    data_model::pre_dispatch::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "pre_dispatch"))]
                {
                    log::warn!("File key {:?} is not handled as the feature pre_dispatch is not activated", file_key);
                }
            }
            "PRUDENTIAL" => {
                #[cfg(feature = "prudentials")]
                {
                    data_model::prudentials::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "prudentials"))]
                {
                    log::warn!("File key {:?} is not handled as the feature prudentials is not activated", file_key);
                }
            }
            "MTPASA" | "RESERVE_DATA" => {
                #[cfg(feature = "reserve_data")]
                {
                    data_model::reserve_data::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "reserve_data"))]
                {
                    log::warn!("File key {:?} is not handled as the feature reserve_data is not activated", file_key);
                }
            }
            "SETCFG" | "SETTLEMENTS_CONFIG" | "SETTLEMENT_CONFIG" => {
                #[cfg(feature = "settlement_config")]
                {
                    data_model::settlement_config::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "settlement_config"))]
                {
                    log::warn!("File key {:?} is not handled as the feature settlement_config is not activated", file_key);
                }
            }
            "SETTLEMENTS" => {
                #[cfg(feature = "settlement_data")]
                {
                    data_model::settlement_data::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("File key {:?} is not handled as the feature settlement_data is not activated", file_key);
                }
            }
            "STPASA" => {
                #[cfg(feature = "stpasa_solution")]
                {
                    data_model::stpasa_solution::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "stpasa_solution"))]
                {
                    log::warn!("File key {:?} is not handled as the feature stpasa_solution is not activated", file_key);
                }
            }
            "TRADING" => {
                #[cfg(feature = "trading_data")]
                {
                    data_model::trading_data::save(mms_file, &file_key, client, chunk_size).await?;
                }
                #[cfg(not(feature = "trading_data"))]
                {
                    log::warn!("File key {:?} is not handled as the feature trading_data is not activated", file_key);
                }
            }

            _ => {
                log::error!("Unexpected file key {:?}", file_key);
            }
        }
    }
Ok(())
}