
Api Dashboard

Your Account
Your Searches
Api Documentation

Google Search API
AI Overview
Ad Results
Answer Box
Available On
Broaden Searches
Buying Guide
DMCA Messages
Discover More Places
Discussions and Forums
Events Results
Find Results On
Grammar Check
Immersive Products
Inline Images
Inline People Also Search For
Inline Products
Inline Shopping
Inline Videos
Jobs Results
Knowledge Graph
Local Ads
Local News
Local Pack
News Results
Organic Results
Pagination
Perspectives
Places Sites
Popular Destinations
Product Result
Product Sites
Questions And Answers
Recipes Results
Refine By
Refine This Search
Related Questions
Related Searches
Scholarly Articles
Short Videos
Showtimes Results
Spell Check
Sports Results
Things To Know
Top Carousel
Top Insights
Top Sights
Top Stories
Twitter Results
Visual Stories
Google Maps API
Google Jobs API
Google Shopping API
Google Images API
Google News API
Google Local API
Google Trends API
Google Autocomplete API
Google Light Search API
Google About This Result
Google Lens API
Google Finance API
Google Related Questions API
Google Scholar API
Google Play Store API
Google Product API
Google Immersive Product API
Google Reverse Image API
Google Events API
Google Local Services API
Google Videos API
Google Patents API
Google Ads Transparency API
Google Flights API
Google Hotels API
Google Food API
Google AI Overview API
Baidu Search API
Bing Search API
DuckDuckGo Search API
Yahoo! Search API
Yandex Search API
Ebay Search API
YouTube Search API
Walmart Search API
The Home Depot Search API
Apple App Store API
Naver Search API
Yelp Search API
Extra APIs
API Studio

Playground
Search
Sign In
Register
Google Search Engine Results API
API uptime
100.000%
/search API endpoint allows you to scrape the results from Google search engine via our SerpApi service. Head to the playground for a live and interactive demo. You can query https://serpapi.com/search using a GET request.

API Parameters
Search Query
q

Required

Parameter defines the query you want to search. You can use anything that you would use in a regular Google search. e.g. inurl:, site:, intitle:. We also support advanced search query parameters such as as_dt and as_eq. See the full list of supported advanced search query parameters.

Geographic Location
location

Optional

Parameter defines from where you want the search to originate. If several locations match the location requested, we'll pick the most popular one. Head to the /locations.json API if you need more precise control. The location and uule parameters can't be used together. It is recommended to specify location at the city level in order to simulate a real user’s search. If location is omitted, the search may take on the location of the proxy.

uule

Optional

Parameter is the Google encoded location you want to use for the search. uule and location parameters can't be used together.

Advanced Google Parameters
ludocid

Optional

Parameter defines the id (CID) of the Google My Business listing you want to scrape. Also known as Google Place ID.

lsig

Optional

Parameter that you might have to use to force the knowledge graph map view to show up. You can find the lsig ID by using our Local Pack API or Google Local API.
lsig ID is also available via a redirect Google uses within Google My Business.

kgmid

Optional

Parameter defines the id (KGMID) of the Google Knowledge Graph listing you want to scrape. Also known as Google Knowledge Graph ID. Searches with kgmid parameter will return results for the originally encrypted search parameters. For some searches, kgmid may override all other parameters except start, and num parameters.

si

Optional

Parameter defines the cached search parameters of the Google Search you want to scrape. Searches with si parameter will return results for the originally encrypted search parameters. For some searches, si may override all other parameters except start, and num parameters. si can be used to scrape Google Knowledge Graph Tabs.

ibp

Optional

Parameter is responsible for rendering layouts and expansions for some elements (e.g., gwp;0,7 to expand searches with ludocid for expanded knowledge graph).

uds

Optional

Parameter enables to filter search. It's a string provided by Google as a filter. uds values are provided under the section: filters with uds, q and serpapi_link values provided for each filter.

Localization
google_domain

Optional

Parameter defines the Google domain to use. It defaults to google.com. Head to the Google domains page for a full list of supported Google domains.

gl

Optional

Parameter defines the country to use for the Google search. It's a two-letter country code. (e.g., us for the United States, uk for United Kingdom, or fr for France). Head to the Google countries page for a full list of supported Google countries.

hl

Optional

Parameter defines the language to use for the Google search. It's a two-letter language code. (e.g., en for English, es for Spanish, or fr for French). Head to the Google languages page for a full list of supported Google languages.

cr

Optional

Parameter defines one or multiple countries to limit the search to. It uses country{two-letter upper-case country code} to specify countries and | as a delimiter. (e.g., countryFR|countryDE will only search French and German pages). Head to the Google cr countries page for a full list of supported countries.

lr

Optional

Parameter defines one or multiple languages to limit the search to. It uses lang_{two-letter language code} to specify languages and | as a delimiter. (e.g., lang_fr|lang_de will only search French and German pages). Head to the Google lr languages page for a full list of supported languages.

Advanced Filters
tbs

Optional

(to be searched) parameter defines advanced search parameters that aren't possible in the regular query field. (e.g., advanced search for patents, dates, news, videos, images, apps, or text contents).

safe

Optional

Parameter defines the level of filtering for adult content. It can be set to active or off, by default Google will blur explicit content.

nfpr

Optional

Parameter defines the exclusion of results from an auto-corrected query when the original query is spelled wrong. It can be set to 1 to exclude these results, or 0 to include them (default). Note that this parameter may not prevent Google from returning results for an auto-corrected query if no other results are available.

filter

Optional

Parameter defines if the filters for 'Similar Results' and 'Omitted Results' are on or off. It can be set to 1 (default) to enable these filters, or 0 to disable these filters.

Search Type
tbm

Optional

(to be matched) parameter defines the type of search you want to do.

It can be set to:
(no tbm parameter): regular Google Search,
isch: Google Images API,
lcl - Google Local API
vid: Google Videos API,
nws: Google News API,
shop: Google Shopping API,
pts: Google Patents API,
or any other Google service.

Pagination
start

Optional

Parameter defines the result offset. It skips the given number of results. It's used for pagination. (e.g., 0 (default) is the first page of results, 10 is the 2nd page of results, 20 is the 3rd page of results, etc.).

Google Local Results only accepts multiples of 20(e.g. 20 for the second page results, 40 for the third page results, etc.) as the start value.

num

Optional

Parameter defines the maximum number of results to return. (e.g., 10 (default) returns 10 results, 40 returns 40 results, and 100 returns 100 results).

Serpapi Parameters
engine

Optional

Set parameter to google (default) to use the Google API engine.

device

Optional

Parameter defines the device to use to get the results. It can be set to desktop (default) to use a regular browser, tablet to use a tablet browser (currently using iPads), or mobile to use a mobile browser (currently using iPhones).

no_cache

Optional

Parameter will force SerpApi to fetch the Google results even if a cached version is already present. A cache is served only if the query and all parameters are exactly the same. Cache expires after 1h. Cached searches are free, and are not counted towards your searches per month. It can be set to false (default) to allow results from the cache, or true to disallow results from the cache. no_cache and async parameters should not be used together.

async

Optional

Parameter defines the way you want to submit your search to SerpApi. It can be set to false (default) to open an HTTP connection and keep it open until you got your search results, or true to just submit your search to SerpApi and retrieve them later. In this case, you'll need to use our Searches Archive API to retrieve your results. async and no_cache parameters should not be used together. async should not be used on accounts with Ludicrous Speed enabled.

zero_trace

Optional

Enterprise only. Parameter enables ZeroTrace mode. It can be set to false (default) or true. Enable this mode to skip storing search parameters, search files, and search metadata on our servers. This may make debugging more difficult.

api_key

Required

Parameter defines the SerpApi private key to use.

output

Optional

Parameter defines the final output you want. It can be set to json (default) to get a structured JSON of the results, or html to get the raw html retrieved.

API Results
JSON Results
JSON output includes structured data for organic results, local results, ad results, the knowledge graph, direct answer boxes, images results, news results, shopping results, video results, and more.

A search status is accessible through search_metadata.status. It flows this way: Processing -> Success || Error. If a search has failed, error will contain an error message. search_metadata.id is the search ID inside SerpApi.

HTML Results
HTML output is useful to debug JSON results or support features not supported yet by SerpApi.
HTML output gives you the raw HTML results from Google.

API Examples
Example with 
q
:
Coffee
parameter
GET


https://serpapi.com/search.json?engine=google&q=Coffee

Code to integrate


Ruby

require 'google_search_results' 

params = {
  engine: "google",
  q: "Coffee",
  api_key: "a602132fb4dd2bad19d4df9532f26aa36d8bfadd8b08311f5fd96db7178b261c"
}

search = GoogleSearch.new(params)
organic_results = search.get_hash[:organic_results]

JSON Example

