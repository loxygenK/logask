use sea_orm::DbErr;

#[derive(Debug, thiserror::Error)]
#[cfg_attr(release, error("error during loading/saving data to the database"))]
#[cfg_attr(
    not(release),
    error("error during loading/saving data to the database: {0}")
)]
pub struct RepositoryError(#[from] DbErr);

pub type RepositoryResult<T> = Result<T, RepositoryError>;
