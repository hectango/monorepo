drop table if exists hectango.video;
create table hectango.video (
    video_id bigint generated always as identity,
    hash text not null,
    length integer not null,
    title text not null,
    creator_address varchar(32) not null,
    state varchar(20) not null,
    url text not null,
    description text not null
);