{
  "search_metadata": {
    "id": "61afb3ace7d08a685b3bcbb1",
    "status": "Success",
    "json_endpoint": "https://serpapi.com/searches/c292c1c1fe17fc58/61afb3ace7d08a685b3bcbb1.json",
    "created_at": "2021-12-07 19:19:08 UTC",
    "processed_at": "2021-12-07 19:19:08 UTC",
    "google_url": "https://www.google.com/search?q=coffee&oq=coffee&uule=w+CAIQICIaQXVzdGluLFRleGFzLFVuaXRlZCBTdGF0ZXM&hl=en&gl=us&sourceid=chrome&ie=UTF-8",
    "raw_html_file": "https://serpapi.com/searches/c292c1c1fe17fc58/61afb3ace7d08a685b3bcbb1.html",
    "total_time_taken": 1.52
  },
  "search_parameters": {
    "engine": "google",
    "q": "coffee",
    "location_requested": "Austin, Texas, United States",
    "location_used": "Austin,Texas,United States",
    "google_domain": "google.com",
    "hl": "en",
    "gl": "us",
    "device": "desktop"
  },
  "search_information": {
    "organic_results_state": "Results for exact spelling",
    "query_displayed": "coffee",
    "total_results": 1340000000,
    "time_taken_displayed": 0.99
  },
  "recipes_results": [
    {
      "title": "Bulletproof Coffee Recipe",
      "link": "https://www.bulletproof.com/recipes/bulletproof-diet-recipes/bulletproof-coffee-recipe/",
      "source": "Bulletproof",
      "total_time": "5 min",
      "ingredients": [
        "Mct oil",
        "bulletproof coffee",
        "grass fed"
      ],
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTBL6y0xwQPtXHfPXpTETKfmTlFKCc53AZ66YJ4wmBf4BZQbMmR2szgOg&s=0"
    },
    {
      "title": "Whipped Coffee",
      "link": "https://cooking.nytimes.com/recipes/1021005-whipped-coffee",
      "source": "NYT Cooking - The New York Times",
      "rating": 4,
      "reviews": 770,
      "ingredients": [
        "Instant coffee",
        "ice",
        "milk",
        "hot water"
      ],
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRfvf7kwy5yjTVCoKUpROKBXX2UY9TH4Ko0WIxieGKGIwKUkgAxaTwTdg&s=0"
    },
    {
      "title": "Coffee recipes",
      "link": "https://www.bbcgoodfood.com/recipes/collection/coffee-recipes",
      "source": "BBC Good Food",
      "ingredients": [
        "Instant coffee"
      ],
      "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSlgcwczn4_p-f4SEiHM8jGoIGyUiTuTxTrsGvsggtcbkyGqk5wkLRsjw&s=0"
    }
  ],
  "shopping_results": [
    {
      "position": 1,
      "block_position": "top",
      "title": "mudwtr.com - MUD\\WTR | Mushroom Coffee Alternative, 30 servings",
      "price": "$50.00",
      "extracted_price": 50,
      "link": "https://www.google.com/aclk?sa=l&ai=DChcSEwiZtL6MtNL0AhWBn7MKHTqRDpEYABAEGgJxbg&ae=2&sig=AOD64_2QRwv8qgRnd6Jr65C9UhyHPLwhXA&ctype=5&q=&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ5bgDegQIAhA9&adurl=",
      "source": "mudwtr.com",
      "reviews": 6000,
      "thumbnail": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/3dfd005c088bc5e4e2b7ab1c2868aa47743a6687f658162aac59c78c75d56d1d.png"
    },
    {
      "position": 2,
      "block_position": "top",
      "title": "mudwtr.com - MUD\\WTR | Mushroom Coffee Replacement, 90 servings",
      "price": "$125.00",
      "extracted_price": 125,
      "link": "https://www.google.com/aclk?sa=l&ai=DChcSEwiZtL6MtNL0AhWBn7MKHTqRDpEYABADGgJxbg&ae=2&sig=AOD64_37xmneT6EL2jcvAGOifAFCwRsKLg&ctype=5&q=&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ5bgDegQIAhBI&adurl=",
      "source": "mudwtr.com",
      "reviews": 2000,
      "thumbnail": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/3dfd005c088bc5e4e2b7ab1c2868aa4728d69aa6f14be4eb6102830c5aa2461d.jpeg"
    },
    {
      "position": 3,
      "block_position": "top",
      "title": "Angelino's Coffee - 96ct Flavored Coffee Experience",
      "price": "$49.95",
      "extracted_price": 49.95,
      "link": "https://www.google.com/aclk?sa=l&ai=DChcSEwiZtL6MtNL0AhWBn7MKHTqRDpEYABAFGgJxbg&ae=2&sig=AOD64_0OyhR7klpz0J1QXcXRHdOf6zGtAA&ctype=5&q=&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ5bgDegQIAhBT&adurl=",
      "source": "Angelino's Coffee",
      "reviews": 55,
      "thumbnail": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/3dfd005c088bc5e4e2b7ab1c2868aa4785ccdcf05a2f2c29fef1863c2e52c20a.png"
    }
  ],
  "local_map": {
    "link": "https://www.google.com/search?gl=us&hl=en&q=coffee&npsic=0&rflfq=1&rldoc=1&rllag=30267485,-97742560,126&tbm=lcl&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQtgN6BAgfEAc",
    "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/b58ac7a94530b87c33a3ef84f967f2f8.png",
    "gps_coordinates": {
      "latitude": 30.267485,
      "longitude": -97.74256
    }
  },
  "local_results": {
    "more_locations_link": "https://www.google.com/search?gl=us&hl=en&tbs=lf:1,lf_ui:9&tbm=lcl&q=coffee&rflfq=1&num=10&uule=w+CAIQICIaQXVzdGluLFRleGFzLFVuaXRlZCBTdGF0ZXM&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQjGp6BAgfEGI",
    "places": [
      {
        "position": 1,
        "title": "Houndstooth Coffee",
        "place_id": "11265938073076301333",
        "lsig": "AB86z5Vdw6C2pJpM0xQ6JUx2KONU",
        "place_id_search": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Austin%2C+Texas%2C+United+States&lsig=AB86z5Vdw6C2pJpM0xQ6JUx2KONU&ludocid=11265938073076301333&q=coffee&tbm=lcl",
        "rating": 4.6,
        "reviews": 746,
        "price": "$$",
        "type": "Coffee shop",
        "address": "401 Congress Ave #100c · In Frost Bank Tower",
        "thumbnail": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/ff7f6a31fb4c327e6006e2b3ceb404a5f7a61b6a65199dc7462885107dab20d4c83002a8ec0eb1e0.jpeg",
        "gps_coordinates": {
          "latitude": 30.2664,
          "longitude": -97.74278
        }
      },
      {
        "position": 2,
        "title": "Starbucks",
        "place_id": "10605736027611436825",
        "lsig": "AB86z5XTJ_Io_anVBu2fU6Zaqu3b",
        "place_id_search": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Austin%2C+Texas%2C+United+States&lsig=AB86z5XTJ_Io_anVBu2fU6Zaqu3b&ludocid=10605736027611436825&q=coffee&tbm=lcl",
        "rating": 4.1,
        "reviews": 509,
        "price": "$$",
        "type": "Coffee shop",
        "address": "600 Congress Ave",
        "thumbnail": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/ff7f6a31fb4c327e6006e2b3ceb404a5befc37eb0b3b33e75015a537212b37488eacfc7600ad8af3.jpeg",
        "gps_coordinates": {
          "latitude": 30.26826,
          "longitude": -97.74296
        }
      },
      {
        "position": 3,
        "title": "The Hideout Coffee House",
        "place_id": "15498522356495312950",
        "lsig": "AB86z5WSxdnDKVF_iLXNN6Lg0UQ5",
        "place_id_search": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Austin%2C+Texas%2C+United+States&lsig=AB86z5WSxdnDKVF_iLXNN6Lg0UQ5&ludocid=15498522356495312950&q=coffee&tbm=lcl",
        "rating": 4.4,
        "reviews": 374,
        "price": "$",
        "type": "Coffee shop",
        "address": "617 Congress Ave",
        "thumbnail": "https://lh5.googleusercontent.com/p/AF1QipM9lw0KHNIMH0w_GQGJVMDRJkjCdWeyTwmOdJy0=w92-h92-n-k-no",
        "gps_coordinates": {
          "latitude": 30.268572,
          "longitude": -97.742165
        }
      }
    ]
  },
  "knowledge_graph": {
    "title": "Coffee",
    "type": "Drink",
    "kgmid": "/m/02vqfm",
    "knowledge_graph_search_link": "https://www.google.com/search?kgmid=/m/02vqfm&hl=en-US&q=Coffee&kgs=e5a0eb9eeef80765&shndl=0&source=sh/x/kp/1&entrypoint=sh/x/kp",
    "serpapi_knowledge_graph_search_link": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en-US&kgmid=%2Fm%2F02vqfm&location=Austin%2C+Texas%2C+United+States&q=Coffee",
    "header_images": [
      {
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a8030d0f84891c9ef7fe21e7b8b27d7c20036bdb007f4c06f7d.jpeg",
        "source": "https://en.wikipedia.org/wiki/Coffee"
      },
      {
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a8030d0f84891c9ef7f7c3c5fd0133d77faa56cbc324a3ece25.jpeg",
        "source": "https://www.nbcnews.com/better/lifestyle/how-tap-health-benefits-coffee-ncna1096031"
      },
      {
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a8030d0f84891c9ef7f8cf5b470290189d19afb9eaffe17b13f.jpeg",
        "source": "https://austin.eater.com/maps/best-coffee-austin-cafes-patio-latte-pour-over"
      },
      {
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a8030d0f84891c9ef7f6387935aaacf98275d04f002fb06c161.jpeg",
        "source": "https://www.cancer.org/latest-news/coffee-and-cancer-what-the-research-really-shows.html"
      }
    ],
    "description": "Coffee is a brewed drink prepared from roasted coffee beans, the seeds of berries from certain Coffea species. From the coffee fruit, the seeds are separated to produce a stable, raw product: unroasted green coffee.",
    "source": {
      "name": "Wikipedia",
      "link": "https://en.wikipedia.org/wiki/Coffee"
    },
    "patron_saint": "Saint Drogo",
    "patron_saint_links": [
      {
        "text": "Patron saint",
        "link": "https://www.google.com/search?gl=us&hl=en&q=coffee+patron+saint&stick=H4sIAAAAAAAAAOPgE-LUz9U3MCorTMvVksrPttIvzsgvKklLTC6xKkgsKcrPiy9OzMwrWcQqnJyflpaaqgARVQCLAgBzUMsxPwAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ6BMoAHoECEwQAg"
      }
    ],
    "species_of_coffee": [
      {
        "name": "Coffea arabica",
        "link": "https://www.google.com/search?gl=us&hl=en&q=Coffea+arabica&stick=H4sIAAAAAAAAAONgFuLUz9U3MCorTMtVAjNNTJPSLbREspOt9JPzc3Pz86xS8svzyhOLUopXMcoBxXJyUpNLMvPz9JMy83Py0zOTE3PiiwtSkzNTixex8jnnp6WlJiokFiUmAWV2sDICAOGVXZhjAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQxA16BAhIEAU",
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a807d547b329424c7b52547f1fa893b42705bf58be621d9c2cac17e5a18fdd042db.jpeg"
      },
      {
        "name": "Robusta coffee",
        "link": "https://www.google.com/search?gl=us&hl=en&q=Robusta+coffee&stick=H4sIAAAAAAAAAONgFuLUz9U3MCorTMtVAjPNkquyLbREspOt9JPzc3Pz86xS8svzyhOLUopXMcoBxXJyUpNLMvPz9JMy83Py0zOTE3PiiwtSkzNTixex8gXlJ5UWlyQqJOenpaWm7mBlBAAoykDxYwAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQxA16BAhIEAc",
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a807d547b329424c7b52547f1fa893b4270cdc3268a8313cf7011d578aaf9251050.jpeg"
      },
      {
        "name": "Coffea liberica",
        "link": "https://www.google.com/search?gl=us&hl=en&q=Coffea+liberica&stick=H4sIAAAAAAAAAONgFuLUz9U3MCorTMtV4gIxjZOTipKrtESyk630k_Nzc_PzrFLyy_PKE4tSilcxygHFcnJSk0sy8_P0kzLzc_LTM5MTc-KLC1KTM1OLF7HyO-enpaUmKuRkJqUWAaV2sDICANH1w39lAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQxA16BAhIEAk",
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a807d547b329424c7b52547f1fa893b42708e0ea2b183a2e0dfa7dc3bd4e9d90eeb.jpeg"
      },
      {
        "name": "Hemileia vastatrix",
        "link": "https://www.google.com/search?gl=us&hl=en&q=Hemileia+vastatrix&stick=H4sIAAAAAAAAAONgFuLUz9U3MCorTMtVAjOTsiyNzbVEspOt9JPzc3Pz86xS8svzyhOLUopXMcoBxXJyUpNLMvPz9JMy83Py0zOTE3PiiwtSkzNTixexCnmk5mbmpGYmKpQlFpcklhRlVuxgZQQAWk1xqWcAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQxA16BAhIEAs",
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a807d547b329424c7b52547f1fa893b42706d4c4f9485b41f59675e6618aebad693.jpeg"
      },
      {
        "name": "Coffea stenophylla",
        "link": "https://www.google.com/search?gl=us&hl=en&q=Coffea+stenophylla&stick=H4sIAAAAAAAAAONgFuLUz9U3MCorTMtV4tZP1zc0MjQsKMk10xLJTrbST87Pzc3Ps0rJL88rTyxKKV7FKAcUy8lJTS7JzM_TT8rMz8lPz0xOzIkvLkhNzkwtXsQq5JyflpaaqFBckpqXX5BRmZOTuIOVEQBk-tZUaQAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQxA16BAhIEA0",
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a807d547b329424c7b52547f1fa893b42700d69a607720a3447ed2deea63060f4a3.jpeg"
      }
    ],
    "species_of_coffee_link": "https://www.google.com/search?gl=us&hl=en&q=Species+of+coffee&stick=H4sIAAAAAAAAAONgFuLUz9U3MCorTMtVQjC1RLKTrfST83Nz8_OsUvLL88oTi1KKVzHKAcVyclKTSzLz8_STMvNz8tMzkxNz4osLUpMzU4sXsQoGQ1gK-WkKyflpaampO1gZAaPUmVRmAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQMSgAegQISBAB",
    "species_of_coffee_stick": "H4sIAAAAAAAAAONgFuLUz9U3MCorTMtVQjC1RLKTrfST83Nz8_OsUvLL88oTi1KKVzHKAcVyclKTSzLz8_STMvNz8tMzkxNz4osLUpMzU4sXsQoGQ1gK-WkKyflpaampO1gZAaPUmVRmAAAA",
    "coffee_books": [
      {
        "name": "The World Atlas of Coffee: F...",
        "link": "https://www.google.com/search?gl=us&hl=en&q=The+World+Atlas+of+Coffee:+From+Beans+to+Brewing+-+Coffees+Explored,+Explained+and+Enjoyed&stick=H4sIAAAAAAAAAC3JQQqCQBSAYSSEWrQoOsCjZRSjQRDuMuwEQdBudN6YOvNezUjadVp2go5XRLuf7x8OpiNhRbS-37Sdj0Up4jjfmH5b1vVi1hSJKNhapkRxR510yr-CydeMwaKtmETO3PhneD5eEE7sjIJda6QH1rBnrRETODi2kKIkDy1D6rCrqITV_3vI-qthh2r5K1kRKpCkIKOaH6jeYfABYiUriKYAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQxA16BAhKEAU",
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a804f82f30abd848a4df49f867f2b624ead490a6d28fb7d25ba.jpeg"
      },
      {
        "name": "Craft Coffee: A Manual",
        "link": "https://www.google.com/search?gl=us&hl=en&q=Craft+Coffee:+A+Manual&stick=H4sIAAAAAAAAAONgFuLUz9U3MCorTMtV4tVP1zc0TKmwTDe2yC7TEslOttJPzs_Nzc-zSskvzytPLEopXsUoCBTLyUlNLsnMz9NPys_PLl7EKuZclJhWouCcn5aWmmql4Kjgm5hXmpizg5URAFoyNJBiAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQxA16BAhKEAc",
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a804f82f30abd848a4d724635725d09f09c611c2eb40d8f939c.jpeg"
      },
      {
        "name": "The Professional Barista's...",
        "link": "https://www.google.com/search?gl=us&hl=en&q=The+Professional+Barista%27s+Handbook&stick=H4sIAAAAAAAAAONgFuLUz9U3MCorTMtV4tFP1zc0SsrNzS0qttASyU620k_Oz83Nz7NKyS_PK08sSilexSgIFMvJSU0uyczP00_Kz88uXsSqHJKRqhBQlJ-WWlwMFE7MUXBKLMosLklUL1bwSMxLASnbwcoIAIZvHchuAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQxA16BAhKEAk",
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a804f82f30abd848a4d83c851ac211daf71b43bbeca19cdffec.jpeg"
      },
      {
        "name": "The Curious Barista's...",
        "link": "https://www.google.com/search?gl=us&hl=en&q=The+Curious+Barista%27s+Guide+to+Coffee&stick=H4sIAAAAAAAAAONgFuLUz9U3MCorTMtV4tVP1zc0TDJLLssuNCzSEslOttJPzs_Nzc-zSskvzytPLEopXsUoCBTLyUlNLsnMz9NPys_PLl7EqhqSkargXFqUmV9arOCUWJRZXJKoXqzgXpqZkqpQkq_gnJ-Wlpq6g5URAKd3Aq9xAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQxA16BAhKEAs",
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a804f82f30abd848a4d9945d688f96b6a8839c80e39c6bc12bf.jpeg"
      },
      {
        "name": "Uncommon Grounds: The Hist...",
        "link": "https://www.google.com/search?gl=us&hl=en&q=Uncommon+Grounds:+The+History+of+Coffee+and+How+it+Transformed+our+World&stick=H4sIAAAAAAAAACXKMQrCMBSHcYoU1E3xAA9Hl4iIQlcHe4CKc20SWpq8P76kBj2OoyfweCpuHz--8Wg-UV6tN7er9cvpL7dpv3OP1aJvCtXAe3ChkTjVosMrm33NOdPEDqwuQB-eeXni_0hHwcA6FFS1hsouRMidYOkAa42hmjWVSNRFqqTmYCHeaMIgdIY4_c6zD11GmuORAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQxA16BAhKEA0",
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a804f82f30abd848a4d77cc62d518b9e77fe6f8d8ac319dd7e7.jpeg"
      }
    ],
    "coffee_books_link": "https://www.google.com/search?gl=us&hl=en&q=Coffee+books&stick=H4sIAAAAAAAAAONgFuLUz9U3MCorTMtVQjC1RLKTrfST83Nz8_OsUvLL88oTi1KKVzEKAsVyclKTSzLz8_ST8vOzixex8jjnp6WlpiqAuTtYGQHBbXIpVAAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQMSgAegQIShAB",
    "coffee_books_stick": "H4sIAAAAAAAAAONgFuLUz9U3MCorTMtVQjC1RLKTrfST83Nz8_OsUvLL88oTi1KKVzEKAsVyclKTSzLz8_ST8vOzixex8jjnp6WlpiqAuTtYGQHBbXIpVAAAAA",
    "people_also_search_for": [
      {
        "name": "Tea",
        "link": "https://www.google.com/search?gl=us&hl=en&q=Tea&stick=H4sIAAAAAAAAAONgFuLUz9U3MCorTMtV4gAxzZNzKrQEgzNTUssTK4v9UitKgktSC4oXsTKHpCbuYGUEAEjPygozAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQxA16BAhJEAU",
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a8089b8d07732f39762500dde409851a6ef38d30761dbfc211161c7207b14588ee7.jpeg"
      },
      {
        "name": "Espresso",
        "link": "https://www.google.com/search?gl=us&hl=en&q=Espresso&stick=H4sIAAAAAAAAAONgFuLUz9U3MCorTMtV4gAxk0vSsrUEgzNTUssTK4v9UitKgktSC4oXsXK4FhcUpRYX5-9gZQQADwmvdzgAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQxA16BAhJEAc",
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a8089b8d07732f39762500dde409851a6efb1d2c721e38a3e0c4f69575558e9f6ca.jpeg"
      },
      {
        "name": "Drink",
        "link": "https://www.google.com/search?gl=us&hl=en&q=Beverage&stick=H4sIAAAAAAAAAONgFuLUz9U3MCorTMtV4tZP1zc0MkoryDMr0hIMzkxJLU-sLPZLrSgJLkktKF7EyuGUWpZalJieuoOVEQCpF_cVOwAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQxA16BAhJEAk",
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a8089b8d07732f39762500dde409851a6efee840f3eb7471deee823fdfa92a76c46.jpeg"
      },
      {
        "name": "Iced coffee",
        "link": "https://www.google.com/search?gl=us&hl=en&q=Iced+coffee&stick=H4sIAAAAAAAAAONgFuLUz9U3MCorTMtVAjNNs5PKDLUEgzNTUssTK4v9UitKgktSC4oXsXJ7JqemKCTnp6Wlpu5gZQQADVX9azwAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQxA16BAhJEAs",
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a8089b8d07732f39762500dde409851a6ef1a135664f23df48459463d6ea31881ad.jpeg"
      },
      {
        "name": "Latte",
        "link": "https://www.google.com/search?gl=us&hl=en&q=Latte&stick=H4sIAAAAAAAAAONgFuLUz9U3MCorTMtVAjMN84pMzbQEgzNTUssTK4v9UitKgktSC4oXsbL6JJaUpO5gZQQAJJusijYAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQxA16BAhJEA0",
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a8089b8d07732f39762500dde409851a6efde0190811b8b674eac4ab637e518e9cb.jpeg"
      }
    ],
    "people_also_search_for_link": "https://www.google.com/search?gl=us&hl=en&q=Coffee&stick=H4sIAAAAAAAAAONgFuLUz9U3MCorTMtVQjC1BIMzU1LLEyuL_VIrSoJLUguKF7GyOeenpaWm7mBlBABkIv_mNwAAAA&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQMSgAegQISRAB",
    "people_also_search_for_stick": "H4sIAAAAAAAAAONgFuLUz9U3MCorTMtVQjC1BIMzU1LLEyuL_VIrSoJLUguKF7GyOeenpaWm7mBlBABkIv_mNwAAAA",
    "see_results_about": [
      {
        "name": "Coffee bean",
        "extensions": [
          "A coffee bean is a seed of the Coffea plant and the source for ..."
        ],
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a80d449011f57086f189f559fd2a72b80c1e5a9e9078bffd5915248cedad7d63311.jpeg"
      },
      {
        "name": "Coffee",
        "extensions": [
          "Plant"
        ],
        "image": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/465c36209ea9860426d7785256518a80d449011f57086f189f559fd2a72b80c12e5d2aedf43d3fbc5b802f8946d87f54.jpeg"
      }
    ],
    "list": {
      "total_fat": [
        "0 g",
        "0%"
      ],
      "saturated_fat": [
        "0 g",
        "0%"
      ],
      "trans_fat_regulation": [
        "0 g"
      ],
      "cholesterol": [
        "0 mg",
        "0%"
      ],
      "sodium": [
        "5 mg",
        "0%"
      ],
      "potassium": [
        "116 mg",
        "3%"
      ],
      "total_carbohydrate": [
        "0 g",
        "0%"
      ],
      "dietary_fiber": [
        "0 g",
        "0%"
      ],
      "sugar": [
        "0 g"
      ],
      "protein": [
        "0.3 g",
        "0%"
      ],
      "caffeine": [
        "95 mg"
      ],
      "vitamin_c": [
        "0%"
      ],
      "calcium": [
        "0%"
      ],
      "iron": [
        "0%"
      ],
      "vitamin_d": [
        "0%"
      ],
      "vitamin_b6": [
        "0%"
      ],
      "cobalamin": [
        "0%"
      ],
      "magnesium": [
        "1%"
      ]
    }
  },
  "discover_more_places": [
    {
      "title": "Takeout",
      "link": "https://www.google.com/search?gl=us&hl=en&tbm=lcl&q=takeout+food&rflfq=1&num=10&uule=w+CAQQCFISCS8DzKCZtUSGEXrVadRLRptd&lsspp=CdSKrZykpVuv&rlt=Takeout&owsq=coffee&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ9s8CegQIOBAE",
      "images": [
        "https://lh3.googleusercontent.com/QE3L93qm5Lz1UMlSZwPP_DREG4xig_H6_qMqcER9_vPPskpTePFHDhuILq1Cwk0=w157-h157-n"
      ]
    },
    {
      "title": "Delivery",
      "link": "https://www.google.com/search?gl=us&hl=en&tbm=lcl&q=Delivery+Food&rflfq=1&num=10&uule=w+CAQQCFISCS8DzKCZtUSGEXrVadRLRptd&lsspp=&rlt=Delivery&owsq=coffee&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ9s8CegQIOBAG",
      "images": [
        "https://lh3.googleusercontent.com/t204sQ6xcx_aUaQET-81rKJRika8r7Q2Dr3zl3UyXNgDkeorQDpIGeR5EGLssZmW=w157-h157-n"
      ]
    },
    {
      "title": "Coffee and Wi-Fi",
      "link": "https://www.google.com/search?gl=us&hl=en&tbm=lcl&q=cafe+with+wifi&rflfq=1&num=10&uule=w+CAQQCFISCS8DzKCZtUSGEXrVadRLRptd&lsspp=CTZoWvL3zxXX&rlt=Coffee+and+Wi-Fi&owsq=coffee&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ9s8CegQIOBAI",
      "images": [
        "https://lh5.googleusercontent.com/p/AF1QipP4YxFeZQnTsnN7nOlW6J4JKNnTeMFAnRVwPK_J=w157-h157-n-k-no"
      ]
    },
    {
      "title": "Coffee shops",
      "link": "https://www.google.com/search?gl=us&hl=en&tbm=lcl&q=coffee+shop&rflfq=1&num=10&uule=w+CAQQCFISCS8DzKCZtUSGEXrVadRLRptd&lsspp=CUIyoDqm9Y3K&rlt=Coffee+shops&owsq=coffee&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ9s8CegQIOBAK",
      "images": [
        "https://lh5.googleusercontent.com/p/AF1QipPYvOSktWe_NkPi4236dTY_RweAX9zlxBTrc1U_=w157-h157-n-k-no"
      ]
    },
    {
      "title": "Best coffee",
      "link": "https://www.google.com/search?gl=us&hl=en&tbm=lcl&q=best+coffee&rflfq=1&num=10&uule=w+CAQQCFISCS8DzKCZtUSGEXrVadRLRptd&lsspp=Cbd7QWznHn1_&rlt=Best+coffee&owsq=coffee&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ9s8CegQIOBAM",
      "images": [
        "https://lh5.googleusercontent.com/p/AF1QipMpglhK3bFIXSKOi-QBXohDKa6E9yQJUuah8Ivj=w157-h157-n-k-no"
      ]
    },
    {
      "title": "Best breakfasts",
      "link": "https://www.google.com/search?gl=us&hl=en&tbm=lcl&q=best+breakfast&rflfq=1&num=10&uule=w+CAQQCFISCS8DzKCZtUSGEXrVadRLRptd&lsspp=CaT5tVd2YmWY&rlt=Best+breakfasts&owsq=coffee&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ9s8CegQIOBAO",
      "images": [
        "https://lh5.googleusercontent.com/p/AF1QipN3TIQ6EmxL07AawtKEQaQJnAnp4AqRc6Op1Y5v=w157-h157-n-k-no"
      ]
    }
  ],
  "related_questions": [
    {
      "question": "What coffee does to your body?",
      "snippet": "The Bottom Line Not only can your daily cup of joe help you feel more energized, burn fat and improve physical performance, it may also lower your risk of several conditions, such as type 2 diabetes, cancer and Alzheimer's and Parkinson's disease. In fact, coffee may even boost longevity.Aug 14, 2017",
      "title": "13 Health Benefits of Coffee, Based on Science - Healthline",
      "link": "https://www.healthline.com/nutrition/top-13-evidence-based-health-benefits-of-coffee",
      "displayed_link": "https://www.healthline.com › nutrition › top-13-eviden..."
    },
    {
      "question": "Is coffee made from poop?",
      "snippet": "Kopi luwak is coffee made from coffee cherries that have been eaten, digested, and defecated by the Asian palm civet, a small mammal that looks like a cross between a cat and a raccoon. The beans are then cleaned and processed. In the West, kopi luwak has become known as \"cat poop coffee.\"Nov 9, 2018",
      "title": "Kopi Luwak: 'World's Most Expensive Coffee' Is a Tourist Trap",
      "link": "https://www.businessinsider.com/kopi-luwak-cat-poop-worlds-most-expensive-coffee-taste-test-2018-11",
      "displayed_link": "https://www.businessinsider.com › kopi-luwak-cat-poop-..."
    },
    {
      "question": "Why is coffee bad for you?",
      "snippet": "Too much caffeine can also cause anxiety in people with panic or anxiety disorders. For those who drink coffee, experts suggest brewing it with a paper filter, because unfiltered coffee is associated with higher rates of early death, and can contain compounds that raise levels of LDL, or “bad,” cholesterol.",
      "title": "Is coffee good or bad for your health? | News",
      "link": "https://www.hsph.harvard.edu/news/hsph-in-the-news/is-coffee-good-or-bad-for-your-health/",
      "displayed_link": "https://www.hsph.harvard.edu › news › hsph-in-the-news"
    },
    {
      "question": "Is it healthy to drink coffee everyday?",
      "snippet": "Like so many foods and nutrients, too much coffee can cause problems, especially in the digestive tract. But studies have shown that drinking up to four 8-ounce cups of coffee per day is safe. Sticking to those boundaries shouldn't be hard for coffee drinkers in the U.S., since most drink just a cup of java per day.May 5, 2017",
      "title": "The Case For Drinking Coffee Is Stronger Than Ever - Time Magazine",
      "link": "https://time.com/4768860/is-coffee-good-for-you/",
      "displayed_link": "https://time.com › is-coffee-good-for-you"
    }
  ],
  "organic_results": [
    {
      "position": 1,
      "title": "Coffee - Wikipedia",
      "link": "https://en.wikipedia.org/wiki/Coffee",
      "redirect_link": "https://www.google.com/url?sa=t&source=web&rct=j&opi=89978449&url=https://en.wikipedia.org/wiki/Coffee&ved=2ahUKEwiR5vqbm5KDAxUJSzABHetUBPsQFnoECA8QAQ",
      "displayed_link": "https://en.wikipedia.org › wiki › Coffee",
      "snippet": "Coffee is a brewed drink prepared from roasted coffee beans, the seeds of berries from certain Coffea species. From the coffee fruit, the seeds are ...",
      "sitelinks": {
        "inline": [
          {
            "title": "Coffee bean",
            "link": "https://en.wikipedia.org/wiki/Coffee_bean"
          },
          {
            "title": "History",
            "link": "https://en.wikipedia.org/wiki/History_of_coffee"
          },
          {
            "title": "Coffee production",
            "link": "https://en.wikipedia.org/wiki/Coffee_production"
          },
          {
            "title": "Coffee preparation",
            "link": "https://en.wikipedia.org/wiki/Coffee_preparation"
          }
        ]
      },
      "rich_snippet": {
        "bottom": {
          "extensions": [
            "Region of origin: Horn of Africa and ‎South Ara...‎",
            "Color: Black, dark brown, light brown, beige",
            "Introduced: 15th century"
          ],
          "detected_extensions": {
            "introduced_th_century": 15
          }
        }
      },
      "about_this_result": {
        "source": {
          "description": "Wikipedia is a free content, multilingual online encyclopedia written and maintained by a community of volunteers through a model of open collaboration, using a wiki-based editing system. Individual contributors, also called editors, are known as Wikipedians.",
          "source_info_link": "https://en.wikipedia.org/wiki/Wikipedia",
          "security": "secure",
          "icon": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/1757d19c4614c530c07ecd54bbf57867d01831e29d004ddc31b8875ca5ef437c1d9699009c53cdb8c75ee43c7c816d63.png"
        },
        "keywords": [
          "coffee"
        ],
        "languages": [
          "English"
        ],
        "regions": [
          "the United States"
        ]
      },
      "about_page_link": "https://google.com/search?q=About+https://en.wikipedia.org/wiki/Coffee&tbm=ilp&ilps=AOR-xxt3p1dy2npn9cHfxrbKVqVZCHn3Cg",
      "about_page_serpapi_link": "https://serpapi.com/search.json?engine=google_about_this_result&ilps=AOR-xxt3p1dy2npn9cHfxrbKVqVZCHn3Cg&q=About+https://en.wikipedia.org/wiki/Coffee",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:U6oJMnF-eeUJ:https://en.wikipedia.org/wiki/Coffee+&cd=1&hl=en&ct=clnk&gl=us",
      "related_pages_link": "https://www.google.com/search?gl=us&hl=en&q=related:https://en.wikipedia.org/wiki/Coffee+coffee"
    },
    {
      "position": 2,
      "title": "21 Excellent Coffee Shops in Austin",
      "link": "https://austin.eater.com/maps/best-coffee-austin-cafes-patio-latte-pour-over",
      "redirect_link": "https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwjes5qBsK2DAxXbKlkFHQlYBXUQFnoECA8QAQ&url=https%3A%2F%2Faustin.eater.com%2Fmaps%2Fbest-coffee-austin-cafes-patio-latte-pour-over&usg=AOvVaw0vUb5Alb8C6YUpwuOUzMNK&opi=89978449",
      "displayed_link": "https://austin.eater.com › maps › best-coffee-austin-cafe...",
      "date": "5 days ago",
      "snippet": "21 Excellent Coffee Shops in Austin · 1. Barrett's Coffee · 2. Epoch Coffee · 3. Sa-Tén Coffee & Eats · 4. Houndstooth Coffee · 5. Civil Goat Coffee.",
      "about_this_result": {
        "source": {
          "description": "Eater is a food website by Vox Media. It was co-founded by Lockhart Steele and Ben Leventhal in 2005, and originally focused on dining and nightlife in New York City. Eater launched a national site in 2009, and covered nearly 20 cities by 2012.",
          "source_info_link": "https://en.wikipedia.org/wiki/Eater_(website)",
          "security": "secure",
          "icon": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/1757d19c4614c530c07ecd54bbf578674b65d56d05926615c9e7fa12be6f06fe1855aeabae1de5a388097b071638c318.png"
        },
        "keywords": [
          "coffee"
        ],
        "languages": [
          "English"
        ]
      },
      "about_page_link": "https://google.com/search?q=About+https://austin.eater.com/maps/best-coffee-austin-cafes-patio-latte-pour-over&tbm=ilp&ilps=AOR-xxt1YQD-eKACl4hhka8ptbcB-c-VJQ",
      "about_page_serpapi_link": "https://serpapi.com/search.json?engine=google_about_this_result&ilps=AOR-xxt1YQD-eKACl4hhka8ptbcB-c-VJQ&q=About+https://austin.eater.com/maps/best-coffee-austin-cafes-patio-latte-pour-over",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:uhpBGQL0eGQJ:https://austin.eater.com/maps/best-coffee-austin-cafes-patio-latte-pour-over+&cd=14&hl=en&ct=clnk&gl=us"
    },
    {
      "position": 3,
      "title": "Home | The Coffee Bean & Tea Leaf",
      "link": "https://www.coffeebean.com/",
      "redirect_link": "https://www.google.com/url?sa=t&source=web&rct=j&opi=89978449&url=https://www.coffeebean.com/&ved=2ahUKEwj3gdzSm5KDAxXdIEQIHW5OCPkQFnoECAkQAQ",
      "displayed_link": "https://www.coffeebean.com",
      "snippet": "Icon of a bag of coffee being shipped to you. Subscriptions. Never run out of your favorite coffees, teas and powders again with our auto-delivery subscription.",
      "sitelinks": {
        "inline": [
          {
            "title": "Store locator",
            "link": "https://www.coffeebean.com/store-locator"
          },
          {
            "title": "Coffee",
            "link": "https://www.coffeebean.com/cafe-menu/coffee"
          },
          {
            "title": "Cafe Menu",
            "link": "https://www.coffeebean.com/cafe-menu"
          },
          {
            "title": "Flavored Coffee",
            "link": "https://store.coffeebean.com/collections/flavored-coffee"
          }
        ]
      },
      "about_this_result": {
        "source": {
          "description": "The Coffee Bean & Tea Leaf is an American coffee shop chain founded in 1963. Since 2019, it is a trade name of Ireland-based Super Magnificent Coffee Company Ireland Limited, itself wholly owned by Philippines-based Jollibee Foods Corporation.",
          "source_info_link": "https://en.wikipedia.org/wiki/The_Coffee_Bean_%26_Tea_Leaf",
          "security": "secure",
          "icon": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/1757d19c4614c530c07ecd54bbf578672f5ff28f8f4481f4c2d8836164c49755e13bd84b4ed7686eedb810a0168c2551.png"
        },
        "keywords": [
          "coffee"
        ],
        "related_keywords": [
          "coffees"
        ],
        "languages": [
          "English"
        ],
        "regions": [
          "the United States"
        ]
      },
      "about_page_link": "https://google.com/search?q=About+https://www.coffeebean.com/&tbm=ilp&ilps=AOR-xxsK_iBv-AalfdLkk76RB2nfCqGDRg",
      "about_page_serpapi_link": "https://serpapi.com/search.json?engine=google_about_this_result&ilps=AOR-xxsK_iBv-AalfdLkk76RB2nfCqGDRg&q=About+https://www.coffeebean.com/",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:WpQxSYo2c6AJ:https://www.coffeebean.com/+&cd=15&hl=en&ct=clnk&gl=us",
      "related_pages_link": "https://www.google.com/search?gl=us&hl=en&q=related:https://www.coffeebean.com/+coffee"
    },
    {
      "position": 4,
      "title": "coffee - Amazon.com",
      "link": "https://www.amazon.com/coffee/s?k=coffee",
      "redirect_link": "https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwidusrRr62DAxXqM1kFHXkED2MQFnoECBQQAQ&url=https%3A%2F%2Fwww.amazon.com%2Fcoffee%2Fs%3Fk%3Dcoffee&usg=AOvVaw1n6DOCs_IhX9GOg-VvWTwJ&opi=89978449",
      "displayed_link": "https://www.amazon.com › coffee › k=coffee",
      "thumbnail": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/1757d19c4614c530c07ecd54bbf57867eaacdbf8fb9d922ffe20e03024333f84.jpeg",
      "displayed_results": "Results 1 - 48 of 20000+",
      "snippet": "Coffee is the most important way for anyone to get going in the morning. It's also a very popular drink being consumed daily by ...",
      "rich_snippet": {
        "top": {
          "detected_extensions": {
            "results_of": 1
          },
          "extensions": [
            "Results 1 - 48 of 20000+ —"
          ]
        }
      },
      "about_this_result": {
        "source": {
          "description": "Amazon.com, Inc. is an American multinational technology company which focuses on e-commerce, cloud computing, digital streaming, and artificial intelligence. It is one of the Big Five companies in the U.S. information technology industry, along with Google, Apple, Meta, and Microsoft.",
          "source_info_link": "https://en.wikipedia.org/wiki/Amazon_(company)",
          "security": "secure",
          "icon": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/1757d19c4614c530c07ecd54bbf57867bb1ce79e882019c173ffe420ae8d96c57cfdd510b2eae396d5647113b63b4f1f.png"
        },
        "keywords": [
          "coffee"
        ],
        "languages": [
          "English"
        ],
        "regions": [
          "the United States"
        ]
      },
      "about_page_link": "https://google.com/search?q=About+https://www.amazon.com/coffee/s?k%3Dcoffee&tbm=ilp&ilps=AOR-xxvN48fSB4DsG-gRZAFF2iGkNFGOhQ",
      "about_page_serpapi_link": "https://serpapi.com/search.json?engine=google_about_this_result&ilps=AOR-xxvN48fSB4DsG-gRZAFF2iGkNFGOhQ&q=About+https://www.amazon.com/coffee/s?k%3Dcoffee",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:wfQ5Et9Ni-kJ:https://www.amazon.com/coffee/s%3Fk%3Dcoffee+&cd=16&hl=en&ct=clnk&gl=us"
    },
    {
      "position": 5,
      "title": "Peet's Coffee: The Original Craft Coffee",
      "link": "https://www.peets.com/",
      "redirect_link": "https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwj0-9eesK2DAxXuKlkFHa3UBewQFnoECBYQAQ&url=https%3A%2F%2Fwww.peets.com%2F&usg=AOvVaw35gJjisbo_2l0MyTsLBvru&opi=89978449",
      "displayed_link": "https://www.peets.com",
      "snippet": "A rare, highly esteemed coffee that lives up to its legendary expectations. Elegant and aromatic, with refined acidity and milk chocolate sweetness.",
      "about_this_result": {
        "source": {
          "description": "Peet's Coffee is a San Francisco Bay Area-based specialty coffee roaster and retailer owned by JAB Holding Company.",
          "source_info_link": "https://en.wikipedia.org/wiki/Peet's_Coffee",
          "security": "secure",
          "icon": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/1757d19c4614c530c07ecd54bbf57867e49c71126c63a887915bf5d6cc5552890775d1abe1c784cc4147aec8c9e6846a.png"
        },
        "keywords": [
          "coffee"
        ],
        "related_keywords": [
          "coffees"
        ],
        "languages": [
          "English"
        ],
        "regions": [
          "the United States"
        ]
      },
      "about_page_link": "https://google.com/search?q=About+https://www.peets.com/&tbm=ilp&ilps=AOR-xxuNHmAq-m3vpZkBakErLDmn38eeaw",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:BCjzno6zP6wJ:https://www.peets.com/+&cd=17&hl=en&ct=clnk&gl=us",
      "related_pages_link": "https://www.google.com/search?gl=us&hl=en&q=related:https://www.peets.com/+coffee"
    },
    {
      "position": 6,
      "title": "13 Health Benefits of Coffee, Based on Science - Healthline",
      "link": "https://www.healthline.com/nutrition/top-13-evidence-based-health-benefits-of-coffee",
      "displayed_link": "https://www.healthline.com › nutrition › top-13-eviden...",
      "date": "Sep 20, 2018",
      "snippet": "Coffee is one of the world's most popular beverages. Thanks to its high levels of antioxidants and beneficial nutrients, it also seems to be ...",
      "sitelinks": {
        "inline": [
          {
            "title": "Coffee with Lemon",
            "link": "https://www.healthline.com/nutrition/coffee-with-lemon"
          },
          {
            "title": "Why Does Coffee Make You...",
            "link": "https://www.healthline.com/nutrition/why-does-coffee-make-you-poop"
          },
          {
            "title": "Decaf Coffee: Good or Bad?",
            "link": "https://www.healthline.com/nutrition/decaf-coffee-good-or-bad"
          }
        ]
      },
      "about_this_result": {
        "source": {
          "description": "Healthline Media, Inc. is an American website and provider of health information headquartered in San Francisco, CA. It was founded in 2006, and established as a standalone entity in January 2016. As of October 2020, it had a global ranking of 188 by Alexa.",
          "source_info_link": "https://en.wikipedia.org/wiki/Healthline",
          "security": "secure",
          "icon": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/1757d19c4614c530c07ecd54bbf5786739cd13f7bdb40fc38e02182388996832a74c5bfd855ed4b05ef2fb99b4bcded7.png"
        },
        "keywords": [
          "coffee"
        ],
        "languages": [
          "English"
        ],
        "regions": [
          "the United States"
        ]
      },
      "about_page_link": "https://google.com/search?q=About+https://www.healthline.com/nutrition/top-13-evidence-based-health-benefits-of-coffee&tbm=ilp&ilps=AOR-xxt9UXznxCxfopZPvp5Lcg-Davh6Lg",
      "about_page_serpapi_link": "https://serpapi.com/search.json?engine=google_about_this_result&ilps=AOR-xxt9UXznxCxfopZPvp5Lcg-Davh6Lg&q=About+https://www.healthline.com/nutrition/top-13-evidence-based-health-benefits-of-coffee",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:u8fDpqqQvRgJ:https://www.healthline.com/nutrition/top-13-evidence-based-health-benefits-of-coffee+&cd=18&hl=en&ct=clnk&gl=us"
    },
    {
      "position": 7,
      "title": "The History of Coffee",
      "link": "https://www.ncausa.org/about-coffee/history-of-coffee",
      "redirect_link": "https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwjx-sStsK2DAxUeK1kFHZuhBUEQFnoECFsQAQ&url=https%3A%2F%2Fwww.ncausa.org%2Fabout-coffee%2Fhistory-of-coffee&usg=AOvVaw2373DnPczOXlzdqfgXe0nZ&opi=89978449",
      "displayed_link": "https://www.ncausa.org › ... › History of Coffee",
      "snippet": "The story goes that that Kaldi discovered coffee after he noticed that after eating the berries from a certain tree, his goats became so energetic that they did ...",
      "about_this_result": {
        "source": {
          "description": "The National Coffee Association or, is the main market research, consumer information, and lobbying association for the coffee industry in the United States.",
          "source_info_link": "https://en.wikipedia.org/wiki/National_Coffee_Association",
          "security": "secure"
        },
        "keywords": [
          "coffee"
        ],
        "languages": [
          "English"
        ],
        "regions": [
          "the United States"
        ]
      },
      "about_page_link": "https://google.com/search?q=About+https://www.ncausa.org/about-coffee/history-of-coffee&tbm=ilp&ilps=AOR-xxtXJWBYDvmb2eGbgxIkIjhfBGraVw",
      "about_page_serpapi_link": "https://serpapi.com/search.json?engine=google_about_this_result&ilps=AOR-xxtXJWBYDvmb2eGbgxIkIjhfBGraVw&q=About+https://www.ncausa.org/about-coffee/history-of-coffee",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:v1hp0SS8WggJ:https://www.ncausa.org/about-coffee/history-of-coffee+&cd=19&hl=en&ct=clnk&gl=us"
    },
    {
      "position": 8,
      "title": "Starbucks Coffee Company",
      "link": "https://www.starbucks.com/",
      "redirect_link": "https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwj2ht67sK2DAxXlF1kFHZNwCNMQFnoECA4QAQ&url=https%3A%2F%2Fwww.starbucks.com%2F&usg=AOvVaw1G5PhqPw6TVos-9iAP_inH&opi=89978449",
      "displayed_link": "https://www.starbucks.com",
      "snippet": "More than just great coffee. Explore the menu, sign up for Starbucks® Rewards, manage your gift card and more.",
      "about_this_result": {
        "source": {
          "description": "Starbucks Corporation is an American multinational chain of coffeehouses and roastery reserves headquartered in Seattle, Washington. It is the world's largest coffeehouse chain. As of November 2021, the company had 33,833 stores in 80 countries, 15,444 of which were located in the United States.",
          "source_info_link": "https://en.wikipedia.org/wiki/Starbucks",
          "security": "secure",
          "icon": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/1757d19c4614c530c07ecd54bbf57867e221b686da06a3ed72df6854b45df35180eaf7f6564196ccd97fb27d2cf57817.png"
        },
        "keywords": [
          "coffee"
        ],
        "languages": [
          "English"
        ],
        "regions": [
          "the United States"
        ]
      },
      "about_page_link": "https://google.com/search?q=About+https://www.starbucks.com/&tbm=ilp&ilps=AOR-xxvY69hdA-Qdnz5qYjjIfy4l2TW7eA",
      "about_page_serpapi_link": "https://serpapi.com/search.json?engine=google_about_this_result&ilps=AOR-xxvY69hdA-Qdnz5qYjjIfy4l2TW7eA&q=About+https://www.starbucks.com/",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:1vGXgo_FlHkJ:https://www.starbucks.com/+&cd=23&hl=en&ct=clnk&gl=us",
      "related_pages_link": "https://www.google.com/search?gl=us&hl=en&q=related:https://www.starbucks.com/+coffee"
    },
    {
      "position": 9,
      "title": "Coffee Review - The World's Leading Coffee Guide",
      "link": "https://www.coffeereview.com/",
      "redirect_link": "https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwi4yq7PsK2DAxVUFFkFHZg2CjIQFnoECAkQAQ&url=https%3A%2F%2Fwww.coffeereview.com%2F&usg=AOvVaw1RmIsHTmQGccxkf6pVSTSM&opi=89978449",
      "displayed_link": "https://www.coffeereview.com",
      "snippet": "Coffee reviews, espresso ratings, informative articles, and coffee blogs by Kenneth Davids and other coffee experts.",
      "about_this_result": {
        "source": {
          "description": "coffeereview.com was first indexed by Google more than 10 years ago",
          "security": "secure",
          "icon": "https://serpapi.com/searches/61afb3ace7d08a685b3bcbb1/images/1757d19c4614c530c07ecd54bbf57867d314f23e886ecee06ce7b8b5210443a781895281882d1aa4722a64e064facccc.png"
        },
        "keywords": [
          "coffee"
        ],
        "languages": [
          "English"
        ],
        "regions": [
          "the United States"
        ]
      },
      "about_page_link": "https://google.com/search?q=About+https://www.coffeereview.com/&tbm=ilp&ilps=AOR-xxsD9rLq0oHNEmTNVcizAauKlrJO4g",
      "about_page_serpapi_link": "https://serpapi.com/search.json?engine=google_about_this_result&ilps=AOR-xxsD9rLq0oHNEmTNVcizAauKlrJO4g&q=About+https://www.coffeereview.com/",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:c9jncvPT1SsJ:https://www.coffeereview.com/+&cd=24&hl=en&ct=clnk&gl=us"
    }
  ],
  "related_searches": [
    {
      "query": "coffee brands",
      "link": "https://www.google.com/search?gl=us&hl=en&q=Coffee+brands&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ1QJ6BAggEAE"
    },
    {
      "query": "coffee beans",
      "link": "https://www.google.com/search?gl=us&hl=en&q=Coffee+beans&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ1QJ6BAgsEAE"
    },
    {
      "query": "coffee near me",
      "link": "https://www.google.com/search?gl=us&hl=en&q=Coffee+near+me&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ1QJ6BAgkEAE"
    },
    {
      "query": "coffee online",
      "link": "https://www.google.com/search?gl=us&hl=en&q=Coffee+online&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ1QJ6BAgoEAE"
    },
    {
      "query": "types of coffee",
      "link": "https://www.google.com/search?gl=us&hl=en&q=Types+of+coffee&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ1QJ6BAgnEAE"
    },
    {
      "query": "coffee maker",
      "link": "https://www.google.com/search?gl=us&hl=en&q=Coffee+Maker&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ1QJ6BAgmEAE"
    },
    {
      "query": "coffee recipe",
      "link": "https://www.google.com/search?gl=us&hl=en&q=Coffee+recipe&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ1QJ6BAgpEAE"
    },
    {
      "query": "coffee origin",
      "link": "https://www.google.com/search?gl=us&hl=en&q=Coffee+origin&sa=X&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ1QJ6BAgiEAE"
    }
  ],
  "pagination": {
    "current": 1,
    "next": "https://www.google.com/search?q=coffee&gl=us&hl=en&ei=0bOvYffWDvvV1sQP1OGSkAY&start=10&sa=N&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ8NMDegQIAhB2",
    "other_pages": {
      "2": "https://www.google.com/search?q=coffee&gl=us&hl=en&ei=0bOvYffWDvvV1sQP1OGSkAY&start=10&sa=N&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ8tMDegQIAhBk",
      "3": "https://www.google.com/search?q=coffee&gl=us&hl=en&ei=0bOvYffWDvvV1sQP1OGSkAY&start=20&sa=N&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ8tMDegQIAhBm",
      "4": "https://www.google.com/search?q=coffee&gl=us&hl=en&ei=0bOvYffWDvvV1sQP1OGSkAY&start=30&sa=N&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ8tMDegQIAhBo",
      "5": "https://www.google.com/search?q=coffee&gl=us&hl=en&ei=0bOvYffWDvvV1sQP1OGSkAY&start=40&sa=N&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ8tMDegQIAhBq",
      "6": "https://www.google.com/search?q=coffee&gl=us&hl=en&ei=0bOvYffWDvvV1sQP1OGSkAY&start=50&sa=N&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ8tMDegQIAhBs",
      "7": "https://www.google.com/search?q=coffee&gl=us&hl=en&ei=0bOvYffWDvvV1sQP1OGSkAY&start=60&sa=N&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ8tMDegQIAhBu",
      "8": "https://www.google.com/search?q=coffee&gl=us&hl=en&ei=0bOvYffWDvvV1sQP1OGSkAY&start=70&sa=N&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ8tMDegQIAhBw",
      "9": "https://www.google.com/search?q=coffee&gl=us&hl=en&ei=0bOvYffWDvvV1sQP1OGSkAY&start=80&sa=N&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ8tMDegQIAhBy",
      "10": "https://www.google.com/search?q=coffee&gl=us&hl=en&ei=0bOvYffWDvvV1sQP1OGSkAY&start=90&sa=N&ved=2ahUKEwi3g66MtNL0AhX7qpUCHdSwBGIQ8tMDegQIAhB0"
    }
  },
  "serpapi_pagination": {
    "current": 1,
    "next_link": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Austin%2C+Texas%2C+United+States&q=coffee&start=10",
    "next": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Austin%2C+Texas%2C+United+States&q=coffee&start=10",
    "other_pages": {
      "2": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Austin%2C+Texas%2C+United+States&q=coffee&start=10",
      "3": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Austin%2C+Texas%2C+United+States&q=coffee&start=20",
      "4": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Austin%2C+Texas%2C+United+States&q=coffee&start=30",
      "5": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Austin%2C+Texas%2C+United+States&q=coffee&start=40",
      "6": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Austin%2C+Texas%2C+United+States&q=coffee&start=50",
      "7": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Austin%2C+Texas%2C+United+States&q=coffee&start=60",
      "8": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Austin%2C+Texas%2C+United+States&q=coffee&start=70",
      "9": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Austin%2C+Texas%2C+United+States&q=coffee&start=80",
      "10": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Austin%2C+Texas%2C+United+States&q=coffee&start=90"
    }
  }
}
More complex examples with multiple optional parameters
The URL below fetches:

