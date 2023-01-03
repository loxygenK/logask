use sea_orm::DbErr;

pub enum RepositoryReport<T> {
    Created(T),
    Read(T),
    Updated(T),
    Deleted,
}

#[derive(Debug, thiserror::Error)]
#[cfg_attr(release, error("error during loading/saving data to the database"))]
pub enum RepositoryError {
    #[cfg_attr(not(release), error("Error from sea_orm: {0}"))]
    InternalError(#[from] DbErr),

    #[cfg_attr(not(release), error("Validation failure"))]
    ValidationFailure,
}

pub type RepositoryResult<T> = Result<RepositoryReport<T>, RepositoryError>;
