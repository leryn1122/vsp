How to update `tzdb` from [IANA](https://www.iana.org/time-zones):

```bash
VERSION=$(curl --proto '=https' --tlsv1.2 -sSf https://data.iana.org/time-zones/tzdb/version)

curl --proto '=https' --tlsv1.2 -sSf -o tzdb.tar.gz -O "https://data.iana.org/time-zones/releases/tzdata${VERSION}.tar.gz"
```