• The second page (start=10 and num=10) of the results,
• for the search "Fresh Bagels" in Seattle,
• using Google.com in the United States,
• with the Google language UI set to English,
• with the adult content filter on,
• using the demo SerpApi key,
• and output the results in JSON.
GET


https://serpapi.com/search.json?engine=google&q=Fresh+Bagels&location=Seattle-Tacoma,+WA,+Washington,+United+States&hl=en&gl=us&google_domain=google.com&num=10&start=10&safe=active

Code to integrate

cURLRubyPythonJavaScriptPHP.NETJavaGoRustGoogle Sheets

require 'google_search_results' 

params = {
  engine: "google",
  q: "Fresh Bagels",
  location: "Seattle-Tacoma, WA, Washington, United States",
  hl: "en",
  gl: "us",
  google_domain: "google.com",
  num: "10",
  start: "10",
  safe: "active",
  api_key: "a602132fb4dd2bad19d4df9532f26aa36d8bfadd8b08311f5fd96db7178b261c"
}

search = GoogleSearch.new(params)
organic_results = search.get_hash[:organic_results]

JSON Example

{
  "search_metadata": {
    "id": "62d5efcae7d08a486c7b86ad",
    "status": "Success",
    "json_endpoint": "https://serpapi.com/searches/3ed30d31b62f3cfd/62d5efcae7d08a486c7b86ad.json",
    "created_at": "2022-07-18 23:42:02 UTC",
    "processed_at": "2022-07-18 23:42:02 UTC",
    "google_url": "https://www.google.com/search?q=Fresh+Bagels&oq=Fresh+Bagels&uule=w+CAIQICIqU2VhdHRsZS1UYWNvbWEsV0EsV2FzaGluZ3RvbixVbml0ZWQgU3RhdGVz&hl=en&gl=us&num=10&safe=active&start=10&sourceid=chrome&ie=UTF-8",
    "raw_html_file": "https://serpapi.com/searches/3ed30d31b62f3cfd/62d5efcae7d08a486c7b86ad.html",
    "total_time_taken": 1.35
  },
  "search_parameters": {
    "engine": "google",
    "q": "Fresh Bagels",
    "location_requested": "Seattle-Tacoma, WA, Washington, United States",
    "location_used": "Seattle-Tacoma,WA,Washington,United States",
    "google_domain": "google.com",
    "hl": "en",
    "gl": "us",
    "safe": "active",
    "start": 10,
    "num": "10",
    "device": "desktop"
  },
  "search_information": {
    "organic_results_state": "Results for exact spelling",
    "query_displayed": "Fresh Bagels",
    "total_results": 200000000,
    "page_number": 2,
    "time_taken_displayed": 0.63
  },
  "organic_results": [
    {
      "position": 1,
      "title": "The 7 Best Bagel Shops In Seattle - boam",
      "link": "https://boam.com/wa/seattle/best-bagel-shops/",
      "redirect_link": "https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwj2ht67sK2DAxXlF1kFHZNwCNMQFnoECA4QAQ&url=https%3A%2F%2Fboam.com%2Fwa%2Fseattle%2Fbest-bagel-shops%2F&usg=AOvVaw1G5PhqPw6TVos-9iAP_inH&opi=89978449"
      "displayed_link": "https://boam.com › WA › Seattle",
      "thumbnail": "https://serpapi.com/searches/62d5efcae7d08a486c7b86ad/images/02306781b6e93635966d70cf70c16bc2e20ef7a5b15f3cc76dc37b5107d6e34b.jpeg",
      "date": "May 2, 2022",
      "snippet": "Zylberschtein's Delicatessen & Bakery · The Ultimate List of the Best Bagels In Seattle, Ranked · Seattle Actually Has a Bagel Scene Now.",
      "snippet_highlighted_words": [
        "Best Bagels"
      ],
      "about_this_result": {
        "source": {
          "description": "boam.com was first indexed by Google more than 10 years ago",
          "icon": "https://serpapi.com/searches/62d5efcae7d08a486c7b86ad/images/02306781b6e93635966d70cf70c16bc2538e80db255d5d8270fc12ca4267e08fb2f662f2667d2872a277b3d2ec9557e9.png"
        },
        "keywords": [
          "bagels"
        ],
        "related_keywords": [
          "bagel"
        ],
        "languages": [
          "English"
        ],
        "regions": [
          "the United States"
        ]
      },
      "about_page_link": "https://www.google.com/search?q=About+https://boam.com/wa/seattle/best-bagel-shops/&tbm=ilp&ilps=ADNMCi0YHj-gX85Ugpr7byPA3OZJqM5buA",
      "about_page_serpapi_link": "https://serpapi.com/search.json?engine=google_about_this_result&ilps=ADNMCi0YHj-gX85Ugpr7byPA3OZJqM5buA&q=About+https://boam.com/wa/seattle/best-bagel-shops/",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:_46756rn-DYJ:https://boam.com/wa/seattle/best-bagel-shops/+&cd=11&hl=en&ct=clnk&gl=us"
    },
    {
      "position": 2,
      "title": "Einstein Bros. Bagels: Home",
      "link": "https://www.einsteinbros.com/",
      "redirect_link": "https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwjmqP3Hsa2DAxWIF1kFHeISDdEQFnoECBUQAQ&url=https%3A%2F%2Fwww.einsteinbros.com%2F&usg=AOvVaw295ghVT-PGz-aBzCNE_CWq&opi=89978449",
      "displayed_link": "https://www.einsteinbros.com",
      "snippet": "Couple eating Einstein Bros. bagel egg sandwiches with coffee ... Our Menu. Fresh-Baked Bagels, Specialty Sandwiches, Coffee and more.",
      "snippet_highlighted_words": [
        "bagel",
        "Fresh",
        "Bagels"
      ],
      "about_this_result": {
        "source": {
          "description": "Einstein Bros. Bagels is an American chain that specializes in bagels and coffee. In 1996, Berkeley-based Noah's Bagels was bought out by Einstein Bros. Manhattan-based New World Coffee, which bought out Manhattan Bagel in 1998, bought out Einstein Bros. in 2000 post-bankruptcy.",
          "icon": "https://serpapi.com/searches/62d5efcae7d08a486c7b86ad/images/02306781b6e93635966d70cf70c16bc2aecbb5efc76b6e09c807aa832d5370c842dd51865b1a58bcdf7acbad94ce77f8.png"
        },
        "keywords": [
          "fresh",
          "bagels"
        ],
        "related_keywords": [
          "bagel"
        ],
        "languages": [
          "English"
        ],
        "regions": [
          "the United States"
        ]
      },
      "about_page_link": "https://www.google.com/search?q=About+https://www.einsteinbros.com/&tbm=ilp&ilps=ADNMCi3-IrycLvrpky0-3olpqCMPCat1JQ",
      "about_page_serpapi_link": "https://serpapi.com/search.json?engine=google_about_this_result&ilps=ADNMCi3-IrycLvrpky0-3olpqCMPCat1JQ&q=About+https://www.einsteinbros.com/",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:W6Fk1TntVPwJ:https://www.einsteinbros.com/+&cd=12&hl=en&ct=clnk&gl=us",
      "related_pages_link": "https://www.google.com/search?safe=active&hl=en&gl=us&q=related:https://www.einsteinbros.com/+Fresh+Bagels"
    },
    {
      "position": 3,
      "title": "Here's where to find the best bagels in Seattle - Daily Hive",
      "link": "https://dailyhive.com/seattle/best-bagels-in-seattle",
      "redirect_link": "https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwjh6uPTsa2DAxVdEVkFHU96AwsQFnoECBgQAQ&url=https%3A%2F%2Fdailyhive.com%2Fseattle%2Fbest-bagels-in-seattle&usg=AOvVaw3FdhRzbq_sIq03UpGpX3pQ&opi=89978449",
      "displayed_link": "https://dailyhive.com › seattle › best-bagels-in-seattle",
      "thumbnail": "https://serpapi.com/searches/62d5efcae7d08a486c7b86ad/images/02306781b6e93635966d70cf70c16bc2ec9e582273005b8653ba8b50d218eaa0.jpeg",
      "date": "Jan 15, 2021",
      "snippet": "Whether you're after a dense, slightly sweet bagel or a soft, doughy New York-style bagel, check out our picks for where to find the best ...",
      "snippet_highlighted_words": [
        "bagel",
        "bagel",
        "best"
      ],
      "about_this_result": {
        "source": {
          "description": "Daily Hive, formerly known as Vancity Buzz, is a Canadian online newspaper based in Vancouver, British Columbia. It began digital publishing in 2008 and became Western Canada's largest online-only publication by 2016.",
          "icon": "https://serpapi.com/searches/62d5efcae7d08a486c7b86ad/images/02306781b6e93635966d70cf70c16bc215a2020d35579a21332efc9087e61bfefff91a7125cdab79a97a1cb6288ce84f.png"
        },
        "keywords": [
          "bagels"
        ],
        "related_keywords": [
          "bagel"
        ],
        "languages": [
          "English"
        ],
        "regions": [
          "the United States"
        ]
      },
      "about_page_link": "https://www.google.com/search?q=About+https://dailyhive.com/seattle/best-bagels-in-seattle&tbm=ilp&ilps=ADNMCi1lx-vDLNV9Lk3ui6_0CxYsT1zIug",
      "about_page_serpapi_link": "https://serpapi.com/search.json?engine=google_about_this_result&ilps=ADNMCi1lx-vDLNV9Lk3ui6_0CxYsT1zIug&q=About+https://dailyhive.com/seattle/best-bagels-in-seattle",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:9m9a9OBplrAJ:https://dailyhive.com/seattle/best-bagels-in-seattle+&cd=13&hl=en&ct=clnk&gl=us"
    },
    {
      "position": 4,
      "title": "Best Bagels in Seattle - Do206",
      "link": "https://do206.com/p/id/17521",
      "redirect_link": "https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwicruTisa2DAxUtF1kFHdsADNUQFnoECBAQAw&url=https%3A%2F%2Fdo206.com%2Fp%2Fbest-bagels-in-seattle&usg=AOvVaw1RxU-et6iPtTD5paDCDJ3o&opi=89978449",
      "displayed_link": "https://do206.com › ...",
      "thumbnail": "https://serpapi.com/searches/62d5efcae7d08a486c7b86ad/images/02306781b6e93635966d70cf70c16bc2befd0979111c077ee87b8eb551995b80.jpeg",
      "snippet": "Best Bagels in Seattle · Eltana Wood-Fired Bagel Cafe · Rubinstein Bagels · Westman's Bagel and Coffee · Rachel's Bagels & Burritos · Zylberschtein's Delicatessen & ...",
      "snippet_highlighted_words": [
        "Best Bagels"
      ],
      "about_this_result": {
        "source": {
          "description": "do206.com was first indexed by Google more than 10 years ago",
          "icon": "https://serpapi.com/searches/62d5efcae7d08a486c7b86ad/images/02306781b6e93635966d70cf70c16bc27005428c3e2adb10c3725e23efe4140797bfd06e54dff911537b7fc496355839.png"
        },
        "keywords": [
          "fresh",
          "bagels"
        ],
        "languages": [
          "English"
        ],
        "regions": [
          "the United States"
        ]
      },
      "about_page_link": "https://www.google.com/search?q=About+https://do206.com/p/id/17521&tbm=ilp&ilps=ADNMCi0NS-fBBl4Sc4IWKniB87uup6G3hA",
      "about_page_serpapi_link": "https://serpapi.com/search.json?engine=google_about_this_result&ilps=ADNMCi0NS-fBBl4Sc4IWKniB87uup6G3hA&q=About+https://do206.com/p/id/17521",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:If60T0P9qFoJ:https://do206.com/p/id/17521+&cd=14&hl=en&ct=clnk&gl=us"
    },
    {
      "position": 5,
      "title": "“Great fresh bagels” Review of Seattle Bagel Bakery - CLOSED",
      "link": "https://www.tripadvisor.com/ShowUserReviews-g60878-d3846442-r537463219-Seattle_Bagel_Bakery-Seattle_Washington.html",
      "reirect_link": "https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwj94rTzsa2DAxWhLFkFHfW6B1QQFnoECCAQAQ&url=https%3A%2F%2Fwww.tripadvisor.com%2FShowUserReviews-g60878-d3846442-r537463219-Seattle_Bagel_Bakery-Seattle_Washington.html&usg=AOvVaw3XMsBJvQs_mDRyB4Gw_f32&opi=89978449",
      "displayed_link": "https://www.tripadvisor.com › ... › Seattle Bagel Bakery",
      "snippet": "Seattle Bagel Bakery: Great fresh bagels - See 33 traveler reviews, 9 candid photos, and great deals for Seattle, WA, at Tripadvisor.",
      "snippet_highlighted_words": [
        "fresh bagels"
      ],
      "rich_snippet": {
        "top": {
          "detected_extensions": {
            "rating": 5,
            "review_by_jennaviles": 2
          },
          "extensions": [
            "Rating: 5",
            "‎Review by jennaviles2"
          ]
        }
      },
      "about_this_result": {
        "source": {
          "description": "Tripadvisor, Inc. is an American online travel company that operates a website and mobile app with user-generated content and a comparison shopping website. It also offers online hotel reservations and bookings for transportation, lodging, travel experiences, and restaurants.",
          "icon": "https://serpapi.com/searches/62d5efcae7d08a486c7b86ad/images/02306781b6e93635966d70cf70c16bc289f7cc417af569cbacf6edd042617b3405b0d86156f8aaffc2aceff58ced1713.png"
        },
        "keywords": [
          "fresh",
          "bagels"
        ],
        "related_keywords": [
          "bagel"
        ],
        "languages": [
          "English"
        ],
        "regions": [
          "the United States"
        ]
      },
      "about_page_link": "https://www.google.com/search?q=About+https://www.tripadvisor.com/ShowUserReviews-g60878-d3846442-r537463219-Seattle_Bagel_Bakery-Seattle_Washington.html&tbm=ilp&ilps=ADNMCi2e3NRuFXI24IoiwLIVxH2Drgljtw",
      "about_page_serpapi_link": "https://serpapi.com/search.json?engine=google_about_this_result&ilps=ADNMCi2e3NRuFXI24IoiwLIVxH2Drgljtw&q=About+https://www.tripadvisor.com/ShowUserReviews-g60878-d3846442-r537463219-Seattle_Bagel_Bakery-Seattle_Washington.html",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:USHKQJmIIkkJ:https://www.tripadvisor.com/ShowUserReviews-g60878-d3846442-r537463219-Seattle_Bagel_Bakery-Seattle_Washington.html+&cd=15&hl=en&ct=clnk&gl=us"
    },
    {
      "position": 6,
      "title": "Seattle Bagel Oasis",
      "link": "https://seattlebageloasis.com/index.html",
      "redirect_link": "https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwiRytCUsq2DAxUyElkFHRjXAvsQFnoECAcQAQ&url=https%3A%2F%2Fseattlebageloasis.com%2Findex.html&usg=AOvVaw1N8YPQWFOp3XrcMFeN0sa6&opi=89978449",
      "displayed_link": "https://seattlebageloasis.com",
      "snippet": "Since 1988, Bagel Oasis has been baking the best bagels in Seattle with only natural ingredients and old fashioned baking methods.",
      "snippet_highlighted_words": [
        "best bagels"
      ],
      "about_this_result": {
        "source": {
          "description": "seattlebageloasis.com was first indexed by Google more than 10 years ago"
        },
        "keywords": [
          "fresh",
          "bagels"
        ],
        "related_keywords": [
          "bagel"
        ],
        "languages": [
          "English"
        ],
        "regions": [
          "the United States"
        ]
      },
      "about_page_link": "https://www.google.com/search?q=About+https://seattlebageloasis.com/index.html&tbm=ilp&ilps=ADNMCi0cIKpzkxKZyu7Rc93S2GFf2tN9xg",
      "about_page_serpapi_link": "https://serpapi.com/search.json?engine=google_about_this_result&ilps=ADNMCi0cIKpzkxKZyu7Rc93S2GFf2tN9xg&q=About+https://seattlebageloasis.com/index.html",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:rIswffXb-xcJ:https://seattlebageloasis.com/index.html+&cd=16&hl=en&ct=clnk&gl=us"
    },
    {
      "position": 7,
      "title": "Big Apple Bagels",
      "link": "https://bigapplebagels.com/",
      "redirect_link": "https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwihgOCjsq2DAxUqFVkFHbytCXgQFnoECA8QAQ&url=https%3A%2F%2Fbigapplebagels.com%2F&usg=AOvVaw08jeDUpbNxR4dRvY2CQ5r9&opi=89978449",
      "displayed_link": "https://bigapplebagels.com",
      "snippet": "Our Menu. We have so much more in store than just our fabulous fresh bagels, muffins, and cream cheese. Our restaurants offer a comfortable atmosphere in which ...",
      "snippet_highlighted_words": [
        "fresh bagels"
      ],
      "about_this_result": {
        "source": {
          "description": "Big Apple Bagels is an American franchised chain of bakery-cafes based in Deerfield, Illinois. Coffee, along with a variety of other related products are sold. The products are sold as three different brands: Big Apple Bagels, Brewster's Coffee, and My Favorite Muffin.",
          "icon": "https://serpapi.com/searches/62d5efcae7d08a486c7b86ad/images/02306781b6e93635966d70cf70c16bc22e72198089a3c7cfd4d24be6ac914eb6d61e4b4a6de9575d030e75448c9db590.png"
        },
        "keywords": [
          "fresh",
          "bagels"
        ],
        "languages": [
          "English"
        ],
        "regions": [
          "the United States"
        ]
      },
      "about_page_link": "https://www.google.com/search?q=About+https://bigapplebagels.com/&tbm=ilp&ilps=ADNMCi20AyQKpKCXX1ZjNnADw-yHobWmkg",
      "about_page_serpapi_link": "https://serpapi.com/search.json?engine=google_about_this_result&ilps=ADNMCi20AyQKpKCXX1ZjNnADw-yHobWmkg&q=About+https://bigapplebagels.com/",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:FTkNXecwWj4J:https://bigapplebagels.com/+&cd=17&hl=en&ct=clnk&gl=us"
    },
    {
      "position": 8,
      "title": "Best Sellers in Bagels - Amazon.com",
      "link": "https://www.amazon.com/gp/bestsellers/grocery/18770265011",
      "redirect_link": "https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwj_4aqwsq2DAxXHEVkFHbKNA6wQFnoECBQQAQ&url=https%3A%2F%2Fwww.amazon.com%2FBest-Sellers-Bagels%2Fzgbs%2Fgrocery%2F18770265011&usg=AOvVaw1PMKvxHueZiaLwvdaG8491&opi=89978449",
      "displayed_link": "https://www.amazon.com › bestsellers › grocery",
      "snippet": "Discover the best Bagels in Best Sellers. Find the top 100 most popular items in Amazon Grocery & Gourmet Food Best Sellers.",
      "snippet_highlighted_words": [
        "best Bagels"
      ],
      "about_this_result": {
        "source": {
          "description": "Amazon.com, Inc. is an American multinational technology company which focuses on e-commerce, cloud computing, digital streaming, and artificial intelligence. It has been referred to as \"one of the most influential economic and cultural forces in the world\", and is one of the world's most valuable brands.",
          "icon": "https://serpapi.com/searches/62d5efcae7d08a486c7b86ad/images/02306781b6e93635966d70cf70c16bc2ec6b8603c599601b41d359cc74e73aa4e762e7d839303a57faed29f59c6d8339.png"
        },
        "keywords": [
          "fresh",
          "bagels"
        ],
        "languages": [
          "English"
        ],
        "regions": [
          "the United States"
        ]
      },
      "about_page_link": "https://www.google.com/search?q=About+https://www.amazon.com/gp/bestsellers/grocery/18770265011&tbm=ilp&ilps=ADNMCi04QxXe0QPeMLZktODViW9RXktYZg",
      "about_page_serpapi_link": "https://serpapi.com/search.json?engine=google_about_this_result&ilps=ADNMCi04QxXe0QPeMLZktODViW9RXktYZg&q=About+https://www.amazon.com/gp/bestsellers/grocery/18770265011",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:16Kt5ovsjskJ:https://www.amazon.com/gp/bestsellers/grocery/18770265011+&cd=18&hl=en&ct=clnk&gl=us"
    },
    {
      "position": 9,
      "title": "The 9 Best Bagel Shops in Washington State!",
      "link": "https://bestthingswa.com/bagel-shops/",
      "redirect_link": "https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwivzpS_sq2DAxUXFmIAHRD2B0QQFnoECA4QAQ&url=https%3A%2F%2Fbestthingswa.com%2Fbagel-shops%2F&usg=AOvVaw35bAShnaKtGL8ZV1uu-MZQ&opi=89978449",
      "displayed_link": "https://bestthingswa.com › bagel-shops",
      "snippet": "Seattle Bagel Bakery in Tukwila creates classic light and chewy bagels from scratch every day. This grab and go bagel bakery has all the ingredients for a great ...",
      "snippet_highlighted_words": [
        "Bagel",
        "bagels",
        "bagel"
      ],
      "about_this_result": {
        "source": {
          "description": "bestthingswa.com was first indexed by Google in August 2016"
        },
        "keywords": [
          "fresh",
          "bagels"
        ],
        "related_keywords": [
          "bagel"
        ],
        "languages": [
          "English"
        ],
        "regions": [
          "the United States"
        ]
      },
      "about_page_link": "https://www.google.com/search?q=About+https://bestthingswa.com/bagel-shops/&tbm=ilp&ilps=ADNMCi3xYN2ttt8Ev-h9QIRxh1x4rWpxVw",
      "about_page_serpapi_link": "https://serpapi.com/search.json?engine=google_about_this_result&ilps=ADNMCi3xYN2ttt8Ev-h9QIRxh1x4rWpxVw&q=About+https://bestthingswa.com/bagel-shops/",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:5W1XuIBYImkJ:https://bestthingswa.com/bagel-shops/+&cd=19&hl=en&ct=clnk&gl=us"
    },
    {
      "position": 10,
      "title": "KEEPING BAGELS FRESH - Richmond - Nate's Bagels",
      "link": "https://www.natesbagelsrva.com/keeping-bagels-fresh",
      "redirect_link": "https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwj0-a3Osq2DAxXlFlkFHVmpCD4QFnoECB4QAQ&url=https%3A%2F%2Fwww.natesbagelsrva.com%2Fkeeping-bagels-fresh&usg=AOvVaw0PHutGsKRF2Zt7aAOT4qgC&opi=89978449",
      "displayed_link": "https://www.natesbagelsrva.com › keeping-bagels-fresh",
      "thumbnail": "https://serpapi.com/searches/62d5efcae7d08a486c7b86ad/images/02306781b6e93635966d70cf70c16bc26c90a941f649baa8c57ca509f8d3d634.png",
      "snippet": "BAGEL STORAGE. Nothing beats a freshly baked bagel. That said, unsliced bagels can be brought back to almost-fresh by being stored properly.",
      "snippet_highlighted_words": [
        "BAGEL",
        "freshly",
        "bagel",
        "bagels",
        "fresh"
      ],
      "about_this_result": {
        "source": {
          "description": "natesbagelsrva.com was first indexed by Google in January 2016",
          "icon": "https://serpapi.com/searches/62d5efcae7d08a486c7b86ad/images/02306781b6e93635966d70cf70c16bc2d8343d2f6d209aee83985f1b784909167a44eba06d1adbb6bb133e4b40f3e856.png"
        },
        "keywords": [
          "fresh",
          "bagels"
        ],
        "related_keywords": [
          "freshly",
          "bagel"
        ],
        "languages": [
          "English"
        ],
        "regions": [
          "the United States"
        ]
      },
      "about_page_link": "https://www.google.com/search?q=About+https://www.natesbagelsrva.com/keeping-bagels-fresh&tbm=ilp&ilps=ADNMCi0zyqgLTwffJCWEUo95wR1YUFpBHg",
      "about_page_serpapi_link": "https://serpapi.com/search.json?engine=google_about_this_result&ilps=ADNMCi0zyqgLTwffJCWEUo95wR1YUFpBHg&q=About+https://www.natesbagelsrva.com/keeping-bagels-fresh",
      "cached_page_link": "https://webcache.googleusercontent.com/search?q=cache:ct2DxPva7S8J:https://www.natesbagelsrva.com/keeping-bagels-fresh+&cd=20&hl=en&ct=clnk&gl=us"
    }
  ],
  "related_searches": [
    {
      "query": "Fresh Bagels near Bellevue, WA",
      "link": "https://www.google.com/search?safe=active&hl=en&gl=us&q=fresh+bagels+near+bellevue,+wa&sa=X&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDVAnoECCkQAQ"
    },
    {
      "query": "bagels near me",
      "link": "https://www.google.com/search?safe=active&hl=en&gl=us&q=Bagels+near+me&sa=X&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDVAnoECCIQAQ"
    },
    {
      "query": "best bagels in seattle",
      "link": "https://www.google.com/search?safe=active&hl=en&gl=us&q=Best+bagels+in+Seattle&sa=X&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDVAnoECCEQAQ"
    },
    {
      "query": "blazing bagels",
      "link": "https://www.google.com/search?safe=active&hl=en&gl=us&q=Blazing+Bagels&sa=X&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDVAnoECCwQAQ"
    },
    {
      "query": "bagel seattle",
      "link": "https://www.google.com/search?safe=active&hl=en&gl=us&q=Bagel+Seattle&sa=X&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDVAnoECCYQAQ"
    },
    {
      "query": "best bagels near me",
      "link": "https://www.google.com/search?safe=active&hl=en&gl=us&q=Best+bagels+near+me&sa=X&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDVAnoECCsQAQ"
    },
    {
      "query": "rubinstein bagels menu",
      "link": "https://www.google.com/search?safe=active&hl=en&gl=us&q=Rubinstein+bagels+menu&sa=X&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDVAnoECCMQAQ"
    },
    {
      "query": "loxsmith bagels seattle",
      "link": "https://www.google.com/search?safe=active&hl=en&gl=us&q=Loxsmith+bagels+seattle&sa=X&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDVAnoECCoQAQ"
    }
  ],
  "pagination": {
    "current": 2,
    "previous": "https://www.google.com/search?q=Fresh+Bagels&safe=active&hl=en&gl=us&ei=y-_VYqeiDqLIptQP-aK0sAo&start=0&sa=N&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDx0wN6BAgBEDQ",
    "next": "https://www.google.com/search?q=Fresh+Bagels&safe=active&hl=en&gl=us&ei=y-_VYqeiDqLIptQP-aK0sAo&start=20&sa=N&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDw0wN6BAgBEEk",
    "other_pages": {
      "1": "https://www.google.com/search?q=Fresh+Bagels&safe=active&hl=en&gl=us&ei=y-_VYqeiDqLIptQP-aK0sAo&start=0&sa=N&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDy0wN6BAgBEDY",
      "3": "https://www.google.com/search?q=Fresh+Bagels&safe=active&hl=en&gl=us&ei=y-_VYqeiDqLIptQP-aK0sAo&start=20&sa=N&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDy0wN6BAgBEDk",
      "4": "https://www.google.com/search?q=Fresh+Bagels&safe=active&hl=en&gl=us&ei=y-_VYqeiDqLIptQP-aK0sAo&start=30&sa=N&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDy0wN6BAgBEDs",
      "5": "https://www.google.com/search?q=Fresh+Bagels&safe=active&hl=en&gl=us&ei=y-_VYqeiDqLIptQP-aK0sAo&start=40&sa=N&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDy0wN6BAgBED0",
      "6": "https://www.google.com/search?q=Fresh+Bagels&safe=active&hl=en&gl=us&ei=y-_VYqeiDqLIptQP-aK0sAo&start=50&sa=N&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDy0wN6BAgBED8",
      "7": "https://www.google.com/search?q=Fresh+Bagels&safe=active&hl=en&gl=us&ei=y-_VYqeiDqLIptQP-aK0sAo&start=60&sa=N&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDy0wN6BAgBEEE",
      "8": "https://www.google.com/search?q=Fresh+Bagels&safe=active&hl=en&gl=us&ei=y-_VYqeiDqLIptQP-aK0sAo&start=70&sa=N&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDy0wN6BAgBEEM",
      "9": "https://www.google.com/search?q=Fresh+Bagels&safe=active&hl=en&gl=us&ei=y-_VYqeiDqLIptQP-aK0sAo&start=80&sa=N&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDy0wN6BAgBEEU",
      "10": "https://www.google.com/search?q=Fresh+Bagels&safe=active&hl=en&gl=us&ei=y-_VYqeiDqLIptQP-aK0sAo&start=90&sa=N&ved=2ahUKEwjns9vRzoP5AhUipIkEHXkRDaY4ChDy0wN6BAgBEEc"
    }
  },
  "serpapi_pagination": {
    "current": 2,
    "previous_link": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Seattle-Tacoma%2C+WA%2C+Washington%2C+United+States&num=10&q=Fresh+Bagels&safe=active&start=0",
    "previous": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Seattle-Tacoma%2C+WA%2C+Washington%2C+United+States&num=10&q=Fresh+Bagels&safe=active&start=0",
    "next_link": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Seattle-Tacoma%2C+WA%2C+Washington%2C+United+States&num=10&q=Fresh+Bagels&safe=active&start=20",
    "next": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Seattle-Tacoma%2C+WA%2C+Washington%2C+United+States&num=10&q=Fresh+Bagels&safe=active&start=20",
    "other_pages": {
      "1": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Seattle-Tacoma%2C+WA%2C+Washington%2C+United+States&num=10&q=Fresh+Bagels&safe=active&start=0",
      "3": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Seattle-Tacoma%2C+WA%2C+Washington%2C+United+States&num=10&q=Fresh+Bagels&safe=active&start=20",
      "4": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Seattle-Tacoma%2C+WA%2C+Washington%2C+United+States&num=10&q=Fresh+Bagels&safe=active&start=30",
      "5": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Seattle-Tacoma%2C+WA%2C+Washington%2C+United+States&num=10&q=Fresh+Bagels&safe=active&start=40",
      "6": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Seattle-Tacoma%2C+WA%2C+Washington%2C+United+States&num=10&q=Fresh+Bagels&safe=active&start=50",
      "7": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Seattle-Tacoma%2C+WA%2C+Washington%2C+United+States&num=10&q=Fresh+Bagels&safe=active&start=60",
      "8": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Seattle-Tacoma%2C+WA%2C+Washington%2C+United+States&num=10&q=Fresh+Bagels&safe=active&start=70",
      "9": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Seattle-Tacoma%2C+WA%2C+Washington%2C+United+States&num=10&q=Fresh+Bagels&safe=active&start=80",
      "10": "https://serpapi.com/search.json?device=desktop&engine=google&gl=us&google_domain=google.com&hl=en&location=Seattle-Tacoma%2C+WA%2C+Washington%2C+United+States&num=10&q=Fresh+Bagels&safe=active&start=90"
    }
  }
}
Api Status ‧GitHub ‧Legal ‧Security ‧Libraries ‧Release Notes ‧Public Roadmap ‧Support
© 2016-2024 SerpApi, LLC.






