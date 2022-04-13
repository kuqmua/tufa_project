### init submodules
```
git submodule init && git submodule update && cd tufa_client && git checkout main && cd .. && cd tufa_server && git checkout main && cd ..
```

<table style="width:200px">
   <thead>
      <tr>
         <th>tufa_client</th>
         <th>tufa_server</th>
      </tr>
   </thead>
   <tbody>
      <tr>
         <td>example git submodule</td>
         <td>example git submodule</td>
      </tr>
      <tr>
         <td>git submodule add https://github.com/kuqmua/tufa_client.git```</td>
         <td>git submodule add https://github.com/kuqmua/tufa_server.git</td>
      </tr>
      <tr>
         <td>empty</td>
         <td>up databases</td>
      </tr>
      <tr>
         <td>empty</td>
         <td>cd tufa_server && sudo docker-compose up -d && cd ..</td>
      </tr>
      <tr>
         <td>empty</td>
         <td>run postgres migrations</td>
      </tr>
      <tr>
         <td>empty</td>
         <td>cd tufa_server && sqlx migrate run && cd ..</td>
      </tr>
   </tbody>
</table>

<table style="width:200px">
   <thead>
      <tr>
         <th>tufa_client</th>
         <th>tufa_server</th>
      </tr>
   </thead>
   <tbody>
      <tr>
         <td>example git submodule</td>
         <td>example git submodule</td>
      </tr>
      <tr>
         <td>git submodule add https://github.com/kuqmua/tufa_client.git</td>
         <td>git submodule add https://github.com/kuqmua/tufa_server.git</td>
      </tr>
      <tr>
         <td>empty</td>
         <td>up databases</td>
      </tr>
      <tr>
         <td>empty</td>
         <td>cd tufa_server && sudo docker-compose up -d && cd ..</td>
      </tr>
      <tr>
         <td>empty</td>
         <td>run postgres migrations</td>
      </tr>
      <tr>
         <td>empty</td>
         <td>cd tufa_server && sqlx migrate run && cd ..</td>
      </tr>
   </tbody>
</table>
