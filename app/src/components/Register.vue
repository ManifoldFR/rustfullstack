<template>
    <div>
        <h2>Sign up</h2>
        <form method="post" v-on:submit.prevent="onSubmit">
            <input type="text" placeholder="username" 
                name="username" v-model="store.username"
            >
        </form>

        <p id="new-user-greet" v-show="store.username.length">
            Welcome, <span class="register-uname">{{ store.username }}</span>.
        </p>
    </div>
</template>

<script>
import axios from 'axios'
const API_ENDPOINT = "http://127.0.0.1:3030"

const client = axios.create({
    baseURL: API_ENDPOINT,
    withCredentials: false
})

export default {
    data() {
        return {
            store: {
                username: ''
            }
        }
    },
    methods: {
        onSubmit() {
            let add_user = client.post(
                '/users',
                this.store
            ).then(response => {
                console.log(response.statusText)
                console.log(response.headers)
                console.log(response.config)
                console.log(response.data)
            })
        }
    }
}
</script>


<style>

    .register-uname {
        color: teal;
    }
</style>
