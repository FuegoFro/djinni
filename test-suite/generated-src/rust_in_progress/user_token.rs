// AUTOGENERATED FILE - DO NOT MODIFY!
// This file generated by Djinni from user_token.djinni

use mopa;

pub trait UserToken: mopa::Any {
    fn whoami(&self) -> String;
}

mopafy!(UserToken);
