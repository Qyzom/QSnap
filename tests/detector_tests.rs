use qsnap::media::detector::SceneDetector;

#[test]
fn test_clip_range_calculation_simple() {
    let scenes = vec![12.0, 24.5, 43.8, 62.1, 91.0];
    let ranges = SceneDetector::calculate_clip_ranges(100.0, 30.0, 3, &scenes);
    assert!(!ranges.is_empty());
    assert_eq!(ranges[0].0, 0.0);
    // Should snap near 30s scene (none close) or ideal 30s
    assert!(ranges[0].1 > 20.0 && ranges[0].1 <= 35.0);
}

#[test]
fn test_clip_range_calculation_with_snapping() {
    // Scene change right at 29.5s when clip_duration is 30.0s -> should snap to 29.5
    let scenes = vec![29.5, 60.2];
    let ranges = SceneDetector::calculate_clip_ranges(100.0, 30.0, 2, &scenes);
    assert_eq!(ranges[0].0, 0.0);
    assert_eq!(ranges[0].1, 29.5);
}
