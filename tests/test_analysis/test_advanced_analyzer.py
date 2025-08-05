from src.analysis.advanced import AdvancedAnalyzer, AnalysisResult
from src.memory import CharacterMemory
from src.models import Character


def _build_memory() -> CharacterMemory:
    memory = CharacterMemory()
    memory.add(
        Character(
            name="Alice",
            appearance="blue eyes",
            personality_traits=["kind"],
        )
    )
    return memory


def test_analyze_generation_success() -> None:
    analyzer = AdvancedAnalyzer(memory=_build_memory())
    result = analyzer.analyze_generation("Alice with blue eyes was kind to everyone.")
    assert isinstance(result, AnalysisResult)
    assert result.spelling
    assert result.logic
    assert result.consistency
    assert result.style


def test_character_inconsistency() -> None:
    analyzer = AdvancedAnalyzer(memory=_build_memory())
    result = analyzer.analyze_generation("Alice with green eyes was cruel.")
    assert not result.consistency
