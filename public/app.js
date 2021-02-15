const server = 'http://localhost:8000';
/**
 * 
 * @param {string} url 
 * @param {any} data 
 * @returns {any} json
 */
const post = (url, data) => {
    return fetch(server + url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data),
    }).then(res => res.json())
        .catch(console.error);// make sure to return the response
}


// Login
post('/user', {
    name: ''
})