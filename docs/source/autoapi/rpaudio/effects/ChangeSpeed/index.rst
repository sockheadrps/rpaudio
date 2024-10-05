rpaudio.effects.ChangeSpeed
===========================

.. py:module:: rpaudio.effects.ChangeSpeed




Module Contents
---------------

.. py:class:: ChangeSpeed(duration=0.0, start_val=1.0, end_val=1.5, apply_after=None)

   Represents a speed change effect for audio.

   :param duration: Duration of the speed change effect in seconds. Defaults to 0.0.
   :param start_val: Starting speed value. Defaults to 1.0.
   :param end_val: Ending speed value. Defaults to 1.5.
   :param apply_after: Time in seconds after which to apply the effect. Defaults to None.

   Example:

   .. code-block:: python

       change_speed = ChangeSpeed(duration=2.0, start_val=1.0, end_val=1.2)
       # Changes audio speed over 2 seconds from normal speed (1.0) to faster (1.2)


