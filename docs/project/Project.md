
Project structure

```plaintext
├── bin
│   ├── vsp
│   ├── vspc -> vsp
│   ├── vspdb -> vsp
│   ├── vspm -> vsp
│   └── vspsh -> vsp
├── conf
│   └── env
├── etc -> conf
├── legal
├── lib
│   ├── src.tar.gz
│   └── tzdb.dat
├── share
├── COPYRIGHT
└── README.md
```

Runtime edition only provides minimum runtime environment for compiler without develop toolkits.

```plaintext
├── bin
│   ├── vsp
│   └── vspc -> vsp
├── conf
│   └── env
├── etc -> conf
├── legal
├── lib
│   ├── src.tar.gz
│   └── tzdb.dat
├── share
├── COPYRIGHT
└── README.md
```

while the container edition provides a single binary executable.

```plaintext
└── bin
    ├── vsp
    └── vspc -> vsp
```
