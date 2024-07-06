use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().unique_key().not_null())      // 设置唯一值
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Firstname).string().null())
                    .col(ColumnDef::new(User::Lastname).string().null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),     // to own设置我们自己的字符串，DEFAULT CURRENT_TIMESTAMP默认设置当前时间戳
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum User {
    // User为数据库表名
    Table, // 标识为数据库表
    Id,    // 为字段名
    Email,
    Password,
    Firstname,
    Lastname,
    CreatedAt,
    UpdatedAt,
}
