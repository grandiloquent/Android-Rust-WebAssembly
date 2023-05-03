(()=>{
    const strings=`CREATE TABLE IF NOT EXISTS "video" (
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
)`;
    let array=[...strings.matchAll(/"([a-zA-Z0-9_]+)"\s+(TEXT|INTEGER)/g)];
    console.log(array
    .map(x=>{
        return `${x[1]}`;
    }).join(","));
    console.log(array
    .map(x=>{
        return `${x[1]} = ?`;
    }).join(","));
    console.log(array
    .map(x=>{
        if(x[2]==='TEXT')
        return `${x[1]}:&str`;
       else
            return `${x[1]}:u32`;
    }).join(","))
})();