### init git submodules
git submodule init && git submodule update

### submodules up to date
cd tufa_client && git checkout main && cd tufa_server && git checkout main && cd ..

### submodules add exaples
git submodule add https://github.com/kuqmua/tufa_client.git
git submodule add https://github.com/kuqmua/tufa_server.git