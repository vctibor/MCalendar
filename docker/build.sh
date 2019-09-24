#!/bin/sh

cargo build --release

cp ../target/release/mcalendar .

cp -r ../static .

cp -r ../templates .

tar -cf mcalendar.tar mcalendar config.toml static templates

docker build -t mcalendar .

rm -rf ./static
rm -rf ./templates
rm -rf ./mcalendar
rm -rf ./mcalendar.tar

docker rm -f -v mcal

docker run --name mcal --restart unless-stopped \
	-p 192.168.196.97:8000:8000 -d mcalendar

sudo firewall-cmd --zone=public --add-port=8000/tcp --permanent

sudo firewall-cmd --reload

docker container ls
