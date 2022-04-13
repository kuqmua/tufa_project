### init submodules
```
git submodule init && git submodule update && cd tufa_client && git checkout main && cd .. && cd tufa_server && git checkout main && cd ..
```
### example git submodule
```
git submodule add https://github.com/kuqmua/tufa_client.git
```
```
git submodule add https://github.com/kuqmua/tufa_server.git
```
### up databases
```
cd tufa_server && sudo docker-compose up -d && cd ..
```
### run postgres migrations
```  
cd tufa_server && sqlx migrate run && cd ..
```
