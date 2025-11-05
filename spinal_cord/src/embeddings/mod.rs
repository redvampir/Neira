/* neira:meta
id: NEI-20251103-embeddings-mod
intent: feature
summary: Модуль для работы с семантическими эмбеддингами.
*/

pub mod client;

pub use client::{EmbeddingsClient, EmbeddingPrefix, EmbeddingsError, Result};
