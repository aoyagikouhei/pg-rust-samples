use serde::{Serialize, Deserialize};
use strum::{Display, EnumString};
// ユーザー
#[derive(Display, EnumString, Clone, PartialEq, Eq, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "type_enum_user", rename_all = "snake_case")]
pub enum UserKbn {
    Normal, // 一般
    Admin, // 管理者
}

impl Default for UserKbn {
    fn default() -> Self {
        Self::Normal
    }
}

