use std::sync::{Arc, RwLock};

use log::debug;
use serde_json::{json, Value};

use crate::behaviour::entity::gate::properties::LogicalGateProperties;
use crate::behaviour::entity::gate::LogicalGateFunction;
use crate::frp::Stream;
use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter, ReactiveEntityInstance};
use crate::reactive::entity::expression::{Expression, ExpressionValue, OperatorPosition};
use crate::reactive::entity::gate::Gate;
use crate::reactive::entity::operation::Operation;
use crate::reactive::entity::Disconnectable;

pub type LogicalExpressionValue = ExpressionValue<bool>;

/// Generic implementation of logical_gates operations with two inputs (LHS,RHS) and one result.
///
/// The implementation is realized using reactive streams.
pub struct LogicalGate<'a> {
    pub lhs: RwLock<Stream<'a, LogicalExpressionValue>>,

    pub rhs: RwLock<Stream<'a, LogicalExpressionValue>>,

    pub f: LogicalGateFunction,

    pub internal_result: RwLock<Stream<'a, bool>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl LogicalGate<'_> {
    pub fn new(e: Arc<ReactiveEntityInstance>, f: LogicalGateFunction) -> LogicalGate<'static> {
        let lhs = e
            .properties
            .get(LogicalGateProperties::LHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| match v.as_bool() {
                Some(b) => (OperatorPosition::LHS, b),
                None => (OperatorPosition::LHS, LogicalGateProperties::LHS.default_value()),
            });
        let rhs = e
            .properties
            .get(LogicalGateProperties::RHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| -> LogicalExpressionValue {
                match v.as_bool() {
                    Some(b) => (OperatorPosition::RHS, b),
                    None => (OperatorPosition::RHS, LogicalGateProperties::RHS.default_value()),
                }
            });

        let expression = lhs.merge(&rhs).fold(
            Expression::new(LogicalGateProperties::LHS.default_value(), LogicalGateProperties::RHS.default_value()),
            |old_state, (o, value)| match *o {
                OperatorPosition::LHS => old_state.lhs(*value),
                OperatorPosition::RHS => old_state.rhs(*value),
            },
        );

        // The internal result
        let internal_result = expression.map(move |e| f(e.lhs, e.rhs));

        // TODO: handle result based on outbound property id and inbound property id
        let handle_id = e.properties.get(LogicalGateProperties::RESULT.as_ref()).unwrap().id.as_u128();

        let logical_gate = LogicalGate {
            lhs: RwLock::new(lhs),
            rhs: RwLock::new(rhs),
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Connect the internal result with the stream of the result property
        logical_gate.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                debug!("Setting result of logical gate: {}", v);
                e.set(LogicalGateProperties::RESULT.to_string(), json!(*v));
            },
            handle_id,
        );

        logical_gate
    }

    /// TODO: extract to trait "Named"
    /// TODO: unit test
    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for LogicalGate<'_> {
    /// TODO: Add guard: disconnect only if actually connected
    fn disconnect(&self) {
        debug!("Disconnect logical gate {} {}", self.type_name(), self.handle_id);
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for LogicalGate<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(LogicalGateProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(LogicalGateProperties::RESULT.as_ref()).unwrap()
    }
}

impl Gate for LogicalGate<'_> {
    fn rhs(&self, value: Value) {
        self.entity.set(LogicalGateProperties::RHS.as_ref(), value);
    }
}

/// Automatically disconnect streams on destruction
impl Drop for LogicalGate<'_> {
    fn drop(&mut self) {
        debug!("Drop logical gate");
        self.disconnect();
    }
}
