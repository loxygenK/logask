use sea_orm::DbErr;

pub enum RepositoryReport<T> {
    Created(T),
    Read(T),
    Updated,
    Deleted,
}

#[derive(Debug, thiserror::Error)]
#[cfg_attr(release, error("error during loading/saving data to the database"))]
#[cfg_attr(
    not(release),
    error("error during loading/saving data to the database: {0}")
)]
pub struct RepositoryError(#[from] DbErr);

pub type RepositoryResult<T> = Result<RepositoryReport<T>, RepositoryError>;
