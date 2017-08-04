-- Your SQL goes here
create TABLE posts (
	id int primary key auto_increment,
	title varchar(100) not null,
	body text not null,
	published bool not null default false
)
