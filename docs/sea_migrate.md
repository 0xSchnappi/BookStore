# sea migrate
## install sea
```shell
cargo install sea-orm-cli
```
## 创建迁移目录
```shell
sea migrate init -d ./src/migrator
```
为了速度我们可以删除目录下的Cargo.toml和README.md文件
生成author表和book表
```shell
sea migrate generate -d ./src/migrator create_author_table
sea migrate generate -d ./src/migrator create_book_table 
```
生成数据库实例
```shell
```