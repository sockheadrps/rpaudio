import rpaudio
import asyncio


AUDIO_FILE = r"C:\Users\16145\Desktop\exc.mp3"
AUDIO_FILE = r"examples/ex.wav"

def on_audio_stop():
    print("Audio has stopped")

async def play_audio():
    handler = rpaudio.AudioSink(callback=on_audio_stop).load_audio(AUDIO_FILE)
    print(handler.metadata)

    handler.set_volume(0.5)
    handler.play()


    # handler.play()
    count = 0
    while True:
        await asyncio.sleep(1)
        print(handler.get_volume())


        count += 1
        if count == 4:
            # Pause the audio for 2 seconds
            print ("Pausing audio")
            print(handler.get_volume())

            handler.pause()


        if count == 6:
            # Resume the audio
            # handler.play()
            print(handler.get_volume())


        if count == 7:
            # turn down the volume
            print("Resuming audio, raise volume")
            # handler.set_volume(1.0)
            handler.play()

        if count == 8:
            # Seek to 33.5 seconds
            print(f"Current position: {handler.get_pos()}")
            handler.try_seek(33.5)
            await asyncio.sleep(1)
            print(f"Position after seek: {handler.get_pos()}")

        if count == 10:
            # Change the playback speed to 1.5
            # handler.set_speed(1.5)
            print(f"Playback speed: {handler.get_speed()}")

        if count == 12:
            pass
            # Set the playback speed back to 1.0
            # handler.set_speed(1.0)
            # fade audio down to 0.2
            # handler.set_fade(5.0, handler.get_volume(), 0.2)

        if count == 18:
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
