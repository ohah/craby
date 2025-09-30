import * as mod from '@craby/cli-bindings';

export type BindingMethod = keyof typeof mod;

export function getBindings() {
  return mod;
}
