use std::u8;
use base64::encode;
use rand::prelude::*;
use sha2::Sha512;
use hmac::{Hmac};
use std::str;

type HmacSha512 = Hmac<Sha512>;

const SALT_LEN: usize = 64;
const KEY_LEN: usize = 512;
const ITERATIONS: usize = 10000;

pub struct PasswordResult<'b> {
    pub password: &'b str,
    pub salt: &'b str,
}

impl<'b> PasswordResult<'b> {
    fn new<'a>(password: &'a str, salt: &'a str) -> PasswordResult<'a> {
        PasswordResult { password: password, salt: salt }
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

#[inline]
fn to_owned_str<'a>(value: String) -> &'a str {
    let bytes = &value.as_bytes();
    unsafe { str::from_utf8_unchecked(bytes.to_owned()) }
}

pub fn create_password_hash<'a>(password: &str) -> PasswordResult<'a> {
    let salt = create_salt();
    let mut derived_key = [0u8; KEY_LEN];
    pbkdf2::pbkdf2::<HmacSha512>(password.as_bytes(), &salt, ITERATIONS, &mut derived_key);

    // TODO: how to convert array to AsRef<[u8]>?
    let keyVec: Vec<u8> = derived_key.iter().cloned().collect();

    let password_hash = to_base64(keyVec).to_owned();
    let salt_hash = to_base64(salt);

    // trying to copy String value
    let x = to_owned_str::<'a>(password_hash);
    let y = to_owned_str::<'a>(salt_hash);

    //let xv = password_hash.to_owned().as_bytes(); 
    //let yv = salt_hash.as_bytes();
    //let x: &'a str = unsafe { str::from_utf8_unchecked(&xv.to_owned()) };
    //let y: &'a str = unsafe { str::from_utf8_unchecked(&yv.to_owned()) };

    PasswordResult::new(x, y)
}
