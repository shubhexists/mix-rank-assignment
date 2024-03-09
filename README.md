# Mix Rank Assignment
This README Section would contain a log of my thoughts of my approach of the problem. 
The requirements can be found in the REQUIREMENTS.md.

## UNDERSTANDING THE PROJECT 
- I went through the data in the `data.db` SQLite file. It had 3 tables - 
    1) `app` - It consists of the applications we need to crawl 
    2) `sdk` - List of SDKs we need to check for each Application
    3) `app_sdk` - It has a list of `sdk_id` and `app_id` that is actually used in the application.
  
## STRUCTURE

`app` -
```py
(0, 'id', 'int', 0, None, 1)
(1, 'name', 'text', 0, None, 0)
(2, 'company_url', 'text', 0, None, 0)
(3, 'release_date', 'date', 0, None, 0)
(4, 'genre_id', 'int', 0, None, 0)
(5, 'artwork_large_url', 'text', 0, None, 0)
(6, 'seller_name', 'text', 0, None, 0)
(7, 'five_star_ratings', 'int', 0, None, 0)
(8, 'four_star_ratings', 'int', 0, None, 0)
(9, 'three_star_ratings', 'int', 0, None, 0)
(10, 'two_star_ratings', 'int', 0, None, 0)
(11, 'one_star_ratings', 'int', 0, None, 0)
```

`sdk` - 
```py
(0, 'id', 'int', 0, None, 1)
(1, 'name', 'text', 0, None, 0)
(2, 'slug', 'text', 0, None, 0)
(3, 'url', 'text', 0, None, 0)
(4, 'description', 'text', 0, None, 0)
```

`app_sdk` - 
```py
(0, 'app_id', 'int', 0, None, 1)
(1, 'sdk_id', 'int', 0, None, 2)
(2, 'installed', 'boolean', 0, None, 0)
```

## API
- Trying to implement initial implementation all in Rust.
My choice of Rust is just based on my experience in Rust and 
it's ability to build performant web servers.
I am using the `actix-web` framework to build web server.

- Currently thinking of making 2 api routes, one to give out
the churns between SDKs and other to give out the examples 
`ADDITION` - API Route added to fetch all the sdks on the client side

- Tried using static types for quite some time but because of variable data formats
it is resulting in not compiling of the code, hence not using static types for this implementation

## CLIENT
- Client application is in NextJS. Initial though was I could use ShadCN library for a better UI
but that would add the complexity of the client code hence decided to not use ShadCN, continued with
NextJS though

- The Instructions talked about displaying an image. From what I could think of, I found displaying the 
table that is quite similar to that image would be much more convenient. It still looks the same.
 
# IMPORTANT POINTS 
- All my interactions with ChatGPT for this assignment are recorded in 
   https://chat.openai.com/share/6ff35006-62f2-4af8-8ec5-990cfc824e31

- I did not had prior experience of nix and the `default.nix` file provided had python setup.
Hence, I could not set-up nix. So for now I'll link the setup 
instructions below. I'll be adding Nix in the next couple of days after I get an understanding of it.

# SET-UP
You need Rust/ Cargo installed in your machine for running the backend. https://www.rust-lang.org/tools/install 


You would also need Node installed for setting up the client. 

1) Clone the git repo 
```
git clone https://github.com/shubhexists/mix-rank-assignment 
```
2) Navigate to the project and then the `api` Directory to build/run the Rust backend - 
```
cd mix-rank-assignment/api 

cargo run
```
This will export the backend routes on port 8080.

3) Now navigate to the client directory and download the dependencies.
```
cd ../client

npm install
```

4) Now start the client
```
npm run dev
```
This will start the client application on port 3000.

# That's IT. THANK YOU
