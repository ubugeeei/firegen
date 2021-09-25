<div align="center">
  <img src="https://user-images.githubusercontent.com/71201308/134777775-a850f363-60f5-4180-893b-975a2ece126c.png">
</div>

<div align="center">
   <a href="https://github.com/Ubugeeei/firegen/actions/workflows/rust.yml">
    <img src="https://github.com/Ubugeeei/firegen/actions/workflows/rust.yml/badge.svg" alt="Rust CI" />
  </a>
  
  <p>Generator of Firestore rules and type safe client code.</p>
</div>

<br />

# Usage [WIP]
## Install from npm or curl.
```sh
$ npm install -g firegen
```
## Setting your yml.

```yml
# firegen.yml
schemaPath: ./schema/**/**.fireSchema
export:
  rulesPath: ./generated/firestore.rules
  clientCodePath: ./generated/firestoreClient.ts

```

## Create firestore schema files.
```ts
// User.fireSchema

/**
 * definition schema
 */
Document User {
  username: Text
  mail?: Text
  age: Int
  todos: Todo
}
/**
 * [WIP] Rules schema
 */
//ex1
Document Todo rules TodoRules {
  id: Int
  description: Text
  memo?: Text
}
Rule TodoRules {
 get: request.auth != null
 list: true
 create: request.auth.uid == userKey
 update: request.auth.uid == userKey
 delete: false
}

//ex2 normalization rules
Document Todo rules AllowReadOnlyLoginUser & AllowWriteOriginalUser {
  id: number
  description: string
  memo?: string
}
Rule AllowReadOnlyLoginUser {
  get: request.auth != null
  list: request.auth != null
}
Rule AllowWriteOriginalUser {
  create: request.auth.uid == userId
  update: request.auth.uid == userId
  delete: request.auth.uid == userId
}

```


## Run command to generate files.
```sh
$ firegen generate
```
```ts
/**
 * export as
 */
export interface User {
  id: number
  username: string
  mail?: string
  age: number
  todos: Todo[]
}
export interface UserCreateInput {
  username: string
  mail?: string
  age: number
  todos: Todo[]
}
export interface UserUpdateInput {
  username?: string
  mail?: string
  age?: number
}
export interface Todo {
  id: number
  description: string
  memo?: string
  createdAt?: string
  isDone: boolean
}
export interface TodoCreateInput {
  description: string
  memo?: string
  createdAt?: string
  isDone: boolean
}
export interface TodoUpdateInput {
  description: string
  memo?: string
  createdAt?: string
  isDone: boolean
}

import firebase from "firebase"
const db = firebase.firestore()
export const firestoreClient = {
  getUsers: async (): Promise<User[]> =>
    await db.collection('users').get(),

  getUser: async (id: number): Promise<User> =>
    await db.collection('users').doc(id).get(),

  createUser: async (input: UserCreateInput) =>
    await db.collection('users').add(input),

  updateUser: async (userId: number, input: UserUpdateInput) =>
    await db.collection('users').doc(userId).update(input),

  deleteUser: async (id: number) =>
    await db.collection('users').doc(id).delete(),

  getTodosByUser: async (id: number): Promise<Todo[]> =>
    await db.collection('users').doc(id).collection('todos').get(),

  getTodoByUser: async (userId: number, todoId: number): Promise<Todo> =>
    await db.collection('users').doc(userId).collection('todos').doc(todoId).get(),

  createTodo: async (userId: number, input: TodoCreateInput) =>
    await db.collection('users').doc(userId).collection('todos').add(input),

  updateTodo: async (userId: number, todoId: number, input: TodoUpdateInput) =>
    await db.collection('users').doc(userId).collection('todos').doc(todoId).update(input),

  deleteTodo: async (userId: number, todoId: number) =>
    await db.collection('users').doc(userId).collection('todos').doc(todoId).delete()
}
```

```ts
/**
 * use in your project!
 */
const userId = store.user.id
const user = await firestoreClient.getUser(userId)
const todos = await firestoreClient.getTodosByUser(userId)

const todoId = 22
await firestoreClient.deleteTodo(userId, todoId)

```
