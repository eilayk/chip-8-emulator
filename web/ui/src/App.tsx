import { useEffect, useRef } from 'react';
import { EmulatorCanvas, type EmulatorCanvasHandle } from './components/EmulatorCanvas';
import { useChip8 } from './hooks/useChip8';
import { PRELOADED_ROMS } from './lib/roms';
import { Header } from './components/Header';
import { RomSelector } from './components/RomSelector';

function App() {
  const canvasRef = useRef<EmulatorCanvasHandle>(null);

  const onDraw = (display: Uint8Array) => {
    canvasRef.current?.draw(display);
  };

  const { loadRom } = useChip8(onDraw);

  // Load default ROM on mount
  useEffect(() => {
    const loadDefault = async () => {
        try {
            const response = await fetch(PRELOADED_ROMS[0].path);
            const buffer = await response.arrayBuffer();
            loadRom(new Uint8Array(buffer));
        } catch (e) {
            console.error("Failed to load default ROM", e);
        }
    };
    loadDefault();
  }, [loadRom]);

  return (
    <div className="dark min-h-screen bg-background flex flex-col items-center justify-center p-8 gap-8 font-sans text-foreground">
      <Header />
      
      <main className="flex flex-col items-center gap-8 w-full max-w-4xl">
        <RomSelector onRomLoaded={loadRom} />
        
        <div className="flex flex-col items-center gap-6">
          <EmulatorCanvas ref={canvasRef} />
          
          <div className="text-muted-foreground text-sm font-mono bg-muted px-4 py-2 rounded border shadow-sm">
            Keypad: 1234 / QWER / ASDF / ZXCV
          </div>
        </div>
      </main>
    </div>
  );
}

export default App
