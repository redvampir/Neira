pub struct MaterialProcessor {
    parser: Arc<ContentParser>,
    validator: Arc<ContentValidator>,
    knowledge_base: Arc<KnowledgeBase>,
}

impl MaterialProcessor {
    pub async fn process_book(&self, book: UploadedBook) -> Result<ProcessingReport, String> {
        // Парсим содержимое
        let content = self
            .parser
            .parse_book(&book)
            .with_progress_callback(|progress| self.notify_progress("Читаю книгу...", progress))
            .await?;

        // Проверяем содержимое
        let validated = self.validator.validate_content(content).await?;

        // Сохраняем в базу знаний
        self.knowledge_base.integrate_knowledge(validated).await
    }

    pub async fn process_user_notes(&self, notes: UserNotes) -> Result<(), String> {
        // Анализируем заметки
        let processed = self.parser.parse_notes(&notes).await?;

        // Интегрируем с существующими знаниями
        self.knowledge_base.merge_notes(processed).await
    }
}
