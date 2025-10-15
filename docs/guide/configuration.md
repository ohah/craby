# Configuration

This guide covers Craby's configuration options.

## Codegen

The `craby.toml` file in your project root defines the code generation configuration:

```toml
[project]
name = "my_project"
source_dir = "src"
```

- `name`: The name of your project. This is used for naming generated modules and files.
- `source_dir`: The directory path to scan for TypeScript source files during code generation. Craby will recursively search this directory and its subdirectories to find spec files and use them as targets for code generation.

::: warning
Spec files **must** be prefixed with `Native` (e.g., `NativeCalculator.ts`) to be recognized by the code generator.
:::
