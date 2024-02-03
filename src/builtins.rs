use rand::Rng;

#[macro_export]
macro_rules! fire {
    ($($arg:expr),*) => {
        $(println!("{}", $arg))*;
    }
}

pub struct Builtin {}

pub fn load() {}

pub fn random(min: isize, max: isize) -> isize {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(min..=max);
    random_number
}