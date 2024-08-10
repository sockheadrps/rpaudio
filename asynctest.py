import rpaudio
import asyncio

def on_audio_stop():
    print("Audio has stopped")

async def play_audio():
    handler = rpaudio.AudioHandler(callback=on_audio_stop)
    handler.load_audio("ex.wav")
    handler.play()
    count = 0
    while handler.is_playing:
        await asyncio.sleep(1)
        count += 1

        if count == 4:
            handler.pause()
            await asyncio.sleep(2)
            handler.play()
            await asyncio.sleep(1)
            handler.stop()


async def print_every_second():
    for i in range(10):
        await asyncio.sleep(1)

async def main():
    await asyncio.gather(play_audio(), print_every_second())

asyncio.run(main())
