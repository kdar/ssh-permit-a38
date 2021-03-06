use cli_flow;
use database::{Database, Host};

pub fn add(db: &mut Database, hostname: &str) {
    if db.host_get(hostname).is_some() {
        cli_flow::errorln(&format!("Hostname {} already exists", hostname));
    }

    // <= 1 char ':' allowed
    if hostname.matches(":").count() > 1 {
        cli_flow::errorln("Hostname format invalid. More than than one ':' found");
    }

    // check that port part is integer
    let host_splitted: Vec<&str> = hostname.split(':').collect();
    if host_splitted.len() == 2 {
        if !host_splitted[1].parse::<i32>().is_ok() {
            cli_flow::errorln("Hostname format invalid. Port is not a integer");
        }
    }

    // add new host
    let mut host_new = vec![
        Host {
            hostname: hostname.to_owned(),
            ..Default::default()
        },
    ];

    db.hosts.append(&mut host_new);
    cli_flow::okln(&format!("Successfully added host {}", hostname));
}

pub fn remove(db: &mut Database, hostname: &str) {
    if !db.host_get(hostname).is_some() {
        cli_flow::errorln(&format!("Hostname {} not known", hostname));
    }

    db.hosts.retain(|h| h.hostname != hostname);
    cli_flow::okln(&format!("Successfully removed host {}", hostname));
}

pub fn list(db: &mut Database, hostname_filter: &str, print_raw: bool) {
    for host in &db.hosts {
        if !hostname_filter.is_empty() && hostname_filter != host.hostname {
            continue;
        }

        if print_raw {
            println!("{:?}", host);
            continue;
        }

        println!("\n{}", host.hostname);
        println!(
            "{}",
            (0..host.hostname.len()).map(|_| "=").collect::<String>()
        );

        println!("\n## Authorized Users");
        for user in &host.authorized_users {
            println!("* {}", user);
        }

        println!("\n## Authorized Groups");
        for group in &host.authorized_user_groups {
            println!("* {}", group);
        }

        println!("");
    }
}
