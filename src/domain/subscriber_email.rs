use validator::validate_email;

#[derive(Debug)]
pub struct SubscriberEmail(String);
impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        let err_message = format!("{s} is wrong email");
        validate_email(&s).then_some(Self(s)).ok_or(err_message)
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SubscriberEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We just forward to the Display implementation of
        // the wrapped String.
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use fake::{faker::internet::en::SafeEmail, Fake};
    use proptest::prelude::*;
    use rand::{rngs::StdRng, SeedableRng};

    use super::SubscriberEmail;
    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert!(SubscriberEmail::parse(email).is_err());
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert!(SubscriberEmail::parse(email).is_err());
    }
    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        //let mail = SafeEmail().fake_with_rng::<String, _>(&mut StdRng::from_seed([1,9,2, 32]));

        assert!(SubscriberEmail::parse(email).is_err());
    }

    proptest! {
        #[test]
        fn valid_emails_are_parsed_successfully(
            email in prop::array::uniform32(1u8..).prop_map(|v| SafeEmail().fake_with_rng::<String, _>(&mut StdRng::from_seed(v)))
        ) {
            println!("{email}");
            prop_assert!(SubscriberEmail::parse(email).is_ok())
        }
    }
}