use std::error::Error;

use async_openai::{
    types::{
        ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
        CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema,
    },
    Client,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let schema = json!({
      "type": "object",
      "properties": {
        "steps": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "explanation": { "type": "string" },
              "output": { "type": "string" }
            },
            "required": ["explanation", "output"],
            "additionalProperties": false
          }
        },
        "final_answer": { "type": "string" }
      },
      "required": ["steps", "final_answer"],
      "additionalProperties": false
    });

    let response_format = ResponseFormat::JsonSchema {
        json_schema: ResponseFormatJsonSchema {
            description: None,
            name: "math_reasoning".into(),
            schema: Some(schema),
            strict: Some(true),
        },
    };

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-4o-2024-08-06")
        .messages([
            ChatCompletionRequestSystemMessage::from(
                "You are a helpful math tutor. Guide the user through the solution step by step.",
            )
            .into(),
            ChatCompletionRequestUserMessage::from("how can I solve 8x + 7 = -23").into(),
        ])
        .response_format(response_format)
        .build()?;

    let response = client.chat().create(request).await?;

    for choice in response.choices {
        if let Some(content) = choice.message.content {
            print!("{content}")
        }
    }

    Ok(())
}



use std::error::Error;

use async_openai::{
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Who won the world series in 2020?")
                .build()?
                .into(),
            ChatCompletionRequestAssistantMessageArgs::default()
                .content("The Los Angeles Dodgers won the World Series in 2020.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Where was it played?")
                .build()?
                .into(),
        ])
        .build()?;

    println!("{}", serde_json::to_string(&request).unwrap());

    let response = client.chat().create(request).await?;

    println!("\nResponse:\n");
    for choice in response.choices {
        println!(
            "{}: Role: {}  Content: {:?}",
            choice.index, choice.message.role, choice.message.content
        );
    }

    Ok(())
}


 Docs.rs
 async-openai-0.25.0 
 Platform 
 Feature flags
