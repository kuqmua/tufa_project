### init submodules
```
git submodule init && git submodule update && cd tufa_client && git checkout main && cd .. && cd tufa_server && git checkout main && cd ..
```
### tufa_client
<table style="width:200px">
   <thead>
      <tr>
         <th>description</th>
         <th>command</th>
      </tr>
   </thead>
   <tbody>
      <tr>
         <td>example git submodule</td>
         <td>
         ```
            git submodule add https://github.com/kuqmua/tufa_server.git
         ```
         </td>
      </tr>
   </tbody>
</table>

### tufa_server
<table style="width:200px">
   <thead>
      <tr>
         <th>description</th>
         <th>command</th>
      </tr>
   </thead>
   <tbody>
      <tr>
         <td>example git submodule</td>
         <td>
         ```
            git submodule add https://github.com/kuqmua/tufa_client.git</td>
         ```
      </tr>
      <tr>
         <td>up databases</td>
         <td>
         ```
            cd tufa_server && sudo docker-compose up -d && cd ..
         ```
         </td>
      </tr>
      <tr>
         <td>run postgres migrations</td>
         <td>
         ```  
            cd tufa_server && sqlx migrate run && cd ..
         ```
         </td>
      </tr>
   </tbody>
</table>
