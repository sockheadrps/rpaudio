# rpaudio

`rpaudio` is a Rust-based Python library for handling audio operations, designed to provide simple and efficient audio management. It leverages Rust's performance and concurrency safety to offer a robust solution for Python audio applications.


## API

- **AudioSink**: Simple audio access and control for individual files.
- **AudioChannel**: Handle and process audio files in a queue via channels.
- **ChannelManager**: Multi-channel grouping and management.
- **AudioSink.metadata** Access information about audio files if present.
- **Effects** : FadeIn, FadeOut, ChangeSpeed

**Supports: MP3, WAV, Vorbis and Flac (mp4 + AAC will also be supported in a future release)**

**Python 3.8+**



## Getting Started ([Read the Docs](https://sockheadrps.github.io/rpaudio/))


```py
import rpaudio
import asyncio
from rpaudio import FadeIn, FadeOut, ChangeSpeed

kill_audio = False
AUDIO_FILE = r"C:\Users\16145\Desktop\exc.mp3"

def on_audio_stop():
    global kill_audio
    kill_audio = True
    print("Audio has stopped")

async def play_audio():
    handler = rpaudio.AudioSink(callback=on_audio_stop).load_audio(AUDIO_FILE)
    print(handler.metadata)
    await asyncio.sleep(0.5)
    

    fade_in_effect = FadeIn(start_val=0.0, end_val=1.0, apply_after=handler.get_pos(), duration=3.0)

    fade_out_effect = FadeOut(duration=6.0, apply_after=handler.get_pos() + 7.0)
    speed_up = ChangeSpeed(apply_after=3.0, end_val=1.5, duration=3.0)


    effects_list = [speed_up, fade_in_effect, fade_out_effect]
    handler.apply_effects(effects_list)
    handler.set_volume(0.0)
    handler.play()

    while not kill_audio:
        await asyncio.sleep(1)

async def sleep_loop():
    global kill_audio
    i = 0
    while not kill_audio:
        await asyncio.sleep(1)
        i += 1

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


# Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute.
