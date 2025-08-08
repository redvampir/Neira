from __future__ import annotations

from pathlib import Path
from typing import Dict

from src.core.config import get_logger
from src.memory.lazy_loader import LazyMemoryLoader
from src.neurons import NeuronNetwork
from src.analysis.advanced import AdvancedAnalyzer, AnalysisResult
from src.analysis.self_corrector import SelfCorrector
from src.emotions.engine import NeyraEmotions
from src.processing.queue import ProcessingQueue
from src.processing.types import Task, Priority


class EnhancedNeyra:
    """Enhanced version of Neyra with modular components.

    The class wires together memory loading, a simple neuron network,
    analysis, self-correction, emotional tracking and task processing.
    """

    def __init__(self, memory_dir: str | Path = "data") -> None:
        self.logger = get_logger(__name__)
        self.memory_loader = LazyMemoryLoader(memory_dir)
        self.network = NeuronNetwork()
        self.analyzer = AdvancedAnalyzer()
        self.corrector = SelfCorrector()
        self.emotions = NeyraEmotions()
        self.queue = ProcessingQueue()
        self.metrics: Dict[str, float | int] = {}

    # ------------------------------------------------------------------
    def _determine_priority(self, command: str) -> tuple[Priority, str]:
        """Determine task priority from the command.

        A leading ``!`` marks a high priority task, ``?`` marks a low
        priority task, everything else is considered medium.
        The returned command is stripped from priority markers and whitespace.
        """

        cmd = command.strip()
        if cmd.startswith("!"):
            return Priority.HIGH, cmd[1:].strip()
        if cmd.startswith("?"):
            return Priority.LOW, cmd[1:].strip()
        return Priority.MEDIUM, cmd

    # ------------------------------------------------------------------
    def process_command_enhanced(self, command: str) -> str:
        """Process a command using all subsystems.

        Steps:
            1. Parse the command and assign a priority.
            2. Add the task to the queue and run it through the neuron network.
            3. Analyze and self-correct the network output.
            4. Update emotional state and record metrics.
        """

        self.logger.info("Received command: %s", command)
        priority, clean_command = self._determine_priority(command)
        self.logger.debug("Determined priority: %s", priority)

        task = Task(self.network.process, args=(clean_command,))
        self.queue.add_task(task, priority)

        attempts = 0
        result = ""
        processed = self.queue.process_next()
        attempts += 1
        if processed is not None:
            result = processed
        self.logger.debug("Network result: %s", result)

        analysis: AnalysisResult = self.analyzer.analyze_generation(result)
        self.logger.debug("Analysis: %s", analysis)

        corrected, corrections = self.corrector.correct_errors(result)
        self.logger.debug("Corrections: %s", corrections)

        success = all(getattr(analysis, field) for field in vars(analysis))
        mood_before = self.emotions.mood
        self.emotions.update_mood_from_task(clean_command, success)
        final_response = self.emotions.apply_mood_to_response(corrected)

        quality = sum(int(getattr(analysis, f)) for f in vars(analysis)) / len(vars(analysis))
        self.metrics = {
            "quality": quality,
            "attempts": attempts,
            "mood": self.emotions.mood,
            "mood_delta": self.emotions.mood - mood_before,
        }
        self.logger.info("Metrics: %s", self.metrics)

        return final_response


__all__ = ["EnhancedNeyra"]
