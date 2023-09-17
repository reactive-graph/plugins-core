use inexor_rgf_behaviour::entity_behaviour;
use inexor_rgf_behaviour::PropertyObserverContainer;
use inexor_rgf_behaviour_api::behaviour_validator;
use inexor_rgf_behaviour_api::prelude::*;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive::ReactiveEntity;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

pub use function::LogicalOperationFunction;

use inexor_rgf_model_logical::LogicalOperationProperties::LHS;
use inexor_rgf_model_result::ResultBooleanProperties::RESULT;

pub mod function;

entity_behaviour!(
    LogicalOperation,
    LogicalOperationFactory,
    LogicalOperationFsm,
    LogicalOperationBehaviourTransitions,
    LogicalOperationValidator,
    f,
    LogicalOperationFunction
);

behaviour_validator!(LogicalOperationValidator, Uuid, ReactiveEntity, LHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for LogicalOperationBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self
            .reactive_instance
            .get(LHS)
            .and_then(|v| v.as_bool())
            .ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        self.reactive_instance.set(RESULT, json!(f(lhs)));
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for LogicalOperationBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |v: &Value| {
            if let Some(v) = v.as_bool() {
                reactive_instance.set(RESULT, json!(f(v)));
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for LogicalOperationBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for LogicalOperationBehaviourTransitions {}
