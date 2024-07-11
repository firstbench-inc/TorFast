function search() {
    const searchInput = document.getElementById('searchInput').value.trim();

    const elasticSearchUrl = 'http://localhost:9200/logs/_search?q='+searchInput;

    const searchData = {
        query: {
            match: {
                title: searchInput
            }
        }
    };

    const fetchOptions = {
        method: 'GET',
        // headers: {
        //     'Content-Type': 'application/json'
        // },
        // body: JSON.stringify(searchData)
    };

    fetch(elasticSearchUrl, fetchOptions)
        .then(response => {
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            return response.json();
        })
        .then(data => {
            const hits = data.hits.hits;
            console.log(hits)
            displayResults(hits);
        })
        .catch(error => {
            console.error('Error fetching data:', error);
        });
}

function displayResults(results) {
    const resultsContainer = document.getElementById('results');
    resultsContainer.innerHTML = '';

    results.forEach(result => {
        const { _source } = result;
        const resultItem = document.createElement('div');
        resultItem.classList.add('result-item');
        resultItem.innerHTML = `
            <a href="${_source.link}" target="_blank">${_source.title}</a>
        `;
        resultsContainer.appendChild(resultItem);
    });
}

