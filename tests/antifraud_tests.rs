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
    assert!(cfg.micro_drift);
    assert!(cfg.micro_rotation);
    assert!(cfg.color_jitter);
    assert!(cfg.ultrasonic_bed);
}

#[test]
fn test_antifraud_generation_bounds() {
    let cfg = AntiFraudConfig::default();
    for _ in 0..100 {
        let instance = cfg.generate_instance();
        assert!(instance.speed_factor >= cfg.min_speed && instance.speed_factor <= cfg.max_speed);
        assert!(instance.zoom_factor >= cfg.min_zoom && instance.zoom_factor <= cfg.max_zoom);
        assert!(instance.pitch_factor >= 0.98 && instance.pitch_factor <= 1.02);
        assert!(instance.rotation_angle >= 0.2 && instance.rotation_angle <= 0.5);
        assert!(instance.contrast >= 1.01 && instance.contrast <= 1.05);
        assert!(instance.strip_metadata);
        assert!(instance.add_grain);
        assert!(instance.micro_drift);
        assert!(instance.ultrasonic_bed);
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
    assert_eq!(instance.rotation_angle, 0.0);
    assert_eq!(instance.contrast, 1.0);
    assert!(!instance.strip_metadata);
    assert!(!instance.add_grain);
    assert!(!instance.mirror_horizontal);
    assert!(!instance.micro_drift);
    assert!(!instance.ultrasonic_bed);
}
