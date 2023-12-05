#!/bin/sh
count=0
while true
do
  defmt-print -e $1 tcp --port 6666
  if [ $? -ne 0 ]; then
    count=$((count+1))
    echo "Failed to connect to OpenOCD RTT tcp server, retrying in 2 seconds"
    sleep 2
    clear
  else
    break
  fi
done
