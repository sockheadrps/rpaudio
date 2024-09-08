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
    handler.set_volume(0.1)
    handler.try_seek(100)

    delayed_fade_in = FadeIn(apply_after=5.0, duration=6.0, end_vol=0.9)
    delayed_fade_out = FadeOut(apply_after=11.0, duration=6.0, end_vol=0.0, start_vol=1.0)
    speed_up = ChangeSpeed(apply_after=5.0, duration=6.0, end_speed=1.4)

    effects_list = [delayed_fade_in, delayed_fade_out, speed_up]
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
