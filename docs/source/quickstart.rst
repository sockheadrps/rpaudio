Quick Start Guide
=================

Rpaudio can be use in both synchronous and asynchronous Python code. 
To quickly get started with rpaudio, follow these steps:

1. Install rpaudio:

    .. code-block:: bash


        pip install rpaudio


2. Use rpaudio in your synchronous Python code:

    .. code-block:: python


        import rpaudio
        import time

        kill_audio = False
        AUDIO_FILE = r"C:\Users\16145\Desktop\a1.mp3"

        def on_audio_stop():
            global kill_audio
            kill_audio = True
            print("Audio has stopped")

        def play_audio():
            # Initialize and load the audio file
            handler = rpaudio.AudioSink(callback=on_audio_stop).load_audio(AUDIO_FILE)

            # Set the volume to 50% (optional)
            handler.set_volume(0.5)
            handler.play()
            
            # Loop until the audio completes, print the time elapsed
            i = 0
            while not kill_audio:
                i += 1
                time.sleep(1)
                print(i)

        if __name__ == "__main__":
            play_audio()



3. Use rpaudio in your asyncio Python code:

    .. code-block:: python


        import rpaudio
        import asyncio
        from rpaudio import FadeIn, FadeOut, ChangeSpeed

        kill_audio = False
        AUDIO_FILE = r"C:\Users\16145\Desktop\code_24\frpaudio\rpaudio\examples\ex.wav"


        def on_audio_stop():
            global kill_audio
            kill_audio = True
            print("Audio has stopped")


        async def play_audio():
            handler = rpaudio.AudioSink(callback=on_audio_stop).load_audio(AUDIO_FILE)
            print(handler.metadata)

            fade_in_effect = FadeIn(start_val=0.0, end_val=1.0, duration=3.0)
            fade_out_effect = FadeOut(duration=2.0)
            speed_up = ChangeSpeed(apply_after=1.0, end_val=1.5, duration=3.0)

            effects_list = [fade_in_effect,  fade_out_effect, speed_up]
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

