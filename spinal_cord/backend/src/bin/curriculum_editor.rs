/* neira:meta
id: NEI-20280425-120200-curriculum-editor
intent: feature
summary: |
  Добавлен CLI-инструмент curriculum_editor для просмотра статистики
  и расширения словаря учебного курса с валидацией.
*/
use backend::training::curriculum::{
    default_curriculum_path, CurriculumError, RussianLiteracyCurriculum, WordEntry,
};
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let options = CliOptions::parse(&args)?;
    let dataset_path = options.path.clone().unwrap_or_else(default_curriculum_path);
    let mut curriculum = RussianLiteracyCurriculum::load_from_path(&dataset_path)?;

    print_overview(&curriculum, &dataset_path)?;

    if options.stats_only {
        return Ok(());
    }

    let modified = if let Some(add_word) = options.add_word {
        apply_add_word(&mut curriculum, add_word)?
    } else {
        interactive_session(&mut curriculum)?
    };

    if modified {
        let file = File::create(&dataset_path)?;
        serde_json::to_writer_pretty(file, &curriculum)?;
        println!("Словарь обновлён и сохранён в {}", dataset_path.display());
    }

    Ok(())
}

#[derive(Debug, Default)]
struct CliOptions {
    path: Option<PathBuf>,
    stats_only: bool,
    add_word: Option<AddWordArguments>,
}

#[derive(Debug, Clone, Default)]
struct AddWordArguments {
    word: Option<String>,
    syllables: Option<Vec<String>>,
    meaning: Option<String>,
    theme: Option<String>,
    level: Option<u8>,
}

impl CliOptions {
    fn parse(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut options = CliOptions::default();
        let mut idx = 0;
        while idx < args.len() {
            match args[idx].as_str() {
                "--path" => {
                    idx += 1;
                    let value = args
                        .get(idx)
                        .ok_or_else(|| io_error("для --path требуется значение"))?;
                    options.path = Some(PathBuf::from(value));
                }
                "--stats" => {
                    options.stats_only = true;
                }
                "--add-word" => {
                    options.add_word = Some(AddWordArguments::default());
                }
                "--word" => {
                    idx += 1;
                    let value = args
                        .get(idx)
                        .ok_or_else(|| io_error("для --word требуется значение"))?;
                    options
                        .add_word
                        .get_or_insert_with(AddWordArguments::default)
                        .word = Some(value.clone());
                }
                "--syllables" => {
                    idx += 1;
                    let value = args
                        .get(idx)
                        .ok_or_else(|| io_error("для --syllables требуется значение"))?;
                    let syllables = parse_syllables(value);
                    if syllables.is_empty() {
                        return Err(io_error("--syllables не может быть пустым"));
                    }
                    options
                        .add_word
                        .get_or_insert_with(AddWordArguments::default)
                        .syllables = Some(syllables);
                }
                "--meaning" => {
                    idx += 1;
                    let value = args
                        .get(idx)
                        .ok_or_else(|| io_error("для --meaning требуется значение"))?;
                    options
                        .add_word
                        .get_or_insert_with(AddWordArguments::default)
                        .meaning = Some(value.clone());
                }
                "--theme" => {
                    idx += 1;
                    let value = args
                        .get(idx)
                        .ok_or_else(|| io_error("для --theme требуется значение"))?;
                    options
                        .add_word
                        .get_or_insert_with(AddWordArguments::default)
                        .theme = Some(value.clone());
                }
                "--level" => {
                    idx += 1;
                    let value = args
                        .get(idx)
                        .ok_or_else(|| io_error("для --level требуется значение"))?;
                    let parsed: u8 = value
                        .parse()
                        .map_err(|_| io_error("--level должен быть числом от 0 до 255"))?;
                    options
                        .add_word
                        .get_or_insert_with(AddWordArguments::default)
                        .level = Some(parsed);
                }
                unexpected => {
                    return Err(io_error(&format!("неизвестный аргумент {unexpected}")));
                }
            }
            idx += 1;
        }
        Ok(options)
    }
}

