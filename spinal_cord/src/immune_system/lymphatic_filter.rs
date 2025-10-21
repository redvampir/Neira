/* neira:meta
id: NEI-20270618-000000-lymphatic-filter-module
intent: feature
summary: Лимфатический фильтр сканирует рабочее пространство и выявляет дубликаты функций, поддерживая кэш, гибкие параметры и генерацию патчей.
*/
use crate::config::env_flag;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};
use syn::{visit::Visit, ImplItemMethod, Item, ItemFn, ItemMod};
use walkdir::WalkDir;

/// Отчёт о найденном дубликате функции.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicationReport {
    pub gene_id: String,
    pub file: PathBuf,
    pub similarity: f32,
    pub rationale: String,
    pub patch: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FunctionFingerprint {
    gene_id: String,
    file: PathBuf,
    signature: String,
    behavior: String,
    semantic: String,
    structure: String,
}

#[derive(Default, Serialize, Deserialize)]
struct CachedFile {
    mtime: u64,
    functions: Vec<FunctionFingerprint>,
}

#[derive(Default, Serialize, Deserialize)]
struct FingerprintCache {
    cargo_lock_mtime: u64,
    files: HashMap<String, CachedFile>,
}

struct FnCollector {
    file: PathBuf,
    functions: Vec<FunctionFingerprint>,
}

impl<'ast> Visit<'ast> for FnCollector {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.functions.push(fingerprint(node, self.file.clone()));
        syn::visit::visit_item_fn(self, node);
    }

    fn visit_impl_item_method(&mut self, node: &'ast ImplItemMethod) {
        let item_fn = ItemFn {
            attrs: node.attrs.clone(),
            vis: node.vis.clone(),
            sig: node.sig.clone(),
            block: Box::new(node.block.clone()),
        };
        self.functions
            .push(fingerprint(&item_fn, self.file.clone()));
        syn::visit::visit_impl_item_method(self, node);
    }

    fn visit_item_mod(&mut self, node: &'ast ItemMod) {
        if let Some((_, items)) = &node.content {
            for item in items {
                self.visit_item(item);
            }
        }
    }
}

static SYNONYMS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    [
        ("create", "init"),
        ("initialize", "init"),
        ("init", "init"),
        ("remove", "delete"),
        ("delete", "delete"),
        ("add", "insert"),
        ("insert", "insert"),
        ("fetch", "get"),
        ("retrieve", "get"),
        ("get", "get"),
    ]
    .into_iter()
    .collect()
});

/// Сканирует рабочее пространство и выявляет дубликаты функций.
pub fn scan_workspace() -> Vec<DuplicationReport> {
    let root = std::env::var("LYMPHATIC_SCAN_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    let ignore_paths = load_ignore(&root);
    let staged_only = env_flag("LYMPHATIC_STAGED_ONLY", false);
    let staged_files: HashSet<PathBuf> = if staged_only {
        collect_staged_files()
    } else {
        HashSet::new()
    };

    let cache_path = root.join("logs/lymphatic_cache.json");
    let mut cache = load_cache(&cache_path);
    let cargo_lock_mtime = mtime(&root.join("Cargo.lock"));
    if cache.cargo_lock_mtime != cargo_lock_mtime {
        cache.files.clear();
        cache.cargo_lock_mtime = cargo_lock_mtime;
    }

    let mut fingerprints = Vec::new();
    for entry in WalkDir::new(&root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }

        let rel = entry.path().strip_prefix(&root).unwrap_or(entry.path());
        if ignore_paths.iter().any(|p| rel.starts_with(p)) {
            continue;
        }
        if staged_only && !staged_files.contains(rel) {
            continue;
        }

        let path_str = rel.to_string_lossy().to_string();
        let file_mtime = mtime(entry.path());
        if let Some(cached) = cache.files.get(&path_str) {
            if cached.mtime == file_mtime {
                fingerprints.extend(cached.functions.clone());
                continue;
            }
        }

        if let Ok(content) = fs::read_to_string(entry.path()) {
            if let Ok(file) = syn::parse_file(&content) {
                let mut collector = FnCollector {
                    file: entry.path().to_path_buf(),
                    functions: Vec::new(),
                };
                collector.visit_file(&file);
                if !collector.functions.is_empty() {
                    cache.files.insert(
                        path_str.clone(),
                        CachedFile {
                            mtime: file_mtime,
                            functions: collector.functions.clone(),
                        },
                    );
                    fingerprints.extend(collector.functions);
                }
            }
        }
    }
    save_cache(&cache_path, &cache);

    let semantic_weight = std::env::var("LYMPHATIC_SEMANTIC_WEIGHT")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(1.0);

    let mut reports = Vec::new();
    for i in 0..fingerprints.len() {
        for j in (i + 1)..fingerprints.len() {
            let a = &fingerprints[i];
            let b = &fingerprints[j];
            let mut matched = Vec::new();
            let mut score = 0.0f64;

            if a.signature == b.signature {
                matched.push("signature");
                score += 1.0;
            }
            if a.behavior == b.behavior {
                matched.push("behavior");
                score += 1.0;
            }
            let semantic_score = strsim::jaro_winkler(&a.semantic, &b.semantic);
            if semantic_score >= 0.8 {
                matched.push("semantic");
            }
            score += semantic_score * semantic_weight;
            if a.structure == b.structure {
                matched.push("structure");
                score += 1.0;
            }

            let similarity = score / (3.0 + semantic_weight);
            if similarity >= 0.8 {
                let patch = generate_patch(a, b, &root);
                reports.push(DuplicationReport {
                    gene_id: b.gene_id.clone(),
                    file: b.file.clone(),
                    similarity: similarity as f32,
                    rationale: format!("совпадения: {}", matched.join(", ")),
                    patch,
                });
            }
        }
    }
    reports
}

