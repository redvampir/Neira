import js from "@eslint/js";
import tseslint from "typescript-eslint";

export default [
  {
    // Exclude vendored sources such as the Rust and Cargo trees from linting
    // so `eslint .` only checks our project files.
    ignores: [
      "cargo/**",
      "rust/**",
      "binutils-gdb/**",
      "llvm-project/**",
      "nasm/**",
    ],
  },
  js.configs.recommended,
  ...tseslint.configs.recommended,
];
