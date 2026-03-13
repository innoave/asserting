use crate::prelude::*;
use crate::recursive_comparison::value::{
    string, struct_with_fields, uint16, uint32, unit_variant,
};
use crate::std::string::{String, ToString};
use serde::Serialize;

#[derive(Serialize)]
enum Gender {
    Male,
    Female,
    NonBinary,
    PreferNotToSay,
}

#[derive(Serialize)]
struct Person {
    id: usize,
    name: String,
    age: u8,
    gender: Gender,
    address: Address,
}

#[derive(Serialize)]
struct Address {
    id: usize,
    street: String,
    zip: u16,
    city: String,
}

#[derive(Serialize)]
struct PersonDto {
    name: String,
    age: u8,
    gender: Gender,
    address: AddressDto,
}

#[derive(Serialize)]
struct AddressDto {
    street: String,
    zip: u16,
    city: String,
}

#[test]
fn struct_is_equal_to_using_recursive_comparison_all_fields() {
    let person = Person {
        id: 123,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::Female,
        address: Address {
            id: 91,
            street: "Main Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    assert_that(&person)
        .using_recursive_comparison()
        .is_equal_to(Person {
            id: 123,
            name: "Silvia".into(),
            age: 25,
            gender: Gender::Female,
            address: Address {
                id: 91,
                street: "Main Street".into(),
                zip: 12345,
                city: "New York".into(),
            },
        });
}

#[test]
fn verify_struct_is_equal_to_using_recursive_comparison_all_fields_fails() {
    let person = Person {
        id: 123,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::NonBinary,
        address: Address {
            id: 91,
            street: "Second Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    let failures = verify_that(&person)
        .named("person")
        .using_recursive_comparison()
        .is_equal_to(Person {
            id: 123,
            name: "Silvia".to_string(),
            age: 21,
            gender: Gender::Female,
            address: Address {
                id: 91,
                street: "Main Street".into(),
                zip: 12345,
                city: "New York".into(),
            },
        })
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected person to be equal to Person { id: 123, name: "Silvia", age: 21, gender: Female, address: Address { id: 91, street: "Main Street", zip: 12345, city: "New York" } } (using recursive comparison)
   but was: Person { id: 123, name: "Silvia", age: 25, gender: NonBinary, address: Address { id: 91, street: "Second Street", zip: 12345, city: "New York" } }
  expected: Person { id: 123, name: "Silvia", age: 21, gender: Female, address: Address { id: 91, street: "Main Street", zip: 12345, city: "New York" } }

  non equal fields:
    age: expected <21> but was <25>
    gender: expected <Female> but was <NonBinary>
    address.street: expected <"Main Street"> but was <"Second Street">

"#
        ]
    );
}

#[test]
fn struct_is_equal_to_using_recursive_comparison_ignoring_one_field() {
    let person = Person {
        id: 123,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::Female,
        address: Address {
            id: 91,
            street: "Main Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    assert_that(&person)
        .using_recursive_comparison()
        .ignoring_field("gender")
        .is_equal_to(Person {
            id: 123,
            name: "Silvia".into(),
            age: 25,
            gender: Gender::PreferNotToSay,
            address: Address {
                id: 91,
                street: "Main Street".into(),
                zip: 12345,
                city: "New York".into(),
            },
        });
}

#[test]
fn verify_struct_is_equal_to_using_recursive_comparison_ignoring_one_field_fails() {
    let person = Person {
        id: 123,
        name: "silvia".into(),
        age: 25,
        gender: Gender::Female,
        address: Address {
            id: 91,
            street: "Second Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    let failures = verify_that(&person)
        .named("person")
        .using_recursive_comparison()
        .ignoring_field("gender")
        .is_equal_to(Person {
            id: 123,
            name: "Silvia".to_string(),
            age: 21,
            gender: Gender::PreferNotToSay,
            address: Address {
                id: 91,
                street: "Main Street".into(),
                zip: 12345,
                city: "New York".into(),
            },
        })
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected person to be equal to Person { id: 123, name: "Silvia", age: 21, gender: PreferNotToSay, address: Address { id: 91, street: "Main Street", zip: 12345, city: "New York" } } (using recursive comparison)
   but was: Person { id: 123, name: "silvia", age: 25, gender: Female, address: Address { id: 91, street: "Second Street", zip: 12345, city: "New York" } }
  expected: Person { id: 123, name: "Silvia", age: 21, gender: PreferNotToSay, address: Address { id: 91, street: "Main Street", zip: 12345, city: "New York" } }

  non equal fields:
    name: expected <"Silvia"> but was <"silvia">
    age: expected <21> but was <25>
    address.street: expected <"Main Street"> but was <"Second Street">

  the following fields were ignored:
    gender

"#
        ]
    );
}

#[test]
fn struct_is_equal_to_using_recursive_comparison_ignoring_one_field_two_levels_deep() {
    let person = Person {
        id: 123,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::NonBinary,
        address: Address {
            id: 91,
            street: "Second Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    assert_that(&person)
        .using_recursive_comparison()
        .ignoring_field("address.street")
        .is_equal_to(Person {
            id: 123,
            name: "Silvia".into(),
            age: 25,
            gender: Gender::NonBinary,
            address: Address {
                id: 91,
                street: "Main Street".into(),
                zip: 12345,
                city: "New York".into(),
            },
        });
}

#[test]
fn verify_struct_is_equal_to_using_recursive_comparison_ignoring_one_field_two_levels_deep_fails() {
    let person = Person {
        id: 123,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::Female,
        address: Address {
            id: 90,
            street: "Second Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    let failures = verify_that(&person)
        .named("person")
        .using_recursive_comparison()
        .ignoring_field("address.street")
        .is_equal_to(Person {
            id: 123,
            name: "Silvia".to_string(),
            age: 25,
            gender: Gender::PreferNotToSay,
            address: Address {
                id: 91,
                street: "Main Street".into(),
                zip: 12345,
                city: "New York".into(),
            },
        })
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected person to be equal to Person { id: 123, name: "Silvia", age: 25, gender: PreferNotToSay, address: Address { id: 91, street: "Main Street", zip: 12345, city: "New York" } } (using recursive comparison)
   but was: Person { id: 123, name: "Silvia", age: 25, gender: Female, address: Address { id: 90, street: "Second Street", zip: 12345, city: "New York" } }
  expected: Person { id: 123, name: "Silvia", age: 25, gender: PreferNotToSay, address: Address { id: 91, street: "Main Street", zip: 12345, city: "New York" } }

  non equal fields:
    gender: expected <PreferNotToSay> but was <Female>
    address.id: expected <91> but was <90>

  the following fields were ignored:
    address.street

"#
        ]
    );
}

#[test]
fn struct_is_equal_to_using_recursive_comparison_ignoring_one_field_and_all_its_subfields() {
    let person = Person {
        id: 123,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::NonBinary,
        address: Address {
            id: 91,
            street: "Second Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    assert_that(&person)
        .using_recursive_comparison()
        .ignoring_field("address")
        .is_equal_to(Person {
            id: 123,
            name: "Silvia".into(),
            age: 25,
            gender: Gender::NonBinary,
            address: Address {
                id: 0,
                street: "Main Street".into(),
                zip: 33333,
                city: "Chicago".into(),
            },
        });
}

#[test]
fn verify_struct_is_equal_to_using_recursive_comparison_ignoring_one_field_and_all_its_subfields_fails(
) {
    let person = Person {
        id: 123,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::NonBinary,
        address: Address {
            id: 91,
            street: "Second Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    let failures = verify_that(&person)
        .named("person")
        .using_recursive_comparison()
        .ignoring_field("address")
        .is_equal_to(Person {
            id: 0,
            name: "Silvia".to_string(),
            age: 25,
            gender: Gender::NonBinary,
            address: Address {
                id: 0,
                street: "Main Street".into(),
                zip: 33333,
                city: "Chicago".into(),
            },
        })
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected person to be equal to Person { id: 0, name: "Silvia", age: 25, gender: NonBinary, address: Address { id: 0, street: "Main Street", zip: 33333, city: "Chicago" } } (using recursive comparison)
   but was: Person { id: 123, name: "Silvia", age: 25, gender: NonBinary, address: Address { id: 91, street: "Second Street", zip: 12345, city: "New York" } }
  expected: Person { id: 0, name: "Silvia", age: 25, gender: NonBinary, address: Address { id: 0, street: "Main Street", zip: 33333, city: "Chicago" } }

  non equal fields:
    id: expected <0> but was <123>

  the following fields were ignored:
    address.id
    address.street
    address.zip
    address.city

"#
        ]
    );
}

#[test]
fn struct_is_equal_to_using_recursive_comparison_ignoring_three_fields_repeated_method_calls() {
    let person = Person {
        id: 123,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::Male,
        address: Address {
            id: 91,
            street: "Second Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    assert_that(&person)
        .using_recursive_comparison()
        .ignoring_field("id")
        .ignoring_field("address.street")
        .ignoring_field("gender")
        .is_equal_to(Person {
            id: 0,
            name: "Silvia".into(),
            age: 25,
            gender: Gender::Female,
            address: Address {
                id: 91,
                street: "Main Street".into(),
                zip: 12345,
                city: "New York".into(),
            },
        });
}

#[test]
fn verify_struct_is_equal_to_using_recursive_comparison_ignoring_three_fields_repeated_method_calls_fails(
) {
    let person = Person {
        id: 123,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::Male,
        address: Address {
            id: 91,
            street: "Second Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    let failures = verify_that(&person)
        .named("person")
        .using_recursive_comparison()
        .ignoring_field("id")
        .ignoring_field("address.street")
        .ignoring_field("gender")
        .is_equal_to(Person {
            id: 0,
            name: "Silvia".to_string(),
            age: 21,
            gender: Gender::Female,
            address: Address {
                id: 0,
                street: "Main Street".to_string(),
                zip: 33333,
                city: "Chicago".to_string(),
            },
        })
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected person to be equal to Person { id: 0, name: "Silvia", age: 21, gender: Female, address: Address { id: 0, street: "Main Street", zip: 33333, city: "Chicago" } } (using recursive comparison)
   but was: Person { id: 123, name: "Silvia", age: 25, gender: Male, address: Address { id: 91, street: "Second Street", zip: 12345, city: "New York" } }
  expected: Person { id: 0, name: "Silvia", age: 21, gender: Female, address: Address { id: 0, street: "Main Street", zip: 33333, city: "Chicago" } }

  non equal fields:
    age: expected <21> but was <25>
    address.id: expected <0> but was <91>
    address.zip: expected <33333> but was <12345>
    address.city: expected <"Chicago"> but was <"New York">

  the following fields were ignored:
    id
    gender
    address.street

"#
        ]
    );
}

#[test]
fn struct_is_equal_to_using_recursive_comparison_ignoring_three_fields() {
    let person = Person {
        id: 123,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::Male,
        address: Address {
            id: 91,
            street: "Second Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    assert_that(&person)
        .using_recursive_comparison()
        .ignoring_fields(["id", "gender", "address.street"])
        .is_equal_to(Person {
            id: 0,
            name: "Silvia".into(),
            age: 25,
            gender: Gender::Female,
            address: Address {
                id: 91,
                street: "Main Street".into(),
                zip: 12345,
                city: "New York".into(),
            },
        });
}

#[test]
fn verify_struct_is_equal_to_using_recursive_comparison_ignoring_three_fields_fails() {
    let person = Person {
        id: 123,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::Male,
        address: Address {
            id: 91,
            street: "Second Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    let failures = verify_that(&person)
        .named("person")
        .using_recursive_comparison()
        .ignoring_fields(["address.id", "gender", "address.city"])
        .is_equal_to(Person {
            id: 0,
            name: "Silvia".to_string(),
            age: 25,
            gender: Gender::Female,
            address: Address {
                id: 0,
                street: "Main Street".to_string(),
                zip: 33333,
                city: "Chicago".to_string(),
            },
        })
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected person to be equal to Person { id: 0, name: "Silvia", age: 25, gender: Female, address: Address { id: 0, street: "Main Street", zip: 33333, city: "Chicago" } } (using recursive comparison)
   but was: Person { id: 123, name: "Silvia", age: 25, gender: Male, address: Address { id: 91, street: "Second Street", zip: 12345, city: "New York" } }
  expected: Person { id: 0, name: "Silvia", age: 25, gender: Female, address: Address { id: 0, street: "Main Street", zip: 33333, city: "Chicago" } }

  non equal fields:
    id: expected <0> but was <123>
    address.street: expected <"Main Street"> but was <"Second Street">
    address.zip: expected <33333> but was <12345>

  the following fields were ignored:
    gender
    address.id
    address.city

"#
        ]
    );
}

#[test]
fn struct_is_equal_to_using_recursive_comparison_ignoring_id_fields_on_different_levels() {
    let person = Person {
        id: 123,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::Female,
        address: Address {
            id: 91,
            street: "Main Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    assert_that(&person)
        .using_recursive_comparison()
        .ignoring_fields(["id", "address.id"])
        .is_equal_to(Person {
            id: 0,
            name: "Silvia".into(),
            age: 25,
            gender: Gender::Female,
            address: Address {
                id: 0,
                street: "Main Street".into(),
                zip: 12345,
                city: "New York".into(),
            },
        });
}

#[test]
fn verify_struct_is_equal_to_using_recursive_comparison_ignoring_id_fields_on_different_levels_fails(
) {
    let person = Person {
        id: 123,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::Female,
        address: Address {
            id: 91,
            street: "Main Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    let failures = verify_that(&person)
        .named("person")
        .using_recursive_comparison()
        .ignoring_fields(["id", "address.id"])
        .is_equal_to(Person {
            id: 0,
            name: "Silvia".to_string(),
            age: 21,
            gender: Gender::Female,
            address: Address {
                id: 0,
                street: "Main Street".to_string(),
                zip: 33333,
                city: "New York".to_string(),
            },
        })
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected person to be equal to Person { id: 0, name: "Silvia", age: 21, gender: Female, address: Address { id: 0, street: "Main Street", zip: 33333, city: "New York" } } (using recursive comparison)
   but was: Person { id: 123, name: "Silvia", age: 25, gender: Female, address: Address { id: 91, street: "Main Street", zip: 12345, city: "New York" } }
  expected: Person { id: 0, name: "Silvia", age: 21, gender: Female, address: Address { id: 0, street: "Main Street", zip: 33333, city: "New York" } }

  non equal fields:
    age: expected <21> but was <25>
    address.zip: expected <33333> but was <12345>

  the following fields were ignored:
    id
    address.id

"#
        ]
    );
}

#[test]
fn struct_is_equal_to_using_recursive_comparison_comparing_only_specified_fields() {
    let person = Person {
        id: 456,
        name: "Silvia".into(),
        age: 27,
        gender: Gender::Female,
        address: Address {
            id: 291,
            street: "Second Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    assert_that!(person)
        .using_recursive_comparison()
        .comparing_only_field("name")
        .comparing_only_fields(["gender", "address.zip", "address.city"])
        .is_equal_to(Person {
            id: 0,
            name: "Silvia".into(),
            age: 25,
            gender: Gender::Female,
            address: Address {
                id: 0,
                street: "Main Street".into(),
                zip: 12345,
                city: "New York".into(),
            },
        });
}

#[test]
fn verify_struct_is_equal_to_using_recursive_comparison_comparing_only_specified_fields_fails() {
    let person = Person {
        id: 123,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::Female,
        address: Address {
            id: 91,
            street: "Second Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    let failures = verify_that(&person)
        .named("person")
        .using_recursive_comparison()
        .comparing_only_field("name")
        .comparing_only_fields(["gender", "address.zip", "address.city"])
        .is_equal_to(Person {
            id: 0,
            name: "Silvia".to_string(),
            age: 21,
            gender: Gender::Female,
            address: Address {
                id: 0,
                street: "Main Street".to_string(),
                zip: 33333,
                city: "Chicago".to_string(),
            },
        })
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected person to be equal to Person { id: 0, name: "Silvia", age: 21, gender: Female, address: Address { id: 0, street: "Main Street", zip: 33333, city: "Chicago" } } (using recursive comparison)
   but was: Person { id: 123, name: "Silvia", age: 25, gender: Female, address: Address { id: 91, street: "Second Street", zip: 12345, city: "New York" } }
  expected: Person { id: 0, name: "Silvia", age: 21, gender: Female, address: Address { id: 0, street: "Main Street", zip: 33333, city: "Chicago" } }

  non equal fields:
    address.zip: expected <33333> but was <12345>
    address.city: expected <"Chicago"> but was <"New York">

  the following fields were ignored:
    id
    age
    address.id
    address.street

"#
        ]
    );
}

#[test]
fn struct_is_equal_to_equivalent_type() {
    let person = Person {
        id: 456,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::Female,
        address: Address {
            id: 291,
            street: "Main Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    assert_that(&person)
        .using_recursive_comparison()
        .ignoring_not_expected_fields()
        .is_equal_to(PersonDto {
            name: "Silvia".into(),
            age: 25,
            gender: Gender::Female,
            address: AddressDto {
                street: "Main Street".into(),
                zip: 12345,
                city: "New York".into(),
            },
        });
}

#[test]
fn verify_struct_is_equal_to_equivalent_type_do_not_ignore_not_expected_fields_fails() {
    let person = Person {
        id: 456,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::Female,
        address: Address {
            id: 291,
            street: "Main Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    let failures = verify_that(&person)
        .named("person")
        .using_recursive_comparison()
        .is_equal_to(PersonDto {
            name: "Silvia".into(),
            age: 25,
            gender: Gender::Female,
            address: AddressDto {
                street: "Main Street".into(),
                zip: 12345,
                city: "New York".into(),
            },
        })
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected person to be equal to PersonDto { name: "Silvia", age: 25, gender: Female, address: AddressDto { street: "Main Street", zip: 12345, city: "New York" } } (using recursive comparison)
   but was: Person { id: 456, name: "Silvia", age: 25, gender: Female, address: Address { id: 291, street: "Main Street", zip: 12345, city: "New York" } }
  expected: PersonDto { name: "Silvia", age: 25, gender: Female, address: AddressDto { street: "Main Street", zip: 12345, city: "New York" } }

  the following fields were not expected:
    id: 456
    address.id: 291

"#
        ]
    );
}

#[test]
fn verify_struct_is_equal_to_equivalent_type_fails_all_fields_different() {
    let person = Person {
        id: 456,
        name: "silvia".into(),
        age: 27,
        gender: Gender::Male,
        address: Address {
            id: 291,
            street: "Second Street".into(),
            zip: 33333,
            city: "Chicago".into(),
        },
    };

    let failures = verify_that(&person)
        .named("person")
        .using_recursive_comparison()
        .ignoring_not_expected_fields()
        .is_equal_to(PersonDto {
            name: "Silvia".into(),
            age: 25,
            gender: Gender::Female,
            address: AddressDto {
                street: "Main Street".into(),
                zip: 12345,
                city: "New York".into(),
            },
        })
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected person to be equal to PersonDto { name: "Silvia", age: 25, gender: Female, address: AddressDto { street: "Main Street", zip: 12345, city: "New York" } } (using recursive comparison)
   but was: Person { id: 456, name: "silvia", age: 27, gender: Male, address: Address { id: 291, street: "Second Street", zip: 33333, city: "Chicago" } }
  expected: PersonDto { name: "Silvia", age: 25, gender: Female, address: AddressDto { street: "Main Street", zip: 12345, city: "New York" } }

  non equal fields:
    name: expected <"Silvia"> but was <"silvia">
    age: expected <25> but was <27>
    gender: expected <Female> but was <Male>
    address.street: expected <"Main Street"> but was <"Second Street">
    address.zip: expected <12345> but was <33333>
    address.city: expected <"New York"> but was <"Chicago">

  the following fields were ignored:
    id
    address.id

"#
        ]
    );
}

#[test]
fn verify_struct_is_equal_to_equivalent_type_fails_for_different_type() {
    #[derive(Serialize)]
    struct PersonDto {
        name: String,
        age: u16,
        gender: Gender,
        address: AddressDto,
    }

    let person = Person {
        id: 456,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::Female,
        address: Address {
            id: 291,
            street: "Second Street".into(),
            zip: 33333,
            city: "New York".into(),
        },
    };

    let failures = verify_that(&person)
        .named("person")
        .using_recursive_comparison()
        .ignoring_not_expected_fields()
        .is_equal_to(PersonDto {
            name: "Silvia".into(),
            age: 25,
            gender: Gender::Female,
            address: AddressDto {
                street: "Main Street".into(),
                zip: 12345,
                city: "New York".into(),
            },
        })
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected person to be equal to PersonDto { name: "Silvia", age: 25, gender: Female, address: AddressDto { street: "Main Street", zip: 12345, city: "New York" } } (using recursive comparison)
   but was: Person { id: 456, name: "Silvia", age: 25, gender: Female, address: Address { id: 291, street: "Second Street", zip: 33333, city: "New York" } }
  expected: PersonDto { name: "Silvia", age: 25, gender: Female, address: AddressDto { street: "Main Street", zip: 12345, city: "New York" } }

  non equal fields:
    age: value <25> was equal, but type was <u8> and expected type is <u16>
    address.street: expected <"Main Street"> but was <"Second Street">
    address.zip: expected <12345> but was <33333>

  the following fields were ignored:
    id
    address.id

"#
        ]
    );
}

#[test]
fn struct_is_equivalent_to_struct_with_relevant_fields() {
    let person = Person {
        id: 456,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::Female,
        address: Address {
            id: 291,
            street: "Second Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    assert_that(&person)
        .using_recursive_comparison()
        .ignoring_not_expected_fields()
        .is_equivalent_to(struct_with_fields([
            ("name", string("Silvia")),
            ("gender", unit_variant("Gender", "Female")),
            (
                "address",
                struct_with_fields([("zip", uint16(12345)), ("city", string("New York"))]),
            ),
        ]));
}

#[test]
fn verify_struct_is_equivalent_to_struct_with_relevant_fields_do_not_ignore_not_expected_fields_fails(
) {
    let person = Person {
        id: 456,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::Female,
        address: Address {
            id: 291,
            street: "Main Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    let failures = verify_that(&person)
        .named("person")
        .using_recursive_comparison()
        .is_equivalent_to(struct_with_fields([
            ("name", string("Silvia")),
            ("gender", unit_variant("Gender", "Female")),
            (
                "address",
                struct_with_fields([("zip", uint16(12345)), ("city", string("New York"))]),
            ),
        ]))
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected person to be equivalent to { name: "Silvia", gender: Female, address: { zip: 12345, city: "New York" } } (using recursive comparison)
   but was: Person { id: 456, name: "Silvia", age: 25, gender: Female, address: Address { id: 291, street: "Main Street", zip: 12345, city: "New York" } }
  expected: { name: "Silvia", gender: Female, address: { zip: 12345, city: "New York" } }

  the following fields were not expected:
    id: 456
    age: 25
    address.id: 291
    address.street: "Main Street"

"#
        ]
    );
}

#[test]
fn verify_struct_is_equivalent_to_struct_with_relevant_fields_fails_for_different_type() {
    let person = Person {
        id: 456,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::Female,
        address: Address {
            id: 291,
            street: "Main Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    let failures = verify_that(&person)
        .named("person")
        .using_recursive_comparison()
        .ignoring_not_expected_fields()
        .is_equivalent_to(struct_with_fields([
            ("name", string("Silvia")),
            ("gender", unit_variant("Gender", "Female")),
            (
                "address",
                struct_with_fields([("zip", uint32(12345)), ("city", string("New York"))]),
            ),
        ]))
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected person to be equivalent to { name: "Silvia", gender: Female, address: { zip: 12345, city: "New York" } } (using recursive comparison)
   but was: Person { id: 456, name: "Silvia", age: 25, gender: Female, address: Address { id: 291, street: "Main Street", zip: 12345, city: "New York" } }
  expected: { name: "Silvia", gender: Female, address: { zip: 12345, city: "New York" } }

  non equal fields:
    address.zip: value <12345> was equal, but type was <u16> and expected type is <u32>

  the following fields were ignored:
    id
    age
    address.id
    address.street

"#
        ]
    );
}

#[test]
fn struct_is_equivalent_to_value_from_macro() {
    let person = Person {
        id: 456,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::Female,
        address: Address {
            id: 291,
            street: "Main Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    assert_that(&person)
        .using_recursive_comparison()
        .ignoring_not_expected_fields()
        .is_equivalent_to(value!({
            name: "Silvia",
            gender: Gender::Female,
            address: {
                zip: 12345_u16,
                city: "New York",
            },
        }));
}

#[test]
fn struct_is_equivalent_to_value_with_additional_field_from_macro() {
    let person = Person {
        id: 456,
        name: "Silvia".into(),
        age: 25,
        gender: Gender::Female,
        address: Address {
            id: 291,
            street: "Main Street".into(),
            zip: 12345,
            city: "New York".into(),
        },
    };

    assert_that(&person)
        .using_recursive_comparison()
        .ignoring_not_expected_fields()
        .is_equivalent_to(value!({
            name: "Silvia",
            gender: Gender::Female,
            other_field: ("present in actual value", false),
            address: {
                zip: 12345_u16,
                city: "New York",
                state: "not present in actual value",
            },
        }));
}
