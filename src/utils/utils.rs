use bson::DateTime as BsonDateTime;
use chrono::{DateTime, Timelike, Utc};

use crate::models::{client_request::ClientRequest, hog::Hog, hog_record::HogRecord};

pub fn get_timestamp() -> DateTime<Utc> {
    Utc::now()
        .with_nanosecond((Utc::now().nanosecond() / 1_000_000) * 1_000_000)
        .unwrap()
}

pub fn convert_timestamp_chrono_to_bson(datetime: DateTime<Utc>) -> BsonDateTime {
    BsonDateTime::from_chrono(datetime)
}

pub fn convert_timestamp_bson_to_chrono(datetime: BsonDateTime) -> DateTime<Utc> {
    datetime.to_chrono()
}

pub fn convert_timestamp_bson_to_string(datetime: BsonDateTime) -> String {
    datetime.to_chrono().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

pub fn rfc3339_str_to_bson(rfc3339: &str) -> Result<BsonDateTime, chrono::ParseError> {
    let chrono_dt: DateTime<Utc> = rfc3339.parse()?;
    Ok(BsonDateTime::from_chrono(chrono_dt))
}


pub fn convert_hog_records_to_hogs(hog_records: Vec<HogRecord>) -> Vec<Hog> {
    hog_records
        .into_iter()
        .map(|record| {
            let client_request = ClientRequest {
                log_timestamp: convert_timestamp_bson_to_string(record.log_timestamp),
                log_level: record.log_level,
                log_message: record.log_message,
                log_data: record.log_data,
                log_type: record.log_type,
                log_source: record.log_source,
                log_source_id: record.log_source_id,
            };
            Hog {
                client_request,
                hog_uuid: record.hog_uuid,
                hog_timestamp: record.hog_timestamp.map(|dt| dt.to_chrono()),
                id: record.id.map(|oid| oid.to_hex()), // ObjectId -> String
                created_at: record.created_at.map(convert_timestamp_bson_to_chrono),
            }
        })
        .collect()
}


use mongodb::bson::oid::ObjectId;

pub fn convert_hogs_to_hog_records(hogs: Vec<Hog>) -> Vec<HogRecord> {
    hogs
        .into_iter()
        .map(|hog| {
            HogRecord {
                log_timestamp: rfc3339_str_to_bson(&hog.client_request.log_timestamp).unwrap(),
                log_level: hog.client_request.log_level,
                log_message: hog.client_request.log_message,
                log_data: hog.client_request.log_data,
                log_type: hog.client_request.log_type,
                log_source: hog.client_request.log_source,
                log_source_id: hog.client_request.log_source_id,
                hog_uuid: hog.hog_uuid,
                hog_timestamp: hog.hog_timestamp.map(convert_timestamp_chrono_to_bson),
                created_at: Some(BsonDateTime::from_chrono(get_timestamp())),
                id: hog.id.and_then(|s| ObjectId::parse_str(&s).ok()), // String -> ObjectId
            }
        })
        .collect()
}


pub fn convert_hog_to_hog_record(hog: &Hog) -> HogRecord {
    HogRecord {
        log_timestamp: rfc3339_str_to_bson(&hog.client_request.log_timestamp).unwrap(),
        log_level: hog.client_request.log_level.clone(),
        log_message: hog.client_request.log_message.clone(),
        log_data: hog.client_request.log_data.clone(),
        log_type: hog.client_request.log_type.clone(),
        log_source: hog.client_request.log_source.clone(),
        log_source_id: hog.client_request.log_source_id.clone(),
        hog_uuid: hog.hog_uuid.clone(),
        hog_timestamp: hog.hog_timestamp.map(convert_timestamp_chrono_to_bson),
        created_at: Some(BsonDateTime::from_chrono(get_timestamp())),
        id: hog.id.as_ref().and_then(|s| ObjectId::parse_str(s).ok()), // String -> ObjectId
    }
}


pub fn convert_hog_record_to_hog(hog_record: &HogRecord) -> Hog {
    Hog {
        client_request: ClientRequest {
            log_timestamp: convert_timestamp_bson_to_string(hog_record.log_timestamp),
            log_level: hog_record.log_level.clone(),
            log_message: hog_record.log_message.clone(),
            log_data: hog_record.log_data.clone(),
            log_type: hog_record.log_type.clone(),
            log_source: hog_record.log_source.clone(),
            log_source_id: hog_record.log_source_id.clone(),
        },
        hog_uuid: hog_record.hog_uuid.clone(),
        hog_timestamp: hog_record.hog_timestamp.map(|dt| dt.to_chrono()),
        id: hog_record.id.map(|oid| oid.to_hex()), // ObjectId -> String
        created_at: hog_record.created_at.map(convert_timestamp_bson_to_chrono),
    }
}
