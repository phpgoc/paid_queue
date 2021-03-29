#[derive(cli_table::Table)]
struct ServiceProvider {
    #[table(title = "ID 英文名称", justify = "cli_table::format::Justify::Right")]
    id: String,
    #[table(title = "提供商名称")]
    name: String,
    #[table(title = "描述")]
    description: String,
    #[table(title = "秘钥")]
    key: String,
}
