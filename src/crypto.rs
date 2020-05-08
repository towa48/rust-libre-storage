use std::u8;
use base64::{encode,decode};
use rand::prelude::*;
use sha2::Sha512;
use hmac::{Hmac};
use std::str;
use std::borrow::Cow;

type HmacSha512 = Hmac<Sha512>;

const SALT_LEN: usize = 64;
const KEY_LEN: usize = 512;
const ITERATIONS: usize = 10000;

pub struct PasswordResult<'a> {
    pub password: Cow<'a, str>,
    pub salt: Cow<'a, str>,
}

impl<'a> PasswordResult<'a> {
   fn new<S>(password: S, salt: S) -> PasswordResult<'a>
       where S: Into<Cow<'a, str>>
   {
       PasswordResult { password: password.into(), salt: salt.into() }
   }
}

fn create_salt() -> Vec<u8> {
    let mut salt = [0u8; SALT_LEN];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut salt);

    // TODO: how to return owned slice?
    salt.iter().cloned().collect()
}

fn to_base64<T: AsRef<[u8]>>(value: T) -> String {
    encode(value)
}

fn from_base64(value: &str) -> Vec<u8> {
    decode(value).unwrap()
}

pub fn create_password_hash<'a>(password: &str) -> PasswordResult<'a> {
    let salt = create_salt();
    let password_hash = get_password_hash_internal(password, &salt);

    let salt_hash = to_base64(salt);
    PasswordResult::new(password_hash, salt_hash)
}

pub fn get_password_hash(password: &str, salt_hash: &str) -> String {
    get_password_hash_internal(password, &from_base64(salt_hash))
}

fn get_password_hash_internal(password: &str, salt: &[u8]) -> String {
    let mut derived_key = [0u8; KEY_LEN];
    pbkdf2::pbkdf2::<HmacSha512>(password.as_bytes(), salt, ITERATIONS, &mut derived_key);

    // TODO: how to convert array to AsRef<[u8]>?
    let key_vec: Vec<u8> = derived_key.iter().cloned().collect();

    to_base64(key_vec)
}