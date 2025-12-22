import React, { useRef, useImperativeHandle, useState, useEffect } from 'react';

const SCREEN_WIDTH = 64;
const SCREEN_HEIGHT = 32;
const SCALE = 10;

export interface EmulatorCanvasHandle {
  draw: (display: Uint8Array) => void;
}

export interface EmulatorCanvasProps {
  ref: React.Ref<EmulatorCanvasHandle>;
}

const OFF_COLOR = '#1e293b'; // slate-800
const ON_COLOR = '#4ade80';  // green-400

export const EmulatorCanvas = ({ref}: EmulatorCanvasProps) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useImperativeHandle(ref, () => ({
    draw: (display: Uint8Array) => {
        const ctx = canvasRef.current?.getContext('2d');
        if (!ctx) return;
        
        // Clear screen
        ctx.fillStyle = OFF_COLOR;
        ctx.fillRect(0, 0, SCREEN_WIDTH * SCALE, SCREEN_HEIGHT * SCALE);

        // Draw pixels
        ctx.fillStyle = ON_COLOR;
        for (let i = 0; i < display.length; i++) {
            if (display[i] !== 0) {
                const x = i % SCREEN_WIDTH;
                const y = Math.floor(i / SCREEN_WIDTH);
                ctx.fillRect(x * SCALE, y * SCALE, SCALE, SCALE);
            }
        }
    }
  }));

  return (
    <canvas
      ref={canvasRef}
      width={SCREEN_WIDTH * SCALE}
      height={SCREEN_HEIGHT * SCALE}
      className="border-4 border-muted-foreground/20 rounded-lg shadow-2xl bg-muted"
    />
  );
};
