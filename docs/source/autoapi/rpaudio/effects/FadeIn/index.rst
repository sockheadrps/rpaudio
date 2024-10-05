rpaudio.effects.FadeIn
======================

.. py:module:: rpaudio.effects.FadeIn




Module Contents
---------------

.. py:class:: FadeIn(duration=5.0, start_val=None, end_val=1.0, apply_after=None)

   Represents a fade-in effect for audio.

   :param duration: Duration of the fade-in effect in seconds. Defaults to 5.0.
   :param start_val: Starting volume value. Defaults to None.
   :param end_val: Ending volume value. Defaults to 1.0.
   :param apply_after: Time in seconds after which to apply the effect. Defaults to None.

   Example:

   .. code-block:: python

       fade_in = FadeIn(duration=3.0, start_val=0.2, end_val=1.0)
       # Applies a fade-in effect over 3 seconds, starting from 0.2 volume to full volume (1.0)


