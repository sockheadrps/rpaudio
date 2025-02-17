# This file is automatically generated by pyo3_stub_gen
# ruff: noqa: E501, F401

import rpaudio
import typing

class MetaData:
    title: typing.Optional[str]
    artist: typing.Optional[str]
    date: typing.Optional[str]
    year: typing.Optional[str]
    album_title: typing.Optional[str]
    album_artist: typing.Optional[str]
    track_number: typing.Optional[str]
    total_tracks: typing.Optional[str]
    disc_number: typing.Optional[str]
    total_discs: typing.Optional[str]
    genre: typing.Optional[str]
    composer: typing.Optional[str]
    comment: typing.Optional[str]
    sample_rate: typing.Optional[int]
    channels: typing.Optional[str]
    duration: typing.Optional[float]
    def __new__(cls,audio_sink:AudioSink): ...
    def as_dict(self) -> dict:
        ...


