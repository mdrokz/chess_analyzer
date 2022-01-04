# Chess Analyzer

This program goes to match archives of a chess.com user and opens all the matches to queue analysis request and then repeats this process till there are no pages left.


# Usage - 

1. you need to have a chess account with gold or diamond membership
2. there are 5 arguments (`username`,`password`,`target_user`,`tab_cooldown`,`page_cooldown`)
3. pass number to `tab_cooldown` for waiting after opening 20 tabs, this prevents rate limiting. 
4. pass number to `page_cooldown` for waiting after opening every page in the table, this prevents rate limiting.
5. pass args like this `cargo run -- username password target_user tab_cooldown page_cooldown`
