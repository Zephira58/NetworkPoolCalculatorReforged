use std::{thread, time::Duration, io::Write};
use chrono::Utc;
use colour::*;

fn credits(){
    print!("{esc}c", esc = 27 as char);
    println!("Made by Xanthus");
    println!("Check out my other works at https://github.com/Xanthus58");
    println!("Email me at 'Xanthus58@protonmail.com'");
    thread::sleep(Duration::from_secs(5));
}

fn main() {
    print!("{esc}c", esc = 27 as char);
    let time = Utc::now();
    let mpayout = 0;
    let mut ModValue = 0;
    let ModReason = "";

    println!("Enter the account ID you wish yo calculate for: ");
    let mut id = String::new();
    std::io::stdin().read_line(&mut id).unwrap();
    let id = id.trim();


    let mut w1:f32=0.0;
    let mut input = String::new();

    println!("Please enter the first week of average payout:");
    std::io::stdin().read_line(&mut input).expect("Not a valid string");
    w1 = input.trim().parse().expect("Not a valid number");

    let mut w2:f32=0.0;
    let mut input = String::new();

    println!("Please enter the seccond week of average payout:");
    std::io::stdin().read_line(&mut input).expect("Not a valid string");
    w2 = input.trim().parse().expect("Not a valid number");

    let mut w3:f32=0.0;
    let mut input = String::new();

    println!("Please enter the third week of average payout:");
    std::io::stdin().read_line(&mut input).expect("Not a valid string");
    w3 = input.trim().parse().expect("Not a valid number");

    let mut w4:f32=0.0;
    let mut input = String::new();

    println!("Please enter the fourth week of average payout:");
    std::io::stdin().read_line(&mut input).expect("Not a valid string");
    w4 = input.trim().parse().expect("Not a valid number");


    let mut watts:f32=0.0;
    let mut input = String::new();
    
    println!("Please input your current Kw/h usage(watts): ");
    std::io::stdin().read_line(&mut input).expect("Not a valid string");
    watts = input.trim().parse().expect("Not a valid number");

    let mut erates:f32=0.0;
    let mut input = String::new();

    println!("Please enter your Kw/h electric costs:");
    std::io::stdin().read_line(&mut input).expect("Not a valid string");
    erates = input.trim().parse().expect("Not a valid number");
    
    let mut activity:f32=0.0;
    let mut input = String::new();

    println!("Please enter your expected mining activity per day (24 hours in one day)");
    std::io::stdin().read_line(&mut input).expect("Not a valid string");
    activity = input.trim().parse().expect("Not a valid number");

    let monthactivity = activity * 30.0;
    let monthwats = watts * monthactivity;
    let kw = monthwats/1000.0;
    let cost = erates*kw;
    let modifer = 0;

    let dmonthly = w1+w2+w3+w4;
    let mean = dmonthly/4.0;
    let weekly = mean*7.0;
    let month = weekly*4.0;
    let tax = month*0.2;
    let payout = month-tax;
    let profit = payout - cost;
    let mut mpayout = payout;

    loop{
    let Y = "Y";
    let y = "y";
    let N = "N";
    let n = "n";

    print!("{esc}c", esc = 27 as char);
    let mut ModYN = String::new();
    println!("Are there any modifiers (Y/N)");
    std::io::stdin().read_line(&mut ModYN).unwrap();

    let ModYN = ModYN.trim();
    
    if ModYN == y || ModYN == Y{
        ModValue = 1;
        print!("{esc}c", esc = 27 as char);
        let mut modifer:f32=0.0;
        let mut input = String::new();
    
        println!("Please enter the modifer: ");
        std::io::stdin().read_line(&mut input).expect("Not a valid string");
        modifer = input.trim().parse().expect("Not a valid number");

        let mut ModReason = String::new();
        println!("Whats the modifer reason: ");
        std::io::stdin().read_line(&mut ModReason).unwrap();
        let ModReason = ModReason.trim();

        println!("Type (A/S) for addition and subtraction respectfully: ");
        let mut modtype = String::new();
        let _b1 = std::io::stdin().read_line(&mut modtype).unwrap();
        let modtype = modtype.trim();

        let a = "a";
        let A = "A";
        let s = "s";
        let S = "S";

        if modtype == s || modtype == S{
            mpayout = payout - modifer;
            break
        }
        else if modtype == a || modtype == A{
            mpayout = payout - modifer;
            break
        }
        else{
            break
        }

    } 
    else if ModYN == n || ModYN == N{
        break
    }
    else{
        println!("Invalid input, please respond with 'Y' or 'N'");
        thread::sleep(Duration::from_secs(5));
    }
    }
    print!("{esc}c", esc = 27 as char);
    println!("{}", time);
    println!("{} has made", id);
    green!("${} \n", payout,);
    if ModValue == 1{
    println!("Modified Payout: ");
    green!("${} \n", mpayout);
    println!("Modifier reason: ");
    println!("{}", ModReason); // Broken and needs work
    println!("Modifer amount: ");
    println!("{}", modifer); // Broken and needs work
    }
    println!("Estimated electric bill: ");
    red!("${} \n", cost);
    println!("Estimated profit margin: ");
    green!("${} \n", profit);

    //Plan to add file writting features here

    let mut privkey = String::new();
    std::io::stdin().read_line(&mut privkey).unwrap();
    let privkey = privkey.trim();
    loop{
        if privkey == "3121"{
            println!("Tax Collected: ");
            green!("${}",tax);
            thread::sleep(Duration::from_secs(5));
            break
        }
        else{
            break
        }
    }
    credits();
}