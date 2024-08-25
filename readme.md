# rpaudio

`rpaudio` is a Rust-based Python library for handling audio operations, designed to provide simple and efficient audio management. It leverages Rust's performance and safety features to offer a robust solution for Python audio applications.

## Features

- **Single Audio Access**: Supports simple audio API access.
- **Audio Queue Management**: Efficient handling and processing of audio files via channels.
- **Channel Management**: Manage multiple audio channels with the AudioManager.
- **Safe and Performant**: Built with Rust for optimal performance and concurrency.
- **Built in access to metadata** Easily access information about audio files

## Installation

You can install `rpaudio` via PyPI using pip:

```bash
pip install rpaudio
```

## Getting Started


```py
import rpaudio
import asyncio

def on_audio_stop():
    print("Audio has stopped")

async def play_audio():
    handler = rpaudio.AudioSink(callback=on_audio_stop).load_audio("Acrylic.mp3")

    handler.play()
    count = 0
    while True:
        await asyncio.sleep(1)
        count += 1

        if count == 4:
            # Pause the audio for 2 seconds
            print ("Pausing audio")
            handler.pause()

        if count == 6:
            # Resume the audio, but turn down the volume
            print("Resuming audio, lowering volume")
            handler.set_volume(0.5)
            handler.play()

        if count == 8:
            # Seek to 10 seconds
            print(f"Current position: {handler.get_pos()}")
            handler.try_seek(10.) 
            await asyncio.sleep(1)
            print(f"Position after seek: {handler.get_pos()}")

        if count == 10:
            # Change the playback speed to 1.5
            handler.set_speed(1.5) 
            print(f"Playback speed: {handler.get_speed()}")
            await asyncio.sleep(1)
            handler.stop()


async def sleep_loop():
    for i in range(10):
        print(f"Sleeping {i}")
        await asyncio.sleep(1)

async def main():
    await asyncio.gather(play_audio(), sleep_loop())

asyncio.run(main())
```

## Documentation

The full documentation is available at [rpaudio's Read the Docs](https://sockheadrps.github.io/rpaudio/).