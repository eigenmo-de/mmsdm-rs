#[cfg(feature = "sql_server")]
pub async fn save_all<'a, S>(
    file: impl Into<mmsdm_core::MmsFile<'a>>,
    skip_keys: Option<&std::collections::HashSet<mmsdm_core::FileKey>>,
    client: &mut tiberius::Client<S>,
    chunk_size: Option<usize>,
) -> mmsdm_core::Result<()>
where
    S: futures_util::AsyncRead + futures_util::AsyncWrite + Unpin + Send,
{
    let mut mms_file = file.into();
    for file_key in mms_file.file_keys() {
        if skip_keys.map(|set| set.contains(&file_key)).unwrap_or(false) {
            log::info!(
                "Skippping file key {} as it is in the list of keys to skip", file_key
            );
            continue;
        }
        match file_key.data_set_name.as_str() {
            "ANCILLIARY_SERVICES" => {
                #[cfg(feature = "ancillary_services")]
                {
                    mmsdm_ancillary_services::save(
                            &mut mms_file,
                            &file_key,
                            client,
                            chunk_size,
                        )
                        .await?;
                }
                #[cfg(not(feature = "ancillary_services"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature ancillary_services is not activated",
                        file_key
                    );
                }
            }
            "ASOFFER" => {
                #[cfg(feature = "asoffer")]
                {
                    mmsdm_asoffer::save(&mut mms_file, &file_key, client, chunk_size)
                        .await?;
                }
                #[cfg(not(feature = "asoffer"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature asoffer is not activated",
                        file_key
                    );
                }
            }
            "BID" | "BIDS" | "OFFER" => {
                #[cfg(feature = "bids")]
                {
                    mmsdm_bids::save(&mut mms_file, &file_key, client, chunk_size)
                        .await?;
                }
                #[cfg(not(feature = "bids"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature bids is not activated",
                        file_key
                    );
                }
            }
            "BILLING_CONFIG" => {
                #[cfg(feature = "billing_config")]
                {
                    mmsdm_billing_config::save(
                            &mut mms_file,
                            &file_key,
                            client,
                            chunk_size,
                        )
                        .await?;
                }
                #[cfg(not(feature = "billing_config"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature billing_config is not activated",
                        file_key
                    );
                }
            }
            "BILLING" => {
                #[cfg(feature = "billing_run")]
                {
                    mmsdm_billing_run::save(&mut mms_file, &file_key, client, chunk_size)
                        .await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature billing_run is not activated",
                        file_key
                    );
                }
            }
            "DEMAND" | "FORECAST" | "OPERATIONAL_DEMAND" | "ROOFTOP" => {
                #[cfg(feature = "demand_forecasts")]
                {
                    mmsdm_demand_forecasts::save(
                            &mut mms_file,
                            &file_key,
                            client,
                            chunk_size,
                        )
                        .await?;
                }
                #[cfg(not(feature = "demand_forecasts"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature demand_forecasts is not activated",
                        file_key
                    );
                }
            }
            "DISPATCH" | "DISPATCHOCD" | "PRICELOAD" => {
                #[cfg(feature = "dispatch")]
                {
                    mmsdm_dispatch::save(&mut mms_file, &file_key, client, chunk_size)
                        .await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature dispatch is not activated",
                        file_key
                    );
                }
            }
            "AP" | "FORCE_MAJEURE" => {
                #[cfg(feature = "force_majeure")]
                {
                    mmsdm_force_majeure::save(
                            &mut mms_file,
                            &file_key,
                            client,
                            chunk_size,
                        )
                        .await?;
                }
                #[cfg(not(feature = "force_majeure"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature force_majeure is not activated",
                        file_key
                    );
                }
            }
            "GD_INSTRUCT" => {
                #[cfg(feature = "gd_instruct")]
                {
                    mmsdm_gd_instruct::save(&mut mms_file, &file_key, client, chunk_size)
                        .await?;
                }
                #[cfg(not(feature = "gd_instruct"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature gd_instruct is not activated",
                        file_key
                    );
                }
            }
            "GCRHS" | "GENCONDATA" | "GENCONSET" | "GENCONSETINVOKE" | "GENCONSETTRK"
            | "GENERIC_CONSTRAINT" | "GEQDESC" | "GEQRHS" | "SPDCPC" | "SPDICC"
            | "SPDRC" => {
                #[cfg(feature = "generic_constraint")]
                {
                    mmsdm_generic_constraint::save(
                            &mut mms_file,
                            &file_key,
                            client,
                            chunk_size,
                        )
                        .await?;
                }
                #[cfg(not(feature = "generic_constraint"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature generic_constraint is not activated",
                        file_key
                    );
                }
            }
            "BID" | "DISPATCHBNC" | "DISPATCHOCD" | "METERDATA" | "METER_DATA" | "MR"
            | "OFFER" | "SETTLEMENTS" | "TRADING" => {
                #[cfg(feature = "historical")]
                {
                    mmsdm_historical::save(&mut mms_file, &file_key, client, chunk_size)
                        .await?;
                }
                #[cfg(not(feature = "historical"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature historical is not activated",
                        file_key
                    );
                }
            }
            "IRAUCTION" | "IRAUCTION_BIDS" | "IRAUCTION_CONFIG" => {
                #[cfg(feature = "irauction")]
                {
                    mmsdm_irauction::save(&mut mms_file, &file_key, client, chunk_size)
                        .await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature irauction is not activated",
                        file_key
                    );
                }
            }
            "MARKET_CONFIG" => {
                #[cfg(feature = "market_config")]
                {
                    mmsdm_market_config::save(
                            &mut mms_file,
                            &file_key,
                            client,
                            chunk_size,
                        )
                        .await?;
                }
                #[cfg(not(feature = "market_config"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature market_config is not activated",
                        file_key
                    );
                }
            }
            "MARKET_NOTICE" => {
                #[cfg(feature = "market_notice")]
                {
                    mmsdm_market_notice::save(
                            &mut mms_file,
                            &file_key,
                            client,
                            chunk_size,
                        )
                        .await?;
                }
                #[cfg(not(feature = "market_notice"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature market_notice is not activated",
                        file_key
                    );
                }
            }
            "MCC" => {
                #[cfg(feature = "mcc_dispatch")]
                {
                    mmsdm_mcc_dispatch::save(
                            &mut mms_file,
                            &file_key,
                            client,
                            chunk_size,
                        )
                        .await?;
                }
                #[cfg(not(feature = "mcc_dispatch"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature mcc_dispatch is not activated",
                        file_key
                    );
                }
            }
            "METERDATA" => {
                #[cfg(feature = "meter_data")]
                {
                    mmsdm_meter_data::save(&mut mms_file, &file_key, client, chunk_size)
                        .await?;
                }
                #[cfg(not(feature = "meter_data"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature meter_data is not activated",
                        file_key
                    );
                }
            }
            "MTPASA" => {
                #[cfg(feature = "mtpasa")]
                {
                    mmsdm_mtpasa::save(&mut mms_file, &file_key, client, chunk_size)
                        .await?;
                }
                #[cfg(not(feature = "mtpasa"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature mtpasa is not activated",
                        file_key
                    );
                }
            }
            "NETWORK" => {
                #[cfg(feature = "network")]
                {
                    mmsdm_network::save(&mut mms_file, &file_key, client, chunk_size)
                        .await?;
                }
                #[cfg(not(feature = "network"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature network is not activated",
                        file_key
                    );
                }
            }
            "P5MIN" => {
                #[cfg(feature = "p5min")]
                {
                    mmsdm_p5min::save(&mut mms_file, &file_key, client, chunk_size)
                        .await?;
                }
                #[cfg(not(feature = "p5min"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature p5min is not activated",
                        file_key
                    );
                }
            }
            "PARTICIPANT_REGISTRATION" => {
                #[cfg(feature = "participant_registration")]
                {
                    mmsdm_participant_registration::save(
                            &mut mms_file,
                            &file_key,
                            client,
                            chunk_size,
                        )
                        .await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature participant_registration is not activated",
                        file_key
                    );
                }
            }
            "PDPASA" => {
                #[cfg(feature = "pdpasa")]
                {
                    mmsdm_pdpasa::save(&mut mms_file, &file_key, client, chunk_size)
                        .await?;
                }
                #[cfg(not(feature = "pdpasa"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature pdpasa is not activated",
                        file_key
                    );
                }
            }
            "PREDISPATCH" => {
                #[cfg(feature = "pre_dispatch")]
                {
                    mmsdm_pre_dispatch::save(
                            &mut mms_file,
                            &file_key,
                            client,
                            chunk_size,
                        )
                        .await?;
                }
                #[cfg(not(feature = "pre_dispatch"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature pre_dispatch is not activated",
                        file_key
                    );
                }
            }
            "PRUDENTIAL" => {
                #[cfg(feature = "prudentials")]
                {
                    mmsdm_prudentials::save(&mut mms_file, &file_key, client, chunk_size)
                        .await?;
                }
                #[cfg(not(feature = "prudentials"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature prudentials is not activated",
                        file_key
                    );
                }
            }
            "MTPASA" | "RESERVE_DATA" => {
                #[cfg(feature = "reserve_data")]
                {
                    mmsdm_reserve_data::save(
                            &mut mms_file,
                            &file_key,
                            client,
                            chunk_size,
                        )
                        .await?;
                }
                #[cfg(not(feature = "reserve_data"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature reserve_data is not activated",
                        file_key
                    );
                }
            }
            "SETCFG" | "SETTLEMENTS_CONFIG" | "SETTLEMENT_CONFIG" => {
                #[cfg(feature = "settlement_config")]
                {
                    mmsdm_settlement_config::save(
                            &mut mms_file,
                            &file_key,
                            client,
                            chunk_size,
                        )
                        .await?;
                }
                #[cfg(not(feature = "settlement_config"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature settlement_config is not activated",
                        file_key
                    );
                }
            }
            "SETTLEMENTS" => {
                #[cfg(feature = "settlement_data")]
                {
                    mmsdm_settlement_data::save(
                            &mut mms_file,
                            &file_key,
                            client,
                            chunk_size,
                        )
                        .await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature settlement_data is not activated",
                        file_key
                    );
                }
            }
            "STPASA" => {
                #[cfg(feature = "stpasa_solution")]
                {
                    mmsdm_stpasa_solution::save(
                            &mut mms_file,
                            &file_key,
                            client,
                            chunk_size,
                        )
                        .await?;
                }
                #[cfg(not(feature = "stpasa_solution"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature stpasa_solution is not activated",
                        file_key
                    );
                }
            }
            "TRADING" => {
                #[cfg(feature = "trading_data")]
                {
                    mmsdm_trading_data::save(
                            &mut mms_file,
                            &file_key,
                            client,
                            chunk_size,
                        )
                        .await?;
                }
                #[cfg(not(feature = "trading_data"))]
                {
                    log::warn!(
                        "File key {:?} is not handled as the feature trading_data is not activated",
                        file_key
                    );
                }
            }
            _ => {
                log::error!("Unexpected file key {:?}", file_key);
            }
        }
    }
    Ok(())
}
