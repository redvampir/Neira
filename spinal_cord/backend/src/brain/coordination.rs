use crate::neural::NeuralNetwork;
use crate::immune::ImmuneSystem;

pub struct BrainCoordinator {
    neural_net: Arc<NeuralNetwork>,
    immune_sys: Arc<ImmuneSystem>,
    state: RwLock<SystemState>,
}

impl BrainCoordinator {
    pub async fn process_organ_request(&self, request: OrganRequest) -> Result<(), String> {
        // Проверка иммунной системой
        self.immune_sys.verify_request(&request).await?;
        
        // Распространение через нервную систему
        let signal = NeuralSignal {
            source: request.organ_id.clone(),
            signal_type: SignalType::Request,
            priority: request.priority,
            payload: request.data,
        };
        
        self.neural_net.propagate_signal(signal).await?;
        
        // Обновление состояния системы
        let mut state = self.state.write().await;
        state.record_interaction(request);
        
        Ok(())
    }

    pub async fn optimize_connections(&self) {
        // Анализ эффективности связей
        let stats = self.neural_net.get_connection_stats().await;
        
        // Перестройка неэффективных связей
        for (connection, efficiency) in stats {
            if efficiency < 0.5 {
                self.rebuild_connection(connection).await;
            }
        }
    }
}
