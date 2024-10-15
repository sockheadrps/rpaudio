
class FadeIn:
    """
    Represents a fade-in effect for audio.

    :param duration: Duration of the fade-in effect in seconds. Defaults to 5.0.
    :param start_val: Starting volume value. Defaults to None.
    :param end_val: Ending volume value. Defaults to 1.0.
    :param apply_after: Time in seconds after which to apply the effect. Defaults to None.

    Example:

    .. code-block:: python

        fade_in = FadeIn(duration=3.0, start_val=0.2, end_val=1.0)
        # Applies a fade-in effect over 3 seconds, starting from 0.2 volume to full volume (1.0)
    """

    def __init__(self, duration=5.0, start_val=None, end_val=1.0, apply_after=None):
        pass


class FadeOut:
    """
    Represents a fade-out effect for audio.

    :param duration: Duration of the fade-out effect in seconds. Defaults to 5.0.
    :param start_val: Starting volume value. Defaults to 1.0.
    :param end_val: Ending volume value. Defaults to None.
    :param apply_after: Time in seconds after which to apply the effect. Defaults to None.

    Example:

    .. code-block:: python

        fade_out = FadeOut(duration=4.0, start_val=1.0, end_val=0.0)
        # Applies a fade-out effect over 4 seconds, fading from full volume (1.0) to silence (0.0)
    """

    def __init__(self, duration=5.0, start_val=1.0, end_val=None, apply_after=None):
        pass


class ChangeSpeed:
    """
    Represents a speed change effect for audio.

    :param duration: Duration of the speed change effect in seconds. Defaults to 0.0.
    :param start_val: Starting speed value. Defaults to 1.0.
    :param end_val: Ending speed value. Defaults to 1.5.
    :param apply_after: Time in seconds after which to apply the effect. Defaults to None.

    Example:

    .. code-block:: python

        change_speed = ChangeSpeed(duration=2.0, start_val=1.0, end_val=1.2)
        # Changes audio speed over 2 seconds from normal speed (1.0) to faster (1.2)
    """

    def __init__(self, duration=0.0, start_val=1.0, end_val=1.5, apply_after=None):
        pass
