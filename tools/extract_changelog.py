#!/usr/bin/env python3
"""
Инструмент извлечения метаданных из neira:meta комментариев
Генерирует CHANGELOG.md из всех файлов проекта

Использование:
    python tools/extract_changelog.py [--output CHANGELOG.md] [--format md|json]
"""
import re
import sys
import json
from pathlib import Path
from collections import defaultdict
from typing import List, Dict, Any
import argparse


class MetaExtractor:
    """Парсер neira:meta блоков из Rust и Markdown файлов"""
    
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
    
    def __init__(self, root: Path):
        self.root = root
        self.entries: List[Dict[str, Any]] = []
    
    def extract_from_file(self, filepath: Path) -> List[Dict[str, Any]]:
        """Извлечь все neira:meta из одного файла"""
        try:
            content = filepath.read_text(encoding='utf-8')
        except Exception as e:
            print(f"⚠️  Не удалось прочитать {filepath}: {e}", file=sys.stderr)
            return []
        
        # Выбираем паттерн в зависимости от расширения
        if filepath.suffix in ['.rs', '.toml']:
            pattern = self.RUST_PATTERN
        elif filepath.suffix == '.md':
            pattern = self.MD_PATTERN
        else:
            return []
        
        results = []
        for match in pattern.finditer(content):
            meta_text = match.group(1).strip()
            entry = self._parse_meta(meta_text, filepath)
            if entry:
                results.append(entry)
        
        return results
    
    def _parse_meta(self, text: str, filepath: Path) -> Dict[str, Any]:
        """Парсинг YAML-подобного формата внутри neira:meta"""
        entry = {
            'file': str(filepath.relative_to(self.root)),
            'id': None,
            'intent': None,
            'summary': None,
            'raw': text
        }
        
        # Простой парсинг ключ:значение
        lines = text.split('\n')
        current_key = None
        current_value = []
        
        for line in lines:
            line = line.strip()
            if not line:
                continue
            
            # Если строка начинается с известного ключа
            if ':' in line and not line.startswith(' '):
                if current_key and current_value:
                    entry[current_key] = ' '.join(current_value).strip()
                
                key, _, value = line.partition(':')
                current_key = key.strip()
                current_value = [value.strip()]
            else:
                # Продолжение значения
                if current_key:
                    current_value.append(line)
        
        # Сохранить последний ключ
        if current_key and current_value:
            entry[current_key] = ' '.join(current_value).strip()
        
        # Валидация: нужен хотя бы id или summary
        if not entry.get('id') and not entry.get('summary'):
            return None
        
        return entry
    
    def scan_project(self, extensions=('.rs', '.md', '.toml')):
        """Сканировать весь проект"""
        for ext in extensions:
            for filepath in self.root.rglob(f'*{ext}'):
                # Пропускаем target/, node_modules/, .git/
                if any(p in filepath.parts for p in ['target', 'node_modules', '.git', 'binutils-gdb', 'cargo', 'llvm-project', 'nasm', 'rust']):
                    continue
                
                entries = self.extract_from_file(filepath)
                self.entries.extend(entries)
        
        # Сортируем по id (обратный порядок — новые первыми)
        self.entries.sort(key=lambda x: x.get('id', ''), reverse=True)
    
    def to_markdown(self) -> str:
        """Генерация CHANGELOG в Markdown"""
        lines = [
            "# Neira Change Log",
            "",
            "Автоматически сгенерированный журнал изменений из `neira:meta` комментариев.",
            "",
            "---",
            ""
        ]
        
        # Группировка по intent
        by_intent = defaultdict(list)
        for entry in self.entries:
            intent = entry.get('intent', 'other')
            by_intent[intent].append(entry)
        
        for intent in sorted(by_intent.keys()):
            lines.append(f"## {intent.upper()}")
            lines.append("")
            
            for entry in by_intent[intent]:
                meta_id = entry.get('id', 'N/A')
                summary = entry.get('summary', 'Нет описания')
                filepath = entry.get('file', 'unknown')
                
                lines.append(f"### `{meta_id}`")
                lines.append(f"- **Файл**: `{filepath}`")
                lines.append(f"- **Суть**: {summary}")
                lines.append("")
        
        return '\n'.join(lines)
    
    def to_json(self) -> str:
        """Генерация JSON для программной обработки"""
        return json.dumps(self.entries, ensure_ascii=False, indent=2)


def main():
    parser = argparse.ArgumentParser(description='Извлечение neira:meta метаданных')
    parser.add_argument('--output', '-o', default='CHANGELOG.md', help='Путь к выходному файлу')
    parser.add_argument('--format', '-f', choices=['md', 'json'], default='md', help='Формат вывода')
    parser.add_argument('--root', '-r', default='.', help='Корневая директория проекта')
    
    args = parser.parse_args()
    
    root = Path(args.root).resolve()
    if not root.exists():
        print(f"❌ Директория {root} не найдена", file=sys.stderr)
        sys.exit(1)
    
    print(f"🔍 Сканирование {root}...")
    extractor = MetaExtractor(root)
    extractor.scan_project()
    
    print(f"✅ Найдено {len(extractor.entries)} neira:meta записей")
    
    # Генерация
    if args.format == 'md':
        content = extractor.to_markdown()
    else:
        content = extractor.to_json()
    
    # Запись
    output_path = Path(args.output)
    output_path.write_text(content, encoding='utf-8')
    print(f"📝 Записано в {output_path}")


if __name__ == '__main__':
    main()
