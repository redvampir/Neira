/* neira:meta
id: NEI-20250923-policy-engine-core
intent: code
summary: Каркас Policy Engine: проверка capability/ролей и унифицированные отказы.
*/

use serde::Serialize;

#[derive(Debug, Clone, Copy)]
pub enum Capability {
    FactoryAdapter,
    OrgansBuilder,
}

#[derive(Debug, Clone)]
pub struct PolicyEngine;

#[derive(Debug, Serialize)]
pub struct PolicyError {
    pub code: &'static str,
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability: Option<&'static str>,
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn require_capability(
        &self,
        hub: &crate::synapse_hub::SynapseHub,
        cap: Capability,
    ) -> Result<(), PolicyError> {
        match cap {
            Capability::FactoryAdapter => {
                if hub.factory_is_adapter_enabled() {
                    Ok(())
                } else {
                    Err(PolicyError {
                        code: "capability_disabled",
                        reason: "factory_adapter is disabled".into(),
                        capability: Some("factory_adapter"),
                    })
                }
            }
            Capability::OrgansBuilder => {
                if hub.organ_builder_enabled() {
                    Ok(())
                } else {
                    Err(PolicyError {
                        code: "capability_disabled",
                        reason: "organs_builder is disabled".into(),
                        capability: Some("organs_builder"),
                    })
                }
            }
        }
    }
}

impl Default for PolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

/* neira:meta
id: NEI-20240513-policy-default
intent: chore
summary: Добавлен Default для PolicyEngine для удовлетворения lint new_without_default.
*/
