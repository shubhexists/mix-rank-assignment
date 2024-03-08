import sqlite3

def read_database(file_path):
    try:
        connection = sqlite3.connect(file_path)
        cursor = connection.cursor()
        query = """SELECT 
            t1.*, t2.*, sdk1.*, sdk2.*, COUNT(DISTINCT t1.app_id) AS count_1
        FROM 
            app_sdk AS t1, app_sdk AS t2, sdk AS sdk1, sdk AS sdk2
        WHERE 
            t1.app_id = t2.app_id
            AND t1.sdk_id = sdk1.id
            AND t2.sdk_id = sdk2.id
            AND t1.installed = 0
            AND t2.installed = 1
            AND sdk1.slug IN ('stripe', 'paypal', 'alipay')
            AND sdk2.slug IN ('stripe', 'paypal', 'alipay')
        GROUP BY 
            sdk1.id, sdk2.id;"""
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
