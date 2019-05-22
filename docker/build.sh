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
<<<<<<< HEAD
	-p 8000:8000 -d \
	mcalendar
=======
	-p 8000:8000 -d mcalendar
>>>>>>> 38b9bc8d5527697e4d87cb0f998618ffe67186d3

sudo firewall-cmd --zone=public --add-port=8000/tcp --permanent

sudo firewall-cmd --reload

docker container ls