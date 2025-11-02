pub struct SelfDiagnostics {
    system_monitor: Arc<SystemMonitor>,
    error_analyzer: Arc<ErrorAnalyzer>,
    repair_manager: Arc<RepairManager>,
}

impl SelfDiagnostics {
    pub async fn run_diagnostics(&self) -> DiagnosticsReport {
        // Проверяем все системы
        let systems = self.system_monitor
            .check_all_systems()
            .await?;
            
        // Анализируем ошибки
        let errors = self.error_analyzer
            .analyze_errors(systems)
            .await?;
            
        // Пытаемся исправить проблемы
        if !errors.is_empty() {
            self.repair_manager
                .attempt_repair(errors)
                .await?;
        }
        
        DiagnosticsReport {
            systems,
            errors,
            fixes: self.repair_manager.applied_fixes().await,
        }
    }
}
