drop database if exists hectango_test;
drop user if exists tester;
create user tester with password 'tester';
create database hectango_test;
grant all privileges on database hectango_test to tester;
