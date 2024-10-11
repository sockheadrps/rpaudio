![Pytest](https://img.shields.io/badge/Pytest-56/56-brightgreen)
![Version](https://img.shields.io/badge/Version-0.0.13-blue)
![PyPi](https://img.shields.io/pypi/dd/rpaudio
)
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

### Example Projects
**[FastAPI LAN Audio Web Client](https://github.com/sockheadrps/RpaudioFastAPIExample)**

## Getting Started ([Read the Docs](https://sockheadrps.github.io/rpaudio/))

```py
import rpaudio
import asyncio
from rpaudio import FadeIn, FadeOut, ChangeSpeed

kill_audio = False
AUDIO_FILE = r"C:\Users\16145\Desktop\code_24\frpaudio\rpaudio\examples\ex.wav"


def on_audio_stop():
    global kill_audio
    kill_audio = True
    print("Audio has stopped")


async def play_audio():
    handler = rpaudio.AudioSink(callback=on_audio_stop).load_audio(AUDIO_FILE)
    print(handler.metadata)

    fade_in_effect = FadeIn(start_val=0.0, end_val=1.0, duration=3.0)
    fade_out_effect = FadeOut(duration=2.0)
    speed_up = ChangeSpeed(apply_after=1.0, end_val=0.8, duration=3.0)

    effects_list = [fade_in_effect,  fade_out_effect, speed_up]
    handler.apply_effects(effects_list)

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

No additional OS-level dependencies are required for `rpaudio` on Windows. Ensure you have Python 3.8+ installed, and you can directly use `pip` to install the library:

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
