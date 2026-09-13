# Web API Example

DDDで実装してます。  

データは永続化されません。  
サーバーを停止すればデータは消えます。

```
docker build -t rust-web-api .
```

```
docker run --rm -it -p 3000:3000 rust-web-api --log=debug --allow-signup --static-dir=html
```

```
curl -i -X POST -H 'content-type: application/json;' -d '{"account":"testuser","password":"P@55w0rd","confirmPassword":"P@55w0rd"}' http://localhost:3000/service/auth/signup
```

```
curl -i -X POST -H 'content-type: application/json;' -d '{"account":"testuser","password":"P@55w0rd"}' http://localhost:3000/service/auth/signin
```

```
curl -s -X POST -H 'content-type: application/json' -d '{"account":"testuser","password":"P@55w0rd"}' http://localhost:3000/service/auth/signin | jq -r '.token'>token.txt
```

```
curl -i -X PUT -H "Authorization: Bearer $(cat token.txt)" -H "Content-Type: application/json"  -d '{"dueDate":"2026-09-12T00:00:00Z","title":"test title","description":"test description"}' http://localhost:3000/service/todo
```

```
curl -s -X PUT -H "Authorization: Bearer $(cat token.txt)" -H "Content-Type: application/json"  -d '{"dueDate":"2026-09-12T00:00:00Z","title":"test title","description":"test description"}' http://localhost:3000/service/todo|jq -r '.id'>id.txt
```

```
TODO_ID=$(cat id.txt) && curl -i -X POST -H "Authorization: Bearer $(cat token.txt)" -H "Content-Type: application/json"  -d '{"id":"'"$TODO_ID"'","dueDate":"2026-09-13T00:00:00Z","title":"change title","description":"change description"}' http://localhost:3000/service/todo
```

```
curl -i -H "Authorization: Bearer $(cat token.txt)" http://localhost:3000/service/todo/list
```

```
curl -i -X DELETE -H "Authorization: Bearer $(cat token.txt)" http://localhost:3000/service/todo/$(cat id.txt)
```
