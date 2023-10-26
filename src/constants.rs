use std::collections::HashMap;

use crate::terminal;

pub struct Get;

impl Get {
    pub fn aliases() -> HashMap<&'static str, &'static str> {
        HashMap::from([
            // Variables
            ("$[date]", "$[now:date]"),
            ("$[time]", "$[now:time]"),
            ("$[datetime]", "$[now:datetime]"),
            ("$[sys:arch]", "$[sys:architecture]"),
            ("$[architecture]", "$[sys:architecture]"),
            ("$[arch]", "$[sys:architecture]"),
            ("$[sys:os]", "$[sys:operating_system]"),
            ("$[operating_system]", "$[sys:operating_system]"),
            ("$[os]", "$[sys:operating_system]"),
            ("$[hostname]", "$[sys:hostname]"),
            ("$[username]", "$[sys:username]"),
            ("$[file]", "$[caller:file]"),
            ("$[line]", "$[caller:line]"),
            // Styles
            ("$[black]", "$[fg:black]"),
            ("$[red]", "$[fg:red]"),
            ("$[green]", "$[fg:green]"),
            ("$[yellow]", "$[fg:yellow]"),
            ("$[blue]", "$[fg:blue]"),
            ("$[purple]", "$[fg:purple]"),
            ("$[cyan]", "$[fg:cyan]"),
            ("$[white]", "$[fg:white]"),
            ("$[gray]", "$[fg:gray]"),
            ("$[brightred]", "$[fg:brightred]"),
            ("$[brightgreen]", "$[fg:brightgreen]"),
            ("$[brightyellow]", "$[fg:brightyellow]"),
            ("$[brightblue]", "$[fg:brightblue]"),
            ("$[brightpurple]", "$[fg:brightpurple]"),
            ("$[brightcyan]", "$[fg:brightcyan]"),
            ("$[brightwhite]", "$[fg:brightwhite]"),
            ("$[bblack]", "$[bg:black]"),
            ("$[bred]", "$[bg:red]"),
            ("$[bgreen]", "$[bg:green]"),
            ("$[byellow]", "$[bg:yellow]"),
            ("$[bblue]", "$[bg:blue]"),
            ("$[bpurple]", "$[bg:purple]"),
            ("$[bcyan]", "$[bg:cyan]"),
            ("$[bwhite]", "$[bg:white]"),
            ("$[bgray]", "$[bg:gray]"),
            ("$[bbrightred]", "$[bg:brightred]"),
            ("$[bbrightgreen]", "$[bg:brightgreen]"),
            ("$[bbrightyellow]", "$[bg:brightyellow]"),
            ("$[bbrightblue]", "$[bg:brightblue]"),
            ("$[bbrightpurple]", "$[bg:brightpurple]"),
            ("$[bbrightcyan]", "$[bg:brightcyan]"),
            ("$[bbrightcyan]", "$[bg:bbrightwhite]"),
            ("$[bold]", "$[effect:bold]"),
            ("$[dim]", "$[effect:dim]"),
            ("$[underline]", "$[effect:underline]"),
            ("$[blink]", "$[effect:blink]"),
            ("$[inverse]", "$[effect:inverse]"),
            ("$[strikethrough]", "$[effect:strikethrough]"),
            ("$[reset]", "$[effect:reset]"),
        ])
    }

    pub fn styling() -> HashMap<&'static str, &'static str> {
        HashMap::from([
            // Foreground Colors
            ("$[fg:black]", terminal::Color::BLACK),
            ("$[fg:red]", terminal::Color::RED),
            ("$[fg:green]", terminal::Color::GREEN),
            ("$[fg:yellow]", terminal::Color::YELLOW),
            ("$[fg:blue]", terminal::Color::BLUE),
            ("$[fg:purple]", terminal::Color::PURPLE),
            ("$[fg:cyan]", terminal::Color::CYAN),
            ("$[fg:white]", terminal::Color::WHITE),
            ("$[fg:gray]", terminal::Color::GRAY),
            ("$[fg:brightred]", terminal::Color::BRIGHT_RED),
            ("$[fg:brightgreen]", terminal::Color::BRIGHT_GREEN),
            ("$[fg:brightyellow]", terminal::Color::BRIGHT_YELLOW),
            ("$[fg:brightblue]", terminal::Color::BRIGHT_BLUE),
            ("$[fg:brightpurple]", terminal::Color::BRIGHT_PURPLE),
            ("$[fg:brightcyan]", terminal::Color::BRIGHT_CYAN),
            ("$[fg:brightwhite]", terminal::Color::BRIGHT_WHITE),
            // Background Colors
            ("$[bg:black]", terminal::Background::BLACK),
            ("$[bg:red]", terminal::Background::RED),
            ("$[bg:green]", terminal::Background::GREEN),
            ("$[bg:yellow]", terminal::Background::YELLOW),
            ("$[bg:blue]", terminal::Background::BLUE),
            ("$[bg:purple]", terminal::Background::PURPLE),
            ("$[bg:cyan]", terminal::Background::CYAN),
            ("$[bg:white]", terminal::Background::WHITE),
            ("$[bg:gray]", terminal::Background::GRAY),
            ("$[bg:brightred]", terminal::Background::BRIGHT_RED),
            ("$[bg:brightgreen]", terminal::Background::BRIGHT_GREEN),
            ("$[bg:brightyellow]", terminal::Background::BRIGHT_YELLOW),
            ("$[bg:brightblue]", terminal::Background::BRIGHT_BLUE),
            ("$[bg:brightpurple]", terminal::Background::BRIGHT_PURPLE),
            ("$[bg:brightcyan]", terminal::Background::BRIGHT_CYAN),
            ("$[bg:brightwhite]", terminal::Background::BRIGHT_WHITE),
            // Special Effects
            ("$[effect:bold]", terminal::Effect::BOLD),
            ("$[effect:dim]", terminal::Effect::DIM),
            ("$[effect:underline]", terminal::Effect::UNDERLINE),
            ("$[effect:blink]", terminal::Effect::BLINK),
            ("$[effect:inverse]", terminal::Effect::INVERSE),
            ("$[effect:strikethrough]", terminal::Effect::STRIKETHROUGH),
            ("$[effect:reset]", terminal::Effect::RESET),
        ])
    }
}
