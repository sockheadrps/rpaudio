import rpaudio
import asyncio

def on_audio_stop():
    print("Audio has stopped")

async def play_audio():
    handler = rpaudio.AudioSink(callback=on_audio_stop)
    handler.load_audio("Acrylic.mp3")

    handler.play()
    count = 0
    while handler.is_playing:
        await asyncio.sleep(1)
        count += 1
        print(f"is_playing: {handler.is_playing}, count: {count}")

        if count == 4:
            handler.pause()
            await asyncio.sleep(2)
            handler.play()
            handler.set_volume(0.5)

            print(f"Current position: {handler.get_pos()}")
            handler.try_seek(10.0) 
            print(f"Position after seek: {handler.get_pos()}")
            handler.set_speed(1.5) 
            print(f"Playback speed: {handler.get_speed()}")

            await asyncio.sleep(10)
            handler.stop()
        if handler.is_playing is False:
            break

async def sleep_loop():
    for i in range(10):
        print(f"Sleeping {i}")
        await asyncio.sleep(1)

async def main():
    await asyncio.gather(play_audio(), sleep_loop())

asyncio.run(main())
