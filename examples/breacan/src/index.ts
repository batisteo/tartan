// @ts-ignore
import init, { parsetartan } from 'breacan';

const input = document.getElementById("threadcount");

input.addEventListener("input", onInput);
let parseTartan;

function onInput(e: InputEvent) {
  if (parseTartan) {
    parsetartan(e.target.value, 400, 400)
  }
}

init().then(() => {
  parseTartan = parsetartan;
  parsetartan(input.value, 400, 400)
});
