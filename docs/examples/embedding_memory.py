from src.memory import MemoryIndex

if __name__ == "__main__":
    memory = MemoryIndex(vector_backend="faiss")
    memory.set("Earth is a planet", True)
    memory.set("Mars is red", True)
    print(memory.similar("Tell me about the red planet", k=2))
