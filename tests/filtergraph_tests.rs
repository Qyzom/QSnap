use qsnap::antifraud::AntiFraudConfig;
use qsnap::render::filtergraph::FilterGraphBuilder;
use qsnap::templates::schema::LayoutTemplate;
use std::path::Path;

#[test]
fn test_filtergraph_builder_structure() {
    let template = LayoutTemplate::default();
    let antifraud = AntiFraudConfig::default();
    let filters = antifraud.generate_instance();
    let font_path = Path::new("ui/fonts/Unbounded.ttf");

    let (v_filter, a_filter) = FilterGraphBuilder::build(&template, &filters, Some(font_path));

    // Must scale and crop top and bottom video
    assert!(v_filter.contains("scale=1080:960"));
    assert!(v_filter.contains("vstack=inputs=2"));
    assert!(v_filter.contains("drawbox="));
    assert!(v_filter.contains("drawtext="));
    assert!(v_filter.contains("crop=")); // drift
    assert!(v_filter.contains("rotate=")); // rotation
    assert!(v_filter.contains("eq=")); // color jitter
    assert!(v_filter.contains("[vout]"));

    // Audio filter
    assert!(a_filter.contains("[0:a]"));
    assert!(a_filter.contains("[aout]"));
}

#[test]
fn test_filtergraph_mirroring() {
    let template = LayoutTemplate::default();
    let mut antifraud = AntiFraudConfig::default();
    antifraud.mirror_horizontal = true;
    let filters = antifraud.generate_instance();

    let (v_filter, _) = FilterGraphBuilder::build(&template, &filters, None);
    assert!(v_filter.contains("hflip"));
}

#[test]
fn test_filtergraph_sanitization() {
    let mut template = LayoutTemplate::default();
    template.banner.text = "Quote's & colon: test".to_string();
    let antifraud = AntiFraudConfig::default();
    let filters = antifraud.generate_instance();

    let (v_filter, _) = FilterGraphBuilder::build(&template, &filters, None);
    assert!(v_filter.contains("\\'"));
    assert!(v_filter.contains("\\:"));
}
