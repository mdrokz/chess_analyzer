(function () {

    const element = document.getElementsByClassName('.archive-games-total')[0];

    const total_matches = parseInt(element.innerText.split(':')[1].replace(',', ''));

    return Math.round(total_matches / 50);
})()