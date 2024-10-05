import python.rpaudio.rpaudio as rpaudio
from python.rpaudio.rpaudio import FadeIn, FadeOut
import asyncio
from datetime import datetime, timedelta


"""
This snippet is a test for the rpaudio module. It demonstrates control over audio playback using a single AudioChannel class.
"""

complete_1: bool = False
complete_2: bool = False


def on_audio_stop() -> None:
    print("Audio has stopped")


async def play_audio(channel_1, channel_2) -> None:
    channel_1.auto_consume = True
    channel_2.auto_consume = True
    await asyncio.sleep(0.1)

    channel_1.current_audio.pause()
    channel_2.current_audio.pause()
    channel_1.current_audio.set_volume(0.5)
    channel_2.current_audio.set_volume(0.5)

    await asyncio.sleep(1)
    channel_1.current_audio.play()





    i = 0
    while True:
      await asyncio.sleep(1)
      i += 1

      if i == 3:
        channel_1.current_audio.pause()
      
      if i == 4:
        channel_2.current_audio.pause()

      if i == 5:
        channel_1.current_audio.play()
        channel_2.current_audio.play()

      if i == 6:
         channel_1.current_audio.set_volume(1.0)
      
      if i == 7:
        channel_2.current_audio.set_volume(1.0)

      if i == 8:
        channel_1.current_audio.stop()
          
        
        



async def sleep_loop() -> None:
    global complete_1, complete_2
    while True:
        print("Sleeping...")
        if complete_1 and complete_2:
            break
        await asyncio.sleep(1)


async def main() -> None:
    AUDIO_FILE = r"C:\Users\16145\Desktop\exc.mp3"
    AUDIO_FILE_2 = r"C:\Users\16145\Desktop\a2.mp3"

    audio_1 = rpaudio.AudioSink(
        callback=on_audio_stop).load_audio(AUDIO_FILE)
    audio_2 = rpaudio.AudioSink(
        callback=on_audio_stop).load_audio(AUDIO_FILE_2)
    channel_l = rpaudio.AudioChannel()
    channel_l.push(audio_1)
    channel_l.push(audio_2)

    audio_3 = rpaudio.AudioSink(
        callback=on_audio_stop).load_audio(AUDIO_FILE)
    audio_4 = rpaudio.AudioSink(
        callback=on_audio_stop).load_audio(AUDIO_FILE_2)
    channel_2 = rpaudio.AudioChannel()
    channel_2.push(audio_3)
    channel_2.push(audio_4)


    await asyncio.gather(play_audio(channel_l, channel_2), sleep_loop())

asyncio.run(main())
vvvvvvv