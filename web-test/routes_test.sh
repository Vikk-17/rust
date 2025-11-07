# Test GET route

routes=("testing", "")
method=("GET", "POST")

for route in "${routes[@]}"
do
    echo "Sending request to $route"
    curl -X GET http://localhost:8080/${route}
    echo
done

# curl -X GET http://localhost:8080/testing
#
# echo 
#
# curl -X GET http://localhost:8080/
#
# echo 
#
# # Test POST route
# curl -H 'Content-Type: application/json' \
#     -d '{"title": "John", "body": "Carmack"}' \
#     -X POST \
#     http://localhost:8080/post
#
# echo
#
