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
  getTodos() {
    return apiClient.get('/todos')
  },
  putTodo(todo, id) {
    return apiClient.put('/todos/' + id, todo)
  },
  deleteTodo(id) {
    return apiClient.delete('/todos/' + id)
  },
  addTodo(todo) {
    return apiClient.post('/todos', todo)
  }
}
