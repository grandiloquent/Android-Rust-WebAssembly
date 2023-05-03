use rusqlite::{Connection};

pub fn execute_modify_video_table(
    conn: &Connection
    ) -> Result<(), rusqlite::Error> {
    conn.execute_batch(r#"BEGIN;
ALTER TABLE video RENAME TO temp_video;
CREATE TABLE IF NOT EXISTS "video" (
	"id"	INTEGER NOT NULL UNIQUE,
	"uri"	TEXT NOT NULL,
	"title"	TEXT,
    "subtitle"	TEXT,
    "duration"	INTEGER,
	"file"	TEXT,
	"image"	TEXT,
	"source_type"	INTEGER,
    "views"	INTEGER,
	"hidden"	INTEGER,
	"create_at"	INTEGER,
	"update_at"	INTEGER,
	PRIMARY KEY("id" AUTOINCREMENT)
);
INSERT INTO video(id,uri,title,subtitle,file,image,source_type,views,hidden,create_at,update_at) SELECT id,uri,title,subtitle,file,image,source_type,views,hidden,create_at,update_at FROM temp_video;
DROP TABLE temp_video;
     END;"#)
}
