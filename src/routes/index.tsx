import { SongList } from "@/components/song-list";
import { StatusButtons } from "@/components/status-buttons";
import { SongAdder } from "@/components/song-adder";
import { DeviceSelector } from "@/components/device-selector";

export function HomePage() {
  return (
    <>
      <SongAdder />
      <DeviceSelector />
      <SongList />
      <br />
      <StatusButtons />
    </>
  );
}
