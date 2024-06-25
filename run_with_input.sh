#!/bin/bash

# Run the application in the background
/usr/local/bin/crawle-rs &

# Get the process ID of the application
APP_PID=$!

# Loop to send 'q' input every 1 minute
while true; do
  echo "q" > /proc/$APP_PID/fd/0
  sleep 60
done

