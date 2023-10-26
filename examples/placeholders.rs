extern crate tangra;

use tangra::levels::Level;

fn main() {
    let mut logger = tangra::Logger::new();
    logger.println("$[bold]$[underline]Styling:");
    logger.println("$[bold]Foreground colors:");
    logger.println("$[fg:black]Black$[reset]\t$[fg:red]Red$[reset]\t\t$[fg:green]Green$[reset]\t\t$[fg:yellow]Yellow$[reset]\t\t$[fg:blue]Blue$[reset]\t\t$[fg:purple]Purple$[reset]\t\t$[fg:cyan]Cyan$[reset]\t\t$[fg:white]White");
    logger.println("$[fg:gray]Gray$[reset]\t$[fg:brightred]Bright red$[reset]\t$[fg:brightgreen]Bright green$[reset]\t$[fg:brightyellow]Bright yellow\t$[fg:brightblue]Bright blue$[reset]\t$[fg:brightpurple]Bright purple$[reset]\t$[fg:brightcyan]Bright cyan$[reset]\t$[fg:brightwhite]Bright white");
    logger.println("$[bold]Background colors:");
    logger.println("$[bg:black]Black$[reset]\t$[bg:red]Red$[reset]\t\t$[bg:green]Green$[reset]\t\t$[bg:yellow]Yellow$[reset]\t\t$[bg:blue]Blue$[reset]\t\t$[bg:purple]Purple$[reset]\t\t$[bg:cyan]Cyan$[reset]\t\t$[bg:white]White");
    logger.println("$[bg:gray]Gray$[reset]\t$[bg:brightred]Bright red$[reset]\t$[bg:brightgreen]Bright green$[reset]\t$[bg:brightyellow]Bright yellow\t$[bg:brightblue]Bright blue$[reset]\t$[bg:brightpurple]Bright purple$[reset]\t$[bg:brightcyan]Bright cyan$[reset]\t$[bg:brightwhite]Bright white");
    logger.println("$[bold]Special Effects:");
    logger.println("$[effect:bold]Bold$[reset]\t$[effect:dim]Dim$[reset]\t\t$[effect:underline]Underline$[reset]\t$[effect:blink]Blink$[reset]\t\t$[effect:inverse]Inverse$[reset]\t\t$[effect:strikethrough]Strikethrough$[reset]");

    logger.println("\n$[bold]$[underline]Variables:");
    logger.println("$[bold]Caller:");
    logger.println("Function: $[caller:function]\tShort function: $[caller:shortfunction]\t\tFile: $[caller:file]\t\t\t\tLine: $[caller:line]");

    logger.println("$[bold]Logging Level:");
    logger.set_logging_level(Level::FATAL);
    logger.println("Level Color: $[level:color]Color$[reset]\tLevel Name: $[level:name]\t\tLevel Short Name: $[level:shortname]");
    logger.set_logging_level(Level::NONE);
    logger.println("$[bold]Date & Time Now:");
    logger.println("Date: $[now:date]\tTime: $[now:time]\t\t\tDate & Time: $[now:datetime]");
    logger.println("$[bold]System:");
    logger.println("Architecture: $[sys:architecture]\tHostname: $[sys:hostname]\tOperating System: $[sys:operating_system]\t\tUsername: $[username]");
}
