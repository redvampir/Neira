/* neira:meta
id: NEI-20270615-lymphatic-filter-module
intent: feature
summary: Лимфатический фильтр сканирует рабочее пространство и выявляет дубликаты функций.
*/
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use walkdir::WalkDir;

use syn::{Item, ItemFn};

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

/// Сканирует текущий рабочий каталог в поиске дубликатов функций.
pub fn scan_workspace() -> Vec<DuplicationReport> {
    let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut fingerprints = Vec::new();

    for entry in WalkDir::new(&root).into_iter().filter_map(Result::ok) {
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
                        fingerprints.push(fingerprint(&func, entry.path().to_path_buf()));
                    }
                }
            }
        }
    }

    let mut reports = Vec::new();
    for i in 0..fingerprints.len() {
        for j in (i + 1)..fingerprints.len() {
            let a = &fingerprints[i];
            let b = &fingerprints[j];
            let mut matched = Vec::new();
            let mut score = 0.0;
            if a.signature == b.signature {
                matched.push("signature");
                score += 1.0;
            }
            if a.behavior == b.behavior {
                matched.push("behavior");
                score += 1.0;
            }
            if a.semantic == b.semantic {
                matched.push("semantic");
                score += 1.0;
            }
            if a.structure == b.structure {
                matched.push("structure");
                score += 1.0;
            }
            let similarity = score / 4.0;
            if similarity >= 0.8 {
                reports.push(DuplicationReport {
                    gene_id: b.gene_id.clone(),
                    file: b.file.clone(),
                    similarity,
                    rationale: format!("совпадения: {}", matched.join(", ")),
                });
            }
        }
    }
    reports
}

fn fingerprint(func: &ItemFn, file: PathBuf) -> FunctionFingerprint {
    let gene_id = func.sig.ident.to_string();
    let signature = hash(&format!("{:?}", func.sig));
    let behavior = hash(&simplify_behavior(func));
    let semantic = hash(&collect_semantic(func));
    let structure = hash(&format!("{:?}", func.block));
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
            if let Ok(syn::Meta::NameValue(meta)) = attr.parse_meta() {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit),
                    ..
                }) = meta.value
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
