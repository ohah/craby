import { run as runCli } from './cli';
import { logger } from './logger';
import { getBindings } from './utils/bindings';

export async function run(baseCommand = 'crabygen') {
  const { setup } = getBindings();

  const verbose = Boolean(process.argv.find((arg) => arg === '-v' || arg === '--verbose'));

  try {
    setup(verbose ? 'debug' : process.env.RUST_LOG);
    runCli(baseCommand);
  } catch (error) {
    logger.error(error instanceof Error ? error.message : 'unknown error');
    process.exit(1);
  }
}
