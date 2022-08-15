//#![deny(clippy::all)]

#[cfg(test)]
use mockall::{automock, predicate::*};

static mut CALL_COUNTER: usize = 0;

#[cfg_attr(test, automock)]
pub trait SecretSequence {
    fn next_number(&self) -> u32;
}

pub struct Dummy;

impl Dummy {
    pub fn next_number(&self) -> u32 {
        42
    }
}

impl SecretSequence for Dummy {
    fn next_number(&self) -> u32 {
        unsafe {
            CALL_COUNTER += 1;
            match CALL_COUNTER {
                1 => 2,
                2 => 3,
                3 => 5,
                4 => 7,
                5 => 23,
                6 => 42,
                _ => 0,
            }
        }
    }
}

pub fn next_number_user(sec: &dyn SecretSequence) -> u32 {
    sec.next_number();
    sec.next_number();
    sec.next_number()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_next_number() {
        let dummy = Dummy;
        let secret_seq_dummy = &dummy as &dyn SecretSequence;
        assert_eq!(secret_seq_dummy.next_number(), 2);
        assert_eq!(secret_seq_dummy.next_number(), 3);
        assert_eq!(secret_seq_dummy.next_number(), 5);
        assert_eq!(secret_seq_dummy.next_number(), 7);
        assert_eq!(secret_seq_dummy.next_number(), 23);
        assert_eq!(secret_seq_dummy.next_number(), 42);
        assert_eq!(secret_seq_dummy.next_number(), 0);
        assert_eq!(secret_seq_dummy.next_number(), 0);
    }

    #[test]
    #[serial]
    fn test_next_number_user() {
        let mut mock = MockSecretSequence::new();
        mock.expect_next_number().times(3).returning(|| 32);

        assert_eq!(32, next_number_user(&mock));
    }
}
