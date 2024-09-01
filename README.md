# 从数据库生成模型代码
[sql_reverse](https://github.com/ptechen/sql_reverse)

[Rbatis](https://rbatis.github.io/rbatis.io/)

## install
cargo install sql_reverse
## reverse sample
sql_reverse mysql -f templates/reverse.yml -p 'templates/*' -n rbatis.tera -c templates/mysql_rbatis.json

## my reverse
sh reverse.sh

## Template Struct:
```rust
#[derive(Serialize)]
pub struct Template {
    pub table_name: String,
    pub struct_name: String,
    pub fields: Vec<Field>, 
    pub comment: String,
}

#[derive(Serialize, Clone)]
pub struct Field {
    pub field_name: String,
    pub field_type: String,
    pub comment: String,
    /// NO, YES
    pub is_null: String,
}

```
// NaiveDateTime -> DateTime
