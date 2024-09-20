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
    handler.set_volume(0.2)

    # handler.try_seek(100)
    await asyncio.sleep(0.2)
    

    fade_in_effect = FadeIn(start_val=0.0, end_val=1.0, apply_after=handler.get_pos(), duration=3.0)

    fade_out_effect = FadeOut(duration=6.0, apply_after=handler.get_pos() + 7.0)
    speed_up = ChangeSpeed(apply_after=0.1, end_val=1.5)

    effects_list = [speed_up]
    handler.apply_effects(effects_list)
    # handler.set_volume(0.0)
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
