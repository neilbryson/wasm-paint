import React, { FunctionComponent, useEffect, useRef } from 'react';

import { render } from './pkg';
import './styles/styles.css';

export const App: FunctionComponent = () => {
  const canvasRef = useRef<HTMLDivElement | null>(null);
  const shouldRenderCanvas = useRef(true);

  useEffect(() => {
    // Call render only once.
    if (shouldRenderCanvas.current && canvasRef.current) {
      shouldRenderCanvas.current = false;

      const { height, width } = canvasRef.current.getBoundingClientRect();
      render(height, width);
    }
  });

  return (
    <div className="app" id="app--container">
      <div id="app--canvas-container" ref={canvasRef} />
      <div id="app--color-picker-container">
        Colour picker will appear here
      </div>
    </div>
  );
};
