pub struct User {}

impl User {
    // zero-initialize the objet before boot
    const fn null() -> Self {
        Self {}
    }
}

#[allow(non_upper_case_globals)]
pub static mut u: User = User::null();
