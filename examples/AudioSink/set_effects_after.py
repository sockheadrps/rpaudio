import rpaudio
import asyncio
from rpaudio import FadeIn

kill_audio = False
AUDIO_FILE = r"C:\Users\16145\Desktop\exc.mp3"

def on_audio_stop():
    global kill_audio
    kill_audio = True
    print("Audio has stopped")

async def play_audio():
    handler = rpaudio.AudioSink(callback=on_audio_stop).load_audio(AUDIO_FILE)
    handler.set_volume(0.1)

    initial_fade = FadeIn()
    delayed_fade = FadeIn(apply_after=10.0, duration=6.0, end_vol=1.0, start_vol=handler.get_volume())
    effects_list = [initial_fade, delayed_fade]
    # handler.apply_effects(effects_list)
    handler.play()

    while not kill_audio:
        await asyncio.sleep(1)
        print(f"FROM PYTHON: {handler.get_volume()}")
        

async def sleep_loop():
    global kill_audio
    i = 0
    while not kill_audio:
        await asyncio.sleep(1)
        i += 1

async def main():
    await asyncio.gather(play_audio(), sleep_loop())

asyncio.run(main())
