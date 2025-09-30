import { Command } from '@commander-js/extra-typings';
import { getBindings } from '../utils/bindings';
import { withVerbose } from '../utils/command';

export const command = withVerbose(
  new Command()
    .name('init')
    .argument('<packageName>', 'The name of the package')
    .action(async (packageName) => {
      getBindings().init({ cwd: process.cwd(), pkgName: packageName });
    }),
);
