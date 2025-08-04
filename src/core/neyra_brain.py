"""
Мозг Нейры - здесь я думаю и учусь.
"""
from typing import List

from src.tags.tag_parser import TagParser
from src.core.neyra_config import NEYRA_GREETING

class Neyra:
    """Я Нейра, и здесь моя основная логика."""

    def __init__(self) -> None:
        """Просыпаюсь и готовлю свои модули."""
        self.parser = TagParser()
        self.known_books: List[str] = []

    def introduce_yourself(self) -> None:
        """Представляюсь пользователю."""
        print(NEYRA_GREETING)

    def load_book(self, path: str) -> None:
        """Загружаю книгу в свою память."""
        self.known_books.append(path)

    def analyze_content(self) -> None:
        """Пока просто отмечаю, что анализирую книги."""
        if self.known_books:
            print("Я обдумываю загруженные истории...")

    def process_command(self, text: str) -> str:
        """Обрабатываю текст с тегами и возвращаю результат."""
        tags = self.parser.parse_user_input(text)
        return f"Полученные теги: {[tag.type for tag in tags]}"
