pub trait UserToken {
    fn whoami(&self) -> String;
}