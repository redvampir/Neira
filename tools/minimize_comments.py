#!/usr/bin/env python3
"""
Инструмент минимизации neira:meta комментариев
Заменяет verbose блоки на компактный формат: // NEI-YYYYMMDD: краткое описание

Использование:
    python tools/minimize_comments.py --path spinal_cord/src/main.rs [--dry-run]
"""
import re
import sys
from pathlib import Path
import argparse


class CommentMinimizer:
    """Минимизатор neira:meta блоков"""
    
    # Паттерн для /* neira:meta ... */
    RUST_PATTERN = re.compile(
        r'/\*\s*neira:meta\s+(.*?)\s*\*/',
        re.DOTALL
    )
    
    # Паттерн для <!-- neira:meta ... -->
    MD_PATTERN = re.compile(
        r'<!--\s*neira:meta\s+(.*?)\s*-->',
        re.DOTALL
    )
    
    def __init__(self, dry_run=False):
        self.dry_run = dry_run
        self.changes = []
    
    def minimize_rust(self, content: str) -> str:
        """Минимизировать Rust комментарии"""
        def replace_meta(match):
            meta_text = match.group(1).strip()
            compact = self._parse_and_minimize(meta_text)
            self.changes.append(compact)
            return compact
        
        return self.RUST_PATTERN.sub(replace_meta, content)
    
    def minimize_markdown(self, content: str) -> str:
        """Минимизировать Markdown комментарии"""
        def replace_meta(match):
            meta_text = match.group(1).strip()
            compact = self._parse_and_minimize_md(meta_text)
            self.changes.append(compact)
            return compact
        
        return self.MD_PATTERN.sub(replace_meta, content)
    
    def _parse_and_minimize(self, text: str) -> str:
        """Парсинг и минимизация для Rust"""
        lines = text.split('\n')
        meta_id = None
        summary = None
        
        for line in lines:
            line = line.strip()
            if line.startswith('id:'):
                meta_id = line.partition(':')[2].strip()
            elif line.startswith('summary:'):
                summary = line.partition(':')[2].strip()
        
        # Если нет id, вернуть пустую строку (удалить)
        if not meta_id:
            return ''
        
        # Извлечь дату из id (NEI-YYYYMMDD-...)
        date_match = re.search(r'NEI-(\d{8})', meta_id)
        if date_match:
            date = date_match.group(1)
        else:
            date = '20251105'  # fallback
        
        # Если нет summary, использовать id как описание
        if not summary:
            summary = meta_id
        
        # Обрезать summary если слишком длинный
        if len(summary) > 80:
            summary = summary[:77] + '...'
        
        return f'// NEI-{date}: {summary}'
    
    def _parse_and_minimize_md(self, text: str) -> str:
        """Парсинг и минимизация для Markdown"""
        lines = text.split('\n')
        meta_id = None
        summary = None
        
        for line in lines:
            line = line.strip()
            if line.startswith('id:'):
                meta_id = line.partition(':')[2].strip()
            elif line.startswith('summary:'):
                summary = line.partition(':')[2].strip()
        
        if not meta_id:
            return ''
        
        date_match = re.search(r'NEI-(\d{8})', meta_id)
        if date_match:
            date = date_match.group(1)
        else:
            date = '20251105'
        
        if not summary:
            summary = meta_id
        
        if len(summary) > 80:
            summary = summary[:77] + '...'
        
        return f'<!-- NEI-{date}: {summary} -->'
    
    def process_file(self, filepath: Path) -> bool:
        """Обработать один файл"""
        try:
            content = filepath.read_text(encoding='utf-8')
        except Exception as e:
            print(f"⚠️  Не удалось прочитать {filepath}: {e}", file=sys.stderr)
            return False
        
        # Выбираем метод в зависимости от расширения
        if filepath.suffix in ['.rs', '.toml']:
            new_content = self.minimize_rust(content)
        elif filepath.suffix == '.md':
            new_content = self.minimize_markdown(content)
        else:
            return False
        
        # Проверка изменений
        if content == new_content:
            print(f"ℹ️  {filepath}: изменений нет")
            return False
        
        if self.dry_run:
            print(f"🔍 {filepath}: найдено {len(self.changes)} блоков для минимизации (dry-run)")
            return True
        
        # Запись
        filepath.write_text(new_content, encoding='utf-8')
        print(f"✅ {filepath}: минимизировано {len(self.changes)} блоков")
        return True


def main():
    parser = argparse.ArgumentParser(description='Минимизация neira:meta комментариев')
    parser.add_argument('--path', '-p', required=True, help='Путь к файлу для обработки')
    parser.add_argument('--dry-run', '-d', action='store_true', help='Только показать, без записи')
    
    args = parser.parse_args()
    
    filepath = Path(args.path)
    if not filepath.exists():
        print(f"❌ Файл {filepath} не найден", file=sys.stderr)
        sys.exit(1)
    
    minimizer = CommentMinimizer(dry_run=args.dry_run)
    changed = minimizer.process_file(filepath)
    
    if changed:
        print(f"\n📝 Минимизация завершена!")
    else:
        print(f"\nℹ️  Изменений не требуется")


if __name__ == '__main__':
    main()
