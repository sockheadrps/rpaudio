import rpaudio
import asyncio
from datetime import datetime, timedelta


"""
This snippet is a test for the rpaudio module. It demonstrates control over audio playback using a single AudioChannel class.
"""


complete_1: bool = False
complete_2: bool = False

def on_audio_stop() -> None:
    print("Audio has stopped")

async def play_audio(channel) -> None:
    global complete_1, complete_2
    start_time: datetime = datetime.now()
    paused_once: bool = False
    channel.auto_consume = True

    while not complete_1:
        await asyncio.sleep(1)

        if channel.current_audio is not None:
            if datetime.now() - start_time > timedelta(seconds=5) and not paused_once:
                print("Pause audio after 5 seconds")
                channel.current_audio.pause()
                await asyncio.sleep(1)
                channel.current_audio.play()
                paused_once = True
                start_time = datetime.now()
            elif paused_once and datetime.now() - start_time > timedelta(seconds=2):
                await asyncio.sleep(1)
                channel.current_audio.stop()

        # Check if the current audio has stopped
        if channel.current_audio is None or not channel.current_audio.is_playing:
            complete_1 = True

    # Wait until complete_1 is True before starting the second phase
    start_time = datetime.now()
    paused_once = False
    while complete_1 and not complete_2:
        await asyncio.sleep(1)

        if channel.current_audio is not None:
            if datetime.now() - start_time > timedelta(seconds=5) and not paused_once:
                print("Pause audio after 5 seconds")
                channel.current_audio.pause()
                await asyncio.sleep(1)
                channel.current_audio.play()
                paused_once = True
                start_time = datetime.now()
            elif paused_once and datetime.now() - start_time > timedelta(seconds=2):
                await asyncio.sleep(1)
                channel.current_audio.stop()

        # Check if the current audio has stopped
        if channel.current_audio is None or not channel.current_audio.is_playing:
            complete_2 = True

    print("Playback complete for both phases")

async def sleep_loop() -> None:
    global complete_1, complete_2
    while True:
        if complete_1 and complete_2:
            break
        await asyncio.sleep(1)

async def main() -> None:
    audio_1 = rpaudio.AudioSink(callback=on_audio_stop)
    audio_1.load_audio("ex.wav")
    channels = audio_1.metadata["channels"]
    duration = audio_1.metadata["duration"]
    print(f"Channels: {channels}, Duration: {duration}")

    audio_2: rpaudio.AudioSink = rpaudio.AudioSink(callback=on_audio_stop)
    audio_2.load_audio("Acrylic.mp3")

    channel_1 = rpaudio.AudioChannel()
    channel_1.auto_consume = True
    channel_1.push(audio_1)
    channel_1.push(audio_2)
    
    await asyncio.gather(play_audio(channel_1), sleep_loop())

asyncio.run(main())
