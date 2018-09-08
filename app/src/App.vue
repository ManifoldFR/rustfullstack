<template>
  <div id="app">
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
  </div>
</template>

<script>
const API_ENDPOINT = "http://127.0.0.1:3030"

export default {
  name: 'app',
  data() {
    return {
      title: 'Welcome to Vue.js!',
      msg: `Serving Rust (warp) API at ${API_ENDPOINT}`,
      users: null
    }
  },
  mounted() {
    const users = fetch(`${API_ENDPOINT}/users`)
      .then(response => {
        return response.json()
      }).then(myJson => {
        this.users = myJson
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
</style>