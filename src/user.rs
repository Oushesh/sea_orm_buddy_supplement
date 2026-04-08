//Data Model Definition using SeaORM
use sea_orm::prelude::*;

#[derive(Debug,Clone,PartialEq, Eq,DeriveEntityModel,Default)]
#[sea_orm(table_name = "users")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub id: i32, // Added pub so main can see it
    pub name: String, // Added pub so main can see it
}

//TODO: Define more model here or other data model

// Fixed: Removed DeriveEntityModel from here
// Added: EnumIter (required by SeaORM for Relations)

#[derive(Debug,Copy, Clone,EnumIter, DeriveRelation)]
pub enum Relation{}

impl ActiveModelBehavior for ActiveModel {}


/*
The magic behind ActiveModel
The #derive Derive Entity Model macro automatically
generates automatically an Active Model for you.

While Model is a read-only snapshot of a database
now, ActiveModel is the version used for Create, Update and
Delete Operations.

 Impl in Rust is a bit like decorators for python.
 Impl works at compile time while decorators in python
 work at runtime changing the behaviour of the function

For rust however its machine code after being compiled
so no overhead.
 */

/*
impl ActiveModelBehavior for ActiveModel {
    fn before_save(self, insert: bool) -> Result<Self, DbErr> {
        if self.name.as_ref().is_empty() {
            return Err(DbErr::Custom("Name cannot be empty".to_owned()));
            Ok(self)
        }
    }

}


Other Examples:

Lets say we define a method inside a class
in Rust it would be:

struct Circle {
    radius: f64,
}

impl Circle {
    // This is a "method" because if takes 'self'

    fn area(&self) -> f64 {
        //Mathematical formula: pi*r^2
        std::f64:consts:PI * self.radius*self.radius
    }

    // This is an "associated function" (like a static method)

    fn new (radius:f64) --> Self {
        Self { radius }
    }

}

Nice. How would we call this one: my_circle.area() or Circle::new(5.0)

2. Implementing Traits (Interface Logic)
(More Complicated Concept) --> Implementing Traits

*/


/*
Motivation for 
 */