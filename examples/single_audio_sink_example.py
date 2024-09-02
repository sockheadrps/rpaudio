import rpaudio
import asyncio

def on_audio_stop():
    print("Audio has stopped")

async def play_audio():
    handler = rpaudio.AudioSink(callback=on_audio_stop).load_audio(r"C:\Users\16145\Desktop\exc.mp3")
    print(handler.metadata)

    # Set a fade in on the audio sink
    handler.set_fade_in(5.0, 0.1, 0.2)

    handler.play()
    count = 0
    while True:
        await asyncio.sleep(1)

        count += 1

        if count == 6:
            # Pause the audio for 2 seconds
            print ("Pausing audio")
            handler.pause()
            print(handler.get_volume())


        if count == 7:
            # Resume the audio, but turn down the volume
            print("Resuming audio, lowering volume")
            # handler.set_volume(1.0)
            handler.play()

        if count == 8:
            # Seek to 10 seconds
            print(f"Current position: {handler.get_pos()}")
            handler.try_seek(33.5)
            await asyncio.sleep(1)
            print(f"Position after seek: {handler.get_pos()}")

        if count == 10:
            # Change the playback speed to 1.5
            handler.set_speed(1.5)
            print(f"Playback speed: {handler.get_speed()}")

        if count == 12:
            # Stop the audio
            handler.stop()


async def sleep_loop():
    for i in range(10):
        print(f"Sleeping {i}")
        await asyncio.sleep(1)

async def main():
    await asyncio.gather(play_audio(), sleep_loop())

asyncio.run(main())
