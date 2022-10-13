Crate:         chrono
Version:       0.4.19
Title:         Potential segfault in `localtime_r` invocations
Date:          2020-11-10
ID:            RUSTSEC-2020-0159
URL:           https://rustsec.org/advisories/RUSTSEC-2020-0159
Solution:      No safe upgrade is available!
Dependency tree: 
chrono 0.4.19
├── tufa_server 0.1.0
├── mongodb 2.1.0
│   └── tufa_server 0.1.0
└── bson 2.1.0
    └── mongodb 2.1.0

Crate:         time
Version:       0.1.43
Title:         Potential segfault in the time crate
Date:          2020-11-18
ID:            RUSTSEC-2020-0071
URL:           https://rustsec.org/advisories/RUSTSEC-2020-0071
Solution:      Upgrade to >=0.2.23
Dependency tree: 
time 0.1.43
└── chrono 0.4.19
    ├── tufa_server 0.1.0
    ├── mongodb 2.1.0
    │   └── tufa_server 0.1.0
    └── bson 2.1.0
        └── mongodb 2.1.0

Crate:         crossbeam-utils
Version:       0.8.6
Warning:       yanked
Dependency tree: 
crossbeam-utils 0.8.6
├── sqlx-core 0.5.10
│   ├── sqlx-macros 0.5.10
│   │   └── sqlx 0.5.10
│   │       └── tufa_server 0.1.0
│   └── sqlx 0.5.10
├── crossbeam-queue 0.3.3
│   └── sqlx-core 0.5.10
└── crossbeam-channel 0.5.2
    └── sqlx-core 0.5.10

Crate:         socket2
Version:       0.4.3
Warning:       yanked
Dependency tree: 
socket2 0.4.3

error: 2 vulnerabilities found!
warning: 2 allowed warnings found
