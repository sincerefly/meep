# meep
:seedling: Simple file upload serve

## QuickStart

### 1）Setting env

```bash
export MEEP_IP=192.168.3.245
export MEEP_PORT=3000
export PUB_URL=http://192.168.3.245:3000
export SAVE_DIR=./meep-data
```

`MEEP_IP` and `MEEP_PORT` is meep bind server address, the submit url will be

```bash
http://192.168.3.245:3000/submit
```

The file public visit url will use `PUB_URL`

```bash
http://192.168.3.245:3000/public/eS8v3Y2lbs0o.jpg
```

### 2）Launch

```bash
git clone https://github.com/sincerefly/meep.git
cd meep
cargo build && cargo run
```

support post multi files, post files with *multipart/form-data* and results:

```json
[
  {
    "fid": "HsEyiwo0b6Iy",
    "fileName": "HsEyiwo0b6Iy.jpg",
    "fileUrl": "http://192.168.3.245:3000/public/HsEyiwo0b6Iy.jpg",
    "size": 166315
  },
  {
    "fid": "p33fnOX6Bldz",
    "fileName": "p33fnOX6Bldz.jpg",
    "fileUrl": "http://192.168.3.245:3000/public/p33fnOX6Bldz.jpg",
    "size": 14251
  }
]
```

### 3）Create Docker Image

Usage muslrust can build a musl target bin

```bash
docker run -v $PWD:/volume -t clux/muslrust cargo build --release
```

Create image with bin file

```bash
cd build-workspace
./build-image.sh
```

Then,  run container and finished~

```bash
docker run -d --name meep -p 3000:3000 -e MEEP_IP="0.0.0.0" -e MEEP_PORT=3000 -e PUB_URL="http://192.168.3.245:3000" -e SAVE_DIR="./meep-data" meep:latest /bin/sh -c 'cd /opt; ./meep'
```

