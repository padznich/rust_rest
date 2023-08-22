Tutor no DB
===



#### Run server

```bash
cargo run
```

OR

```bash
cargo run -p tutor-nodb --bin basic-server
```


Visit

```commandline
http://localhost:3000/health-check
```



#### Run tests

```bash
cargo test
```



#### Notes

---


##### Prepare courses

```commandline
curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"tutor_id":1, "course_name":"Hello , my first course !"}'
curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"tutor_id":1, "course_name":"Hello , my second course !"}'
curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"tutor_id":1, "course_name":"Hello , my third course !"}'
```

##### Get courses for the tutor with id = 1

```commandline
curl localhost:3000/courses/1
```