Rust
 
Find crate
async_openai
0.25.0
All Items
Sections
Creating client
Microsoft Azure Endpoints
Making requests
Examples
Crate Items
Modules
Structs
Crates
async_openai
Type ‘S’ or ‘/’ to search, ‘?’ for more options…
Crate async_openaiCopy item path
Settings
Help

Summary
source
Rust library for OpenAI

Creating client
use async_openai::{Client, config::OpenAIConfig};

// Create a OpenAI client with api key from env var OPENAI_API_KEY and default base url.
let client = Client::new();

// Above is shortcut for
let config = OpenAIConfig::default();
let client = Client::with_config(config);

// OR use API key from different source and a non default organization
let api_key = "sk-..."; // This secret could be from a file, or environment variable.
let config = OpenAIConfig::new()
    .with_api_key(api_key)
    .with_org_id("the-continental");

let client = Client::with_config(config);

// Use custom reqwest client
let http_client = reqwest::ClientBuilder::new().user_agent("async-openai").build().unwrap();
let client = Client::new().with_http_client(http_client);
Microsoft Azure Endpoints
use async_openai::{Client, config::AzureConfig};

let config = AzureConfig::new()
    .with_api_base("https://my-resource-name.openai.azure.com")
    .with_api_version("2023-03-15-preview")
    .with_deployment_id("deployment-id")
    .with_api_key("...");

