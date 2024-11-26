@echo off
start "Back-end Server" cmd /k "cd back-end && cargo run"
start "Front-end Server" cmd /k "cd front-end && dx serve --hot-reload"