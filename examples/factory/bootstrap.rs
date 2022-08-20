use syrette::ptr::TransientPtr;
use syrette::DIContainer;

// Interfaces
use crate::interfaces::user::{IUser, IUserFactory};
use crate::interfaces::user_manager::IUserManager;
//
// Concrete implementations
use crate::user::User;
use crate::user_manager::UserManager;

pub fn bootstrap() -> DIContainer
{
    let mut di_container: DIContainer = DIContainer::new();

    di_container
        .bind::<dyn IUserManager>()
        .to::<UserManager>()
        .unwrap();

    di_container
        .bind::<IUserFactory>()
        .to_factory(&|name, date_of_birth, password| {
            let user: TransientPtr<dyn IUser> =
                TransientPtr::new(User::new(name, date_of_birth, password));

            user
        })
        .unwrap();

    di_container
}
