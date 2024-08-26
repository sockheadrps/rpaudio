# rpaudio

`rpaudio` is a Rust-based Python library for handling audio operations, designed to provide simple and efficient audio management. It leverages Rust's performance and concurrency safety to offer a robust solution for Python audio applications.


## API

- **AudioSink**: Simple audio access and control for individual files.
- **AudioChannel**: Handle and process audio files in a queue via channels.
- **ChannelManager**: Multi-channel grouping and management.
- **AudioSink.metadata** Access information about audio files if present.

**Supports: MP3, WAV, Vorbis and Flac (mp4 + AAC will also be supported in a future release)**

**Python 3.8+**



## Getting Started ([Read the Docs](https://sockheadrps.github.io/rpaudio/))


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


## OS Dependency Installation Instructions


### Windows

No additional OS-level dependencies are required for `rpaudio` on Windows. Ensure you have the latest version of Python installed, and you can directly use `pip` to install the library:

```bash
pip install rpaudio
```


### macOS

To install `rpaudio` on macOS, you need to install `gettext`:


**Install `gettext`**:
```bash
brew install gettext
brew link gettext --force
```

### Linux

To install `rpaudio` on Linux, you may need to install some dependencies based on your distribution:

**For Debian/Ubuntu-based distributions**:
```bash
sudo apt-get update
sudo apt-get install -y pkg-config libasound2-dev
```

**For Red Hat/CentOS-based distributions**:
```bash
sudo yum install -y pkg-config alsa-lib-devel
```

After installing the necessary OS-level dependencies, you can install `rpaudio` using `pip`:

```bash
pip install rpaudio
```

