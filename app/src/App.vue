<template>
  <div>
    <h1>{{ title }}</h1>
    <p>
      {{ msg }}
    </p>

    <div>
      <h2>Currently registered users:</h2>
      <table>
        <thead>
          <tr>
            <th>Username</th>
            <th>Registered</th>
          </tr>
        </thead>
        <tbody v-for="u in users" v-bind:key="u.id">
          <tr>
            <td>
              {{ u.username }}
            </td>
            <td>
              {{ u.created_at }}
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <app-register></app-register>

  </div>
</template>

<script>
import axios from 'axios'
import Register from './components/Register'
const API_ENDPOINT = "http://127.0.0.1:3030"

const client = axios.create({
  baseURL: API_ENDPOINT,
})

export default {
  name: 'app',
  data() {
    return {
      title: 'Welcome to Vue.js!',
      msg: `Serving Rust (warp) API at ${API_ENDPOINT}`,
      users: null
    }
  },
  components: {
    'app-register': Register
  },
  created() {
    const users = client.get('/users')
      .then(response => {
        return response.data
      }).then(myJson => {
        this.users = myJson
      }).catch(err => {
        console.log(err)
      })
  }
}
</script>

<style lang="css">
  h1 {
    color: #362d8b;
  }
  * {
    font-family: 'Roboto', sans-serif;
  }
  body {
    padding: 1em 4em;
  }
</style>