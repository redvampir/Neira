/* neira:meta
id: NEI-20270318-120100-policy-training
intent: feature
summary: |
  PolicyEngine научился проверять флаги learning_microtasks и training_* для
  автоматического обучения и микрозадач.
*/
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
    LearningMicrotasks,
    TrainingPipeline,
    TrainingAutorun,
    ToneState,
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
            Capability::LearningMicrotasks => {
                if hub.learning_microtasks_enabled() {
                    Ok(())
                } else {
                    Err(PolicyError {
                        code: "capability_disabled",
                        reason: "learning_microtasks is disabled".into(),
                        capability: Some("learning_microtasks"),
                    })
                }
            }
            Capability::TrainingPipeline => {
                if hub.training_pipeline_enabled() {
                    Ok(())
                } else {
                    Err(PolicyError {
                        code: "capability_disabled",
                        reason: "training_pipeline is disabled".into(),
                        capability: Some("training_pipeline"),
                    })
                }
            }
            Capability::TrainingAutorun => {
                if hub.training_autorun_enabled() {
                    Ok(())
                } else {
                    Err(PolicyError {
                        code: "capability_disabled",
                        reason: "training_autorun is disabled".into(),
                        capability: Some("training_autorun"),
                    })
                }
            }
            Capability::ToneState => {
                if hub.tone_state_enabled() {
                    Ok(())
                } else {
                    Err(PolicyError {
                        code: "capability_disabled",
                        reason: "tone_state is disabled".into(),
                        capability: Some("tone_state"),
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
