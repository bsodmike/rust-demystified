#![allow(unused_variables)]

use anyhow::Error;
use std::future::Future;

#[derive(Clone)]
pub struct PostgresDatabase {
    ignored_users_repository: PostgresIgnoredUsersRepository,
}

#[derive(Clone)]
struct PostgresIgnoredUsersRepository {
    pool: (),
}

pub struct IgnoredUser;

#[derive(Debug, thiserror::Error)]
pub enum IgnoredUsersRepositoryError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

pub trait IgnoredUsersRepository: Send + Sync {
    fn create(
        &self,
        ignored_user_create: IgnoredUser,
    ) -> impl Future<Output = Result<(), IgnoredUsersRepositoryError>> + Send;
}

pub struct Dependencies<'a, R> {
    pub repository: &'a R,
}

pub struct Request;
pub struct Response;

pub struct AddIgnoredUser<'a, R> {
    repository: &'a R,
}

impl<'a, R> AddIgnoredUser<'a, R>
where
    R: IgnoredUsersRepository,
{
    pub fn new(dependencies: Dependencies<'a, R>) -> Self {
        Self {
            repository: dependencies.repository,
        }
    }

    pub async fn exec(&self, request: Request) -> Result<Response, Error> {
        // NOTE: this call works because the type of `self.repository` is `&R` and `R` implements `IgnoredUsersRepository`
        IgnoredUsersRepository::create(self.repository, IgnoredUser {}).await?;

        Ok(Response)
    }
}

impl IgnoredUsersRepository for PostgresIgnoredUsersRepository {
    async fn create(
        &self,
        ignored_user_create: IgnoredUser,
    ) -> Result<(), IgnoredUsersRepositoryError> {
        // Perform the actual database operation here
        println!("Creating ignored user");

        Ok(())
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let repository = PostgresIgnoredUsersRepository { pool: () };

    // This abstracts the use case from the DB operations that run on the repository
    let dependency = Dependencies {
        repository: &repository,
    };
    let use_case = AddIgnoredUser::new(dependency);

    use_case.exec(Request {}).await?;
    println!("Performed use case");

    Ok(())
}
