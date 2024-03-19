/// Example at:
/// https://gist.github.com/bsodmike/46fc344fa8d5fcb6e30f544bd4409df5
use anyhow::{Context, Error, Result};
use hmac::{digest::core_api::CoreWrapper, EagerHash, Hmac, HmacCore, KeyInit};
use log::trace;
use pbkdf2::pbkdf2;
use sha2::Sha512;

type PrfHasher = Sha512;

const PBKDF_ROUNDS: u32 = 600_000;
const KEY_BUFF_SIZE: usize = 20;

pub fn runner() -> Result<()> {
    example1::run()?;

    Ok(())
}

pub mod aes {
    use aes::cipher;
    use aes::cipher::generic_array::GenericArray;
    use aes_gcm_siv::aead::Buffer;
    use aes_gcm_siv::AesGcmSiv;
    use aes_gcm_siv::{
        aead::{AeadInPlace, KeyInit, OsRng},
        Aes256GcmSiv, Nonce,
    };
    use std::io::Read;
    use std::marker::PhantomData;

    #[derive(Debug)]
    /// FIXME: Allow swiching out the `A` array type.
    pub(crate) struct AesVecBuffer<'a, A> {
        inner: Vec<u8>,
        _life: PhantomData<&'a A>,
    }

    impl<'a, A> AesVecBuffer<'a, A> {
        pub fn inner(&mut self) -> &mut Vec<u8> {
            &mut self.inner
        }
    }

    impl<'a, A> aes_gcm_siv::aead::Buffer for AesVecBuffer<'a, A> {
        fn extend_from_slice(&mut self, other: &[u8]) -> aes_gcm_siv::aead::Result<()> {
            Ok(self.inner.extend(other))
        }

        fn truncate(&mut self, len: usize) {
            self.inner.truncate(len)
        }

        fn len(&self) -> usize {
            self.as_ref().len()
        }

        fn is_empty(&self) -> bool {
            self.as_ref().is_empty()
        }
    }

    impl<'a, A> AsRef<[u8]> for AesVecBuffer<'a, A> {
        fn as_ref(&self) -> &[u8] {
            &self.inner
        }
    }

    impl<'a, A> AsMut<[u8]> for AesVecBuffer<'a, A> {
        fn as_mut(&mut self) -> &mut [u8] {
            &mut self.inner[..]
        }
    }

    impl<'a, A, const N: usize> PartialEq<[u8; N]> for AesVecBuffer<'a, A> {
        fn eq(&self, other: &[u8; N]) -> bool {
            self.inner.eq(other)
        }

        fn ne(&self, other: &[u8; N]) -> bool {
            !self.eq(other)
        }
    }

    pub(crate) struct AesEncrypter<'a> {
        cipher: AesGcmSiv<aes::Aes256>,
        nonce: String,
        buffer: AesVecBuffer<'a, ()>,
    }

    impl<'a> AesEncrypter<'a> {
        pub fn new(nonce: String, plaintext: &'a str) -> Self {
            let key = Aes256GcmSiv::generate_key(&mut OsRng);
            let cipher = Aes256GcmSiv::new(&key);

            // Note: buffer needs 16-bytes overhead for auth tag tag
            let inner: heapless::Vec<u8, 128> = heapless::Vec::new();
            // let inner: Vec<u8> = Vec::new();
            let mut buffer = AesVecBuffer::<()> {
                inner: inner.to_vec(),
                _life: PhantomData,
            };
            buffer.extend_from_slice(plaintext.as_bytes()).unwrap();

            Self {
                cipher,
                nonce,
                buffer,
            }
        }

        #[allow(dead_code)]
        pub fn buffer(&mut self) -> &mut AesVecBuffer<'a, ()> {
            &mut self.buffer
        }

        pub fn encrypt_in_place(&mut self) -> anyhow::Result<()> {
            let mut bytes = self.nonce.as_bytes();
            let mut short_nonce = [0u8; 12];
            bytes.read_exact(&mut short_nonce)?;
            // trace!("Len: {:?}", short_nonce.len());
            let nonce: &GenericArray<u8, cipher::consts::U12> = Nonce::from_slice(&short_nonce[..]); // 96-bits; unique per message

            // Encrypt `buffer` in-place, replacing the plaintext contents with ciphertext
            Ok(self
                .cipher
                .encrypt_in_place(nonce, b"", &mut self.buffer)
                .expect("Encrypt cipher in place"))
        }

        pub fn decrypt_in_place(&mut self) -> anyhow::Result<()> {
            let mut bytes = self.nonce.as_bytes();
            let mut short_nonce = [0u8; 12];
            bytes.read_exact(&mut short_nonce)?;

            let nonce: &GenericArray<u8, cipher::consts::U12> = Nonce::from_slice(&short_nonce[..]); // 96-bits; unique per message

            // Encrypt `buffer` in-place, replacing the plaintext contents with ciphertext
            Ok(self
                .cipher
                .decrypt_in_place(nonce, b"", &mut self.buffer)
                .expect("Decrypt cipher in place"))
        }
    }
}

