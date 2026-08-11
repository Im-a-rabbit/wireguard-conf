#![cfg(feature = "amneziawg-2")]

use wireguard_conf::prelude::*;

#[test]
fn random() {
    let settings = AmneziaSettings::random();

    assert!(settings.validate().is_ok());
}

#[test]
fn validate_jc() {
    let mut settings = AmneziaSettings::random();

    settings.jc = Some(11);

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jc".to_string()))
    );
}

#[test]
fn validate_jmin() {
    let mut settings = AmneziaSettings::random();

    settings.jmin = Some(63);
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jmin".to_string()))
    );

    settings.jmin = Some(1025);
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jmin".to_string()))
    );
}

#[test]
fn validate_jmax() {
    let mut settings = AmneziaSettings::random();

    settings.jmax = Some(63);
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jmax".to_string()))
    );

    settings.jmax = Some(1025);
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jmax".to_string()))
    );
}

#[test]
fn validate_jmin_jmax() {
    let mut settings = AmneziaSettings::random();

    settings.jmin = Some(100);
    settings.jmax = Some(64);

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting(
            "Jmin >= Jmax".to_string()
        ))
    );
}

#[test]
fn validate_s1() {
    let mut settings = AmneziaSettings::random();

    settings.s1 = Some(65);
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S1".to_string()))
    );
}

#[test]
fn validate_s2() {
    let mut settings = AmneziaSettings::random();

    settings.s2 = Some(65);
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S2".to_string()))
    );
}

#[test]
fn validate_s3() {
    let mut settings = AmneziaSettings::random();

    settings.s3 = Some(65);
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S3".to_string()))
    );
}

#[test]
fn validate_s4() {
    let mut settings = AmneziaSettings::random();

    settings.s4 = Some(33);
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S4".to_string()))
    );
}

#[test]
fn validate_h1() {
    let mut settings = AmneziaSettings::builder().build().unwrap();

    settings.h1 = Some(HRange::new(200, 100));
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("H1".to_string()))
    );
}

#[test]
fn validate_h2() {
    let mut settings = AmneziaSettings::builder().build().unwrap();

    settings.h2 = Some(HRange::new(200, 100));
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("H2".to_string()))
    );
}

#[test]
fn validate_h3() {
    let mut settings = AmneziaSettings::builder().build().unwrap();

    settings.h3 = Some(HRange::new(200, 100));
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("H3".to_string()))
    );
}

#[test]
fn validate_h4() {
    let mut settings = AmneziaSettings::builder().build().unwrap();

    settings.h4 = Some(HRange::new(200, 100));
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("H4".to_string()))
    );
}

#[test]
fn validate_h_overlap() {
    let mut settings = AmneziaSettings::random();

    settings.h1 = Some(HRange::new(1000, 2000));
    settings.h2 = Some(HRange::new(1500, 5000));
    settings.h3 = Some(HRange::new(5000, 7000));
    settings.h4 = Some(HRange::new(8000, 9000));
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting(
            "H ranges overlap".to_string()
        ))
    );
}
