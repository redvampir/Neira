/* neira:meta
id: NEI-20251103-memory-mod
intent: feature
summary: Модуль памяти для семантического хранения и поиска диалогов.
*/

pub mod semantic;

pub use semantic::{SemanticMemory, ConversationEmbedding, SearchResult, MemoryError, Result};
