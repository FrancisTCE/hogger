pub async fn build_filter(options: OptionsRequest) {
    return match search_type {
            SearchType::LogSource => {
                if let Some(ref log_source) = options.log_source {
                    doc! { "log_source": log_source }
                } else {
                    doc! {}
                }
            }
            SearchType::LogLevel => {
                println!("log level: {:?}", options.log_level);
                if let Some(ref log_level) = options.log_level {
                    doc! { "log_level": log_level }
                } else {
                    doc! {}
                }
            }
            SearchType::LogMessage => {
                if let Some(ref log_message) = options.log_message {
                    doc! { "log_message": log_message }
                } else {
                    doc! {}
                }
            }
            SearchType::LogData => {
                if let Some(ref log_data) = options.log_data {
                    match bson::to_bson(log_data) {
                        Ok(bson_data) => doc! { "log_data": bson_data },
                        Err(_) => doc! {},
                    }
                } else {
                    doc! {}
                }
            }
            SearchType::LogType => {
                if let Some(ref log_type) = options.log_type {
                    doc! { "log_type": log_type }
                } else {
                    doc! {}
                }
            }
            SearchType::LogSourceId => {
                if let Some(ref log_source_id) = options.log_source_id {
                    doc! { "log_source_id": log_source_id }
                } else {
                    doc! {}
                }
            }
            SearchType::LogUuid => {
                if let Some(ref log_uuid) = options.log_uuid {
                    doc! { "log_uuid": log_uuid }
                } else {
                    doc! {}
                }
            }
            SearchType::HogUuid => {
                if let Some(ref hog_uuid) = options.hog_uuid {
                    doc! { "hog_uuid": hog_uuid }
                } else {
                    doc! {}
                }
            }
            SearchType::HogTimestamp => {
                if let Some(ref timestamp) = options.timestamp {
                    doc! { "hog_timestamp": bson::DateTime::from_millis(timestamp.timestamp_millis()) }
                } else {
                    doc! {}
                }
            }
            SearchType::TimeInterval => {
                if let (Some(ref start), Some(ref end)) =
                    (options.timestamp_start, options.timestamp_end)
                {
                    doc! { "log_timestamp": { 
                        "$gte": bson::DateTime::from_millis(start.timestamp_millis()), 
                        "$lte": bson::DateTime::from_millis(end.timestamp_millis()) 
                    } }
                } else {
                    doc! {}
                }
            }
            SearchType::HogTimeInterval => {
                if let (Some(ref start), Some(ref end)) =
                    (options.timestamp_start, options.timestamp_end)
                {
                    doc! { "hog_timestamp": { 
                        "$gte": bson::DateTime::from_millis(start.timestamp_millis()), 
                        "$lte": bson::DateTime::from_millis(end.timestamp_millis()) 
                    } }
                } else {
                    doc! {}
                }
            }
        };

}