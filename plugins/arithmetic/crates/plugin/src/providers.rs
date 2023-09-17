use inexor_rgf_plugin_api::prelude::providers::*;

#[derive(TypeProvider, Component)]
#[type_provider(tys = "Components", path = "types/components")]
pub struct ArithmeticComponentsProvider {}

#[derive(TypeProvider, Component)]
#[type_provider(tys = "EntityTypes", path = "types/entities")]
pub struct ArithmeticEntityTypesProvider {}

#[cfg(test)]
mod tests {
    use inexor_rgf_graph::prelude::*;

    use inexor_rgf_model_arithmetic::NAMESPACE_ARITHMETIC;
    use inexor_rgf_model_arithmetic::NAMESPACE_ARITHMETIC_F64;
    use inexor_rgf_model_arithmetic::NAMESPACE_ARITHMETIC_I64;
    use inexor_rgf_model_arithmetic::NAMESPACE_ARITHMETIC_U64;

    use super::*;

    #[test]
    fn components_should_exist() {
        let expected_components = ComponentTypeIds::with_namespace(NAMESPACE_ARITHMETIC)
            .ty("arithmetic_gate")
            .ty("arithmetic_operation");
        let component_provider = ArithmeticComponentsProvider {};
        assert!(component_provider
            .get_types()
            .iter()
            .all(|component| expected_components.contains(component.key())));
    }

    #[test]
    fn entity_types_should_exist() {
        let entity_type_provider = ArithmeticEntityTypesProvider {};
        let expected_entity_types_f64 = EntityTypeIds::with_namespace(NAMESPACE_ARITHMETIC_F64)
            .ty("add")
            .ty("decrement")
            .ty("div")
            .ty("increment")
            .ty("max")
            .ty("min")
            .ty("mod")
            .ty("mul")
            .ty("sub");
        let expected_entity_types_i64 = EntityTypeIds::with_namespace(NAMESPACE_ARITHMETIC_I64)
            .ty("add")
            .ty("decrement")
            .ty("div")
            .ty("increment")
            .ty("max")
            .ty("min")
            .ty("mod")
            .ty("mul")
            .ty("sub");
        let expected_entity_types_u64 = EntityTypeIds::with_namespace(NAMESPACE_ARITHMETIC_U64)
            .ty("add")
            .ty("decrement")
            .ty("div")
            .ty("increment")
            .ty("max")
            .ty("min")
            .ty("mod")
            .ty("mul")
            .ty("sub")
            .ty("counter");
        assert!(entity_type_provider.get_types().iter().all(|entity_type| {
            expected_entity_types_f64.contains(entity_type.key())
                || expected_entity_types_i64.contains(entity_type.key())
                || expected_entity_types_u64.contains(entity_type.key())
        }));
    }
}
