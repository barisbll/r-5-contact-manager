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
    fn new(name: String, phone_number: String, email: String) -> Self {
        Self {
            name,
            phone_number,
            email,
        }
    }

    fn update_contact(&mut self, name: String, phone_number: String, email: String) -> &mut Self {
        self.name = name;
        self.phone_number = phone_number;
        self.email = email;
        self
    }
}

struct ContactManager {
    contacts: Vec<Contact>,
}

impl ContactManager {
    fn new() -> Self {
        Self {
            contacts: Vec::new(),
        }
    }

    fn add_contact(&mut self, contact: Contact) {
        self.contacts.push(contact);
    }

    fn display_contacts(&self) {
        for contact in &self.contacts {
            println!("{:?}", contact);
        }
    }
}

fn main() {
    let mut manager = ContactManager::new();

    let mut contact1 = Contact::new(
        String::from("Baris"),
        String::from("123"),
        String::from("test@test.com"),
    );

    contact1.update_contact(
        String::from("Ecem"),
        String::from("456"),
        String::from("ecem@test.com"),
    );

    let contact2 = Contact::new(
        String::from("Alex"),
        String::from("789"),
        String::from("alex@test.com"),
    );

    manager.add_contact(contact1);
    manager.add_contact(contact2);

    manager.display_contacts();
}
