"""Главный файл Нейры - здесь она просыпается и знакомится с пользователем."""

from pathlib import Path

from src.core.config import get_logger
from src.core.neyra_brain import Neyra
from src.interaction.chat_session import ChatSession
from src.models import Character
from src.utils.source_tracker import SourceTracker


def main() -> None:
    """Нейра просыпается и начинает работать!"""
    logger = get_logger(__name__)
    tracker = SourceTracker()
    
    print("🌟 Пробуждение Нейры... 🌟")
    
    try:
        # Создаю Нейру
        neyra = Neyra()

        # Сообщаю статус LLM
        if neyra.llm and neyra.llm.is_available():
            print("\n🤖 LLM активна и готова к работе!")
        else:
            print("\n⚠️ LLM недоступна, использую творческое воображение.")

        # Демонстрация памяти
        demo_character = Character(name="Алиса", personality_traits=["смелая"])
        neyra.characters_memory.add(demo_character)
        neyra.characters_memory.save()
        stored = neyra.characters_memory.get("Алиса")
        print(f"🧠 Память персонажей: {stored}")

        # Нейра представляется
        neyra.introduce_yourself()
        
        # Проверяю, есть ли книги для изучения
        books_dir = Path("data/books/")
        if books_dir.exists() and list(books_dir.glob("*.txt")):
            print("\n📚 Вижу книги для изучения!")

            for book_file in books_dir.glob("*.txt"):
                logger.info(f"Нейра изучает: {book_file.name}")
                tracker.add(f"Изучена книга {book_file.name}", str(book_file), 0.9)
                neyra.load_book(str(book_file))
                
            neyra.analyze_content()
            
            # Демонстрация системы тегов
            demo_command = (
                "@Нейра: Создай короткую сцену@ "
                "@Стиль: загадочный@ "
                "@Эмоция: любопытство@"
            )
            
            print(f"\n🏷️ Демонстрация тегов: {demo_command}")
            result, _ = neyra.iterative_response(demo_command)
            print(f"\n✨ Результат Нейры:\n{result}")
            if tracker.get_sources():
                print("\n🔗 Использованные источники:")
                print(tracker.format_citations())
            
        else:
            print("\n📖 Пока нет книг для изучения.")
            print("Добавьте .txt файлы в папку data/books/ и я их изучу!")
            
        print("\n💫 Нейра готова к работе! Используйте теги для общения.")
        # По умолчанию запускаем интерактивный режим с поддержкой чата.
        # Диалоговый контроллер оставляем для обратной совместимости.
        chat = ChatSession(neyra)
        chat.chat_loop()
        
    except Exception as e:  # pylint: disable=broad-except
        logger.error(f"Ошибка при пробуждении Нейры: {e}")
        print("😟 Что-то пошло не так при пробуждении Нейры...")

if __name__ == "__main__":
    main()
