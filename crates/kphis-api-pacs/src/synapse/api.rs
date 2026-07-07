use axum::body::Bytes;
use reqwest::Method;
use serde_json::Value;
use time::{Date, PrimitiveDateTime, Time, format_description::well_known::Iso8601};

use kphis_model::pacs::{PacsConfig, PacsImageData, PacsXnData};
use kphis_util::{
    datetime::datetime_from_opt,
    error::{AppError, Source},
};

use super::ntlm::AspNetAuth;

pub async fn get_xn_data_inner(xn: i32, pacs_client: &reqwest::Client, pacs_config: &PacsConfig) -> Result<PacsXnData, AppError> {
    // 1. get sessionID
    let mut client = AspNetAuth::new(
        Method::POST,
        &[&pacs_config.pacs_host, "/api/WorkflowEngine/Worklist/Query?identifier=/Synapse/1000004"].concat(),
        &gen_xn_query_body(xn, &pacs_config.pacs_user),
        true,
        &pacs_config.pacs_user,
        &pacs_config.pacs_password,
        None, //Some("syn5ndh"),
        pacs_client,
    );
    let query_body = client.run().await?;
    let session_id = get_session_id(&query_body)?;

    // 2. get instanceEUID
    client.set_next_request(
        Method::POST,
        &[&pacs_config.pacs_host, "/api/WorkflowEngine/Worklist/Retrieve?identifier=/Synapse/1000004"].concat(),
        &gen_xn_request_body(xn, &session_id, &pacs_config.pacs_user),
        true,
    );
    let study_body = client.run().await?;
    let study_uid = get_instance_euid(&study_body)?;

    // 3. get seriesIuid + objectUID
    client.set_next_request(Method::GET, &gen_series_path(&study_uid, &pacs_config.pacs_host, &pacs_config.pacs_data_source), &[], true);
    let series_body = client.run().await?;

    let result = get_xn_data_synapse(&series_body).map_err(|e| Source::SerdeJson.to_error(500, e, "GetXnData"))?;

    // 4. delete sessionID
    client.set_next_request(
        Method::DELETE,
        &[&pacs_config.pacs_host, "/api/WorkflowEngine/Worklist/Cache?sessionId=", &session_id].concat(),
        &[],
        false,
    );
    let _ = client.run().await?;

    Ok(result)
}

pub async fn get_thumbnail_inner(file_path: &str, study_uid: &str, pacs_client: &reqwest::Client, pacs_config: &PacsConfig) -> Result<Bytes, AppError> {
    let mut client = AspNetAuth::new(
        Method::GET,
        &gen_thumb_path(file_path, study_uid, &pacs_config.pacs_host, &pacs_config.pacs_data_source),
        &[],
        false,
        &pacs_config.pacs_user,
        &pacs_config.pacs_password,
        None, //Some("syn5ndh"),
        pacs_client,
    );
    let jpeg_body = client.run().await?;

    Ok(jpeg_body)
}

pub async fn get_image_inner(study_uid: &str, series_uid: &str, object_uid: &str, pacs_client: &reqwest::Client, pacs_config: &PacsConfig) -> Result<Bytes, AppError> {
    let mut client = AspNetAuth::new(
        Method::GET,
        &gen_wado_path(study_uid, series_uid, object_uid, &pacs_config.pacs_host, &pacs_config.pacs_data_source),
        &[],
        false,
        &pacs_config.pacs_user,
        &pacs_config.pacs_password,
        None, //Some("syn5ndh"),
        pacs_client,
    );
    let jpeg_body = client.run().await?;

    Ok(jpeg_body)
}

fn gen_xn_query_body(xn: i32, pacs_user: &str) -> Vec<u8> {
    let xn_req_str = [
        r#"
{
  "Filters": [
    {
      "Column": "risStudyEUID",
      "Operator": "begins-with",
      "Value": ""#,
        &xn.to_string(),
        r#"",
      "SecondValue": null,
      "Group": 1,
      "DataType": "string"
    }
  ],
  "Sorts": [
    {
      "Column": "displayTimestamp",
      "Direction": -1
    }
  ],
  "Culture": "en-US",
  "ForceRefresh": false,
  "UserLogin": ""#,
        pacs_user,
        r#"",
  "WorklistType": 0,
  "TargetDataSources": []
}"#,
    ]
    .concat();
    xn_req_str.as_bytes().to_vec()
}

