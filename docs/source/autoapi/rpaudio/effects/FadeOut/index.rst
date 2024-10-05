rpaudio.effects.FadeOut
=======================

.. py:module:: rpaudio.effects.FadeOut




Module Contents
---------------

.. py:class:: FadeOut(duration=5.0, start_val=1.0, end_val=None, apply_after=None)

   Represents a fade-out effect for audio.

   :param duration: Duration of the fade-out effect in seconds. Defaults to 5.0.
   :param start_val: Starting volume value. Defaults to 1.0.
   :param end_val: Ending volume value. Defaults to None.
   :param apply_after: Time in seconds after which to apply the effect. Defaults to None.

   Example:

   .. code-block:: python

       fade_out = FadeOut(duration=4.0, start_val=1.0, end_val=0.0)
       # Applies a fade-out effect over 4 seconds, fading from full volume (1.0) to silence (0.0)


