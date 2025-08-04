"""
Главный файл Нейры - здесь она просыпается и знакомится с пользователем.
"""
import logging
from pathlib import Path
from src.core.neyra_brain import Neyra
from src.interaction.dialog_controller import DialogController

def setup_logging() -> None:
    """Настраиваю систему для записи того, что думает Нейра"""
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - Нейра - %(levelname)s - %(message)s',
        handlers=[
            logging.FileHandler('logs/neyra.log', encoding='utf-8'),
            logging.StreamHandler()
        ]
    )

def main() -> None:
    """Нейра просыпается и начинает работать!"""
    setup_logging()
    logger = logging.getLogger(__name__)
    
    print("🌟 Пробуждение Нейры... 🌟")
    
    try:
        # Создаю Нейру
        neyra = Neyra()
        
        # Нейра представляется
        neyra.introduce_yourself()
        
        # Проверяю, есть ли книги для изучения
        books_dir = Path("data/books/")
        if books_dir.exists() and list(books_dir.glob("*.txt")):
            print("\n📚 Вижу книги для изучения!")
            
            for book_file in books_dir.glob("*.txt"):
                logger.info(f"Нейра изучает: {book_file.name}")
                neyra.load_book(str(book_file))
                
            neyra.analyze_content()
            
            # Демонстрация системы тегов
            demo_command = (
                "@Нейра: Создай короткую сцену@ "
                "@Стиль: загадочный@ "
                "@Эмоция: любопытство@"
            )
            
            print(f"\n🏷️ Демонстрация тегов: {demo_command}")
            result = neyra.process_command(demo_command)
            print(f"\n✨ Результат Нейры:\n{result}")
            
        else:
            print("\n📖 Пока нет книг для изучения.")
            print("Добавьте .txt файлы в папку data/books/ и я их изучу!")
            
        print("\n💫 Нейра готова к работе! Используйте теги для общения.")
        controller = DialogController(neyra)
        controller.interact()
        
    except Exception as e:  # pylint: disable=broad-except
        logger.error(f"Ошибка при пробуждении Нейры: {e}")
        print("😟 Что-то пошло не так при пробуждении Нейры...")

if __name__ == "__main__":
    main()
