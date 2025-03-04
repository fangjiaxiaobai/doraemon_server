-- 创建用户
create user 'doraemon'@'localhost' identified with mysql_native_password by 'admin.123';
grant all privileges on doraemon.* to 'doraemon'@'localhost';
-- 建表
CREATE TABLE dor_vod_col (
    id INT NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    vod_col_name VARCHAR(255) NOT NULL COMMENT '视频名称',
    type_id INT NOT NULL COMMENT '分类ID',
    vod_pic VARCHAR(255) NOT NULL  COMMENT '视频图片',
    vod_actor varchar(255)  NOT NULL comment '演员列表',
    vod_director VARCHAR(128) NOT NULL  comment '导演',
    vod_content text NOT NULL  COMMENT '视频内容',
    vod_lang varchar(64)  NOT NULL comment '视频语言',
    vod_time datetime  NOT NULL comment '视频上映时间',
    vod_state varchar(64) NOT NULL  comment '视频情况, 连载中/已完结',
    vod_remarks varchar(64) NOT NULL  comment '视频情况, 更新到xx',
    vod_area varchar(64) NOT NULL  comment '视频地域',
    PRIMARY KEY (id),
    index idx_vod_area(`vod_area`),
    index idx_type_id(`type_id`),
    index idx_vod_name(`vod_col_name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='视频剧集资源表';

create table dor_vod (
    id  INT NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    vod_col_id INT NOT NULL COMMENT '视频名称(剧集)',
    vod_name VARCHAR(255) NOT NULL COMMENT '视频名称',
    vod_play_url VARCHAR(255) NOT NULL COMMENT '播放url',
    vod_type VARCHAR(64) not null COMMENT '视频类型:mp4.m3u8等',
    vod_player_id VARCHAR(255) NOT NULL COMMENT '播放器',
    PRIMARY key(id),
    index idx_vod_col_id(`vod_col_id`),
    index idx_vod_name(`vod_name`)
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='视频资源表';