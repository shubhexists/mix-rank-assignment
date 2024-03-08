import sqlite3

def read_database(file_path):
    try:
        connection = sqlite3.connect(file_path)
        cursor = connection.cursor()
        query = "SELECT * FROM sdk;"
        cursor.execute(query)
        rows = cursor.fetchall()
        for row in rows:
            print(row)
    except sqlite3.Error as e:
        print(f"{e}")
    finally:
        if connection:
            connection.close()

read_database('./database/data.db')

# SELECT * from app_sdk WHERE app_id = 735945527;
# SELECT * from app_sdk WHERE sdk_id = 18;
# SELECT * from app_sdk WHERE app_id = 1119709057;
# PRAGMA table_info(app);