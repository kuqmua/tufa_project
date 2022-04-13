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

