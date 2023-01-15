use crate::behaviour::entity::random_f64_range::RandomF64RangeFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_random::RandomF64Range;
use crate::model_random::RandomF64RangeProperties::HIGH;
use crate::model_random::RandomF64RangeProperties::LOW;
use crate::model_random::RandomF64RangeProperties::RESULT;
use crate::model_random::RandomF64RangeProperties::TRIGGER;
use crate::model_random::BEHAVIOUR_RANDOM_F64_RANGE;
use crate::model_random::ENTITY_TYPE_RANDOM_F64_RANGE;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;

#[test]
fn random_f64_range_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_RANDOM_F64_RANGE.clone())
        .property_with_default(&TRIGGER)
        .property_with_default(&LOW)
        .property_with_default(&HIGH)
        .property_with_default(&RESULT)
        .build();
    let random_f64_range = RandomF64Range::from(reactive_instance.clone());

    let next;
    {
        let factory = RandomF64RangeFactory::new(BEHAVIOUR_RANDOM_F64_RANGE.clone());
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        random_f64_range.low(-1.0);
        random_f64_range.high(1.0);
        for _ in 0..1000 {
            random_f64_range.trigger();
            let result = random_f64_range.result().unwrap();
            assert!(result >= -1.0 && result < 1.0);
        }
        random_f64_range.low(-100.0);
        random_f64_range.high(100.0);
        for _ in 0..1000 {
            random_f64_range.trigger();
            let result = random_f64_range.result().unwrap();
            assert!(result >= -100.0 && result < 100.0);
        }
        random_f64_range.trigger();
        next = random_f64_range.result().unwrap();
    }

    random_f64_range.trigger();
    let should_not_have_changed = random_f64_range.result().unwrap();
    assert_eq!(next, should_not_have_changed);
}
