use crate::behaviour::entity::random_i64::RandomI64Factory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;
use reactive_graph_model_random::RandomI64;
use reactive_graph_model_random::BEHAVIOUR_RANDOM_I64;
use reactive_graph_model_random::ENTITY_TYPE_RANDOM_I64;
use reactive_graph_model_result::ResultNumberI64;
use reactive_graph_model_result::ResultNumberI64Properties::RESULT;
use reactive_graph_runtime_model::Action;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;

#[test]
fn random_i64_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_RANDOM_I64.clone())
        .property_with_default(&TRIGGER)
        .property_with_default(&RESULT)
        .build();
    let random_i64 = RandomI64::from(reactive_instance.clone());

    let next;
    {
        let factory = RandomI64Factory::new(BEHAVIOUR_RANDOM_I64.clone());
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        random_i64.trigger();
        let previous = random_i64.result().unwrap();
        random_i64.trigger();
        next = random_i64.result().unwrap();
        assert_ne!(previous, next);
    }

    random_i64.trigger();
    let should_not_have_changed = random_i64.result().unwrap();
    assert_eq!(next, should_not_have_changed);
}