fn fingerprint(func: &ItemFn, file: PathBuf) -> FunctionFingerprint {
    let gene_id = func.sig.ident.to_string();
    use quote::ToTokens;
    let signature = hash(&func.sig.to_token_stream().to_string());
    let behavior = hash(&simplify_behavior(func));
    let semantic = collect_semantic(func);
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
            if let syn::Meta::NameValue(meta) = &attr.meta {
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
    normalize_semantic(&text)
}

fn normalize_semantic(text: &str) -> String {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|w| {
            let lw = w.to_lowercase();
            SYNONYMS
                .get(lw.as_str())
                .cloned()
                .unwrap_or(&lw)
                .to_string()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn simplify_behavior(func: &ItemFn) -> String {
    format!("{}", func.block.stmts.len())
}

fn hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn mtime(path: &Path) -> u64 {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn load_ignore(root: &Path) -> Vec<PathBuf> {
    let mut ignores = Vec::new();
    let path = root.join(".lymphaticignore");
    if let Ok(content) = fs::read_to_string(path) {
        for line in content.lines() {
            let line = line.trim();
            if !line.is_empty() {
                ignores.push(PathBuf::from(line));
            }
        }
    }
    ignores
}

fn collect_staged_files() -> HashSet<PathBuf> {
    let mut set = HashSet::new();
    if let Ok(output) = std::process::Command::new("git")
        .args(["diff", "--name-only", "--cached"])
        .output()
    {
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            set.insert(PathBuf::from(line));
        }
    }
    set
}

fn load_cache(path: &Path) -> FingerprintCache {
    fs::read(path)
        .ok()
        .and_then(|data| serde_json::from_slice(&data).ok())
        .unwrap_or_default()
}

fn save_cache(path: &Path, cache: &FingerprintCache) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(data) = serde_json::to_vec_pretty(cache) {
        let _ = fs::write(path, data);
    }
}

fn generate_patch(
    a: &FunctionFingerprint,
    b: &FunctionFingerprint,
    root: &Path,
) -> Option<PathBuf> {
    let content = fs::read_to_string(&b.file).ok()?;
    let file = syn::parse_file(&content).ok()?;
    for item in file.items {
        if let Item::Fn(func) = item {
            if func.sig.ident == b.gene_id {
                use syn::{FnArg, Pat};
                let params: Vec<String> = func
                    .sig
                    .inputs
                    .iter()
                    .filter_map(|arg| match arg {
                        FnArg::Typed(pat_type) => match &*pat_type.pat {
                            Pat::Ident(id) => Some(id.ident.to_string()),
                            _ => None,
                        },
                        _ => None,
                    })
                    .collect();
                let call = format!("{}({})", a.gene_id, params.join(", "));
                let body = match func.sig.output {
                    syn::ReturnType::Default => format!("{{\n    {};\n}}", call),
                    _ => format!("{{\n    {}\n}}", call),
                };
                let new_fn = format!("fn {}({}) {}", b.gene_id, params.join(", "), body);
                let patch_content =
                    format!("--- {0}\n+++ {0}\n@@\n{1}\n", b.file.display(), new_fn);
                let patch_path = root
                    .join("logs")
                    .join("lymphatic_patches")
                    .join(format!("{}-{}.patch", a.gene_id, b.gene_id));
                if let Some(parent) = patch_path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                let _ = fs::write(&patch_path, patch_content);
                return Some(patch_path);
            }
        }
    }
    None
}
