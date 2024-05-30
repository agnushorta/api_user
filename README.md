[Building GraphQL APIs in Rust | A Comprehensive Guide](https://youtu.be/P6Tkid9GdfU?si=xrdjjB0ZGJzRzK5X)

- [async-graphql](https://crates.io/crates/async-graphql) : a high-performance graphql server library that's fully specification compliant
- [async-graphql-axum](https://crates.io/crates/async-graphql-axum) : async-graphql for axum
- [axum](https://crates.io/crates/axum) : Web framework that focuses on ergonomics and modularity
- [serde](https://crates.io/crates/serde) : A generic serialization/deserialization framework
- [tokio](https://crates.io/crates/tokio) : An event-driven, non-blocking I/O platform for writing asynchronous I/O backed applications.


```bash
cargo add async-graphql
cargo add async-graphql-axum
cargo add axum
cargo add serde
cargo add tokio
```


#[derive(Clone)]

## Traits
A trait is a collection of methods defined for an unknown type: Self. They can access other methods declared in the same trait.
[Rust By Example](https://doc.rust-lang.org/rust-by-example/trait.html)

## Derive
The compiler is capable of providing basic implementations for some traits via the #[derive] attribute. These traits can still be manually implemented if a more complex behavior is required.

The following is a list of derivable traits:

Comparison traits: Eq, PartialEq, Ord, PartialOrd.
Clone, to create T from &T via a copy.
Copy, to give a type 'copy semantics' instead of 'move semantics'.
Hash, to compute a hash from &T.
Default, to create an empty instance of a data type.
Debug, to format a value using the {:?} formatter.

[Rust By Example](https://doc.rust-lang.org/rust-by-example/trait/derive.html)

### Clone
When dealing with resources, the default behavior is to transfer them during assignments or function calls. However, sometimes we need to make a copy of the resource as well.

The Clone trait helps us do exactly this. Most commonly, we can use the .clone() method defined by the Clone trait.

[Rust By Example](https://doc.rust-lang.org/rust-by-example/trait/clone.html)

## Traits
**Explanation**:

- Traits are code reuse mechanisms in programming languages that allow sharing methods between classes.
- They allow to abstract over common behavior between different types.
- They are used to define characteristics or behaviors that can be shared by different classes.

**Context**: Traits are used in programming to promote code reuse and facilitate the sharing of behaviors between different classes. In languages like PHP and Rust, traits are a way to add functionality to classes without using inheritance.

**Example Sentence**: The use of traits in PHP allows different classes to have methods in common, making it easier to develop modular and flexible applications.


Sure, in Rust, the `#[object]` attribute is used to specify additional information about an implementation. It can be used to specify the following:

- **The type of the object being implemented.** This can be useful for disambiguating between multiple implementations of the same trait.
- **The visibility of the object being implemented.** This can be used to control who can access the object.
- **The mutability of the object being implemented.** This can be used to control whether the object can be modified.

For example, the following code shows how to use the `#[object]` attribute to specify the type of the object being implemented:

```
#[object(MyType)]
impl MyTrait for MyType {
    // ...
}
```

This code tells the compiler that the implementation of `MyTrait` for `MyType` is for the type `MyType`. This can be useful if there are multiple implementations of `MyTrait` for different types.

Here is an example of using the `#[object]` attribute to specify the visibility of the object being implemented:

```
#[object(pub)]
impl MyTrait for MyType {
    // ...
}
```

This code tells the compiler that the implementation of `MyTrait` for `MyType` is public. This means that it can be accessed by any code that has access to the module in which it is defined.

Finally, here is an example of using the `#[object]` attribute to specify the mutability of the object being implemented:

```
#[object(mut)]
impl MyTrait for MyType {
    // ...
}
```

This code tells the compiler that the implementation of `MyTrait` for `MyType` is mutable. This means that the object can be modified.

The `#[object]` attribute is a powerful tool that can be used to control the behavior of Rust code. It is important to understand how to use it in order to write correct and efficient code.

When you use `#[object]` without specifying any additional information, it is equivalent to using `#[object(default)]`. This means that the compiler will use the default settings for the object being implemented.

The default settings for the object being implemented are as follows:

- **The type of the object being implemented** is inferred from the context.
- **The visibility of the object being implemented** is the same as the visibility of the trait being implemented.
- **The mutability of the object being implemented** is `const`.

For example, the following code shows how to use the `#[object]` attribute without specifying any additional information:

```
#[object]
impl MyTrait for MyType {
    // ...
}
```

This code is equivalent to the following code:

```
#[object(default)]
impl MyTrait for MyType {
    // ...
}
```

In this case, the compiler will infer the type of the object being implemented from the context. The visibility of the object being implemented will be the same as the visibility of the trait being implemented. And the mutability of the object being implemented will be `const`.

It is important to note that the `#[object]` attribute is only necessary if you need to specify non-default settings for the object being implemented. If you are using the default settings, then you can simply omit the `#[object]` attribute.


**Object-Relational Mapping (ORM)**

**Concept:**
ORM is a technique that simplifies the interaction between object-oriented programming languages and relational databases. It allows developers to work with database objects (tables, rows, columns) as if they were objects in the programming language.

**How it Works:**
1. **Define ORM Classes:** Developers create classes in the programming language that represent the tables in the database.
2. **Map Attributes:** Each class attribute corresponds to a column in the table.
3. **Create ORM Objects:** Instances of these classes represent individual rows in the table.
4. **Database Interaction:** ORM frameworks handle the underlying SQL queries and database operations, freeing developers from writing complex SQL code.

**Benefits:**

* **Abstraction:** Hides the complexities of SQL from developers, making database interaction more intuitive.
* **Productivity:** Reduces development time by simplifying database operations.
* **Maintainability:** Enforces data integrity and consistency through object-oriented validation rules.
* **Portability:** Supports different database systems, allowing developers to easily switch between them.

**Popular ORM Frameworks:**

* **Python:** SQLAlchemy, Django ORM
* **Java:** Hibernate, JPA
* **C#:** Entity Framework Core
* **Node.js:** Mongoose, Sequelize

**Example:**

Consider a "Customers" table with columns "id", "name", and "email".

**Java Code with ORM:**

```java
// Customer ORM class
public class Customer {
    private int id;
    private String name;
    private String email;
}

// Usage:
Customer customer = new Customer();
customer.setName("John Doe");
customer.setEmail("john.doe@example.com");
```

**SQL Equivalent:**

```sql
INSERT INTO Customers (name, email) VALUES ('John Doe', 'john.doe@example.com');
```

By using ORM, developers can interact with the database using familiar object-oriented concepts, reducing the need for manual SQL coding.

Sure. Here's an example of using the Diesel ORM in Rust:

```rust
// Define the Customer struct to represent a row in the "customers" table
#[derive(Queryable)]
struct Customer {
    id: i32,
    name: String,
    email: String,
}

// Establish a connection to the database
let connection = SqliteConnection::establish("path/to/database.sqlite").unwrap();

// Create a new customer using the Diesel DSL
let new_customer = Customer {
    id: 0,
    name: "John Doe".to_string(),
    email: "john.doe@example.com".to_string(),
};

// Insert the new customer into the database
diesel::insert_into(customers::table)
    .values(&new_customer)
    .execute(&connection)
    .unwrap();

// Query for all customers
let all_customers = customers::table.load::<Customer>(&connection).unwrap();

// Print the customer information
for customer in all_customers {
    println!("Customer: {}, {}, {}", customer.id, customer.name, customer.email);
}
```

In this example, the `Customer` struct represents a row in the "customers" table. The `Queryable` derive macro allows Diesel to automatically generate the necessary code to map the struct to the database schema.

We establish a connection to the database using the `SqliteConnection` type. Then, we use the Diesel DSL to create a new customer object and insert it into the database using the `insert_into` and `execute` methods.

Finally, we query for all customers using the `load` method and print their information.

Diesel is a powerful and type-safe ORM for Rust that simplifies database interaction and reduces the need for manual SQL coding.