let client = Client::with_config(config);

// Note that `async-openai` only implements OpenAI spec
// and doesn't maintain parity with the spec of Azure OpenAI service.
Making requests

 use async_openai::{Client, types::{CreateCompletionRequestArgs}};

 // Create client
 let client = Client::new();

 // Create request using builder pattern
 // Every request struct has companion builder struct with same name + Args suffix
 let request = CreateCompletionRequestArgs::default()
     .model("gpt-3.5-turbo-instruct")
     .prompt("Tell me the recipe of alfredo pasta")
     .max_tokens(40_u32)
     .build()
     .unwrap();

 // Call API
 let response = client
     .completions()      // Get the API "group" (completions, images, etc.) from the client
     .create(request)    // Make the API call in that "group"
     .await
     .unwrap();

 println!("{}", response.choices.first().unwrap().text);
Examples
For full working examples for all supported features see examples directory in the repository.

Modules
config	Client configurations: OpenAIConfig for OpenAI, AzureConfig for Azure OpenAI Service.
error	Errors originating from API calls, parsing responses, and reading-or-writing to the file system.
types	Types used in OpenAI API requests and responses. These types are created from component schemas in the OpenAPI spec
Structs
AssistantFiles	Files attached to an assistant.
Assistants	Build assistants that can call models and use tools to perform tasks.
Audio	Turn audio into text or text into audio. Related guide: Speech to text
Batches	Create large batches of API requests for asynchronous processing. The Batch API returns completions within 24 hours for a 50% discount.
Chat	Given a list of messages comprising a conversation, the model will return a response.
Client	Client is a container for config, backoff and http_client used to make API calls.
Completions	Given a prompt, the model will return one or more predicted completions, and can also return the probabilities of alternative tokens at each position. We recommend most users use our Chat completions API. Learn more
Embeddings	Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.
Files	Files are used to upload documents that can be used with features like Assistants and Fine-tuning.
FineTuning	Manage fine-tuning jobs to tailor a model to your specific training data.
Images	Given a prompt and/or an input image, the model will generate a new image.
MessageFiles	Files attached to a message.
Messages	Represents a message within a thread.
Models	List and describe the various models available in the API. You can refer to the Models documentation to understand what models are available and the differences between them.
Moderations	Given some input text, outputs if the model classifies it as potentially harmful across several categories.
Runs	Represents an execution run on a thread.
Steps	Represents a step in execution of a run.
Threads	Create threads that assistants can interact with.
VectorStoreFileBatches	Vector store file batches represent operations to add multiple files to a vector store.
VectorStoreFiles	Vector store files represent files inside a vector store.
VectorStores


 Docs.rs
 sqlx-0.8.2 
 Platform 
 Feature flags
