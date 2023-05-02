package psycho.euphoria.plane;

import android.content.Context;
import android.database.Cursor;
import android.database.sqlite.SQLiteDatabase;
import android.database.sqlite.SQLiteOpenHelper;
import android.preference.PreferenceManager;

public class Database extends SQLiteOpenHelper {
    public Database(Context context) {
        super(context, PreferenceManager.getDefaultSharedPreferences(context)
                .getString("db", null), null, 1);
    }

    @Override
    public void onCreate(SQLiteDatabase sqLiteDatabase) {
        sqLiteDatabase.execSQL("create table if not exists cookie (id integer primary key,type integer,value text)");
    }

    public void insertCookie(int type, String cookie) {
        Cursor cursor = getReadableDatabase().rawQuery("select * from cookie where type = ?", new String[]{Integer.toString(type)});
        if (cursor.moveToNext()) {
            getWritableDatabase().execSQL("update cookie set value=? where type=?", new String[]{
                    cookie,
                    Integer.toString(type)
            });
        } else {
            getWritableDatabase().execSQL("insert into cookie (type,value)VALUES(?,?)", new String[]{
                    Integer.toString(type),
                    cookie
            });
        }
    }

    @Override
    public void onUpgrade(SQLiteDatabase sqLiteDatabase, int i, int i1) {

    }
}