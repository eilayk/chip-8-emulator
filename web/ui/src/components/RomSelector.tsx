import React, { useState } from 'react';
import { PRELOADED_ROMS } from '@/lib/roms';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Input } from '@/components/ui/input';

interface RomSelectorProps {
  onRomLoaded: (data: Uint8Array) => void;
}

export const RomSelector: React.FC<RomSelectorProps> = ({ onRomLoaded }) => {
  const [selectedRom, setSelectedRom] = useState<string>(PRELOADED_ROMS[0].path);

  const handleRomChange = async (value: string) => {
    setSelectedRom(value);
    if (value) {
        try {
            const response = await fetch(value);
            const buffer = await response.arrayBuffer();
            onRomLoaded(new Uint8Array(buffer));
        } catch (err) {
            console.error("Failed to load ROM", err);
        }
    }
  };

  const handleFileUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
      const file = e.target.files?.[0];
      if (!file) return;

      try {
        const buffer = await file.arrayBuffer()
        onRomLoaded(new Uint8Array(buffer));
      }
      catch (err) {
        console.error("Failed to load uploaded ROM", err);
      }
  };

  return (
    <div className="flex flex-row w-full max-w-3xl gap-4 flex-1">
        <div className="flex flex-col gap-2 flex-1">
          <label className="text-sm font-medium">Select a ROM</label>
          <Select value={selectedRom} onValueChange={handleRomChange}>
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select a ROM" />
            </SelectTrigger>
            <SelectContent>
              {PRELOADED_ROMS.map((rom) => (
                <SelectItem key={rom.path} value={rom.path}>
                  {rom.name}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        <div className="flex items-center h-10 px-2 self-center">
          <span className="text-xs uppercase text-muted-foreground font-medium">Or</span>
        </div>


        <div className="flex flex-col gap-2 flex-1">
          <label className="text-sm font-medium">Upload .ch8 File</label>
          <Input 
            type="file" 
            accept=".ch8"
            onChange={handleFileUpload}
            className="cursor-pointer"
          />
        </div>
    </div>
  );
}