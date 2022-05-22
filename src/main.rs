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
    let mut modifer = 0;
    print!("{esc}c", esc = 27 as char);
    let time = Utc::now();

    println!("-Enter the account ID you wish to calculate for-");
    let mut id = String::new();
    std::io::stdin().read_line(&mut id).unwrap();
    let id = id.trim();


    let mut w1:f32=0.0;
    let mut input = String::new();

    println!("-Please enter the first week of average payout-");
    std::io::stdin().read_line(&mut input).expect("Not a valid string");
    w1 = input.trim().parse().expect("Not a valid number");

    let mut w2:f32=0.0;
    let mut input = String::new();

    println!("-Please enter the seccond week of average payout-");
    std::io::stdin().read_line(&mut input).expect("Not a valid string");
    w2 = input.trim().parse().expect("Not a valid number");

    let mut w3:f32=0.0;
    let mut input = String::new();

    println!("-Please enter the third week of average payout-");
    std::io::stdin().read_line(&mut input).expect("Not a valid string");
    w3 = input.trim().parse().expect("Not a valid number");

    let mut w4:f32=0.0;
    let mut input = String::new();

    println!("-Please enter the fourth week of average payout-");
    std::io::stdin().read_line(&mut input).expect("Not a valid string");
    w4 = input.trim().parse().expect("Not a valid number");


    let mut watts:f32=0.0;
    let mut input = String::new();
    
    println!("-Please input your current Kw/h usage(watts)-");
    std::io::stdin().read_line(&mut input).expect("Not a valid string");
    watts = input.trim().parse().expect("Not a valid number");

    let mut erates:f32=0.0;
    let mut input = String::new();

    println!("-Please enter your Kw/h electric costs-");
    std::io::stdin().read_line(&mut input).expect("Not a valid string");
    erates = input.trim().parse().expect("Not a valid number");
    
    let mut activity:f32=0.0;
    let mut input = String::new();

    println!("-Please enter your expected mining activity per day (24 hours in one day)-");
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

    let mut modifer: f32 = 0.0;
    let mut mod_reason = String::new();
    let mut mod_value = 0;
    let mut add_sub = "";
    loop{
    let Y = "Y";
    let y = "y";
    let N = "N";
    let n = "n";

    print!("{esc}c", esc = 27 as char);
    let mut mod_yn = String::new();
    println!("-Are there any modifiers (Y/N)-");
    std::io::stdin().read_line(&mut mod_yn).unwrap();
    let mod_yn = mod_yn.trim();

    
    if mod_yn == "y" || mod_yn == "Y" {
        mod_value = 1;
        print!("{esc}c", esc = 27 as char);

        let mut input = String::new();
        //I'm attempting to get the input from the user to define a modifer reason and value
        println!("-Please enter the modifer-");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Not a valid string");
        modifer = input.trim().parse().expect("Not a valid number");

        println!("-Whats the modifer reason-");
        std::io::stdin().read_line(&mut mod_reason).unwrap();
        mod_reason = mod_reason.trim().to_string();
        println!("-Type (A/S) for addition and subtraction respectfully-");
        let mut modtype = String::new();
        let _b1 = std::io::stdin().read_line(&mut modtype).unwrap();
        let modtype = modtype.trim();

        if modtype == "s" || modtype == "S"{
            add_sub = "Subtraction";
            mpayout = payout - modifer;
            break
        }
        else if modtype == "a" || modtype == "A"{
            add_sub = "Addition";
            mpayout = payout - modifer;
            break
        }
        else{
            break
        }

    } 
    else if mod_yn == n || mod_yn == N{
        break
    }
    else{
        println!("-Invalid input, please respond with 'Y' or 'N'-");
        thread::sleep(Duration::from_secs(5));
    }
    }
    print!("{esc}c", esc = 27 as char);
    println!("{}", time);
    println!("-{} has made-", id);
    green!("${} \n", payout,);
    print!("\n");
    if mod_value == 1{
    println!("-Modified Payout-");
    green!("${} \n", mpayout);
    println!("-Modifier reason-");
    println!("{}", mod_reason);
    println!("-Modifer amount-");
    println!("${}", modifer);
    println!("-Modifer Type-");
    println!("{}",add_sub);
    }
    print!("\n");
    println!("-Estimated electric bill-");
    red!("${} \n", cost);
    println!("-Estimated profit margin-");
    green!("${} \n", profit);

    //Plan to add file writting features here

    let t = time.to_string();
    let p = payout.to_string();
    let mut file = std::fs::File::create(id).expect("create failed");
    file.write_all(t.as_bytes()).expect("write failed");
    file.write_all("\n-".as_bytes()).expect("write failed");
    file.write_all(id.as_bytes()).expect("write failed");
    file.write_all(" Has Made-\n$".as_bytes()).expect("write failed");
    file.write_all(p.as_bytes()).expect("write failed");
    file.write_all("\n".as_bytes()).expect("write failed");
    if mod_value == 1{
        let mp = mpayout.to_string();
        let m = modifer.to_string();
        file.write_all("\n-Modified Payout-\n$".as_bytes()).expect("write failed");
        file.write_all(mp.as_bytes()).expect("write failed");
        file.write_all("\n-Modifer Reason-\n".as_bytes()).expect("write failed");
        file.write_all(mod_reason.as_bytes()).expect("write failed");
        file.write_all("\n-Modifer Amount-\n$".as_bytes()).expect("write failed");
        file.write_all(m.as_bytes()).expect("write failed");
        file.write_all("\n-Modifer Type-\n".as_bytes()).expect("write failed");
        file.write_all(add_sub.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }
    let c = cost.to_string();
    let ep = profit.to_string();
    file.write_all("\n-Estimated electric bill-\n$".as_bytes()).expect("write failed");
    file.write_all(c.as_bytes()).expect("write failed");
    file.write_all("\n-Estimated profit margin-\n$".as_bytes()).expect("write failed");
    file.write_all(ep.as_bytes()).expect("write failed");
    print!("\n");
    println!("Recipt file created." );

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