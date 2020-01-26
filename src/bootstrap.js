// Asynchronous import so that WebAssembly works
import('./index').catch(error => console.error('Unable to load `index.js` ', error));