fn get_session_id(body: &[u8]) -> Result<String, AppError> {
    let xn_resp_json: Value = serde_json::from_slice(body).map_err(|e| Source::SerdeJson.to_error(500, e, "GetXnDataSessionId"))?;
    // dbg!(&xn_resp_json);
    xn_resp_json["SessionID"]
        .as_str()
        .map(|s| s.to_owned())
        .ok_or(AppError::new_server(500, "No 'sessionID' in response JSON", "GetXnDataSessionId"))
}

fn gen_xn_request_body(xn: i32, session_id: &str, pacs_user: &str) -> Vec<u8> {
    let xn_req_str = [
        r#"
{
  "Length": 14,
  "Offset": 0,
  "SessionID": ""#,
        session_id,
        r#"",
  "Filters": [
    {
      "Column": "risStudyEUID",
      "Operator": "begins-with",
      "Value": ""#,
        &xn.to_string(),
        r#"",
      "SecondValue": null,
      "Group": 1,
      "DataType": "string"
    }
  ],
  "Sorts": [
    {
      "Column": "displayTimestamp",
      "Direction": -1
    }
  ],
  "Culture": "en-US",
  "UserLogin": ""#,
        pacs_user,
        r#"",
  "WorklistType": 0,
  "TargetDataSources": []
}"#,
    ]
    .concat();
    xn_req_str.as_bytes().to_vec()
}

fn get_instance_euid(body: &[u8]) -> Result<String, AppError> {
    let xn_resp_json: Value = serde_json::from_slice(body).map_err(|e| Source::SerdeJson.to_error(500, e, "GetXnDataInstanceEUID"))?;
    // dbg!(&xn_resp_json);
    if let Some(records) = xn_resp_json["Records"].as_array() {
        if let Some(record) = records.first() {
            if let Some(uid) = record["instanceEUID"].as_str().map(|s| s.to_owned()) {
                Ok(uid)
            } else {
                Err(AppError::new_server(500, "No 'instanceEUID' in response JSON", "GetXnDataInstanceEUID"))
            }
        } else {
            Err(AppError::app_404("GetXnDataInstanceEUID"))
        }
    } else {
        Err(AppError::new_server(500, "No 'Records' in response JSON", "GetXnDataInstanceEUID"))
    }
}

fn gen_series_path(study_uid: &str, pacs_host: &str, pacs_data_source: &str) -> String {
    [
        pacs_host,
        "/ViewerService/api/study?requestType=fujiStudyData\
        &studyuid=",
        study_uid,
        "&UseLocalizer=true\
        &LocalizerFirst=true\
        &UseEcho=true\
        &UseTpi=true\
        &UseContinuity=true\
        &UseMRContinuity=true\
        &v53Rules=true\
        &Anonymous=false\
        &addToHistory=true\
        &rollup=true\
        &dataSource=",
        pacs_data_source,
    ]
    .concat()
}

