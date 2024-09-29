============================= test session starts =============================
platform win32 -- Python 3.11.4, pytest-8.3.2, pluggy-1.5.0 -- C:\Users\16145\Desktop\code_24\frpaudio\rpaudio\.venv\Scripts\python.exe
cachedir: .pytest_cache
rootdir: C:\Users\16145\Desktop\code_24\frpaudio\rpaudio
configfile: pyproject.toml
plugins: anyio-4.6.0, asyncio-0.24.0
asyncio: mode=Mode.STRICT, default_loop_scope=None
collecting ... collected 54 items

tests/test_AudioSink.py::test_fade_in PASSED                             [  1%]
tests/test_AudioSink.py::test_fade_out PASSED                            [  3%]
tests/test_AudioSink.py::test_change_speed PASSED                        [  5%]
tests/test_AudioSink.py::test_effects_applied_over_time PASSED           [  7%]
tests/test_AudioSink.py::test_effect_completion PASSED                   [  9%]
tests/test_AudioSink.py::test_set_duration PASSED                        [ 11%]
tests/test_AudioSink.py::test_set_volume_valid_input PASSED              [ 12%]
tests/test_AudioSink.py::test_set_volume_out_of_range PASSED             [ 14%]
tests/test_AudioSink.py::test_volume_uninitialized_sink PASSED           [ 16%]
tests/test_AudioSink.py::test_get_volume_returns_correct_value PASSED    [ 18%]
tests/test_AudioSink.py::test_cancel_callback PASSED                     [ 20%]
tests/test_AudioSink.py::test_stop_uninitialized_sink PASSED             [ 22%]
tests/test_AudioSink.py::test_pause_uninitialized_sink PASSED            [ 24%]
tests/test_AudioSink.py::test_no_file_provided PASSED                    [ 25%]
tests/test_AudioSink.py::test_audio_handler_no_callback PASSED           [ 27%]
tests/test_AudioSink.py::test_load_audio_multiple_times PASSED           [ 29%]
tests/test_AudioSink.py::test_audio_handler_callback PASSED              [ 31%]
tests/test_AudioSink.py::test_default_values PASSED                      [ 33%]
tests/test_AudioSink.py::test_play_audio PASSED                          [ 35%]
tests/test_AudioSink.py::test_pause PASSED                               [ 37%]
tests/test_AudioSink.py::test_resume PASSED                              [ 38%]
tests/test_AudioSink.py::test_set_volume PASSED                          [ 40%]
tests/test_AudioSink.py::test_try_seek PASSED                            [ 42%]
tests/test_AudioSink.py::test_get_pos PASSED                             [ 44%]
tests/test_AudioSink.py::test_set_speed PASSED                           [ 46%]
tests/test_AudioSink.py::test_get_speed PASSED                           [ 48%]
tests/test_AudioSink.py::test_stop PASSED                                [ 50%]
tests/test_AudioSink.py::test_metadata PASSED                            [ 51%]
tests/test_AudioSink.py::test_callback_called PASSED                     [ 53%]
tests/test_AudioSink_metadata.py::test_metadata_wav PASSED               [ 55%]
tests/test_AudioSink_metadata.py::test_metadata_mp3 PASSED               [ 57%]
tests/test_AudioSink_metadata.py::test_metadata_flac PASSED              [ 59%]
tests/test_ChannelManager.py::test_add_channel PASSED                    [ 61%]
tests/test_ChannelManager.py::test_channel_retrieval PASSED              [ 62%]
tests/test_ChannelManager.py::test_channel_retrieval_not_found PASSED    [ 64%]
tests/test_ChannelManager.py::test_start_all PASSED                      [ 66%]
tests/test_ChannelManager.py::test_stop_all PASSED                       [ 68%]
tests/test_ChannelManager.py::test_drop_channel PASSED                   [ 70%]
tests/test_ChannelManager.py::test_drop_channel_not_found PASSED         [ 72%]
tests/test_Channel_multi_sink.py::test_audio_channel_effects_chain PASSED [ 74%]
tests/test_Channel_multi_sink.py::test_audio_channel_auto_consume PASSED [ 75%]
tests/test_Channel_multi_sink.py::test_audiochannel_multiple_sink_pushes PASSED [ 77%]
tests/test_Channel_multi_sink.py::test_drop_current_audio PASSED         [ 79%]
tests/test_Channel_multi_sink.py::test_current_audio PASSED              [ 81%]
tests/test_Channel_multi_sink.py::test_autoplay_songs PASSED             [ 83%]this  ckis
this  ckis
this  ckis

tests/test_Channel_single_sink.py::test_play_audio PASSED                [ 85%]
tests/test_Channel_single_sink.py::test_pause this  ckis
PASSED                     [ 87%]
tests/test_Channel_single_sink.py::test_resume PASSED                    [ 88%]
tests/test_Channel_single_sink.py::test_set_volume PASSED                [ 90%]
tests/test_Channel_single_sink.py::test_try_seek PASSED                  [ 92%]
tests/test_Channel_single_sink.py::test_get_pos PASSED                   [ 94%]
tests/test_Channel_single_sink.py::test_get_speed PASSED                 [ 96%]
tests/test_Channel_single_sink.py::test_stop PASSED                      [ 98%]
tests/test_Channel_single_sink.py::test_callbacks_called PASSED          [100%]
 1727641033 - All tests completed.


============================= 54 passed in 16.23s =============================
this  ckis
this  ckis
this  ckis
this  ckis
this  ckis
this  ckis
this  ckis
this  ckis
this  ckis
this  ckis
this  ckis
this  ckis
this  ckis
this  ckis
