import rpaudio
import asyncio


AUDIO_FILE = r"C:\Users\16145\Desktop\exc.mp3"


def on_audio_stop():
    print("Audio has stopped")


async def play_audio():
    handler = rpaudio.AudioSink(
        callback=on_audio_stop).load_audio(AUDIO_FILE)
    print(handler.metadata_dict)
    handler.set_volume(0.5)

    handler.play()

    count = 0
    while True:
        await asyncio.sleep(1)

        count += 1
        handler.cancel_callback()

        if count == 4:
            # Pause the audio for 2 seconds
            print("Pausing audio")
            handler.set_volume(0.2)
            print(handler.get_volume())
            handler.pause()
            await asyncio.sleep(1)
            print(handler.is_playing)

        if count == 5:
            # turn down the volume
            print("Resuming audio, raise volume")
            handler.set_volume(0.5)
            handler.play()

        if count == 7:
            # Seek to 10 seconds
            print(f"Current position: {handler.get_pos()}")
            handler.try_seek(10)
            await asyncio.sleep(1)
            print(f"Position after seek: {handler.get_pos()}")

        if count == 10:
            # Stop the audio
            print(handler.get_volume())
            handler.stop()


async def sleep_loop():
    for i in range(20):
        print(f"Sleeping {i}")
        await asyncio.sleep(1)


async def main():
    await asyncio.gather(play_audio(), sleep_loop())

asyncio.run(main())
