// INSERT INTO kphis.system_access_log(access_datetime,access_user,access_host,access_detail)
// VALUES (now(),'user','127.0.0.1:11111','{"program":"IPD_DOCTOR_MAIN_PROGRAM","an":"660001363"}');
pub fn insert_system_access_log(kphis_log: &str) -> String {
    [
        "INSERT INTO ",kphis_log,".system_access_log(access_datetime,access_user,access_host,access_detail) \
        VALUES (NOW(),?,?,?);"
    ].concat()
}

// DELETE FROM kphis_log.history_log WHERE DATEDIFF(NOW(),history_datetime) > ?;
/// days
pub fn delete_expired_history_log(kphis_log: &str) -> String {
    [
        "DELETE FROM ",kphis_log,".history_log WHERE DATEDIFF(NOW(),history_datetime) > ?;"
    ].concat()
}

// DELETE FROM kphis_log.system_access_log WHERE DATEDIFF(NOW(),access_datetime) > ?;
/// days
pub fn delete_expired_access_log(kphis_log: &str) -> String {
    [
        "DELETE FROM ",kphis_log,".system_access_log WHERE DATEDIFF(NOW(),access_datetime) > ?;"
    ].concat()
}

// DELETE FROM kphis_log.message WHERE DATEDIFF(NOW(),message_datetime) > ?;
/// days
pub fn delete_expired_message(kphis_log: &str) -> String {
    [
        "DELETE FROM ",kphis_log,".message WHERE DATEDIFF(NOW(),message_datetime) > ?;"
    ].concat()
}
