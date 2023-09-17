use serde_json::json;

use crate::behaviour_api::behaviour_ty;
use crate::behaviour_api::entity_behaviour_ty;
use crate::NAMESPACE_STRING;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_model_result::ResultString;
use inexor_rgf_reactive_api::entity_model;

properties!(TemplatingProperties, (TEMPLATE, "template", ""), (CONTEXT, "context", json!({})));

entity_ty!(ENTITY_TYPE_TEMPLATING, NAMESPACE_STRING, ENTITY_TYPE_NAME_TEMPLATING, "templating");
behaviour_ty!(BEHAVIOUR_TEMPLATING, NAMESPACE_STRING, BEHAVIOUR_NAME_TEMPLATING, "templating");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_TEMPLATING, ENTITY_TYPE_TEMPLATING, BEHAVIOUR_TEMPLATING);

entity_model!(
    Templating,
    set context value,
    set template string
);
impl ResultString for Templating {}