Rust
 
Find crate
sqlx
0.8.2
All Items
Modules
Macros
Structs
Enums
Traits
Functions
Type Aliases
Attribute Macros
Derive Macros
Crates
sqlx
Type ‘S’ or ‘/’ to search, ‘?’ for more options…
?
Settings
Crate sqlxCopy item path
source · [−]
The async SQL toolkit for Rust, built with ❤️ by the LaunchBadge team.

See our README to get started or browse our example projects. Have a question? Check our FAQ or open a discussion.

§Runtime Support
SQLx supports both the Tokio and async-std runtimes.

You choose which runtime SQLx uses by default by enabling one of the following features:

runtime-async-std
runtime-tokio
The runtime-actix feature also exists but is an alias of runtime-tokio.

If more than one runtime feature is enabled, the Tokio runtime is used if a Tokio context exists on the current thread, i.e. tokio::runtime::Handle::try_current() returns Ok; async-std is used otherwise.

Note that while SQLx no longer produces a compile error if zero or multiple runtime features are enabled, which is useful for libraries building on top of it, the use of nearly any async function in the API will panic without at least one runtime feature enabled.

The chief exception is the SQLite driver, which is runtime-agnostic, including its integration with the query macros. However, SqlitePool does require runtime support for timeouts and spawning internal management tasks.

TLS Support
For securely communicating with SQL servers over an untrusted network connection such as the internet, you can enable Transport Layer Security (TLS) by enabling one of the following features:

tls-native-tls: Enables the native-tls backend which uses the OS-native TLS capabilities:
SecureTransport on macOS.
SChannel on Windows.
OpenSSL on all other platforms.
tls-rustls: Enables the rustls backend, a cross-platform TLS library.
Only supports TLS revisions 1.2 and 1.3.
If you get HandshakeFailure errors when using this feature, it likely means your database server does not support these newer revisions. This might be resolved by enabling or switching to the tls-native-tls feature.
rustls supports several providers of cryptographic primitives. The default (enabled when you use the tls-rustls feature or tls-rustls-ring) is the ring provider, which has fewer build-time dependencies but also has fewer features. Alternatively, you can use tls-rustls-aws-lc-rs to use the aws-lc-rs provider, which enables additional cipher suite support at the cost of more onerous build requirements (depending on platform support).
If more than one TLS feature is enabled, the tls-native-tls feature takes precedent so that it is only necessary to enable it to see if it resolves the HandshakeFailure error without disabling tls-rustls.

Consult the user manual for your database to find the TLS versions it supports.

If your connection configuration requires a TLS upgrade but TLS support was not enabled, the connection attempt will return an error.

The legacy runtime+TLS combination feature flags are still supported, but for forward-compatibility, use of the separate runtime and TLS feature flags is recommended.

Modules
any	SEE DOCUMENTATION BEFORE USE. Runtime-generic database driver.
database	Traits to represent a database driver.
decode	Provides Decode for decoding values from the database.
encode	Provides Encode for encoding values for the database.
error	Types for working with errors produced by SQLx.
migrate
mysqlmysql	MySQL database driver.
pool	Provides the connection pool for asynchronous SQLx connections.
postgrespostgres	PostgreSQL database driver.
prelude	Convenience re-export of common traits.
query	Types and traits for the query family of functions and macros.
query_builder	Runtime query-builder API.
sqlitesqlite	SQLite database driver.
types	Conversions between Rust and SQL types.
Macros
migrate	Embeds migrations into the binary by expanding to a static instance of Migrator.
querymacros	Statically checked SQL query with println!() style syntax.
query_asmacros	A variant of query! which takes a path to an explicitly defined struct as the output type.
query_as_uncheckedmacros	A variant of query_as! which does not check the input or output types. This still does parse the query to ensure it’s syntactically and semantically valid for the current database.
query_filemacros	A variant of query! where the SQL query is stored in a separate file.
query_file_asmacros	Combines the syntaxes of query_as! and query_file!.
query_file_as_uncheckedmacros	A variant of query_file_as! which does not check the input or output types. This still does parse the query to ensure it’s syntactically and semantically valid for the current database.
query_file_scalarmacros	A variant of query_scalar! which takes a file path like query_file!.
query_file_scalar_uncheckedmacros	A variant of query_file_scalar! which does not typecheck bind parameters and leaves the output type to inference. The query itself is still checked that it is syntactically and semantically valid for the database, that it only produces one column and that the number of bind parameters is correct.
query_file_uncheckedmacros	A variant of query_file! which does not check the input or output types. This still does parse the query to ensure it’s syntactically and semantically valid for the current database.
query_scalarmacros	A variant of query! which expects a single column from the query and evaluates to an instance of QueryScalar.
query_scalar_uncheckedmacros	A variant of query_scalar! which does not typecheck bind parameters and leaves the output type to inference. The query itself is still checked that it is syntactically and semantically valid for the database, that it only produces one column and that the number of bind parameters is correct.
query_uncheckedmacros	A variant of query! which does not check the input or output types. This still does parse the query to ensure it’s syntactically and semantically valid for the current database.
Structs
Anyany	Opaque database driver. Capable of being used in place of any SQLx database driver. The actual driver used will be selected at runtime, from the connection url.
AnyConnection	SEE DOCUMENTATION BEFORE USE. Runtime-generic database connection.
MySqlmysql	MySQL database driver.
MySqlConnectionmysql	A connection to a MySQL database.
PgConnectionpostgres	A connection to a PostgreSQL database.
Pool	An asynchronous pool of SQLx database connections.
Postgrespostgres	PostgreSQL database driver.
QueryBuilder	A builder type for constructing queries at runtime.
RawSql	One or more raw SQL statements, separated by semicolons (;).
Sqlitesqlite	Sqlite database driver.
SqliteConnectionsqlite	A connection to an open Sqlite database.
Transaction	An in-progress database transaction or savepoint.
Enums
Either	The enum Either with variants Left and Right is a general purpose sum type with two cases.
Error	Represents all the ways a method can fail within SQLx.
Traits
Acquire	Acquire connections or transactions from a database in a generic way.
AnyExecutorany	An alias for Executor<'_, Database = Any>.
Arguments	A tuple of arguments to be sent to the database.
Column
ColumnIndex	A type that can be used to index into a Row or Statement.
ConnectOptions
Connection	Represents a single database connection.
Database	A database driver.
Decode	A type that can be decoded from the database.
Encode	Encode a single value to be sent to the database.
Execute	A type that may be executed against a database connection.
Executor	A type that contains or can provide a database connection to use for executing queries against the database.
FromRow	A record that can be built from a row returned by the database.
IntoArguments
MySqlExecutormysql	An alias for Executor<'_, Database = MySql>.
PgExecutorpostgres	An alias for Executor<'_, Database = Postgres>.
Row	Represents a single row from the database.
SqliteExecutorsqlite	An alias for Executor<'_, Database = Sqlite>.
Statement	An explicitly prepared statement.
Type	Indicates that a SQL type is supported for a database.
TypeInfo	Provides information about a SQL type for the database driver.
Value	An owned value from the database.
ValueRef	A reference to a single value from the database.
Functions
query	Execute a single SQL query as a prepared statement (transparently cached).
query_as	Execute a single SQL query as a prepared statement (transparently cached). Maps rows to Rust types using FromRow.
query_as_with	Execute a single SQL query, with the given arguments as a prepared statement (transparently cached). Maps rows to Rust types using FromRow.
query_scalar	Execute a single SQL query as a prepared statement (transparently cached) and extract the first column of each row.
query_scalar_with	Execute a SQL query as a prepared statement (transparently cached), with the given arguments, and extract the first column of each row.
query_with	Execute a SQL query as a prepared statement (transparently cached), with the given arguments.
raw_sql	Execute one or more statements as raw SQL, separated by semicolons (;).
Type Aliases
AnyPool	SEE DOCUMENTATION BEFORE USE. Type alias for Pool<Any>.
MySqlPoolmysql	An alias for Pool, specialized for MySQL.
PgPoolpostgres	An alias for Pool, specialized for Postgres.
Result	A specialized Result type for SQLx.
SqlitePoolsqlite	An alias for Pool, specialized for SQLite.
Attribute Macros
test	Mark an async fn as a test with SQLx support.
Derive Macros
Decode
Encode



 Docs.rs
 tokio-1.41.0 
 Platform 
 Feature flags
Rust
 
Find crate
tokio
1.41.0
All Items
Sections
A Tour of Tokio
Authoring applications
Authoring libraries
Working With Tasks
CPU-bound tasks and blocking code
Asynchronous IO
Examples
Feature flags
Supported platforms
Crate Items
Re-exports
Modules
Macros
Attribute Macros
Crates
tokio
Type ‘S’ or ‘/’ to search, ‘?’ for more options…
Crate tokioCopy item path
Settings
Help

Summary
source
A runtime for writing reliable network applications without compromising speed.

Tokio is an event-driven, non-blocking I/O platform for writing asynchronous applications with the Rust programming language. At a high level, it provides a few major components:

Tools for working with asynchronous tasks, including synchronization primitives and channels and timeouts, sleeps, and intervals.
APIs for performing asynchronous I/O, including TCP and UDP sockets, filesystem operations, and process and signal management.
A runtime for executing asynchronous code, including a task scheduler, an I/O driver backed by the operating system’s event queue (epoll, kqueue, IOCP, etc…), and a high performance timer.
Guide level documentation is found on the website.

A Tour of Tokio
Tokio consists of a number of modules that provide a range of functionality essential for implementing asynchronous applications in Rust. In this section, we will take a brief tour of Tokio, summarizing the major APIs and their uses.

The easiest way to get started is to enable all features. Do this by enabling the full feature flag:

tokio = { version = "1", features = ["full"] }
Authoring applications
Tokio is great for writing applications and most users in this case shouldn’t worry too much about what features they should pick. If you’re unsure, we suggest going with full to ensure that you don’t run into any road blocks while you’re building your application.

Example
This example shows the quickest way to get started with Tokio.

tokio = { version = "1", features = ["full"] }
Authoring libraries
As a library author your goal should be to provide the lightest weight crate that is based on Tokio. To achieve this you should ensure that you only enable the features you need. This allows users to pick up your crate without having to enable unnecessary features.

Example
This example shows how you may want to import features for a library that just needs to tokio::spawn and use a TcpStream.

tokio = { version = "1", features = ["rt", "net"] }
Working With Tasks
Asynchronous programs in Rust are based around lightweight, non-blocking units of execution called tasks. The tokio::task module provides important tools for working with tasks:

The spawn function and JoinHandle type, for scheduling a new task on the Tokio runtime and awaiting the output of a spawned task, respectively,
Functions for running blocking operations in an asynchronous task context.
The tokio::task module is present only when the “rt” feature flag is enabled.

The tokio::sync module contains synchronization primitives to use when needing to communicate or share data. These include:

channels (oneshot, mpsc, watch, and broadcast), for sending values between tasks,
a non-blocking Mutex, for controlling access to a shared, mutable value,
an asynchronous Barrier type, for multiple tasks to synchronize before beginning a computation.
The tokio::sync module is present only when the “sync” feature flag is enabled.

The tokio::time module provides utilities for tracking time and scheduling work. This includes functions for setting timeouts for tasks, sleeping work to run in the future, or repeating an operation at an interval.

In order to use tokio::time, the “time” feature flag must be enabled.

Finally, Tokio provides a runtime for executing asynchronous tasks. Most applications can use the #[tokio::main] macro to run their code on the Tokio runtime. However, this macro provides only basic configuration options. As an alternative, the tokio::runtime module provides more powerful APIs for configuring and managing runtimes. You should use that module if the #[tokio::main] macro doesn’t provide the functionality you need.

Using the runtime requires the “rt” or “rt-multi-thread” feature flags, to enable the current-thread single-threaded scheduler and the multi-thread scheduler, respectively. See the runtime module documentation for details. In addition, the “macros” feature flag enables the #[tokio::main] and #[tokio::test] attributes.

CPU-bound tasks and blocking code
Tokio is able to concurrently run many tasks on a few threads by repeatedly swapping the currently running task on each thread. However, this kind of swapping can only happen at .await points, so code that spends a long time without reaching an .await will prevent other tasks from running. To combat this, Tokio provides two kinds of threads: Core threads and blocking threads.

The core threads are where all asynchronous code runs, and Tokio will by default spawn one for each CPU core. You can use the environment variable TOKIO_WORKER_THREADS to override the default value.

The blocking threads are spawned on demand, can be used to run blocking code that would otherwise block other tasks from running and are kept alive when not used for a certain amount of time which can be configured with thread_keep_alive. Since it is not possible for Tokio to swap out blocking tasks, like it can do with asynchronous code, the upper limit on the number of blocking threads is very large. These limits can be configured on the Builder.

To spawn a blocking task, you should use the spawn_blocking function.

#[tokio::main]
async fn main() {
    // This is running on a core thread.

    let blocking_task = tokio::task::spawn_blocking(|| {
        // This is running on a blocking thread.
        // Blocking here is ok.
    });

    // We can wait for the blocking task like this:
    // If the blocking task panics, the unwrap below will propagate the
    // panic.
    blocking_task.await.unwrap();
}
If your code is CPU-bound and you wish to limit the number of threads used to run it, you should use a separate thread pool dedicated to CPU bound tasks. For example, you could consider using the rayon library for CPU-bound tasks. It is also possible to create an extra Tokio runtime dedicated to CPU-bound tasks, but if you do this, you should be careful that the extra runtime runs only CPU-bound tasks, as IO-bound tasks on that runtime will behave poorly.

Hint: If using rayon, you can use a oneshot channel to send the result back to Tokio when the rayon task finishes.

Asynchronous IO
As well as scheduling and running tasks, Tokio provides everything you need to perform input and output asynchronously.

