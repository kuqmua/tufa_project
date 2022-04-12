### just init
git submodule init && git submodule update && cd tufa_client && git checkout main && cd .. && cd tufa_server && git checkout main && cd ..

### init git submodules
git submodule init && git submodule update

### submodules up to date
cd tufa_client && git checkout main && cd tufa_server && git checkout main && cd ..

### up databases
cd tufa_server && sudo docker-compose up -d && cd ..

### run postgres migrations
cd tufa_server && sqlx migrate run && cd ..

### submodules add exaples
git submodule add https://github.com/kuqmua/tufa_client.git
git submodule add https://github.com/kuqmua/tufa_server.git

<table style="width:200px">
   <thead>
      <tr>
         <th>tufa_client</th>
         <th>tufa_server</th>
      </tr>
   </thead>
   <tbody>
      <tr>
         <td>git submodule add</td>
         <td>https://github.com/kuqmua/tufa_client.git</td>
      </tr>
      <tr>
         <td>git submodule add</td>
         <td>https://github.com/kuqmua/tufa_server.git</td>
      </tr>
   </tbody>
</table>
