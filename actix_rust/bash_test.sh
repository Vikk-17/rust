for i in $(seq 1 1000);
do
    curl -X GET localhost:8000
done