The tokio::io module provides Tokio’s asynchronous core I/O primitives, the AsyncRead, AsyncWrite, and AsyncBufRead traits. In addition, when the “io-util” feature flag is enabled, it also provides combinators and functions for working with these traits, forming as an asynchronous counterpart to std::io.

Tokio also includes APIs for performing various kinds of I/O and interacting with the operating system asynchronously. These include:

tokio::net, which contains non-blocking versions of TCP, UDP, and Unix Domain Sockets (enabled by the “net” feature flag),
tokio::fs, similar to std::fs but for performing filesystem I/O asynchronously (enabled by the “fs” feature flag),
tokio::signal, for asynchronously handling Unix and Windows OS signals (enabled by the “signal” feature flag),
tokio::process, for spawning and managing child processes (enabled by the “process” feature flag).
Examples
A simple TCP echo server:

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
Feature flags
Tokio uses a set of feature flags to reduce the amount of compiled code. It is possible to just enable certain features over others. By default, Tokio does not enable any features but allows one to enable a subset for their use case. Below is a list of the available feature flags. You may also notice above each function, struct and trait there is listed one or more feature flags that are required for that item to be used. If you are new to Tokio it is recommended that you use the full feature flag which will enable all public APIs. Beware though that this will pull in many extra dependencies that you may not need.

full: Enables all features listed below except test-util and tracing.
rt: Enables tokio::spawn, the current-thread scheduler, and non-scheduler utilities.
rt-multi-thread: Enables the heavier, multi-threaded, work-stealing scheduler.
io-util: Enables the IO based Ext traits.
io-std: Enable Stdout, Stdin and Stderr types.
net: Enables tokio::net types such as TcpStream, UnixStream and UdpSocket, as well as (on Unix-like systems) AsyncFd and (on FreeBSD) PollAio.
time: Enables tokio::time types and allows the schedulers to enable the built in timer.
process: Enables tokio::process types.
macros: Enables #[tokio::main] and #[tokio::test] macros.
sync: Enables all tokio::sync types.
signal: Enables all tokio::signal types.
fs: Enables tokio::fs types.
test-util: Enables testing based infrastructure for the Tokio runtime.
parking_lot: As a potential optimization, use the _parking_lot_ crate’s synchronization primitives internally. Also, this dependency is necessary to construct some of our primitives in a const context. MSRV may increase according to the _parking_lot_ release in use.
Note: AsyncRead and AsyncWrite traits do not require any features and are always available.

Unstable features
Some feature flags are only available when specifying the tokio_unstable flag:

tracing: Enables tracing events.
Likewise, some parts of the API are only available with the same flag:

task::Builder
Some methods on task::JoinSet
runtime::RuntimeMetrics
runtime::Builder::on_task_spawn
runtime::Builder::on_task_terminate
runtime::Builder::unhandled_panic
runtime::TaskMeta
This flag enables unstable features. The public API of these features may break in 1.x releases. To enable these features, the --cfg tokio_unstable argument must be passed to rustc when compiling. This serves to explicitly opt-in to features which may break semver conventions, since Cargo does not yet directly support such opt-ins.

You can specify it in your project’s .cargo/config.toml file:

[build]
rustflags = ["--cfg", "tokio_unstable"]
The [build] section does not go in a Cargo.toml file. Instead it must be placed in the Cargo config file .cargo/config.toml.
Alternatively, you can specify it with an environment variable:

## Many *nix shells:
export RUSTFLAGS="--cfg tokio_unstable"
cargo build
## Windows PowerShell:
$Env:RUSTFLAGS="--cfg tokio_unstable"
cargo build
Supported platforms
Tokio currently guarantees support for the following platforms:

Linux
Windows
Android (API level 21)
macOS
iOS
FreeBSD
Tokio will continue to support these platforms in the future. However, future releases may change requirements such as the minimum required libc version on Linux, the API level on Android, or the supported FreeBSD release.

Beyond the above platforms, Tokio is intended to work on all platforms supported by the mio crate. You can find a longer list in mio’s documentation. However, these additional platforms may become unsupported in the future.

Note that Wine is considered to be a different platform from Windows. See mio’s documentation for more information on Wine support.

WASM support
Tokio has some limited support for the WASM platform. Without the tokio_unstable flag, the following features are supported:

sync
macros
io-util
rt
time
Enabling any other feature (including full) will cause a compilation failure.

The time module will only work on WASM platforms that have support for timers (e.g. wasm32-wasi). The timing functions will panic if used on a WASM platform that does not support timers.

Note also that if the runtime becomes indefinitely idle, it will panic immediately instead of blocking forever. On platforms that don’t support time, this means that the runtime can never be idle in any way.

Unstable WASM support
Tokio also has unstable support for some additional WASM features. This requires the use of the tokio_unstable flag.

Using this flag enables the use of tokio::net on the wasm32-wasi target. However, not all methods are available on the networking types as WASI currently does not support the creation of new sockets from within WASM. Because of this, sockets must currently be created via the FromRawFd trait.

Re-exports
pub use task::spawn;	rt
Modules
doc	Types which are documented locally in the Tokio crate, but does not actually live here.
fsfs	Asynchronous file utilities.
io	Traits, helpers, and type definitions for asynchronous I/O functionality.
net	TCP/UDP/Unix bindings for tokio.
processprocess	An implementation of asynchronous process management for Tokio.
runtimert	The Tokio runtime.
signalsignal	Asynchronous signal handling for Tokio.
stream	Due to the Stream trait’s inclusion in std landing later than Tokio’s 1.0 release, most of the Tokio stream utilities have been moved into the tokio-stream crate.
syncsync	Synchronization primitives for use in asynchronous contexts.
task	Asynchronous green-threads.
timetime	Utilities for tracking time.
Macros
joinmacros	Waits on multiple concurrent branches, returning when all branches complete.
pin	Pins a value on the stack.
selectmacros	Waits on multiple concurrent branches, returning when the first branch completes, cancelling the remaining branches.
task_localrt	Declares a new task-local key of type tokio::task::LocalKey.
try_joinmacros	Waits on multiple concurrent branches, returning when all branches complete with Ok(_) or on the first Err(_).
Attribute Macros
mainrt and macros	Marks async function to be executed by the selected runtime. This macro helps set up a Runtime without requiring the user to use Runtime or Builder directly.
testrt and macros	Marks async function to be executed by runtime, suitable to test environment. This macro helps set up a Runtime without requiring the user to use Runtime or Builder directly.







 Docs.rs
 ratatui-0.29.0 
 Platform 
 Feature flags
Rust
 
Find crate
logo
ratatui
0.29.0
All Items
Sections
Ratatui
Quickstart
Other documentation
Introduction
Layout
Text and styling
Features
Crate Items
Re-exports
Modules
Macros
Structs
Enums
Functions
Type Aliases
Crates
ratatui
Type ‘S’ or ‘/’ to search, ‘?’ for more options…
Crate ratatuiCopy item path
Settings
Help

Summary
source
Demo

Crate Badge Docs Badge CI Badge Deps.rs Badge
Codecov Badge License Badge Sponsors Badge
Discord Badge Matrix Badge Forum Badge

Ratatui Website · API Docs · Examples · Changelog · Breaking Changes
Contributing · Report a bug · Request a Feature · Create a Pull Request

Ratatui
Ratatui is a crate for cooking up terminal user interfaces in Rust. It is a lightweight library that provides a set of widgets and utilities to build complex Rust TUIs. Ratatui was forked from the tui-rs crate in 2023 in order to continue its development.

Quickstart
Add ratatui and crossterm as dependencies to your cargo.toml:

cargo add ratatui crossterm
Then you can create a simple “Hello World” application:

use crossterm::event::{self, Event};
use ratatui::{text::Text, Frame};

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}
The full code for this example which contains a little more detail is in the Examples directory. For more guidance on different ways to structure your application see the Application Patterns and Hello World tutorial sections in the Ratatui Website and the various Examples. There are also several starter templates available in the templates repository.

Other documentation
Ratatui Website - explains the library’s concepts and provides step-by-step tutorials
Ratatui Forum - a place to ask questions and discuss the library
API Docs - the full API documentation for the library on docs.rs.
Examples - a collection of examples that demonstrate how to use the library.
Contributing - Please read this if you are interested in contributing to the project.
Changelog - generated by git-cliff utilizing Conventional Commits.
Breaking Changes - a list of breaking changes in the library.
You can also watch the FOSDEM 2024 talk about Ratatui which gives a brief introduction to terminal user interfaces and showcases the features of Ratatui, along with a hello world demo.

Introduction
Ratatui is based on the principle of immediate rendering with intermediate buffers. This means that for each frame, your app must render all widgets that are supposed to be part of the UI. This is in contrast to the retained mode style of rendering where widgets are updated and then automatically redrawn on the next frame. See the Rendering section of the Ratatui Website for more info.

Ratatui uses Crossterm by default as it works on most platforms. See the Installation section of the Ratatui Website for more details on how to use other backends (Termion / Termwiz).

Every application built with ratatui needs to implement the following steps:

Initialize the terminal
A main loop that:
Draws the UI
Handles input events
Restore the terminal state
Initialize and restore the terminal
The Terminal type is the main entry point for any Ratatui application. It is generic over a a choice of Backend implementations that each provide functionality to draw frames, clear the screen, hide the cursor, etc. There are backend implementations for Crossterm, Termion and Termwiz.

The simplest way to initialize the terminal is to use the init function which returns a DefaultTerminal instance with the default options, enters the Alternate Screen and Raw mode and sets up a panic hook that restores the terminal in case of panic. This instance can then be used to draw frames and interact with the terminal state. (The DefaultTerminal instance is a type alias for a terminal with the crossterm backend.) The restore function restores the terminal to its original state.

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}
See the backend module and the Backends section of the Ratatui Website for more info on the alternate screen and raw mode.

Drawing the UI
Drawing the UI is done by calling the Terminal::draw method on the terminal instance. This method takes a closure that is called with a Frame instance. The Frame provides the size of the area to draw to and allows the app to render any Widget using the provided render_widget method. After this closure returns, a diff is performed and only the changes are drawn to the terminal. See the Widgets section of the Ratatui Website for more info.

The closure passed to the Terminal::draw method should handle the rendering of a full frame.

use ratatui::{widgets::Paragraph, Frame};

fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| draw(frame))?;
        if handle_events()? {
            break Ok(());
        }
    }
}

fn draw(frame: &mut Frame) {
    let text = Paragraph::new("Hello World!");
    frame.render_widget(text, frame.area());
}
Handling events
Ratatui does not include any input handling. Instead event handling can be implemented by calling backend library methods directly. See the Handling Events section of the Ratatui Website for more info. For example, if you are using Crossterm, you can use the crossterm::event module to handle events.

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            // handle other key events
            _ => {}
        },
        // handle other events
        _ => {}
    }
    Ok(false)
}

Layout
The library comes with a basic yet useful layout management object called Layout which allows you to split the available space into multiple areas and then render widgets in each area. This lets you describe a responsive terminal UI by nesting layouts. See the Layout section of the Ratatui Website for more info.

use ratatui::{
    layout::{Constraint, Layout},
    widgets::Block,
    Frame,
};

fn draw(frame: &mut Frame) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [left_area, right_area] = horizontal.areas(main_area);

    frame.render_widget(Block::bordered().title("Title Bar"), title_area);
    frame.render_widget(Block::bordered().title("Status Bar"), status_area);
    frame.render_widget(Block::bordered().title("Left"), left_area);
    frame.render_widget(Block::bordered().title("Right"), right_area);
}
Running this example produces the following output:

Title Bar───────────────────────────────────
┌Left────────────────┐┌Right───────────────┐
│                    ││                    │
└────────────────────┘└────────────────────┘
Status Bar──────────────────────────────────
Text and styling
The Text, Line and Span types are the building blocks of the library and are used in many places. Text is a list of Lines and a Line is a list of Spans. A Span is a string with a specific style.

The style module provides types that represent the various styling options. The most important one is Style which represents the foreground and background colors and the text attributes of a Span. The style module also provides a Stylize trait that allows short-hand syntax to apply a style to widgets and text. See the Styling Text section of the Ratatui Website for more info.

use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

fn draw(frame: &mut Frame) {
    let areas = Layout::vertical([Constraint::Length(1); 4]).split(frame.area());

    let line = Line::from(vec![
        Span::raw("Hello "),
        Span::styled(
            "World",
            Style::new()
                .fg(Color::Green)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        "!".red().on_light_yellow().italic(),
    ]);
    frame.render_widget(line, areas[0]);

    // using the short-hand syntax and implicit conversions
    let paragraph = Paragraph::new("Hello World!".red().on_white().bold());
    frame.render_widget(paragraph, areas[1]);

    // style the whole widget instead of just the text
    let paragraph = Paragraph::new("Hello World!").style(Style::new().red().on_white());
    frame.render_widget(paragraph, areas[2]);

    // use the simpler short-hand syntax
    let paragraph = Paragraph::new("Hello World!").blue().on_yellow();
    frame.render_widget(paragraph, areas[3]);
}
Features
The crate provides a set of optional features that can be enabled in your cargo.toml file.

default — By default, we enable the crossterm backend as this is a reasonable choice for most applications as it is supported on Linux/Mac/Windows systems. We also enable the underline-color feature which allows you to set the underline color of text.
Generally an application will only use one backend, so you should only enable one of the following features:

crossterm (enabled by default) — enables the CrosstermBackend backend and adds a dependency on crossterm.
termion — enables the TermionBackend backend and adds a dependency on termion.
termwiz — enables the TermwizBackend backend and adds a dependency on termwiz.
The following optional features are available for all backends:

serde — enables serialization and deserialization of style and color types using the serde crate. This is useful if you want to save themes to a file.
macros — enables the border! macro.
palette — enables conversions from colors in the palette crate to Color.
scrolling-regions — Use terminal scrolling regions to make some operations less prone to flickering. (i.e. Terminal::insert_before).
all-widgets — enables all widgets.
Widgets that add dependencies are gated behind feature flags to prevent unused transitive dependencies. The available features are:

widget-calendar — enables the calendar widget module and adds a dependency on time.
The following optional features are only available for some backends:

underline-color (enabled by default) — enables the backend code that sets the underline color. Underline color is only supported by the CrosstermBackend backend, and is not supported on Windows 7.
The following features are unstable and may change in the future:

unstable — Enable all unstable features.
unstable-rendered-line-info — Enables the Paragraph::line_count Paragraph::line_width methods which are experimental and may change in the future. See Issue 293 for more details.
unstable-widget-ref — Enables the WidgetRef and StatefulWidgetRef traits which are experimental and may change in the future.
unstable-backend-writer — Enables getting access to backends’ writers.
Re-exports
pub use crossterm;	crossterm
pub use palette;	palette
pub use termion;	Non-Windows and termion
pub use termwiz;	termwiz
Modules
backend	This module provides the backend implementations for different terminal libraries.
buffer	A module for the Buffer and Cell types.
layout
prelude	A prelude for conveniently writing applications using this library.
style	style contains the primitives used to control how your user interface will look.
symbols
text	Primitives for styled text.
widgets	widgets is a collection of types that implement Widget or StatefulWidget or both.
Macros
assert_buffer_eqDeprecated	Assert that two buffers are equal by comparing their areas and content.
bordermacros	Macro that constructs and returns a combination of the Borders object from TOP, BOTTOM, LEFT and RIGHT.
Structs
CompletedFrame	CompletedFrame represents the state of the terminal after all changes performed in the last Terminal::draw call have been applied. Therefore, it is only valid until the next call to Terminal::draw.
Frame	A consistent view into the terminal state for rendering a single frame.
Terminal	An interface to interact and draw Frames on the user’s terminal.
TerminalOptions	Options to pass to Terminal::with_options
Enums
Viewport	Represents the viewport of the terminal. The viewport is the area of the terminal that is currently visible to the user. It can be either fullscreen, inline or fixed.
Functions
initcrossterm	Initialize a terminal with reasonable defaults for most applications.
init_with_optionscrossterm	Initialize a terminal with the given options and reasonable defaults.
restorecrossterm	Restores the terminal to its original state.
try_initcrossterm	Try to initialize a terminal using reasonable defaults for most applications.
try_init_with_optionscrossterm	Try to initialize a terminal with the given options and reasonable defaults.
try_restorecrossterm	Restore the terminal to its original state.
Type Aliases
DefaultTerminalcrossterm	A type alias for the default terminal type.