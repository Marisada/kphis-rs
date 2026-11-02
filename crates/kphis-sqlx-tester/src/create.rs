// Tool for create all KPHIS associated `CREATE TABLE` sql files from selected database
// by edit `source_url` of function `manual_dump_hosxp_table_schema` below and run a test
// output to `kphis-sqlx-tester/hosxp`
// copy to `kphis-sqlx-tester/test_sqls/create/hosxp` to use as a testing schema

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use sqlx::{AssertSqlSafe, MySql, Pool, Row};
    use std::{fs::File, io::Write};

    const TABLE_NAMES: [&str;84] = [
        "an_stat",
        "clinic",
        "dchstts",
        "dchtype",
        "doctor",
        "drug_interaction",
        "drugitems",
        "drugusage",
        "er_emergency_level",
        "er_emergency_type",
        "er_image",
        "er_nursing_detail",
        "er_pt_type",
        "er_regist",
        "hospcode",
        "icd_codemap",
        "icd9cm1",
        "icd101",
        "ipt",
        "ipt_newborn",
        "ipt_order_no",
        "iptadm",
        "iptdiag",
        "iptoprt",
        "lab_form",
        "lab_form_head",
        "lab_head",
        "lab_items",
        "lab_items_doctor",
        "lab_items_group",
        "lab_items_visible",
        "lab_order",
        "lab_order_image",
        "lab_order_service",
        "lab_specimen_items",
        "marrystatus",
        "medication_reconciliation",
        "medication_reconciliation_detail",
        "medpay_ipd",
        "medplan_ipd",
        "moph_refer_expire_type",
        "nationality",
        "nhso_inscl_code",
        "nondrugitems",
        "oapp",
        "occupation",
        "officer",
        "opd_allergy",
        "opdgroup",
        "opdscreen",
        "opduser",
        "operation_detail",
        "operation_item",
        "operation_list",
        "opitemrece",
        "ovst",
        "ovstdiag",
        "ovstist",
        "patient",
        "patient_image",
        "patient_opd_scan",
        "pe_image",
        "ptnote",
        "pttype",
        "refer_cause",
        "refer_point_list",
        "refer_type",
        "refer_vital_sign",
        "referout",
        "religion",
        "roomno",
        "s_drugitems",
        "serial",
        "sex",
        "sp_use",
        "spclty",
        "sys_var",
        "tambol",
        "thaiaddress",
        "vn_stat",
        "ward",
        "xray_items",
        "xray_items_group",
        "xray_report",
    ];

    // query result has 2 columns, `Table` and `CreateTable`
    fn show_table_sql(db_name: &str, table_name: &str) -> String {
        [
            "SHOW CREATE TABLE `", db_name, "`.`", table_name, "`;"
        ].concat()
    }

    async fn query_show_table(db_name: &str, table_name: &str, pool: &Pool<MySql>) -> sqlx::Result<String> {
        let sql = show_table_sql(db_name, table_name);
        let mut result = sqlx::query(AssertSqlSafe(sql))
            .fetch_one(pool)
            .await
            .map(|r| r.try_get::<String, usize>(1))??;
        result.insert(13, '.');
        result.insert(13, '`');
        result.insert_str(13, db_name);
        result.insert(13, '`');

        Ok(result)
    }

    fn write_to(bytes: &[u8], path: &str) {
        let mut file = File::create(path).unwrap_or_else(|e| {
            panic!("Error creating '{}': {}", path, e);
        });
        if let Err(e) = file.write_all(&bytes) {
            panic!("Error writing to '{}': {}", path, e);
        }
    }

    #[tokio::test]
    #[ignore]
    async fn manual_dump_hosxp_table_schema() {

        let source_url = "mysql://foo:bar@192.168.56.1:3306";

        std::fs::create_dir("hosxp").unwrap();
        let pool: Pool<MySql> = Pool::connect(source_url).await.expect("Error while connecting to database");
        for table in TABLE_NAMES {
            let target_path = ["hosxp/", table,".sql"].concat();
            let result = query_show_table("hos", table, &pool).await.unwrap();
            write_to(result.as_bytes(), &target_path);
        }
    }
}
