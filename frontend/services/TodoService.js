//json-server --watch db.json
import axios from 'axios'

const apiClient = axios.create({
  baseURL: 'http://localhost:8081',
  headers: {
    Accept: 'application/json',
    'Content-Type': 'application/json'
  }
})

export default {
  async getTodos() {
    return await apiClient.get('/todos')
  },
  async putTodo(todo, id) {
    return await apiClient.put('/todos/' + id, todo)
  },
  deleteTodo(id) {
    return apiClient.delete('/todos/' + id)
  },
  async addTodo(todo) {
    return await apiClient.post('/todos', todo)
  }
}
