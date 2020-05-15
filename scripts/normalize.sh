#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

infile="$1"
outfile="${infile}.ogg"
tmpfile="${infile}.wav"

# Trim all silence encountered from beginning to end where there is more than 1 second of silence in audio.
# See: https://ffmpeg.org/ffmpeg-filters.html
ffmpeg -i "$infile" -af silenceremove=stop_periods=-1:stop_duration=1:stop_threshold=-90dB "$tmpfile"

ffmpeg-normalize "$tmpfile" -c:a libvorbis -o "$outfile" -f

rm "$tmpfile"
