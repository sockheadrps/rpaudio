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
    handler.set_volume(0.02)
    effects_list = [FadeIn()]
    handler.set_effects(effects_list)
    handler.play()
    handler.try_seek(200)

    while not kill_audio:
        await asyncio.sleep(1)
        print(f"vol: {handler.get_volume()}")
        

async def sleep_loop():
    global kill_audio
    while not kill_audio:
        await asyncio.sleep(1)

async def main():
    await asyncio.gather(play_audio(), sleep_loop())

asyncio.run(main())
