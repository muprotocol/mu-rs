import './style.css';
import javascriptLogo from './javascript.svg';
import muLogo from '/mu.svg';
import { setupCounter } from './counter.js';

document.querySelector('#app').innerHTML = `
  <div>
    <a href="https://muprotocol.io" target="_blank">
      <img src="${muLogo}" class="logo" alt="Vite logo" />
    </a>
    <a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript" target="_blank">
      <img src="${javascriptLogo}" class="logo vanilla" alt="JavaScript logo" />
    </a>
    <h1>Hello Mu!</h1>
    <div class="card">
      <button id="counter" type="button"></button>
    </div>
  </div>
`;
setupCounter(document.querySelector('#counter'));