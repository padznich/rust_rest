Tutor DB
===


# Notes
```shell
curl -X POST localhost:3000/courses/\
  -H "Content-Type: application/json"\
  -d '{"course_id":4, "tutor_id": 1, "course_name":"This is the fourth course!"}'

curl -X POST localhost:3000/courses/\
  -H "Content-Type: application/json"\
  -d '{"course_id":5, "tutor_id": 1, "course_name":"This is the fifth course!"}'


curl -X POST localhost:3000/tutors/\
  -H "Content-Type: application/json"\
  -d '{ "name":"Jessica", "pic_url": "http://tutor1.com/tutor1.pic", "profile": "Experienced professional"}'

curl -X POST localhost:3000/tutors/\
  -H "Content-Type: application/json"\
  -d '{ "name":"James", "pic_url": "http://james.com/tutor1.pic", "profile": "Expert in thermodynamics"}'
```

