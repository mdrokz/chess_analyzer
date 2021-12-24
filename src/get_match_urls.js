(function () {

    // get matches from the table
    const ar = [...document.getElementsByTagName('tbody')[0].children]

    // get all the url for matches that havent been analyzed
    const match_urls = ar
        .map(x => x.children[3].children[1])
        .filter(x => x && x.innerText.includes('Analyze'))
        .map(x => x.href)

    return match_urls;

})()