drop table if exists hectango.creator;
create table hectango.creator (
    creator_id bigint generated always as identity,
    address varchar(100) not null
);