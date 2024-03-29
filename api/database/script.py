# THIS IS JUST A SAMPLE FILE USED FOR VISUALIZATION AND TESTING QUERIES
# THIS FILE IS NOT USED IN THE APPLICATION

import sqlite3

def read_database(file_path):
    try:
        connection = sqlite3.connect(file_path)
        cursor = connection.cursor()
        query = """PRAGMA table_info(sdk);"""
        cursor.execute(query)
        rows = cursor.fetchall()
        for row in rows:
            print(row)
    except sqlite3.Error as e:
        print(f"{e}")
    finally:
        if connection:
            connection.close()

read_database('./api/database/data.db')

# SELECT * from app_sdk WHERE app_id = 735945527;
# SELECT * from app_sdk WHERE sdk_id = 18;
# SELECT * from app_sdk WHERE app_id = 1119709057;
# PRAGMA table_info(app);
