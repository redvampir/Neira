"""
Установочный скрипт для Нейры.
"""
from setuptools import setup, find_packages

setup(
    name="neyra",
    version="0.1.0",
    author="Создатель Нейры",
    description="Нейра - персональный ИИ-помощник для писателей",
    long_description=open("README.md", encoding="utf-8").read(),
    long_description_content_type="text/markdown",
    packages=find_packages(),
    python_requires=">=3.8",
    install_requires=open("requirements.txt", encoding="utf-8").read().splitlines(),
    setup_requires=["maturin>=1.0"],
    include_package_data=True,
    entry_points={
        "console_scripts": [
            "neyra=main:main",
        ],
    },
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Writers",
        "Topic :: Text Processing :: Creative Writing",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3.8+",
    ],
)
