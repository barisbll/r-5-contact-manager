// Challenge: Build a simple contact manager. Define a Contact struct with fields for
//  a name, phone number, and email.
// Implement methods to add a contact, update contact details, and display the contact list.

#[derive(Debug)]
struct Contact {
    name: String,
    phone_number: String,
    email: String,
}

impl Contact {
    fn add_contact(name: String, phone_number: String, email: String) -> Self {
        Self {
            name,
            phone_number,
            email,
        }
    }

    fn update_contact(&mut self, name: String, phone_number: String, email: String) {
        self.name = name;
        self.phone_number = phone_number;
        self.email = email;
    }

    fn display_contacts(contacts: Vec<Self>) {
        for contact in contacts {
            println!("{contact:?}");
        }
    }
}

fn main() {
    let mut contact = Contact::add_contact(
        String::from("Baris"),
        String::from("123"),
        String::from("test@test.com"),
    );

    contact.update_contact(
        String::from("Ecem"),
        String::from("456"),
        String::from("ecem@test.com"),
    );

    let contact2 = Contact::add_contact(
        String::from("Baris"),
        String::from("123"),
        String::from("test@test.com"),
    );

    let contacts = vec![contact, contact2];

    Contact::display_contacts(contacts);
}
