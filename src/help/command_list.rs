use serenity::framework::standard::{CommandGroup, Command};

#[derive(Debug)]
pub enum CommandSearchResult<'a> {
    RootCommandNotFound(&'a str),
    SubCommandNotFound(&'static Command, &'a str),
    NotParentCommand(&'static Command),
    Fond(&'static Command),
}

#[derive(Debug, Default)]
pub struct CommandList {
    roots: Vec<&'static Command>,
    commands: Vec<&'static Command>,
    parents: Vec<(&'static Command, Option<&'static Command>)>,
}

impl CommandList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_group(&mut self, group: &'static CommandGroup) {
        let options = group.options;

        assert!(!options.owners_only);

        for command in options.commands {
            self.parents.push((command, None));
        }

        // 優先度の高いものを後ろにいれるためにrev()
        self.roots.extend(options.commands.iter().rev());

        let mut stack: Vec<_> = options.commands.iter().collect();
        while let Some(command) = stack.pop() {
            let options = command.options;

            assert!(options.desc.is_some());
            if options.sub_commands.is_empty() {
                assert!(options.usage.is_some());
            }


            self.commands.push(command);

            // 優先度の高いものを後ろにいれるためにrev()
            stack.extend(options.sub_commands.iter().rev());
            for sub_command in options.sub_commands {
                self.parents.push((sub_command, Some(command)));
            }
        }
        self.commands.extend(stack);
        
    }

    pub fn roots(&self) -> &[&'static Command] {
        &self.roots
    }

    pub fn search_from_root<'a, S>(&'a self, query: &'a [S]) -> CommandSearchResult<'a> where
        S: AsRef<str>,
    {
        let mut parent = None;
        for &root in self.roots.iter() {
            if root.options.names.contains(&query[0].as_ref()) {
                parent = Some(root);
                break;
            }
        }
        
        let mut parent = match parent {
            Some(parent) => parent,
            None => return CommandSearchResult::RootCommandNotFound(query[0].as_ref()),
        };

        for query in &query[1..] {
            if parent.options.sub_commands.is_empty() {
                return CommandSearchResult::NotParentCommand(parent);
            }

            let mut found = None;

            for command in parent.options.sub_commands {
                if command.options.names.contains(&query.as_ref()) {
                    found = Some(command);
                    break;
                }
            }

            parent = match found {
                Some(command) => command,
                None => return CommandSearchResult::SubCommandNotFound(parent, query.as_ref()),
            }
        }
        CommandSearchResult::Fond(parent)
    }

    pub fn search_command(&self, query: &str) -> Vec<&Command> {
        let mut result = vec![];

        for &command in self.commands.iter() {
            if command.options.names.contains(&query) {
                result.push(command);
            }
        }

        result
    }

    pub fn parent(&self, command: &Command) -> Option<Option<&'static Command>> {
        for (key, parent) in self.parents.iter() {
            if *key == command {
                return Some(*parent);
            }
        }
        None
    }

    pub fn full_name(&self, mut command: &Command) -> Vec<&str> {
        let mut result = vec![command.options.names[0]];

        while let Some(Some(parent)) = self.parent(command) {
            command = parent;
            result.push(command.options.names[0]);
        }

        result.reverse();
        result
    }
}