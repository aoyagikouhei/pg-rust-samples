use uuid::Uuid;

use crate::prelude::*;

use crate::Pool;

pub mod companies;
pub mod users;

pub async fn make_companies(
    pool: &Pool,
    builder: &mut companies::CompaniesBuilder,
) -> Result<Companies, sqlx::Error> {
    if builder.uuid.is_none() {
        builder.uuid(Uuid::new_v4());
    }
    if builder.company_name.is_none() {
        builder.company_name("UV");
    }
    builder.build().unwrap().insert(pool).await
}

pub async fn make_users(
    pool: &Pool,
    builder: &mut users::UsersBuilder,
) -> Result<Users, sqlx::Error> {
    if builder.uuid.is_none() {
        builder.uuid(Uuid::new_v4());
    }
    if builder.user_name.is_none() {
        builder.user_name("taro");
    }
    if builder.user_mail.is_none() {
        builder.user_mail(format!(
            "{}@example.com",
            builder.user_name.as_ref().unwrap()
        ));
    }
    builder.build().unwrap().insert(pool).await
}

pub async fn make_company_users(
    pool: &Pool,
    company_builder: &mut companies::CompaniesBuilder,
    user_builder: &mut users::UsersBuilder,
) -> Result<(Companies, Users), sqlx::Error> {
    let company = make_companies(pool, company_builder).await?;
    if user_builder.company_uuid.is_none() {
        user_builder.company_uuid(company.uuid);
    }
    let user = make_users(pool, user_builder).await?;
    Ok((company, user))
}
