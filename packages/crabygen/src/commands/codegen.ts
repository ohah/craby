import { Command } from '@commander-js/extra-typings';
import { getBindings } from '../utils/bindings';
import { withVerbose } from '../utils/command';
import { resolveProjectRoot } from '../utils/resolve-project-root';

export async function runCodegen() {
  getBindings().codegen({ projectRoot: resolveProjectRoot() });
}

export const command = withVerbose(new Command().name('codegen').action(runCodegen));
