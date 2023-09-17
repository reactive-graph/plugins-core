use crate::ComponentRepository;
use crate::GitRepository;
use crate::TransferProgress;
use crate::NAMESPACE_GIT;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_base::Describable;
use inexor_rgf_model_base::Named;
use inexor_rgf_model_file::File;
use inexor_rgf_model_file::FilePath;
use inexor_rgf_model_http::ParsedUrl;
use inexor_rgf_model_http::Url;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_reactive_api::entity_model;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_REPOSITORY, NAMESPACE_GIT, ENTITY_TYPE_NAME_REPOSITORY, "repository");

entity_model!(Repository);
impl ComponentRepository for Repository {}
impl TransferProgress for Repository {}
impl GitRepository for Repository {}
impl File for Repository {}
impl FilePath for Repository {}
impl Url for Repository {}
impl ParsedUrl for Repository {}
impl Action for Repository {}
impl Named for Repository {}
impl Describable for Repository {}
