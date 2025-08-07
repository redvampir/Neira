"""Minimal fine-tuning of Qwen2.5-Coder-1.5B using Unsloth and TRL."""

from __future__ import annotations

import argparse
from pathlib import Path

from datasets import load_dataset
from transformers import TrainingArguments
from trl import SFTTrainer
from unsloth import FastLanguageModel


# ---------------------------------------------------------------------------
# Argument parsing

def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Fine-tune Qwen2.5-Coder-1.5B with Unsloth and TRL"
    )
    parser.add_argument(
        "--dataset_path",
        type=str,
        required=True,
        help="Path to a JSON dataset with 'prompt' and 'response' fields",
    )
    parser.add_argument(
        "--output_dir",
        type=str,
        required=True,
        help="Directory to store the fine-tuned model",
    )
    parser.add_argument(
        "--quantization",
        type=str,
        choices=["4bit", "8bit", "none"],
        default="4bit",
        help="Quantization mode for model weights",
    )
    parser.add_argument(
        "--epochs", type=int, default=1, help="Number of training epochs"
    )
    return parser.parse_args()


# ---------------------------------------------------------------------------
# Dataset formatting


def format_example(example: dict, tokenizer) -> str:
    """Build a chat prompt from a dataset example."""

    messages = [
        {"role": "user", "content": example["prompt"]},
        {"role": "assistant", "content": example["response"]},
    ]
    return tokenizer.apply_chat_template(messages, tokenize=False)


# ---------------------------------------------------------------------------
# Main training routine


def main() -> None:
    args = parse_args()

    dataset = load_dataset("json", data_files=args.dataset_path)["train"]

    load_in_4bit = args.quantization == "4bit"
    load_in_8bit = args.quantization == "8bit"

    model, tokenizer = FastLanguageModel.from_pretrained(
        model_name="Qwen/Qwen2.5-Coder-1.5B-Instruct",
        load_in_4bit=load_in_4bit,
        load_in_8bit=load_in_8bit,
        device_map="auto",
    )

    training_args = TrainingArguments(
        output_dir=args.output_dir,
        per_device_train_batch_size=1,
        gradient_accumulation_steps=4,
        num_train_epochs=args.epochs,
        logging_steps=10,
        save_steps=50,
        learning_rate=2e-4,
    )

    trainer = SFTTrainer(
        model=model,
        tokenizer=tokenizer,
        args=training_args,
        train_dataset=dataset,
        formatting_func=lambda ex: format_example(ex, tokenizer),
    )

    trainer.train()

    Path(args.output_dir).mkdir(parents=True, exist_ok=True)
    model.save_pretrained(args.output_dir)
    tokenizer.save_pretrained(args.output_dir)


if __name__ == "__main__":
    main()
