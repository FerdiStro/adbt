use regex::Regex;

/**
All Commands as Enums. After adding new command need to add to other functions to make it fully work.
Also add it in render_header to the CommandLineState
 **/
pub enum CommandMatcher {
    NULL,
    QUIT,
    TEST,
}

pub struct Command {
    pub command_matcher: CommandMatcher,
}

impl Command {
    pub fn default() -> Self {
        Self {
            command_matcher: CommandMatcher::NULL,
        }
    }

    pub fn get_command_matcher(&self, command: &str) -> CommandMatcher {
        [
            self.get_command_matcher_helper::<2>(command, CommandMatcher::QUIT),
            // self.get_command_matcher_helper::<2>(command, CommandMatcher::TEST),
        ]
        .into_iter()
        .find_map(|m| m)
        .unwrap_or(CommandMatcher::NULL)
    }

    fn get_command_matcher_helper<const N: usize>(
        &self,
        command: &str,
        check_command: CommandMatcher,
    ) -> Option<CommandMatcher> {
        if self
            .get_command_matcher_strings(&check_command)
            .iter()
            .any(|matcher| Regex::new(matcher).unwrap().is_match(command))
        {
            Some(check_command)
        } else {
            None
        }
    }

    fn get_command_matcher_strings(
        &self,
        _command_matcher: &CommandMatcher,
    ) -> &'static [&'static str] {
        match _command_matcher {
            CommandMatcher::NULL => &[],
            CommandMatcher::QUIT => &["q", "quit"],
            CommandMatcher::TEST => &["t", "test"],
        }
    }
}
