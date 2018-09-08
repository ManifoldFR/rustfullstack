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
    headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
    },
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
            const send_data = JSON.stringify(this.store)
            console.log(send_data)
            let add_user = client.post(
                `/users`, send_data
            ).then(response => {
                console.log(response)
            }).catch(error => {
                console.log(error)
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
