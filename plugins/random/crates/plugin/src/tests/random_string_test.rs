use crate::behaviour::entity::random_string::RandomStringFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;
use reactive_graph_model_random::RandomString;
use reactive_graph_model_random::RandomStringProperties::LENGTH;
use reactive_graph_model_random::BEHAVIOUR_RANDOM_STRING;
use reactive_graph_model_random::ENTITY_TYPE_RANDOM_STRING;
use reactive_graph_model_result::ResultString;
use reactive_graph_model_result::ResultStringProperties::RESULT;
use reactive_graph_runtime_model::Action;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;

#[test]
fn random_string_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_RANDOM_STRING.clone())
        .property_with_default(&TRIGGER)
        .property_with_default(&LENGTH)
        .property_with_default(&RESULT)
        .build();
    let random_string = RandomString::from(reactive_instance.clone());

    let next;
    {
        let factory = RandomStringFactory::new(BEHAVIOUR_RANDOM_STRING.clone());
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        random_string.trigger();
        let previous = random_string.result().unwrap();
        assert_eq!(10, previous.len());
        random_string.length(20);
        random_string.trigger();
        next = random_string.result().unwrap();
        assert_ne!(previous, next);
        assert_eq!(20, next.len());
    }

    random_string.trigger();
    let should_not_have_changed = random_string.result().unwrap();
    assert_eq!(next, should_not_have_changed);
}
