from src.core.neyra_brain import Neyra
from src.interaction import ChatSession


def test_chat_session_follows_character_context():
    """Follow-up questions should reuse last mentioned character."""
    neyra = Neyra()
    chat = ChatSession(neyra)

    first = chat.ask("Расскажи, как выглядел Вилл")
    assert "Вилл" in first

    second = chat.ask("А как он говорит?")
    assert "Вилл" in second
    assert any(word in second.lower() for word in ["говор", "реч"])

    # Two user messages and two Neyra responses
    assert len(chat.history) == 4


def test_service_commands_manage_history() -> None:
    """Service commands like /status, /memory and /clear work."""

    neyra = Neyra()
    chat = ChatSession(neyra)

    chat.ask("Расскажи, как выглядел Вилл")
    status = chat._handle_service_command("/status")
    assert "Записей" in status
    assert "Вилл" in status

    memory = chat._handle_service_command("/memory")
    assert "Вилл" in memory

    chat._handle_service_command("/clear")
    assert chat.history == []
