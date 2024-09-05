use crate::plugins::EntityTypeProvider;
use crate::providers::ComparisonEntityTypeProviderImpl;
use reactive_graph_graph::NamespacedTypeGetter;

#[test]
fn entity_types_should_exist() {
    let expected_entity_types = vec![
        "equals",
        "greater_than",
        "greater_than_or_equals",
        "lower_than",
        "lower_than_or_equals",
        "not_equals",
    ];
    let entity_type_provider = ComparisonEntityTypeProviderImpl {};
    let entity_types = entity_type_provider.get_entity_types();
    assert_eq!(expected_entity_types.len(), entity_types.len());
    assert_eq!(
        expected_entity_types.len(),
        entity_types
            .into_iter()
            .filter(|entity_type| expected_entity_types.contains(&entity_type.type_name().as_str()))
            .count()
    );
}
