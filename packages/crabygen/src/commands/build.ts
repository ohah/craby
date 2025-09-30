import { Command } from '@commander-js/extra-typings';
import { getBindings } from '../utils/bindings';
import { withVerbose } from '../utils/command';
import { resolveProjectRoot } from '../utils/resolve-project-root';

export const command = withVerbose(
  new Command().name('build').action(async () => {
    getBindings().build({ projectRoot: resolveProjectRoot() });
  }),
);
