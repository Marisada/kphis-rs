use crate::TABLE_CREATE_COLUMNS;

// INSERT IGNORE INTO kphis_extra.image (path,title,create_user,create_datetime,update_user,update_datetime,version) VALUES (?,NULL,?,NOW(),?,NOW(),1)
// RETURNING image_id,path,title,create_user,create_datetime;
/// (path, loginname, loginname) x len
pub fn insert_image(len: usize, kphis_extra: &str) -> String {
    let values = vec!["(?,NULL,?,NOW(),?,NOW(),1)"; len].join(",");
    [
        "INSERT IGNORE INTO ",kphis_extra,".image (path,title",TABLE_CREATE_COLUMNS,") VALUES ",
        &values," RETURNING image_id,path,title,create_datetime;"
    ].concat()
}

// UPDATE kphis_extra.image SET title=?,update_datetime=NOW() WHERE image_id=? AND create_user=?;
/// title, image_id, loginname
pub fn update_image_title(kphis_extra: &str) -> String {
    [
        "UPDATE ",kphis_extra,".image SET title=?,update_datetime=NOW() WHERE image_id=? AND create_user=?;"
    ].concat()
}

// SELECT path FROM kphis_extra.image WHERE image_id IN (?);
/// image_id x len
pub fn select_image_path_by_ids(len: usize, kphis_extra: &str) -> String {
    let in_str = vec!["?"; len].join(",");
    [
        "SELECT path FROM ",kphis_extra,".image WHERE image_id IN (",&in_str,");"
    ].concat()
}

// // SELECT i.image_id,i.path,i.title,i.create_user,i.create_datetime,o.`name` AS create_username
// //   FROM kphis_extra.image i
// //   LEFT JOIN hosxp.opduser o ON o.loginname=i.create_user
// // WHERE i.create_user=? ORDER BY i.image_id;
// /// loginname
// pub fn select_image_by_loginname(hosxp: &str, kphis_extra: &str) -> String {
//     [
//         "SELECT i.image_id,i.path,i.title,i.create_user,i.create_datetime,o.`name` AS create_username \
//         FROM ",kphis_extra,".image i \
//           LEFT JOIN ",hosxp,".opduser o ON o.loginname=i.create_user \
//         WHERE i.create_user=? ORDER BY i.image_id;"
//     )
// }

// DELETE kphis_extra.image,kphis_extra.image_usage FROM kphis_extra.image
//   LEFT JOIN kphis_extra.image_usage ON image.image_id = image_usage.image_id
// WHERE image.image_id IN (?) AND image.create_user=?;
// *** cannot use alias in delete `https://bugs.mysql.com/bug.php?id=82189` ***
/// image_id x len, loginname
pub fn delete_image(len: usize, kphis_extra: &str) -> String {
    let in_str = vec!["?"; len].join(",");
    [
        "DELETE ",kphis_extra,".image,",kphis_extra,".image_usage FROM ",kphis_extra,".image \
            LEFT JOIN ",kphis_extra,".image_usage ON image.image_id = image_usage.image_id \
        WHERE image.image_id IN (",&in_str,") AND image.create_user=?;"
    ].concat()
}

// SELECT i.image_id,i.path,i.title,i.create_user,i.create_datetime,
//   u.image_usage_id,u.usage_id,u.usage_key_id,o.`name` AS create_username
// FROM kphis_extra.image AS i
//   LEFT JOIN kphis_extra.image_usage AS u ON u.image_id=i.image_id
//   LEFT JOIN hosxp.opduser o ON o.loginname=i.create_user
// WHERE u.usage_id=? AND u.usage_key_id=?
// ORDER BY u.image_usage_id;
/// usage_id, usage_key_id
pub fn select_image_usage_by_usage_id(hosxp: &str, kphis_extra: &str) -> String {
    [
        "SELECT i.image_id,i.path,i.title,i.create_datetime,\
          u.image_usage_id,u.usage_id,u.usage_key_id,o.`name` AS create_username \
        FROM ",kphis_extra,".image_usage AS u \
          LEFT JOIN ",kphis_extra,".image AS i ON i.image_id=u.image_id \
          LEFT JOIN ",hosxp,".opduser o ON o.loginname=i.create_user \
        WHERE u.usage_id=? AND u.usage_key_id=? \
        ORDER BY u.image_usage_id;"
    ].concat()
}

// INSERT IGNORE INTO kphis_extra.image_usage (usage_id,usage_key_id,image_id,create_user,create_datetime,update_user,update_datetime,version) VALUES (?,?,?,?,NOW(),?,NOW(),1);
/// (usage_id, usage_key_id, image_id, loginname, loginname) x len
pub fn insert_image_usage(len: usize, kphis_extra: &str) -> String {
    let where_str = vec!["(?,?,?,?,NOW(),?,NOW(),1)"; len].join(",");
    [
        "INSERT IGNORE INTO ",kphis_extra,".image_usage (usage_id,usage_key_id,image_id",TABLE_CREATE_COLUMNS,") VALUES ",&where_str,";"
    ].concat()
}

// DELETE FROM kphis_extra.image_usage WHERE image_usage_id IN (?) AND create_user=?;"
/// image_usage_id x len, loginname
pub fn delete_image_usage(len: usize, kphis_extra: &str) -> String {
    let in_str = vec!["?"; len].join(",");
    [
        "DELETE FROM ",kphis_extra,".image_usage WHERE image_usage_id IN (",&in_str,") AND create_user=?;"
    ].concat()
}