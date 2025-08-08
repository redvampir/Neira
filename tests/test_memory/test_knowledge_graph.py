from src.memory.knowledge_graph import KnowledgeGraph
import time


def test_knowledge_graph_check_claim() -> None:
    kg = KnowledgeGraph()
    kg.add_fact("Алиса", "belongs_to", "Wonderland")
    verdict, conf = kg.check_claim("Алиса принадлежит миру Wonderland")
    assert verdict is True
    assert conf == 1.0


def test_knowledge_graph_performance() -> None:
    kg = KnowledgeGraph()
    for i in range(1000):
        kg.add_fact(f"A{i}", "knows", f"B{i}")
    start = time.time()
    kg.check_claim("A1 связан с B1")
    duration = time.time() - start
    assert duration < 0.5
