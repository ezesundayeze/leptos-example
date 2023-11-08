use leptos::*;

use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
struct TodoItem {
    id: u32,
    content: String,
}

fn new_todo_id() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

#[component]
fn App() -> impl IntoView {
    let todos: (ReadSignal<Vec<TodoItem>>, WriteSignal<Vec<TodoItem>>) = create_signal(vec![
        TodoItem {
            id: new_todo_id(),
            content: "Watch: The Endsars Documentary".to_string(),
        },
        TodoItem {
            id: new_todo_id(),
            content: "Play Football".to_string(),
        },
    ]);

    view! {
        <div class="todo-app">
            <h1>"Todo App"</h1>

            <TodoInput initial_todos={todos} />
            // <TodoList todos={todos} />

        </div>
    }
}

#[component]
fn TodoInput(
    initial_todos: (ReadSignal<Vec<TodoItem>>, WriteSignal<Vec<TodoItem>>),
) -> impl IntoView {
    let (_, set_new_todo) = initial_todos;
    let (default_value, set_default_value) = create_signal("");

    view! {
        <input
        type="text"
        class= "new-todo"
        autofocus=true
        placeholder="Add todo"
        on:keydown= move |event| {
            if event.key() == "Enter" && !event_target_value(&event).is_empty() {
                let input_value = event_target_value(&event);
                let new_todo_item = TodoItem { id: new_todo_id(), content: input_value.clone() };
                set_new_todo.try_update(|todo| todo.push(new_todo_item));
                set_default_value.set("");
            }}
            prop:value=default_value
        />
    }
}

#[component]
fn TodoList(todos: (ReadSignal<Vec<TodoItem>>, WriteSignal<Vec<TodoItem>>)) -> impl IntoView {
    let (todo_list_state, set_todo_list_state) = todos;

    let my_todos = move || {
        todo_list_state
            .get()
            .iter()
            .map(|item| (item.id, item.clone()))
            .collect::<Vec<_>>()
    };

    view! {
        <ul class="todo-list">
            // The `<For/>` component is central here. 
            // It allows for efficient rendering of lists with keys.
        <For
            // each: takes any function that returns an iterator.
            // This should usually be a signal or derived signal.
            // If it's not reactive, just render a Vec<_> instead of <For/>.
            each=my_todos
            // Choose a unique and stable key for each row in your list.
            // Don't use the index of the row as the key, unless your list can only grow.
            // If you move items around in the list, their indices will change, and all of the rows will rerender unnecessarily.
            // A better key would be a unique identifier for each row, such as a database ID.
            key=|counter| counter.0
            // children: receives each item from the `each` iterator and returns a view of it.
            children=move |item| {
                view! {
                    <li class="new-todo" > {item.1.content}
                        <button
                        class="remove"
                            on:click=move |_| {
                                set_todo_list_state.update(|todos| {
                                    todos.retain(|todo| &todo.id != &item.1.id)
                                });
                            }
                        >

                        </button>
                    </li>
                }
            }
        />
    </ul>
    }
}

fn main() {
    leptos::mount_to_body(App);
}
