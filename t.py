import rpaudio
from time import sleep

audio_handler = rpaudio.AudioHandler()
audio_handler.load_audio("ex.wav")
print("Audio loaded")
sleep(1)  # Allow some time for the audio to load

audio_handler.play()
print("Audio playing:", audio_handler.is_playing())
sleep(2)
audio_handler.pause()
print("Audio paused after pause:", audio_handler.is_playing())
sleep(2)
audio_handler.resume()
print("Audio playing after resume:", audio_handler.is_playing())
sleep(2)
print("Stopping audio")
audio_handler.stop()
print("Audio playing after stop:", audio_handler.is_playing())
