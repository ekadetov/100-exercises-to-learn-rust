// TODO: Convert the `Ticket::new` method to return a `Result` instead of panicking.
//   Use `String` as the error type.

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Result<Ticket, String> {
        Self::validate_title(&title)?;
        Self::validate_description(&description)?;

        let ticket = Ticket {
            title,
            description,
            status,
        };

        Ok(ticket)
    }

    fn validate_title(title: &String) -> Result<(), String> {
        if title.is_empty() {
            return Result::Err("Title cannot be empty".into());
        }
        if title.len() > 50 {
            return Result::Err("Title cannot be longer than 50 bytes".into());
        }

        Result::Ok(())
    }

    fn validate_description(description: &String) -> Result<(), String> {
        if description.is_empty() {
            return Result::Err("Description cannot be empty".into());
        }
        if description.len() > 500 {
            return Result::Err("Description cannot be longer than 500 bytes".into());
        }

        Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    fn title_cannot_be_empty() {
        let error = Ticket::new("".into(), valid_description(), Status::ToDo).unwrap_err();
        assert_eq!(error, "Title cannot be empty");
    }

    #[test]
    fn description_cannot_be_empty() {
        let error = Ticket::new(valid_title(), "".into(), Status::ToDo).unwrap_err();
        assert_eq!(error, "Description cannot be empty");
    }

    #[test]
    fn title_cannot_be_longer_than_fifty_chars() {
        let error =
            Ticket::new(overly_long_title(), valid_description(), Status::ToDo).unwrap_err();
        assert_eq!(error, "Title cannot be longer than 50 bytes");
    }

    #[test]
    fn description_cannot_be_longer_than_500_chars() {
        let error =
            Ticket::new(valid_title(), overly_long_description(), Status::ToDo).unwrap_err();
        assert_eq!(error, "Description cannot be longer than 500 bytes");
    }
}
