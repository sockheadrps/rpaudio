import rpaudio
import asyncio
from rpaudio import FadeIn


AUDIO_FILE = r"C:\Users\16145\Desktop\exc.mp3"
def on_audio_stop():
    print("Audio has stopped")

async def play_audio():
    handler = rpaudio.AudioSink(callback=on_audio_stop).load_audio(AUDIO_FILE)
    effects_list = [rpaudio.FadeIn()]
    handler.set_effects(effects_list)
    handler.play()

    while True:
        await asyncio.sleep(1)
        print(f"vol: {handler.get_volume()}")
        

async def sleep_loop():
    for i in range(20):
        print(f"Sleeping {i}")
        await asyncio.sleep(1)

async def main():
    await asyncio.gather(play_audio(), sleep_loop())

asyncio.run(main())
