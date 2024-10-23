import  rpaudio
import asyncio
from rpaudio.effects import FadeIn, FadeOut, ChangeSpeed

kill_audio = False
AUDIO_FILE = r"C:\Users\16145\Desktop\exc.mp3"

def on_audio_stop():
    global kill_audio
    kill_audio = True
    print("Audio has stopped")

async def play_audio():
    handler = rpaudio.AudioSink(callback=on_audio_stop).load_audio(AUDIO_FILE)
    print(handler.metadata.as_dict())
    await asyncio.sleep(0.3)
    handler.set_volume(0.0)

    fade_in_effect = FadeIn(duration=10.0, apply_after=0.0)
    fade_out_effect = FadeOut(duration=10.0, apply_after=10.0)
    speed_up = ChangeSpeed(apply_after=5.0, end_val=1.5, duration=3.0)

    effects_list = [fade_in_effect, fade_out_effect, speed_up]
    handler.apply_effects(effects_list)
    handler.play()

    i = 0

    while not kill_audio:
        i += 1
        await asyncio.sleep(1)
        print(i)
        if i == 15:
            handler.stop()

async def sleep_loop():
    global kill_audio
    i = 0
    while not kill_audio:
        await asyncio.sleep(1)
        i += 1

async def main():
    await asyncio.gather(play_audio(), sleep_loop())

asyncio.run(main())
