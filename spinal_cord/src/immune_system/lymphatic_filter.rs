/* neira:meta
id: NEI-20270615-000000-lymphatic-filter-module
intent: feature
summary: Лимфатический фильтр сканирует рабочее пространство и выявляет дубликаты функций.
*/
/* neira:meta
id: NEI-20270715-000000-lymphatic-filter-perf
intent: perf
summary: Ограничен обход исходников и оптимизирован поиск дубликатов.
*/
use sha2::{Digest, Sha256};
use std::{collections::HashMap, path::PathBuf};
use walkdir::WalkDir;

use quote::ToTokens;
use syn::{Item, ItemFn, Meta};

/// Отчёт о найденном дубликате функции.
#[derive(Debug, Clone)]
pub struct DuplicationReport {
    pub gene_id: String,
    pub file: PathBuf,
    pub similarity: f32,
    pub rationale: String,
}

#[derive(Debug)]
struct FunctionFingerprint {
    gene_id: String,
    file: PathBuf,
    signature: String,
    behavior: String,
    semantic: String,
    structure: String,
}

/// Сканирует исходники в поиске дубликатов функций.
pub fn scan_workspace() -> Vec<DuplicationReport> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let root = if cwd.join("spinal_cord/src").exists() {
        cwd.join("spinal_cord/src")
    } else if cwd.join("src").exists() {
        cwd.join("src")
    } else {
        cwd
    };

    let mut buckets: HashMap<String, Vec<FunctionFingerprint>> = HashMap::new();
    let mut reports = Vec::new();

    let walker = WalkDir::new(root).into_iter().filter_entry(|e| {
        let name = e.file_name().to_string_lossy();
        if e.file_type().is_dir() {
            !matches!(name.as_ref(), "target" | "node_modules" | ".git")
        } else {
            true
        }
    });

    for entry in walker.filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        if let Ok(content) = std::fs::read_to_string(entry.path()) {
            if let Ok(file) = syn::parse_file(&content) {
                for item in file.items {
                    if let Item::Fn(func) = item {
                        let fp = fingerprint(&func, entry.path().to_path_buf());
                        let bucket = buckets.entry(fp.signature.clone()).or_default();
                        for other in bucket.iter() {
                            let mut matched = vec!["signature"];
                            let mut score = 1.0;
                            if fp.behavior == other.behavior {
                                matched.push("behavior");
                                score += 1.0;
                            }
                            if fp.semantic == other.semantic {
                                matched.push("semantic");
                                score += 1.0;
                            }
                            if fp.structure == other.structure {
                                matched.push("structure");
                                score += 1.0;
                            }
                            let similarity = score / 4.0;
                            if similarity >= 0.8 {
                                reports.push(DuplicationReport {
                                    gene_id: other.gene_id.clone(),
                                    file: other.file.clone(),
                                    similarity,
                                    rationale: format!("совпадения: {}", matched.join(", ")),
                                });
                            }
                        }
                        bucket.push(fp);
                    }
                }
            }
        }
    }

    reports
}

fn fingerprint(func: &ItemFn, file: PathBuf) -> FunctionFingerprint {
    let gene_id = func.sig.ident.to_string();
    let signature = hash(&func.sig.to_token_stream().to_string());
    let behavior = hash(&simplify_behavior(func));
    let semantic = hash(&collect_semantic(func));
    let structure = hash(&func.block.to_token_stream().to_string());
    FunctionFingerprint {
        gene_id,
        file,
        signature,
        behavior,
        semantic,
        structure,
    }
}

fn collect_semantic(func: &ItemFn) -> String {
    let mut text = func.sig.ident.to_string();
    for attr in &func.attrs {
        if attr.path().is_ident("doc") {
            if let Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit),
                    ..
                }) = &meta.value
                {
                    text.push_str(&lit.value());
                }
            }
        }
    }
    text.to_lowercase()
}

fn simplify_behavior(func: &ItemFn) -> String {
    format!("{}", func.block.stmts.len())
}

fn hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}
