use diesel::table;

table! {
    sdk {
        id -> Integer,
        name -> Text,
        slug -> Text,
        url ->  Nullable<Text>,
        description ->  Nullable<Text>,
    }
}

table! {
    app (id) {
        id -> Integer,
        name -> Text,
        company_url -> Text,
        release_date -> Nullable<Date>,
        genre_id -> Integer,
        artwork_large_url -> Text,
        seller_name -> Text,
        five_star_ratings -> Integer,
        four_star_ratings -> Integer,
        three_star_ratings -> Integer,
        two_star_ratings -> Integer,
        one_star_ratings -> Integer,
    }
}

table! {
    app_sdk (app_id, sdk_id) {
        app_id -> Integer,
        sdk_id -> Integer,
        installed -> Bool,
    }
}
