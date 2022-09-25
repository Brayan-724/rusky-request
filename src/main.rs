use cli_test::{preload::*, PromptError};

fn main() -> Result<(), PromptError> {
    let mut events = create_events!();
    let mut stdout = create_stdout!();

    let mut prompt1 = create_prompt!(? "Text Prompt"; ["Default"]);
    let val = prompt_it!(prompt1; events stdout)?;
    writeln!(stdout, "{:?}", val).unwrap();

    let mut prompt = create_prompt!(? "Bool Prompt"; ["yes"] Bool);
    let val = prompt_it!(prompt; events stdout)?;
    writeln!(stdout, "{:?}", val).unwrap();

    let mut prompt = create_prompt!(? "Int Prompt"; Float);
    let val = prompt_it!(prompt; events stdout)?;
    writeln!(stdout, "{:?}", val).unwrap();

    Ok(())
}
