import React from 'react';
import { App } from './App';
import {createRoot} from 'react-dom/client';

const container = document.getElementById('root');

if (!container) {
  throw new Error('Please add a #root element in public/index.html');
}

const root = createRoot(container);
root.render(<App />);
