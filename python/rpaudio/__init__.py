from .rpaudio import *
import sys

__doc__ = rpaudio.__doc__
if hasattr(rpaudio, "__all__"):
    __all__ = rpaudio.__all__

del rpaudio

sys.modules["rpaudio.effects"] = effects
sys.modules["rpaudio.exceptions"] = exceptions

