# craby-modules

TypeScript definitions and runtime for Craby modules

## Installation

```bash
npm install craby-modules
# or
pnpm add craby-modules
# or
yarn add craby-modules
```

## Example

```ts
import type { NativeModule } from 'craby-modules';
import { NativeModuleRegistry } from 'craby-modules';

export interface Spec extends NativeModule {
  add(a: number, b: number): number;
  subtract(a: number, b: number): number;
  multiply(a: number, b: number): number;
  divide(a: number, b: number): number;
}

export default NativeModuleRegistry.getEnforcing<Spec>('Calculator');
```

Visit [https://craby.rs](https://craby.rs) for full documentation.

## License

[MIT](LICENSE)
