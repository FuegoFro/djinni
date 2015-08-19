use mopa;

pub trait UserToken: mopa::Any {
    fn whoami(&self) -> String;
}

mopafy!(UserToken);