
To run Back-end server.

```bash
cd back-end
cargo run

```


To run Front-end server.

```bash
cd front-end
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
dx serve --hot-reload

```

To do E2E test,

```bash
cd E2E
pip install -r requirements.txt
python main.py
```
To run both server, you can execute start.bat file.
