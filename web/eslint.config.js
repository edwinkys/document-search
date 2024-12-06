import eslintjs from "@eslint/js"
import eslintts from "typescript-eslint"
import prettier from "eslint-config-prettier"
import svelte from "eslint-plugin-svelte"
import globals from "globals"
import { includeIgnoreFile } from "@eslint/compat"
import { fileURLToPath } from "node:url"

const gitignoreURL = new URL("../.gitignore", import.meta.url)
const gitignorePath = fileURLToPath(gitignoreURL)

export default eslintts.config(
  includeIgnoreFile(gitignorePath),
  prettier,
  eslintjs.configs.recommended,
  ...eslintts.configs.recommended,
  ...svelte.configs["flat/recommended"],
  ...svelte.configs["flat/prettier"],
  {
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node
      }
    }
  },
  {
    files: ["**/*.svelte"],
    languageOptions: {
      parserOptions: {
        parser: eslintts.parser
      }
    }
  }
)
