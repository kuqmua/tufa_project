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
         <th>Header1</th>
         <th>Header2</th>
         <th>Header3</th>
      </tr>
   </thead>
   <tbody>
      <tr>
         <td>data1</td>
         <td>data2</td>
         <td>data3</td>
      </tr>
      <tr>
         <td>data11</td>
         <td>data12</td>
         <td>data13</td>
      </tr>
   </tbody>
</table>

| <div style="width:290px">property</div> | description                           |
| --------------------------------------- | ------------------------------------- |
| `border-bottom-right-radius`            | Defines the shape of the bottom-right |
