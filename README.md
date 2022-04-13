### init submodules
```
git submodule init && git submodule update && cd tufa_client && git checkout main && cd .. && cd tufa_server && git checkout main && cd ..
```
   
<span style="color:green;font-weight:700;font-size:20px">
    markdown color font styles
</span>

### tufa_client
<table>
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

<table style="width:200px">
<thead>
<tr>
<th>tufa_client</th>
<th>tufa-server</th>
</tr>
</thead>
<tbody>
<tr>
<td>description</td>
<td>commands</td>
</tr>
<tr>
<td>example git submodule</td>
<td>example git submodule</td>
</tr>
<tr>
<td>
   
```
git submodule add https://github.com/kuqmua/tufa_client.git
```
</td>
<td>
   
```
git submodule add https://github.com/kuqmua/tufa_server.git
```
</td>
</tr>
<tr>
<td>empty</td>
<td>up databases</td>
</tr>
<tr>
<td>empty</td>
<td>
   
```
cd tufa_server && sudo docker-compose up -d && cd ..
```
</td>
</tr>
<tr>
<td>empty</td>
<td>run postgres migrations</td>
</tr>
<tr>
<td>empty</td>
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

^ Extra blank line above!
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
V Extra blank line below!

</td>
</tr>
<tr>
<td> 400 </td>
<td>

**Markdown** _here_. (Blank lines needed before and after!)

</td>
</tr>
</table>
