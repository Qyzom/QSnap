use qsnap::antifraud::AntiFraudConfig;

#[test]
fn test_antifraud_defaults() {
    let cfg = AntiFraudConfig::default();
    assert!(cfg.enabled);
    assert!(cfg.speed_shift);
    assert!(cfg.pitch_shift);
    assert!(cfg.micro_zoom);
    assert!(cfg.add_grain);
    assert!(cfg.strip_metadata);
}

#[test]
fn test_antifraud_generation_bounds() {
    let cfg = AntiFraudConfig::default();
    for _ in 0..100 {
        let instance = cfg.generate_instance();
        assert!(instance.speed_factor >= cfg.min_speed && instance.speed_factor <= cfg.max_speed);
        assert!(instance.zoom_factor >= cfg.min_zoom && instance.zoom_factor <= cfg.max_zoom);
        assert!(instance.pitch_factor >= 0.98 && instance.pitch_factor <= 1.02);
        assert!(instance.strip_metadata);
        assert!(instance.add_grain);
    }
}

#[test]
fn test_antifraud_disabled() {
    let mut cfg = AntiFraudConfig::default();
    cfg.enabled = false;
    let instance = cfg.generate_instance();
    assert_eq!(instance.speed_factor, 1.0);
    assert_eq!(instance.zoom_factor, 1.0);
    assert_eq!(instance.pitch_factor, 1.0);
    assert!(!instance.strip_metadata);
    assert!(!instance.add_grain);
}