fn get_xn_data_synapse(body: &[u8]) -> serde_json::Result<PacsXnData> {
    let series_resp_json: Value = serde_json::from_slice(&body)?;
    let study = &series_resp_json["studies"][0];
    let study_uid = study["iuid"].as_str().map(|s| s.to_owned());
    let patient = &study["patient"];

    let images = if let (Some(uid), Some(series)) = (&study_uid, study["series"].as_array()) {
        series
            .iter()
            .filter_map(|serie| {
                let file_path = serie["thumbnail"].as_str().map(|s| s.trim_start_matches("thumbnail?path=").to_owned()).unwrap_or_default();
                let image = &serie["sharedImage"];
                let series_datetime = image["seriesDateTime"].as_str().and_then(|s| datetime_8601(s));
                let series_num = image["seriesNum"].as_u64().map(|u| u.to_string());
                if let (Some(series_uid), Some(object_uid)) = (image["seriesIuid"].as_str(), image["iuid"].as_str()) {
                    // here has 2 conditions
                    // 1. images has rows: object_uid is sharedImage.iuid + images.x.iuid
                    // 2. images is empty: object_uid is sharedImage.iuid
                    if let Some(images) = &serie["images"].as_array()
                        && !images.is_empty()
                    {
                        let ims = images
                            .iter()
                            .enumerate()
                            .filter_map(|(i, im)| {
                                im["iuid"].as_str().map(|tail| PacsImageData {
                                    study_uid: uid.clone(),
                                    series_uid: series_uid.to_owned(),
                                    object_uid: [object_uid, tail].concat(),
                                    series_datetime,
                                    label: series_num.as_ref().map(|n| [n, " page ", &(i + 1).to_string()].concat()),
                                    file_path: file_path.clone(),
                                })
                            })
                            .collect::<Vec<PacsImageData>>();
                        // fixed serie["images"] may be "[{}]" (not empty but no data)
                        if ims.is_empty() {
                            Some(vec![PacsImageData {
                                study_uid: uid.clone(),
                                series_uid: series_uid.to_owned(),
                                object_uid: object_uid.to_owned(),
                                series_datetime,
                                label: series_num,
                                file_path,
                            }])
                        } else {
                            Some(ims)
                        }
                    } else {
                        Some(vec![PacsImageData {
                            study_uid: uid.clone(),
                            series_uid: series_uid.to_owned(),
                            object_uid: object_uid.to_owned(),
                            series_datetime,
                            label: series_num,
                            file_path,
                        }])
                    }
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<PacsImageData>>()
    } else {
        Vec::new()
    };

    Ok(PacsXnData {
        fname: patient["afName"].as_str().map(|s| s.to_owned()).unwrap_or_default(),
        lname: patient["alName"].as_str().map(|s| s.to_owned()).unwrap_or_default(),
        mname: patient["amName"].as_str().map(|s| s.to_owned()).unwrap_or_default(),
        sname: patient["asName"].as_str().map(|s| s.to_owned()).unwrap_or_default(),
        birth: patient["birth"].as_str().and_then(|s| datetime_8601(s)),
        ext_id: patient["extId"].as_str().map(|s| s.to_owned()).unwrap_or_default(),
        gender: patient["gender"].as_str().map(|s| s.to_owned()).unwrap_or_default(),
        study_uid,
        images,
    })
}

fn gen_thumb_path(file_path: &str, study_uid: &str, pacs_host: &str, pacs_data_source: &str) -> String {
    [pacs_host, "/ViewerService/api/thumbnail?path=", &file_path, "&studyUID=", &study_uid, "&dataSource=", pacs_data_source].concat()
}

fn gen_wado_path(study_uid: &str, series_uid: &str, object_uid: &str, pacs_host: &str, pacs_data_source: &str) -> String {
    [
        pacs_host,
        "/ViewerService/api/wado?requestType=WADO\
        &studyUID=",
        study_uid,
        "&seriesUID=",
        series_uid,
        "&objectUID=",
        object_uid,
        "&dataSource=",
        pacs_data_source,
        "&frameNumber=1\
        &contentType=image/jpeg",
    ]
    .concat()
}

/// parse ISO-8601 string to time::PrimitiveDateTime, error will be `None`, discard offset
pub fn datetime_8601(text: &str) -> Option<PrimitiveDateTime> {
    let sanitize = text.replace('T', " ");
    let dt = sanitize.split(' ').map(str::trim).collect::<Vec<&str>>();
    if dt.len() != 2 {
        return None;
    }
    let date = date_8601(dt[0]);
    let time = time_8601(dt[1]);

    datetime_from_opt(date, time)
}
/// parse ISO-8601 string to time::Date, error will be `None`
pub fn date_8601(text: &str) -> Option<Date> {
    Date::parse(text, &Iso8601::DEFAULT).ok()
}
/// parse ISO-8601 string to time::Time, error will be `None`, discard offset
pub fn time_8601(text: &str) -> Option<Time> {
    Time::parse(text, &Iso8601::DEFAULT).ok()
}

#[cfg(test)]
#[rustfmt::skip]
pub mod tests {

    use super::*;

    #[test]
    fn test_gen_xn_query_body() {
        let xn = 44654;
        let xn_req_bytes = gen_xn_query_body(xn, "ward");
        let xn_req_json: Value = serde_json::from_slice(&xn_req_bytes).unwrap();
        assert_eq!(xn_req_json["Filters"][0]["Value"].as_str().unwrap(), xn.to_string());
    }

    #[test]
    fn test_get_session_id() {
        let xn_resp_str = r#"
{
    "SessionID": "b6d294a7-d13a-4061-8a32-d2296c348e36",
    "NumResults": 1,
    "Succeeded": true,
    "IsWebQuerySupported": true
}"#;
    let session_id = get_session_id(xn_resp_str.as_bytes()).unwrap();
    assert_eq!(session_id, String::from("b6d294a7-d13a-4061-8a32-d2296c348e36"));
    }

    #[test]
    fn test_gen_xn_request_body() {
        let xn = 44654;
        let session_id = "b6d294a7-d13a-4061-8a32-d2296c348e36";
        let xn_req_bytes = gen_xn_request_body(xn, session_id, "ward");
        let xn_req_json: Value = serde_json::from_slice(&xn_req_bytes).unwrap();
        assert_eq!(xn_req_json["Filters"][0]["Value"].as_str().unwrap(), xn.to_string());
        assert_eq!(xn_req_json["SessionID"].as_str().unwrap(), session_id.to_string());
    }

    #[test]
    fn test_get_instance_euid() {
        let xn_resp_str = r#"
{
    "Records": [
        {
            "primaryLocName": "Emergency Room (ER)",
            "displayTimestamp": "2023-03-30T09:52:47",
            "externalPatientID": "0028116",
            "displayStateFlag": 0,
            "edFindings": null,
            "procedureDescription": "Film CXR (PA)",
            "reason": null,
            "internalPatientUID": "0028116",
            "visitClass": "O",
            "anomalyFlag": "N",
            "studyCompletionDate": "2023-03-30T10:04:00",
            "procedureModifier": null,
            "dataSource": "SYN5NDH",
            "totalNumberNote": 0,
            "studySiteName": "BMS-HOSXP",
            "priority": "R",
            "instanceEUID": "1.2.840.113845.11.1000000002170592405.20230330100002.1008566",
            "patientFullName": "XXX",
            "aliasPatientName": "XXX",
            "birthDate": "1960-11-11T00:00:00",
            "procedureCode": "11",
            "risStudyEUID": "44654",
            "totalNumberDoc": 0,
            "ERFStatus": "Not forwarded",
            "timeZoneOffset": "-08:00",
            "dictationUserName": null,
            "gender": "M",
            "refPhyName": "Unknown, Unknown ",
            "status": "Complete",
            "studyImageCount": 1,
            "reportStatusCode": -1,
            "lastModifiedTimestamp": "2023-03-30T10:04:00",
            "timeZoneAbbrev": "PST",
            "urgentFindings": null,
            "procedureModality": "DX",
            "studyStatusCD": 40,
            "patientUUID": "0B0237A087344817A7FD04DB8526EC5D",
            "studyUID": 1031451,
            "last_display_timestamp": "2023-03-30T16:01:03.578",
            "isStat": "N",
            "aiFindings": null,
            "aiStatus": null,
            "reportImpression": "N/A",
            "studyLock": 0,
            "reservationIndicator": 0,
            "isReservedByOthers": 0
        }
    ],
    "NumResults": 1,
    "ColumnTypes": {
        "medicalAlert": "string",
        "primaryLocName": "string",
        "acquisitionStartTimestamp": "datetime",
        "notePresent": "string",
        "displayTimestamp": "datetime",
        "externalPatientID": "string",
        "displayStateFlag": "number",
        "edFindings": "string",
        "procedureDescription": "string",
        "reason": "string",
        "medicalCheckupNo": "string",
        "imageSenderName": "string",
        "internalPatientUID": "string",
        "homePhone": "string",
        "streetAddress": "string",
        "race": "string",
        "reqPhyName": "string",
        "visitClass": "string",
        "anomalyFlag": "string",
        "studyCompletionDate": "datetime",
        "procedureModifier": "string",
        "dataSource": "string",
        "medicalCheckupCourseNo": "string",
        "scheduleTimestamp": "datetime",
        "totalNumberNote": "number",
        "studySiteName": "string",
        "weight": "string",
        "personalID": "string",
        "totalNumberCad": "number",
        "reportStatus": "string",
        "priority": "string",
        "instanceEUID": "string",
        "patientFullName": "string",
        "aliasPatientName": "string",
        "birthDate": "date",
        "attPhyName": "string",
        "admissionTimeDate": "datetime",
        "procedureCode": "string",
        "risStudyEUID": "string",
        "totalNumberDoc": "number",
        "visitNumber": "string",
        "ERFStatus": "string",
        "timeZoneOffset": "string",
        "dictationUserName": "string",
        "gender": "string",
        "refPhyName": "string",
        "currentLocName": "string",
        "facilityCode": "string",
        "status": "string",
        "studyImageCount": "number",
        "reportStatusCode": "string",
        "lastModifiedTimestamp": "datetime",
        "timeZoneAbbrev": "string",
        "urgentFindings": "string",
        "workPhone": "string",
        "procedureModality": "string",
        "risOrderEUID": "string",
        "imageReceiverName": "string",
        "studyLock": "string",
        "reservationIndicator": "string",
        "studyStatusCD": "string",
        "patientUUID": "string",
        "studyUID": "number",
        "last_display_timestamp": "datetime",
        "isStat": "string",
        "reportImpression": "string",
        "aiFindings": "string",
        "aiStatus": "string",
        "aiAlgorithms": "string",
        "aitlAlgorithms": "string",
        "aiTriage": "string"
    },
    "LastRefreshed": "2023-03-30T09:08:47.9002082Z",
    "IsCacheExpired": false,
    "Offset": 0,
    "IsWebQuerySupported": true
}"#;
        let instance_euid = get_instance_euid(xn_resp_str.as_bytes()).unwrap();
        assert_eq!(instance_euid, String::from("1.2.840.113845.11.1000000002170592405.20230330100002.1008566"));
    }

    #[test]
    fn test_get_xn_data_synapse() {
        let series_resp_str = r#"
{
    "version": "0.0.0.1",
    "studies": [
        {
            "id": 1031217,
            "organizerName": "Fujifilm.Synapse.Visualization.SeriesOrganization.Plugins.Default.Plugin, Fujifilm.Synapse.Visualization.SeriesOrganization.Plugins.Default, Version=5.7.200.0, Culture=neutral, PublicKeyToken=null",
            "iuid": "1.2.840.113845.11.1000000002170592405.20230311085710.1008332",
            "risEuid": "44417",
            "orderEuid": "44417",
            "visitNum": "660311080952",
            "acquired": "2023-03-11T08:49:35",
            "modified": "2023-03-11T09:00:30",
            "dateTime": "2023-03-11T08:48:04",
            "imageCount": 1,
            "nonImageCount": 0,
            "modality": "DX",
            "patient": {
                "id": 1007223,
                "uuid": "697A1E78DAFE48D598474F8CB97FEA88",
                "lName": "XXX",
                "fName": "XXX",
                "mName": "XXX",
                "sName": " ",
                "alName": "XXX",
                "afName": "XXX",
                "amName": "",
                "asName": "XXX",
                "birth": "1958-07-06T00:00:00",
                "extId": "0000475",
                "intUid": "0000475",
                "mrn": "0000475",
                "gender": "F"
            },
            "procedure": {
                "procCode": "74",
                "procDescription": "CHEST AP",
                "procModality": "DX",
                "procBodyParts": ""
            },
            "statusCode": "Complete",
            "dataSourceType": "Synapse",
            "siteUid": 1000004,
            "hasAnomaly": false,
            "series": [
                {
                    "uuid": "8D62B709E1E892399C9DB8F8256BA1B5C308E311",
                    "iuid": "1.2.392.200036.9125.3.2322162091853138.6529855684.58125703",
                    "id": 1042720,
                    "dataSourceType": "Synapse",
                    "sopClassUid": "1.2.840.10008.5.1.4.1.1.1.1",
                    "modality": "DX",
                    "modalityCode": "DX",
                    "num": 1001,
                    "forUid": "",
                    "thumbnail": "thumbnail?path=2023/0311/1031217/1042720/1044362.jpg",
                    "isTemp": false,
                    "isMammo": false,
                    "sharedImage": {
                        "seriesIuid": "1.2.392.200036.9125.3.2322162091853138.6529855684.58125703",
                        "seriesNum": 1001,
                        "acqNum": 0,
                        "acquired": "2023-03-11T08:49:35",
                        "seriesDateTime": "2023-03-11T08:49:35",
                        "cols": 2242,
                        "forUid": "",
                        "iuid": "1.2.392.200036.9125.4.0.3928212649.1175902440.3117517315",
                        "laterality": "U",
                        "localizer": "N",
                        "mfiCid": "",
                        "num": 1001,
                        "psize": "0.15,0.15",
                        "rows": 2505,
                        "seriesUid": 1042720,
                        "sopClassUid": "1.2.840.10008.5.1.4.1.1.1.1",
                        "frame": 1,
                        "id": 1044362,
                        "viewCodeSq": "",
                        "viewModifier": "",
                        "aeTitle": "FUJI_DR01",
                        "manufacturer": "FUJIFILM Corporation",
                        "pmi": "MONOCHROME1",
                        "tomoAlgorithm": "",
                        "volumetricProperties": "",
                        "path": "\\\\SYN5NDH\\st0$\\03112023\\XXX\\44417\\1.2.392.200036.9125.3.2322162091853138.6529855684.58125703",
                        "retention": 100,
                        "offset": 0,
                        "length": 5572516
                    },
                    "images": [
                        {}
                    ]
                }
            ],
            "settingid": 20,
            "nonImageSeries": [],
            "analysisSeries": []
        }
    ]
}"#;
        let result = get_xn_data_synapse(series_resp_str.as_bytes()).unwrap();
        let image = result.images.first().unwrap();
        assert_eq!(image.study_uid, String::from("1.2.840.113845.11.1000000002170592405.20230311085710.1008332"));
        assert_eq!(image.series_uid, String::from("1.2.392.200036.9125.3.2322162091853138.6529855684.58125703"));
        assert_eq!(image.object_uid, String::from("1.2.392.200036.9125.4.0.3928212649.1175902440.3117517315"));
    }
}
