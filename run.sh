curl --request POST \
  --url http://localhost:8888/ \
  --header 'Content-Type: application/json' \
  --data '{
	"client_id": "test",
	"name": "hello_event",
	"data": {
		"test": "value"
	}
}'