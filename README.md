# 从数据库生成代码, 包括模型, Axum请求, 增删改查 
[sql_reverse](https://github.com/ptechen/sql_reverse)

[Rbatis](https://rbatis.github.io/rbatis.io/)

## install
cargo install sql_reverse
## reverse sample
sql_reverse mysql -f templates/reverse.yml -p 'templates/*' -n rbatis.tera -c templates/mysql_rbatis.json

## 使用python命名文件名
python3 reverse.py

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
// 修改了映射 NaiveDateTime -> DateTime

{% -%} // 带-表示紧靠的代码块
{% %} // 不带-表示换行的代码块

