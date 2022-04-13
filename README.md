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
            git submodule add https://github.com/kuqmua/tufa_client.git
         ```
         </td>
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

<table>
<tr>
<td> Status </td> <td> Response </td>
</tr>
<tr>
<td> 200 </td>
<td>


```json
json
{
    "id": 10,
    "username": "alanpartridge",
    "email": "alan@alan.com",
    "password_hash": "$2a$10$uhUIUmVWVnrBWx9rrDWhS.CPCWCZsyqqa8./whhfzBZydX7yvahHS",
    "password_salt": "$2a$10$uhUIUmVWVnrBWx9rrDWhS.",
    "created_at": "2015-02-14T20:45:26.433Z",
    "updated_at": "2015-02-14T20:45:26.540Z"
}
```


</td>
</tr>
</table>
