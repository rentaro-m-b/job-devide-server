pub enum AuthError {
    DieselError(diesel::result::Error),
    PasswordHashError(argon2::password_hash::Error)
}

impl From<diesel::result::Error> for AuthError {
    fn from(err: diesel::result::Error) -> Self {
        AuthError::DieselError(err)
    }
}

impl From<argon2::password_hash::Error> for AuthError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AuthError::PasswordHashError(err)
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
