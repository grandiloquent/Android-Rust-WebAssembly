(() => {
    const strings = `CREATE TABLE IF NOT EXISTS "video" (
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
    let array = [...strings.matchAll(/"([a-zA-Z0-9_]+)"\s+(TEXT|INTEGER)/g)];
    console.log(array
        .map(x => {
            return `${x[1]}`;
        }).join(","));
    console.log(array
        .map(x => {
            return `${x[1]} = ?`;
        }).join(","));
    console.log(array
        .map(x => {
            if (x[2] === 'TEXT')
                return `${x[1]}:&str`;
            else
                return `${x[1]}:u32`;
        }).join(","))
    console.log(array
        .map(x => {
            if (x[2] === 'TEXT')
                return `&video.${x[1]}`;
            else
                return `video.${x[1]}`;
        }).join(","))
})();

/*

$filename="update_video_file.rs";$dir="C:\Users\psycho\Desktop\file\Plane\app\src\main\rust\src\db";Set-Location $dir; New-Item $filename;Get-ChildItem | Where-Object {$_.Name -ne "mod.rs"} | Split-Path -LeafBase | Join-String -FormatString "pub mod {0};`r`n" | Set-Content -Path .\mod.rs;


*/
(() => {
    const str = `accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9
    accept-encoding: gzip, deflate, br
    accept-language: en
    cache-control: no-cache
    pragma: no-cache
    sec-ch-ua: "Google Chrome";v="95", "Chromium";v="95", ";Not A Brand";v="99"
    sec-ch-ua-mobile: ?0
    sec-ch-ua-platform: "Windows"
    sec-fetch-dest: document
    sec-fetch-mode: navigate
    sec-fetch-site: none
    sec-fetch-user: ?1
    upgrade-insecure-requests: 1
    user-agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/95.0.4638.69 Safari/537.36`;

    console.log(str.split('\n')
        .map(x => {
            const pieces=x.split(':');
            return `.header("${pieces[0].trim()}","${pieces[1].trim().replaceAll('"','\\"')}")`;
        }).join('\n'));
})();