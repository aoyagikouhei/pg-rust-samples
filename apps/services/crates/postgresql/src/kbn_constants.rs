use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
// ユーザー
#[derive(Display, EnumString, Clone, PartialEq, Eq, Serialize, Deserialize, Debug, sqlx::Type, Default)]
#[sqlx(type_name = "type_enum_user", rename_all = "snake_case")]
pub enum UserKbn {
    #[default]
    Normal, // 一般
    Admin,  // 管理者
}
