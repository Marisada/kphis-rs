// SELECT *,(SELECT EXISTS(SELECT * FROM kphis_log.message_read WHERE message_id=message.message_id AND read_user=?)) AS user_readed
// FROM kphis_log.message WHERE sender_code <> ? AND person IS NULL AND ward IS NULL AND spclty_id IS NULL AND message_id < 2 ORDER BY message_id DESC LIMIT 10;
/// loginname, sender_code, (oldest_id)
pub fn select_sse_message_global(has_oldest_id: bool, kphis_log: &str, limit: usize) -> String {
    let newer = if has_oldest_id {" AND message_id < ? "} else {""};
    [
        "SELECT *,(SELECT EXISTS(SELECT * FROM ",kphis_log,".message_read WHERE message_id=message.message_id AND read_user=?)) AS readed \
        FROM ",kphis_log,".message WHERE sender_code <> ? AND person IS NULL AND ward IS NULL AND spclty_id IS NULL ", newer, " ORDER BY message_id DESC LIMIT ", &limit.to_string(),";"
    ].concat()
}

// SELECT *,(SELECT EXISTS(SELECT * FROM kphis_log.message_read WHERE message_id=message.message_id AND read_user=?)) AS user_readed
// FROM kphis_log.message WHERE sender_code <> ? AND ward IN ('01','02') AND message_id < 2 ORDER BY message_id DESC LIMIT 10;
/// loginname, sender_code, (oldest_id)
pub fn select_sse_message_ward(
    wards: &[String],
    has_oldest_id: bool,
    kphis_log: &str,
    limit: usize,
) -> String {
    let ward_where = if wards.is_empty() {String::new()} else {[" AND ward IN ('", &wards.join("','"), "') "].concat()};
    let newer = if has_oldest_id {" AND message_id < ? "} else {""};
    [
        "SELECT *,(SELECT EXISTS(SELECT * FROM ",kphis_log,".message_read WHERE message_id=message.message_id AND read_user=?)) AS readed \
        FROM ",kphis_log,".message WHERE sender_code <> ? ",&ward_where,newer," ORDER BY message_id DESC LIMIT ",&limit.to_string(),";"
    ].concat()
}

// SELECT *,(SELECT EXISTS(SELECT * FROM kphis_log.message_read WHERE message_id=message.message_id AND read_user=?)) AS user_readed
// FROM kphis_log.message WHERE sender_code <> ? AND spclty_id IN (1,2) AND message_id < 2 ORDER BY message_id DESC LIMIT 10;
/// loginname, sender_code, (oldest_id)
pub fn select_sse_message_spclty(
    spclty_ids: &[u32],
    has_oldest_id: bool,
    kphis_log: &str,
    limit: usize,
) -> String {
    let spclty_where = if spclty_ids.is_empty() {String::new()} else {[" AND spclty_id IN (", &spclty_ids.iter().map(|u| u.to_string()).collect::<Vec<String>>().join(","), ") "].concat()};
    let newer = if has_oldest_id {" AND message_id < ? "} else {""};
    [
        "SELECT *,(SELECT EXISTS(SELECT * FROM ",kphis_log,".message_read WHERE message_id=message.message_id AND read_user=?)) AS readed \
        FROM ",kphis_log,".message WHERE sender_code <> ? ",&spclty_where,newer," ORDER BY message_id DESC LIMIT ",&limit.to_string(),";"
    ].concat()
}

// SELECT *,(SELECT EXISTS(SELECT * FROM kphis_log.message_read WHERE message_id=message.message_id AND read_user=?)) AS user_readed
// FROM kphis_log.message WHERE person='009' AND message_id < 2 ORDER BY message_id DESC LIMIT 10;
/// loginname, person, (oldest_id)
pub fn select_sse_message_private(has_oldest_id: bool, kphis_log: &str, limit: usize) -> String {
    let newer = if has_oldest_id {" AND message_id < ? "} else {""};
    [
        "SELECT *,(SELECT EXISTS(SELECT * FROM ",kphis_log,".message_read WHERE message_id=message.message_id AND read_user=?)) AS readed \
        FROM ",kphis_log,".message WHERE person=? ",newer," ORDER BY message_id DESC LIMIT ",&limit.to_string(),";"
    ].concat()
}

// INSERT INTO kphis_log.message (message_datetime,message,sender_code,sender_name,person,ward,spclty_id,route,reference,readed) VALUES (?,?,?,?,?,?,?,?,?,?);
/// message_datetime, message, sender_code, sender_name, person, ward, spclty_id, route, reference
pub fn insert_sse_message(kphis_log: &str) -> String {
    [
        "INSERT INTO ",kphis_log,".message (message_datetime,message,sender_code,sender_name,person,ward,spclty_id,route,reference) \
        VALUES (?,?,?,?,?,?,?,?,?);"
    ].concat()
}

// INSERT INTO kphis_log.message_read (message_id,read_user,read_datetime) VALUES
//   (?,?,NOW())
// ON DUPLICATE KEY UPDATE message_id=VALUES(message_id),read_user=VALUES(read_user),read_datetime=VALUES(read_datetime);
pub fn insert_dup_sse_message_read(user: &str, ids: &[u32], kphis_log: &str) -> String {
    let values = ids.iter().map(|id| ["(",&id.to_string(),",'",user,"',NOW())"].concat()).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis_log,".message_read (message_id,read_user,read_datetime) VALUES ",&values,
        " ON DUPLICATE KEY UPDATE message_id=VALUES(message_id),read_user=VALUES(read_user),read_datetime=VALUES(read_datetime);"
    ].concat()
}