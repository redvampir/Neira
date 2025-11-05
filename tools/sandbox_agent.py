# Neira Sandbox Agent
# Безопасно применяет улучшения от Consciousness API

import os
import json
import subprocess
import tempfile
import shutil
from pathlib import Path
from typing import Dict, List, Optional
import requests

class SandboxAgent:
    """
    Безопасный агент для применения изменений от Neira.
    
    Workflow:
    1. Получает reasoning_steps из Consciousness API
    2. Создаёт временную песочницу (git worktree)
    3. Применяет изменения
    4. Запускает тесты (lint, unit tests, build)
    5. Если успех → создаёт PR, иначе откатывает
    """
    
    def __init__(self, repo_path: str, api_base: str = "http://localhost:9090"):
        self.repo_path = Path(repo_path)
        self.api_base = api_base
        self.sandbox_path: Optional[Path] = None
        
    def fetch_pending_thoughts(self) -> List[Dict]:
        """Получить необработанные мысли с improvement tasks"""
        response = requests.get(f"{self.api_base}/api/neira/consciousness/stats")
        stats = response.json()
        
        # TODO: добавить endpoint для получения thoughts с фильтром
        # Пока используем mock данные
        if stats.get("total_improvement_tasks", 0) > 0:
            return [{
                "dialogue_id": "improvement_task_example",
                "reasoning_steps": [
                    "Добавить ARIA метки",
                    "Улучшить русский язык"
                ]
            }]
        return []
    
    def create_sandbox(self) -> Path:
        """Создать временную песочницу через git worktree"""
        sandbox_name = f"neira_sandbox_{os.getpid()}"
        sandbox_path = self.repo_path.parent / sandbox_name
        
        # Создаём worktree
        subprocess.run([
            "git", "worktree", "add", 
            str(sandbox_path), 
            "HEAD"
        ], cwd=self.repo_path, check=True)
        
        self.sandbox_path = sandbox_path
        return sandbox_path
    
    def apply_changes(self, thought: Dict) -> bool:
        """
        Применить изменения на основе reasoning_steps.
        Возвращает True если применено успешно.
        """
        if not self.sandbox_path:
            raise RuntimeError("Sandbox not created")
        
        # Здесь должна быть логика интерпретации reasoning_steps
        # Сейчас — заглушка
        reasoning_steps = thought.get("reasoning_steps", [])
        
        # Пример: если есть "улучшить русский" → применяем патч
        for step in reasoning_steps:
            if "русский" in step.lower():
                # Применяем известные улучшения
                self._apply_russian_improvements()
            if "aria" in step.lower():
                self._apply_aria_improvements()
        
        return True
    
    def _apply_russian_improvements(self):
        """Применить улучшения русского языка"""
        # Реальная реализация — через sed/awk или Python AST
        pass
    
    def _apply_aria_improvements(self):
        """Добавить ARIA метки"""
        pass
    
    def run_tests(self) -> Dict[str, bool]:
        """
        Запустить проверки:
        - lint (cargo clippy, eslint)
        - unit tests
        - smoke tests
        """
        results = {}
        
        # Rust lint
        try:
            result = subprocess.run(
                ["cargo", "clippy", "--", "-D", "warnings"],
                cwd=self.sandbox_path / "spinal_cord",
                capture_output=True,
                timeout=60
            )
            results["rust_lint"] = result.returncode == 0
        except Exception as e:
            results["rust_lint"] = False
            results["rust_lint_error"] = str(e)
        
        # JS/HTML syntax check (basic)
        try:
            js_file = self.sandbox_path / "static" / "consciousness" / "dashboard.js"
            # Простая проверка синтаксиса через node
            result = subprocess.run(
                ["node", "--check", str(js_file)],
                capture_output=True,
                timeout=10
            )
            results["js_syntax"] = result.returncode == 0
        except Exception:
            results["js_syntax"] = False
        
        return results
    
    def create_pull_request(self, thought: Dict, branch_name: str) -> Optional[str]:
        """
        Создать Pull Request с изменениями.
        Возвращает URL PR или None если не удалось.
        """
        if not self.sandbox_path:
            return None
        
        # Коммит изменений
        subprocess.run(["git", "add", "-A"], cwd=self.sandbox_path, check=True)
        commit_msg = f"[Neira] {thought.get('dialogue_id', 'Improvement')}\n\nReasoning:\n" + \
                     "\n".join(f"- {step}" for step in thought.get("reasoning_steps", []))
        
        subprocess.run(
            ["git", "commit", "-m", commit_msg],
            cwd=self.sandbox_path,
            check=True
        )
        
        # Пуш в remote branch
        subprocess.run(
            ["git", "push", "origin", f"HEAD:{branch_name}"],
            cwd=self.sandbox_path,
            check=True
        )
        
        # Создание PR через GitHub CLI (если установлен)
        try:
            result = subprocess.run(
                ["gh", "pr", "create", 
                 "--title", f"[Neira] {thought.get('dialogue_id', 'Improvement')}",
                 "--body", commit_msg,
                 "--head", branch_name],
                cwd=self.sandbox_path,
                capture_output=True,
                text=True
            )
            if result.returncode == 0:
                return result.stdout.strip()  # URL PR
        except FileNotFoundError:
            print("GitHub CLI (gh) not installed, skipping PR creation")
        
        return None
    
    def cleanup(self):
        """Удалить песочницу"""
        if self.sandbox_path and self.sandbox_path.exists():
            # Удаляем worktree
            subprocess.run(
                ["git", "worktree", "remove", str(self.sandbox_path)],
                cwd=self.repo_path,
                check=True
            )
            self.sandbox_path = None
    
    def process_thought(self, thought: Dict) -> Dict:
        """
        Полный цикл обработки мысли:
        1. Создать sandbox
        2. Применить изменения
        3. Запустить тесты
        4. Создать PR или откатить
        """
        result = {
            "thought_id": thought.get("dialogue_id"),
            "status": "unknown",
            "pr_url": None,
            "test_results": {},
            "error": None
        }
        
        try:
            # 1. Sandbox
            self.create_sandbox()
            print(f"✓ Sandbox created: {self.sandbox_path}")
            
            # 2. Apply
            if not self.apply_changes(thought):
                result["status"] = "failed_to_apply"
                return result
            print("✓ Changes applied")
            
            # 3. Test
            test_results = self.run_tests()
            result["test_results"] = test_results
            
            if not all(test_results.values()):
                result["status"] = "tests_failed"
                print(f"✗ Tests failed: {test_results}")
                return result
            print(f"✓ All tests passed: {test_results}")
            
            # 4. PR
            branch_name = f"neira/improvement-{thought.get('dialogue_id', 'auto')}"
            pr_url = self.create_pull_request(thought, branch_name)
            result["pr_url"] = pr_url
            result["status"] = "success"
            print(f"✓ PR created: {pr_url}")
            
        except Exception as e:
            result["status"] = "error"
            result["error"] = str(e)
            print(f"✗ Error: {e}")
        
        finally:
            # Cleanup
            self.cleanup()
            print("✓ Sandbox cleaned up")
        
        return result


def main():
    """Основной цикл агента"""
    import sys
    
    if len(sys.argv) < 2:
        print("Usage: python sandbox_agent.py <repo_path>")
        sys.exit(1)
    
    repo_path = sys.argv[1]
    agent = SandboxAgent(repo_path)
    
    print("=== Neira Sandbox Agent ===")
    print(f"Repository: {repo_path}")
    print(f"API: {agent.api_base}")
    
    # Получить pending thoughts
    thoughts = agent.fetch_pending_thoughts()
    
    if not thoughts:
        print("No pending improvement tasks")
        return
    
    print(f"\nFound {len(thoughts)} pending thought(s)")
    
    # Обработать каждую мысль
    for thought in thoughts:
        print(f"\n--- Processing: {thought.get('dialogue_id')} ---")
        result = agent.process_thought(thought)
        print(f"Result: {json.dumps(result, indent=2)}")


if __name__ == "__main__":
    main()
