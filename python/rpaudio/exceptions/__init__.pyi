
class EffectConflictException(Exception):
    """
    Exception raised when an effect manipulation conflict occurs.

    This exception is raised when a user tries to change the volume or speed of the audio
    while effects are currently being applied.

    Example:

    .. code-block:: python

        handler = rpaudio.AudioSink(callback=on_audio_stop).load_audio(AUDIO_FILE)
        await asyncio.sleep(0.3)
        handler.set_volume(0.0)

        fade_in_effect = FadeIn(duration=10.0, apply_after=0.0)

        effects_list = [fade_in_effect]
        handler.apply_effects(effects_list)
        handler.play()

        i = 0

        while not kill_audio:
            i+=1
            await asyncio.sleep(1)
            print(f"volume: {handler.get_volume()}")
            if i == 4:
                try:
                    handler.set_volume(0.5)
                except EffectConflictException as e:
                    print(f"Error: {e}")
    """