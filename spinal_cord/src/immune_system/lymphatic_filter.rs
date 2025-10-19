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
use std::{collections::HashMap, path::Path, path::PathBuf};
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

/// Сканирует исходники в поиске дубликатов функций.
pub fn scan_workspace() -> Vec<DuplicationReport> {
    let repo_root = find_repo_root();
    let source_roots = discover_source_roots(&repo_root);

    let mut seen: HashMap<String, FunctionFingerprint> = HashMap::new();
    let mut reports = Vec::new();

    for root in source_roots {
        let walker = WalkDir::new(root).into_iter().filter_entry(|entry| {
            if entry.file_type().is_dir() {
                let name = entry.file_name().to_string_lossy();
                !matches!(name.as_ref(), "target" | "node_modules" | ".git" | "dist" | "vendor")
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
                            let dedupe_key = format!(
                                "{}:{}:{}:{}",
                                fp.signature, fp.behavior, fp.semantic, fp.structure
                            );
                            if let Some(existing) = seen.get(&dedupe_key) {
                                reports.push(DuplicationReport {
                                    gene_id: existing.gene_id.clone(),
                                    file: existing.file.clone(),
                                    similarity: 1.0,
                                    rationale:
                                        "совпадения: signature, behavior, semantic, structure".to_string(),
                                });
                            } else {
                                seen.insert(dedupe_key, fp);
                            }
                        }
                    }
                }
            }
        }
    }

    reports
}

fn find_repo_root() -> PathBuf {
    if let Ok(dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let manifest_path = PathBuf::from(dir);
        if let Some(parent) = manifest_path.parent() {
            return parent.to_path_buf();
        }
        return manifest_path;
    }

    let mut dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    while !dir.join("Cargo.toml").exists() {
        if !dir.pop() {
            return PathBuf::from(".");
        }
    }
    dir
}

fn discover_source_roots(repo_root: &Path) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    let candidates = [
        repo_root.join("spinal_cord/src"),
        repo_root.join("sensory_organs/src"),
        repo_root.join("src"),
    ];

    for candidate in candidates.iter() {
        if candidate.exists() {
            roots.push(candidate.clone());
        }
    }

    if roots.is_empty() {
        roots.push(repo_root.to_path_buf());
    }

    roots
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
