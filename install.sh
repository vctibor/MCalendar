#!/usr/bin/sh

# REQUIRES ROOT PRIVILIGES

echo "Stop systemd service"
systemctl stop mcalendar.service

echo "Remove systemd service"
systemctl delete mcalendar.service

echo "Remove systemd unit file"
rm /lib/systemd/system/mcalendar.service

echo "Uninstall old package"
yum remove -y mcalendar

echo "Install new package"
yum install -y target/release/rpmbuild/RPMS/x86_64/*

echo "Copy new systemd unit file"
cp mcalendar.service /lib/systemd/system/mcalendar.service

echo "Start systemd service"
systemctl start mcalendar.service