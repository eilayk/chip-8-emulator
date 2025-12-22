import { useEffect, useRef, useState, useCallback, use } from 'react';
import { Chip8Wasm } from 'web';

const KEY_MAP: { [key: string]: number } = {
  '1': 0x1, '2': 0x2, '3': 0x3, '4': 0xC,
  'q': 0x4, 'w': 0x5, 'e': 0x6, 'r': 0xD,
  'a': 0x7, 's': 0x8, 'd': 0x9, 'f': 0xE,
  'z': 0xA, 'x': 0x0, 'c': 0xB, 'v': 0xF,
};

export function useChip8(onDraw?: (display: Uint8Array) => void) {
  const [chip8, setChip8] = useState<Chip8Wasm | null>(null);
  const requestRef = useRef<number | null>(null);
  const keysPressed = useRef<Uint8Array>(new Uint8Array(16));
  const [isPlaying, setIsPlaying] = useState(false);

  useEffect(() => {
    const c8 = new Chip8Wasm();
    setChip8(c8);
  }, []);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      const key = e.key.toLowerCase();
      if (KEY_MAP[key] !== undefined) {
        keysPressed.current[KEY_MAP[key]] = 1;
      }
    };

    const handleKeyUp = (e: KeyboardEvent) => {
      const key = e.key.toLowerCase();
      if (KEY_MAP[key] !== undefined) {
        keysPressed.current[KEY_MAP[key]] = 0;
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    window.addEventListener('keyup', handleKeyUp);

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('keyup', handleKeyUp);
    };
  }, []);

  const loadRom = useCallback((data: Uint8Array) => {
    if (!chip8) return;
    try {
        chip8.load_rom(data);
        setIsPlaying(true);
    } catch (e) {
        console.error("Failed to load ROM:", e);
    }
  }, [chip8]);

  const loop = useCallback(() => {
    if (!chip8 || !isPlaying) return;

    // Run multiple cycles per frame for better speed
    for (let i = 0; i < 10; i++) {
        chip8.set_pressed_keys(keysPressed.current);
        chip8.cycle();
    }
    chip8.tick_timers();

    if (onDraw) {
        onDraw(chip8.get_display());
    }

    requestRef.current = requestAnimationFrame(loop);
  }, [chip8, isPlaying, onDraw]);

  useEffect(() => {
    if (isPlaying) {
        requestRef.current = requestAnimationFrame(loop);
    }
    return () => {
        if (requestRef.current) cancelAnimationFrame(requestRef.current);
    };
  }, [isPlaying, loop]);

  return { chip8, loadRom, isPlaying, setIsPlaying };
}
