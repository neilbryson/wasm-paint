import React, { FunctionComponent, useRef } from 'react';
import './styles/styles.css';

export const App: FunctionComponent = () => {
  return (
    <div className="app" id="app--container">
      <div id="app--canvas-container" />
      <div id="app--color-picker-container" />
    </div>
  );
};
