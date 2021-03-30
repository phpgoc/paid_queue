use clap::{App, AppSettings};
use cli_table::{print_stdout, Cell, Style, Table};

use microservice_rust_actix::tools::str::read_line_from_input;
use microservice_rust_actix::{const_lib::redis_key, databases::redis_pool::*};
use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;
use std::collections::HashMap;
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
            list_service_provider();
        }
        Some("add") => {
            add_service_provider();
        }
        Some("del") => {
            println!("del no implemented");
        }
        Some("reset") => {
            reset_service_provider();
        }
        _ => {} // If all subcommands are defined above, anything else is unreachable
    }
}
fn list_service_provider() {
    let pool = get_pool();
    let mut conn = pool.get().unwrap();
    let res: Vec<String> = conn.smembers(redis_key::SET_SERVICE_PROVIDER).unwrap();
    if res.is_empty() {
        println!("目前没有服务提供商");
        return;
    }
    let mut vec = vec![];
    for id in res {
        let res: HashMap<String, String> = conn
            .hgetall(format!("{}{}", redis_key::HASH_SERVICE_PROVIDER_PREFIX, id))
            .unwrap();
        vec.push(vec![
            res.get("id").unwrap_or(&String::new()).cell(),
            res.get("name").unwrap_or(&String::new()).cell(),
            res.get("description").unwrap_or(&String::new()).cell(),
            res.get("key").unwrap_or(&String::new()).cell(),
        ])
    }
    let table = vec
        .table()
        .title(vec![
            "id".cell().bold(true),
            "name".cell().bold(true),
            "description".cell().bold(true),
            "key".cell().bold(true),
        ])
        .bold(true);
    println!("服务器提供商list:");
    assert!(print_stdout(table).is_ok());
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
        if description.len() > 1000 {
            println!("{} 描述内容长度过长", description);
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

fn reset_service_provider() {
    let pool = get_pool();
    let mut conn = pool.get().unwrap();

    let id = loop {
        print!("请输入服务提供商的英文名称(如不清楚可通过 pq list 查询):");
        std::io::stdout().flush().unwrap();
        let id = read_line_from_input();
        if conn
            .sismember(redis_key::SET_SERVICE_PROVIDER, &id)
            .unwrap()
        {
            break id;
        }
        println!(" id不存在");
    };
    let hash_service_provider_key = format!("{}{}", redis_key::HASH_SERVICE_PROVIDER_PREFIX, id);
    let res: HashMap<String, String> = conn.hgetall(&hash_service_provider_key).unwrap();
    println!("name : {}", res.get("name").unwrap_or(&String::new()));
    print!("如修改请输入y :");
    std::io::stdout().flush().unwrap();
    let is_modify_name = read_line_from_input();
    if is_modify_name == "y" {
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
        let _: i32 = conn.hset(&hash_service_provider_key, "name", name).unwrap();
    }
    println!(
        "description : {}",
        res.get("description").unwrap_or(&String::new())
    );
    print!("如修改请输入y :");
    std::io::stdout().flush().unwrap();
    let is_modify_description = read_line_from_input();
    if is_modify_description == "y" {
        let description = loop {
            print!("请输入服务商描述信息，可以为空: ");
            std::io::stdout().flush().unwrap();
            let description = read_line_from_input();
            if description.len() > 1000 {
                println!("{} 描述内容长度过长", description);
                continue;
            }
            break description;
        };
        let _: i32 = conn
            .hset(&hash_service_provider_key, "description", description)
            .unwrap();
    }
    println!("key : {}", res.get("key").unwrap_or(&String::new()));
    print!("是否重置秘钥，重置修改输入y :");
    std::io::stdout().flush().unwrap();
    let is_reset_key = read_line_from_input();
    if is_reset_key == "y" {
        let key: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();
        let _: i32 = conn.hset(&hash_service_provider_key, "key", &key).unwrap();
        println!("新的秘钥是 : {}", key);
    }
    println!("修改完成");
}
