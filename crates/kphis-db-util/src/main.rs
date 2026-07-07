use clap::Parser;
use sqlx::{MySql, Pool};

use kphis_api_core::utils::{get_config, get_db};
use kphis_api_query::{schema_update, transform::trigger};

// Command line management
#[derive(Clone, Debug, Parser)]
#[clap(author, version, about, long_about = None,
    rename_all = "kebab-case",
    rename_all_env = "screaming-snake")]
#[command(about = "Database utilities for KPHIS", long_about = None)]
struct Args {
    /// Set config environment by config file name Ex. /volume/config/debug.toml -> debug
    #[arg(value_enum, value_parser, default_value = "debug")]
    mode: String,
    /// Run DROP IF EXISTS and CREATE of all KPHIS's triggers and stored procedures
    #[arg(short, long)]
    trigger: bool,
    /// Run SCHEMA update for all KPHIS's database after v24.01
    #[arg(short, long)]
    schema: bool,
}

#[tokio::main]
async fn main() {
    // parse Clap Arguments
    let args = Args::parse();
    let Args { mode, trigger, schema } = args;

    if trigger || schema {
        // get command line argument, load config
        let config = get_config(&mode);
        let hosxp_dbname = config.get_string("hosxp-dbname").expect("'hosxp-dbname' not found in config file");
        let kphis_dbname = config.get_string("kphis-dbname").expect("'kphis-dbname' not found in config file");
        let kphis_log_dbname = config.get_string("kphis-log-dbname").expect("'kphis-log-dbname' not found in config file");
        let kphis_extra_dbname = config.get_string("kphis-extra-dbname").expect("'kphis-extra-dbname' not found in config file");

        // init pool
        let db_pool = get_db(&config).await;

        if trigger {
            update_stored_procedures(&db_pool, &kphis_dbname, &kphis_extra_dbname).await;
            update_triggers(&db_pool, &hosxp_dbname, &kphis_dbname, &kphis_log_dbname).await;
        }
        if schema {
            update_schemas(&db_pool, &kphis_dbname, &kphis_log_dbname, &kphis_extra_dbname).await;
        }
        println!("DONE.");
    } else {
        println!("No flag defined");
        println!("For more information, try '--help'");
    }
}

async fn update_stored_procedures(pool: &Pool<MySql>, kphis: &str, kphis_extra: &str) {
    println!("Update KPHIS Stored Procedure : proc_count_all_an");
    let _ = trigger::add_any_an_exists_procedure(pool, kphis, kphis_extra)
        .await
        .expect("Failed to create Stored Procedure `proc_count_all_an`");
    println!("Update KPHIS Stored Procedure : proc_update_all_an");
    let _ = trigger::add_update_all_an_procedure(pool, kphis, kphis_extra)
        .await
        .expect("Failed to create Stored Procedure `proc_update_all_an`");
}

async fn update_triggers(pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_log: &str) {
    println!("Update KPHIS Trigger : trg_ipt_log_insert");
    let _ = trigger::add_ipt_log_insert_trigger(pool, kphis, kphis_log)
        .await
        .expect("Failed to create Trigger on ipt_log INSERT `trg_ipt_log_insert`");
    println!("Update HOSxP Trigger : trg_kphis_ipt_log_insert");
    let _ = trigger::add_ipt_insert_trigger(pool, hosxp, kphis_log)
        .await
        .expect("Failed to create Trigger on HOSxP's ipt `trg_kphis_ipt_log_insert`");
    println!("Update HOSxP Trigger : trg_kphis_ipt_log_delete");
    let _ = trigger::add_ipt_delete_trigger(pool, hosxp, kphis_log)
        .await
        .expect("Failed to create Trigger on HOSxP's ipt `trg_kphis_ipt_log_delete`");
}

async fn update_schemas(pool: &Pool<MySql>, kphis: &str, kphis_log: &str, kphis_extra: &str) {
    println!("Update {} database schema", kphis);
    let _ = schema_update::update_kphis(pool, kphis).await.expect("Error update schema");
    println!("Update {} database schema", kphis_log);
    let _ = schema_update::update_kphis_log(pool, kphis_log).await.expect("Error update schema");
    println!("Update {} database schema", kphis_extra);
    let _ = schema_update::update_kphis_extra(pool, kphis_extra).await.expect("Error update schema");
}
