use qsnap::templates::schema::LayoutTemplate;
use qsnap::templates::TemplateManager;

#[test]
fn test_default_template_structure() {
    let tmpl = LayoutTemplate::default();
    assert_eq!(tmpl.target_width, 1080);
    assert_eq!(tmpl.target_height, 1920);
    assert_eq!(tmpl.top_ratio, 0.50);
    assert_eq!(tmpl.bottom_ratio, 0.50);
    assert_eq!(tmpl.banner.font_size, 32);
}

#[test]
fn test_template_json_serialization() {
    let tmpl = LayoutTemplate::default();
    let json_str = serde_json::to_string(&tmpl).expect("Must serialize to JSON");
    let deserialized: LayoutTemplate = serde_json::from_str(&json_str).expect("Must deserialize from JSON");
    assert_eq!(tmpl.id, deserialized.id);
    assert_eq!(tmpl.target_width, deserialized.target_width);
    assert_eq!(tmpl.banner.text, deserialized.banner.text);
}

#[test]
fn test_template_manager_list() {
    let manager = TemplateManager::new().expect("TemplateManager creation");
    let list = manager.list_templates();
    assert!(!list.is_empty(), "Must contain at least default template");
}
