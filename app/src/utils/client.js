import axios from 'axios'

export const API_ENDPOINT = "http://127.0.0.1:3030"

export default axios.create({
    baseURL: API_ENDPOINT,
    headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
    },
})
