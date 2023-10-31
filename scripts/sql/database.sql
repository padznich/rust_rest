/* Drop table if it already exists*/
drop table if exists ezy_course cascade;
drop table if exists ezy_tutor;

/* Create a table. */


create table ezy_tutor (
    id serial primary key,
    name varchar(200) not null,
    pic_url varchar(200) not null,
    profile varchar(2000) not null
);

create table ezy_course (
    id serial primary key,
    tutor_id INT not null,
    name varchar(140) not null,
    description varchar(2000),
    format varchar(30),
    structure varchar(200),
    duration varchar(30),
    price INT,
    language varchar(30),
    level varchar(30),
    posted_time TIMESTAMP default now(),
    CONSTRAINT fk_tutor FOREIGN KEY(tutor_id) REFERENCES ezy_tutor(id) ON DELETE cascade
);
