#![allow(
    clippy::too_many_arguments,
    clippy::new_without_default,
    non_camel_case_types,
    unused_imports
)]
/*
 * Mayastor RESTful API
 *
 * The version of the OpenAPI document: v0
 *
 * Generated by: https://github.com/openebs/openapi-generator
 */

use crate::apis::IntoVec;

/// PoolTopology : Used to determine how to place/distribute the data during volume creation and
/// replica replacement.  If left empty then the control plane will select from all available
/// resources.

/// Used to determine how to place/distribute the data during volume creation and replica
/// replacement.  If left empty then the control plane will select from all available resources.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PoolTopology {
    /// volume pool topology definition through labels
    #[serde(rename = "labelled")]
    labelled(crate::models::LabelledTopology),
}
