import React, { useState } from "react"
import "./App.css"

type Todo = {
  name: string
}

const HARD_CODED = [
  { name: "Wash laundry" },
  { name: "Buy carrots" },
  { name: "Take out trash" },
]

function App(): JSX.Element {
  const [todos, setTodos] = useState(HARD_CODED)
  const [newTodoName, setNewTodoName] = useState("")
  const [notification, setNotification] = useState("")

  function handleNewNoteChange(event: React.ChangeEvent<HTMLInputElement>) {
    setNewTodoName(event.target.value)
  }

  function handleSubmitTodo(event: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
    event.preventDefault()
    if (newTodoName.length > 10) {
      setNotification("Message too long. Max length 140 characters")
      return
    }
    const newTodo = { name: newTodoName }
    setTodos(todos.concat(newTodo))
    setNewTodoName("")
    setNotification("")
  }

  function todosToListing(todos: Todo[]): JSX.Element {
    const todoListings = todos.map((todo: any) => (
      <li key={todo.name}>{todo.name}</li>
    ))
    return <ul>{todoListings}</ul>
  }

  return (
    <div className="App">
      <header className="App-header">
        
  {notification && <p>{notification}</p>}
        <h1>Welcome to Crudster TODO</h1>
        <p>Add todo items by to below</p>
        <form>
          <label>
            Add todo:
            <input value={newTodoName} onChange={handleNewNoteChange} />
          </label>
          <button onClick={(event) => handleSubmitTodo(event)}>Save</button>
        </form>
        <h2>Stuff to do:</h2>
        {todosToListing(todos)}
        <img src="/daily_image" alt="imagenotfound"/>
      </header>
    </div>
  )
}

export default App