use crate::challenge1::aes::AesEncrypter;
use crate::challenge1::encrypter::{Encryptable, Encrypter};

pub(crate) struct EncrypterState<'a>(pub(crate) &'a str, pub(crate) &'a str);

impl<'a> EncrypterState<'a> {
    pub(crate) fn new(password: &'a str, salt: &'a str) -> Self {
        Self(password, salt)
    }
}

pub(crate) fn get_encrypter<'a>(state: EncrypterState<'a>, plaintext: &'a str) -> AesEncrypter<'a> {
    // Create pbkdf
    let buf = [0u8; 20];
    let mut buf_boxed = Box::new(buf);
    let mut encrypter = Encrypter::<()>::new(&mut buf_boxed);
    let pbkdf_key = encrypter.pbkdf_key(state.0, state.1);
    let pbkdf_key_hex = hex::encode(pbkdf_key);
    trace!("Key: {}", &pbkdf_key_hex);

    AesEncrypter::new(pbkdf_key_hex.clone(), plaintext)
}

fn process_pbkdf_key<H>(
    buf_ptr: &mut Box<[u8; KEY_BUFF_SIZE]>,
    password: &str,
    salt: &str, // fmt
) -> anyhow::Result<()>
where
    CoreWrapper<HmacCore<H>>: KeyInit,
    H: hmac::EagerHash,
    <H as EagerHash>::Core: Sync,
{
    let buf = buf_ptr.as_mut();

    pbkdf2::<Hmac<H>>(
        &password.to_string().as_bytes(),
        &salt.to_string().as_bytes(),
        PBKDF_ROUNDS,
        buf,
        // fmt
    )
    .context("HMAC can be initialized with any key length")?;

    Ok(())
}

pub mod encrypter {
    use super::*;
    use std::marker::PhantomData;

    #[derive(Debug)]
    pub(crate) struct Encrypter<'a, S> {
        key: &'a mut Box<[u8; KEY_BUFF_SIZE]>,
        _phat: PhantomData<&'a S>,
    }

    impl<'a, S> Encrypter<'a, S> {
        pub fn new(buf: &'a mut Box<[u8; KEY_BUFF_SIZE]>) -> Self {
            Self {
                key: buf,
                _phat: PhantomData,
            }
        }
    }

    pub trait Encryptable {
        type KeyBuf;

        fn pbkdf_key(&mut self, password: &str, salt: &str) -> Self::KeyBuf;
    }

    impl<T> Encryptable for Encrypter<'_, T> {
        type KeyBuf = [u8; KEY_BUFF_SIZE];

        fn pbkdf_key(&mut self, password: &str, salt: &str) -> Self::KeyBuf {
            process_pbkdf_key::<PrfHasher>(&mut self.key, password, salt).unwrap();

            **self.key
        }
    }
}

pub mod example1 {
    use super::*;

    pub fn run() -> Result<()> {
        let mut enc = get_encrypter(EncrypterState::new("password", "salt"), "plaintext message");
        enc.encrypt_in_place().unwrap();
        // `buffer` now contains the message ciphertext
        trace!("Encrypted cipher text: {}", hex::encode(&enc.buffer()));
        assert_ne!(enc.buffer(), b"plaintext message");

        // Decrypt `buffer` in-place, replacing its ciphertext context with the original plaintext
        enc.decrypt_in_place().unwrap();
        let m = enc.buffer().inner();
        trace!(
            "Decrypted plaintext: {}",
            String::from_utf8(m.to_vec()).unwrap()
        );
        assert_eq!(enc.buffer().as_ref(), b"plaintext message");

        Ok(())
    }
}
