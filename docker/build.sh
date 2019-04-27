#!/bin/sh

sudo rm -rf /home/malky/mcalendar.sled

cargo build --release

cp ../target/release/mcalendar .

cp ../example_config.toml config.toml

cp -r ../static .

cp -r ../templates .

tar -cf mcalendar.tar mcalendar config.toml static templates

scp -r malky@192.168.1.2:/home/malky/mcalendar.sled /home/malky/

docker build -t mcalendar .

docker rm -f -v mcal

docker run --rm --name mcal -p 8000:8000 -d -v /home/malky/mcalendar.sled:/var/mcalendar/mcalendar.sled  mcalendar
