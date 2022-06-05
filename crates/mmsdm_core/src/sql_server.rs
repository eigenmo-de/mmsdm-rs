use futures_util::{AsyncRead, AsyncWrite};

async fn log_file<S>(
    header: &crate::AemoHeader,
    client: &mut tiberius::Client<S>,
    key: &crate::FileKey,
    total_rows: i64,
) -> crate::Result<i64>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    // declare @header table (id bigint not null);
    // output inserted.id into @header
    let first_row = client
        .query(
            "insert into mmsdm.FileLog(
            data_source,
            file_name,
            from_participant,
            to_participant,
            effective_date,
            effective_time,
            serial_number,
            file_name_2,
            serial_number_2,
            data_set,
            sub_type,
            version,
            total_rows
        )
        output inserted.file_log_id
        values (@P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8, @P9, @P10, @P11, @P12, @P13);",
            &[
                &header.data_source,
                &header.file_name,
                &header.from_participant,
                &header.to_participant,
                &header.effective_date,
                &header.effective_time,
                &header.serial_number,
                &header.file_name_2,
                &header.serial_number_2,
                &key.data_set_name.as_str(),
                &key.table_name(),
                &key.version,
                &total_rows,
            ],
        )
        .await?
        .into_row()
        .await?;
    let row = first_row.ok_or(crate::Error::CreateFileLogError)?;
    row.try_get(0)?.ok_or(crate::Error::CreateFileLogError)
}

pub async fn batched_insert<S, D>(
    client: &mut tiberius::Client<S>,
    file_key: &crate::FileKey,
    header: &crate::AemoHeader,
    data: &[D],
    proc: &str,
    chunk_size: Option<usize>,
) -> crate::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    D: serde::Serialize,
{
    let total = i64::try_from(data.len())?;
    let chunk_size = chunk_size.unwrap_or(4_500);

    let file_log_id = log_file(header, client, file_key, total).await?;

    let mut current = 0_i64;

    for chunk in data.chunks(chunk_size) {
        let json = serde_json::to_string(chunk)?;
        if let Err(e) = client.execute(proc, &[&file_log_id, &json]).await {
            client.execute(
                    "update mmsdm.FileLog set [status] = 'E', rows_inserted = @P1, message = @P2 where file_log_id = @P3",
                    &[&current, &e.to_string(), &file_log_id],
                ).await?;
            return Err(e.into());
        } else {
            current += i64::try_from(chunk.len())?;
            client
                .execute(
                    "update mmsdm.FileLog set rows_inserted = @P1 where file_log_id = @P2",
                    &[&current, &file_log_id],
                )
                .await?;
            log::debug!("Progress: {} out of {} rows saved", current, total);
        }
    }
    client
        .execute(
            "update mmsdm.FileLog set [status] = 'C', rows_inserted = @P1 where file_log_id = @P2",
            &[&current, &file_log_id],
        )
        .await?;
    Ok(())
}
