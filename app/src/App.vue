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
              {{ u.created_at.toString() }}
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <app-register></app-register>

  </div>
</template>

<script>
import client, { API_ENDPOINT } from './utils/client'
import Register from './components/Register'


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
        myJson.map(u => {
          u.created_at = new Date(u.created_at)
        })
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

  table {
    border: solid thin;
    border-radius: 0.5rem;
    border-collapse: collapse;
  }

  td, th {
    border: 1px solid #999;
    padding: 0.5rem;
    text-align: left;
  }
</style>