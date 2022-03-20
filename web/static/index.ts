import init, { run_app } from '../pkg'

(async () => {
  console.log(await init());
  run_app();
})()