fn apply_add_word(
    curriculum: &mut RussianLiteracyCurriculum,
    args: AddWordArguments,
) -> Result<bool, Box<dyn std::error::Error>> {
    let word = args
        .word
        .ok_or_else(|| io_error("--add-word требует параметр --word"))?;
    if curriculum.words.iter().any(|entry| entry.word == word) {
        return Err(io_error("слово уже существует в словаре"));
    }
    let syllables = args
        .syllables
        .ok_or_else(|| io_error("--add-word требует параметр --syllables"))?;
    let meaning = args
        .meaning
        .ok_or_else(|| io_error("--add-word требует параметр --meaning"))?;
    let theme = args
        .theme
        .ok_or_else(|| io_error("--add-word требует параметр --theme"))?;
    let level = args
        .level
        .ok_or_else(|| io_error("--add-word требует параметр --level"))?;

    push_and_validate(
        curriculum,
        WordEntry {
            word,
            syllables,
            meaning,
            theme,
            level,
        },
    )
}

fn interactive_session(
    curriculum: &mut RussianLiteracyCurriculum,
) -> Result<bool, Box<dyn std::error::Error>> {
    println!("Добавить новое слово? [y/N]");
    if !confirm()? {
        return Ok(false);
    }

    let word = prompt("Введите слово: ")?;
    if curriculum.words.iter().any(|entry| entry.word == word) {
        println!("Слово уже существует в словаре, изменения не внесены.");
        return Ok(false);
    }
    let syllables_input = prompt("Введите слоги через запятую (например, ма,ши,на): ")?;
    let syllables = parse_syllables(&syllables_input);
    if syllables.is_empty() {
        println!("Слоги не распознаны, отмена добавления.");
        return Ok(false);
    }
    let meaning = prompt("Краткое значение слова: ")?;
    let theme = prompt("Тема слова (например, природа): ")?;
    let level_str = prompt("Уровень сложности (0-255): ")?;
    let level: u8 = match level_str.parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Некорректный уровень, ожидается число от 0 до 255.");
            return Ok(false);
        }
    };

    push_and_validate(
        curriculum,
        WordEntry {
            word,
            syllables,
            meaning,
            theme,
            level,
        },
    )
}

fn push_and_validate(
    curriculum: &mut RussianLiteracyCurriculum,
    entry: WordEntry,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut entry = entry;
    entry.word = entry.word.trim().to_string();
    if entry.word.is_empty() {
        return Err(io_error("слово не может быть пустым"));
    }
    entry.meaning = entry.meaning.trim().to_string();
    entry.theme = entry.theme.trim().to_string();

    curriculum.words.push(entry);
    if let Err(err) = curriculum.validate() {
        curriculum.words.pop();
        return match err {
            CurriculumError::Validation(message) => Err(io_error(&message)),
            other => Err(io_error(&format!("ошибка валидации: {other}"))),
        };
    }
    println!("Добавлено слово. Всего слов: {}", curriculum.words.len());
    Ok(true)
}

fn print_overview(
    curriculum: &RussianLiteracyCurriculum,
    path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let summary = curriculum.summary();
    let themes = curriculum.theme_statistics();
    println!(
        "Курс: {} ({}). Букв: {}. Слогов: {}. Слов: {}.",
        curriculum.id(),
        path.display(),
        summary.letters,
        summary.syllables,
        summary.words
    );
    println!("Темы словаря:");
    for (theme, count) in themes {
        println!("  - {theme}: {count}");
    }
    Ok(())
}

fn parse_syllables(input: &str) -> Vec<String> {
    input
        .split(|ch: char| ch == ',' || ch == ';')
        .map(|part| part.trim().to_string())
        .filter(|part| !part.is_empty())
        .collect()
}

fn confirm() -> Result<bool, Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(matches!(input.trim().to_lowercase().as_str(), "y" | "yes"))
}

fn prompt(message: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{message}");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn io_error(message: &str) -> Box<dyn std::error::Error> {
    Box::new(io::Error::new(
        io::ErrorKind::InvalidInput,
        message.to_string(),
    ))
}
