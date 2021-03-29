use clap::{App, AppSettings};
use cli_table::{ print_stdout, Cell, Style, Table};
use microservice_rust_actix::tools::str::read_line_from_input;
use microservice_rust_actix::{const_lib::redis_key, databases::redis_pool::*};
use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;
use std::io::Write;
use std::ops::DerefMut;

fn main() {
    let matches = App::new("paid_queue command")
        .about("服务提供商")
        .version("0.1.0")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("zxq <phpgoc@icloud.com>")
        .subcommand(App::new("list").about("服务提供商列表"))
        .subcommand(App::new("del").about("删除提供商列表"))
        .subcommand(App::new("add").about("添加提供商列表"))
        .subcommand(App::new("reset").about("重置提供商列表key"))
        .get_matches();
    match matches.subcommand_name() {
        Some("list") => {
            println!("list");
        }
        Some("add") => {
            add_service_provider();
        }
        Some("del") => {
            println!("del");
        }
        Some("reset") => {
            println!("reset");
        }
        _ => {} // If all subcommands are defined above, anything else is unreachable
    }
}

fn add_service_provider() {
    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    let re = Regex::new(r"^[A-Za-z_]{4,20}$").unwrap();
    let id = loop {
        print!("请输入id  只能用英文和下划线: ");
        std::io::stdout().flush().unwrap();
        let id = read_line_from_input();
        if !re.is_match(&id) {
            println!("{} 不合法，只能使用英文和下划线,4-20位", id);
            continue;
        }
        //验证id重复
        if conn
            .sismember(redis_key::SET_SERVICE_PROVIDER, &id)
            .unwrap()
        {
            println!("{} id已存在，请重新输入", id);
            continue;
        }
        break id;
    };

    let name = loop {
        print!("请输入name: ");
        std::io::stdout().flush().unwrap();
        let name = read_line_from_input();
        if name.len() == 0 || name.len() > 200 {
            println!("{} 长度过长", name);
            continue;
        }
        break name;
    };
    //name可以重复，不用验证
    let description = loop {
        print!("请输入服务商描述信息，可以为空: ");
        std::io::stdout().flush().unwrap();
        let description = read_line_from_input();
        if name.len() == 0 || name.len() > 1000 {
            println!("{} 长度过长", description);
            continue;
        }
        break description;
    };
    let key: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();
    redis::cmd("sadd")
        .arg(redis_key::SET_SERVICE_PROVIDER)
        .arg(&id)
        .query::<i64>(conn.deref_mut())
        .unwrap();
    redis::cmd("hmset")
        .arg(format!("{}{}", redis_key::HASH_SERVICE_PROVIDER_PREFIX, id))
        .arg("id")
        .arg(&id)
        .arg("name")
        .arg(&name)
        .arg("description")
        .arg(&description)
        .arg("key")
        .arg(&key)
        .query::<String>(conn.deref_mut())
        .unwrap();

    let table = vec![vec![id.cell(), name.cell(), description.cell(), key.cell()]]
        .table()
        .title(vec![
            "id".cell().bold(true),
            "name".cell().bold(true),
            "description".cell().bold(true),
            "key".cell().bold(true),
        ])
        .bold(true);
    println!("服务器提供商创建成功");
    assert!(print_stdout(table).is_ok());
}
