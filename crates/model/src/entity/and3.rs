use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::flow_ty;
use crate::model::properties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::NAMESPACE_LOGICAL;

properties!(And3Properties, (INPUT_1, "input1", false), (INPUT_2, "input2", false), (INPUT_3, "input3", false));

entity_ty!(ENTITY_TYPE_AND3, NAMESPACE_LOGICAL, ENTITY_TYPE_NAME_AND3, "and3");
flow_ty!(FLOW_TYPE_AND3, NAMESPACE_LOGICAL, FLOW_TYPE_NAME_AND3, "and3");

entity_model!(
    And3,
    get result bool,
    set input1 bool,
    set input2 bool,
    set input3 bool
);
