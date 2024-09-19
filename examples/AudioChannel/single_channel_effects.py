import rpaudio
from rpaudio import FadeIn, FadeOut
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
    await asyncio.sleep(2)
    channel.auto_consume = True
    # print(channel.current_audio )
    first = channel.current_audio


    while not complete_1:
        await asyncio.sleep(1)


        if channel.current_audio is None or not first.is_playing:
            # print("complete_1")
            # print(f"metadata: {channel.current_audio.metadata}")
            # channel.current_audio.try_seek(150)
            complete_1 = True

    while complete_1 and not complete_2:
        await asyncio.sleep(1)

        

        # Check if the current audio has stopped
        if channel.current_audio is None or not channel.current_audio.is_playing:
            complete_2 = True

    print("Playback complete for both phases")

async def sleep_loop() -> None:
    global complete_1, complete_2
    while True:
        # print("Sleeping...")
        if complete_1 and complete_2:
            break
        await asyncio.sleep(1)

async def main() -> None:
    audio_1 = rpaudio.AudioSink(callback=on_audio_stop)
    audio_1.load_audio("examples/ex.wav")

    channels = audio_1.metadata["channels"]
    duration = audio_1.metadata["duration"]
    print(f"Channels: {channels}, Duration: {duration}")

    audio_2 = rpaudio.AudioSink(callback=on_audio_stop)
    audio_2.load_audio("examples/ex.wav")
    # audio_2.set_duration(226.0)

    channel_1 = rpaudio.AudioChannel()
    fade_in_effect = FadeIn(start_val=0.0, end_val=1.0, duration=3.0)
    fade_out_effect = FadeOut(end_val=0.0, duration=3.0)
    # audio_1.try_seek(8)

    effects = [fade_in_effect, fade_out_effect]
    channel_1.set_effects_chain(effects)
    channel_1.auto_consume = True
    channel_1.push(audio_1)
    channel_1.push(audio_2)

    # Await the play_audio and sleep_loop functions concurrently
    await asyncio.gather(play_audio(channel_1), sleep_loop())

asyncio.run(main())
