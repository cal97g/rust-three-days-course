#[derive(Eq,PartialEq,Debug)]
pub enum Command {
    Publish(String),
    Retrieve
}

#[derive(Eq,PartialEq,Debug)]
pub enum Error {
    UnknownVerb,
    UnexpectedPayload,
    MissingPayload,
    EmptyMessage,
    IncompleteMessage,
}

pub fn parse(input: &str) -> Result<Command, Error> {
    let cleaned_input;

    if let Some(pos) = input.find('\n') {
        cleaned_input = &input[0..pos];
    } else {
        return Err(Error::IncompleteMessage)
    }

    let mut split = cleaned_input.splitn(2, ' ');

    if let Some(verb) = split.next() {
        match verb.trim() {
            "RETRIEVE" => {
                if split.next() == None {
                    Ok(Command::Retrieve)
                } else {
                    Err(Error::UnexpectedPayload)
                }
            }
            "PUBLISH" => {
                if let Some(payload) = split.next() {
                    Ok(Command::Publish(payload.trim().into()))
                } else {
                    Err(Error::MissingPayload)
                }
            }
            "" => {
                Err(Error::EmptyMessage)
            }
            _ => { Err(Error::UnknownVerb) }
        }
    } else {
        Err(Error::EmptyMessage)
    }
}