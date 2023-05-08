### Config

Create an .env file and paste your apikey like so:

TWELVE_SECRET=xyz

### Local Docker Setup

Build
```
docker build -t twelve .
```

Run
```
docker run -it --rm --name twelve-rs twelve
```