//TODO: Add modfier calculation and rendering
//TODO: Add recipt file generation
//TODO: Add hidden tax rendering
//TODO: Bug testing
//TODO: Change default values back to 0 for release

#![allow(unused_variables)]

use eframe::egui;
use eframe::egui::Visuals;
//use std::{fs, io::Write};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "NetworkPoolCalculatorReforged",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    name: String,
    modifer_check: bool,
    w1: f32,
    w2: f32,
    w3: f32,
    w4: f32,
    watts: f32,
    e_rates: f32,
    activity: f32,
    mod_reason: String,
    mod_value: u32,
    mod_payout: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Xanthus".to_string(),
            modifer_check: false,
            w1: 1.0,
            w2: 1.0,
            w3: 1.0,
            w4: 1.0,
            watts: 320.0,
            e_rates: 0.2,
            activity: 24.0,
            mod_reason: "Dev test".to_string(),
            mod_value: 5,
            mod_payout: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().visuals = Visuals::dark(); // Makes the buttons dark
            ctx.set_visuals(egui::Visuals::dark()); // Make the ui dark

            ui.horizontal(|ui| {
                ui.label("Enter the account holder username:");
                let id = ui.text_edit_singleline(&mut self.name);
            });

            ui.label("\nEnter the average weekly payouts");
            let w1 = ui.add(egui::DragValue::new(&mut self.w1));
            let w2 = ui.add(egui::DragValue::new(&mut self.w2));
            let w3 = ui.add(egui::DragValue::new(&mut self.w3));
            let w4 = ui.add(egui::DragValue::new(&mut self.w4));

            ui.horizontal(|ui| {
                ui.label("\nEnter your pools current wattage");
                let watts = ui.add(egui::DragValue::new(&mut self.watts));
            });

            ui.horizontal(|ui| {
                ui.label("Enter your current kw/h price");
                let e_rates = ui.add(egui::DragValue::new(&mut self.e_rates));
            });

            ui.horizontal(|ui| {
                ui.label("Enter the average daily activity level in hours");
                let activity = ui.add(egui::Slider::new(&mut self.activity, 0.0..=24.0));
            });

            let checkbox = ui.checkbox(&mut self.modifer_check, "Are there any modifiers?");

            if checkbox.clicked() {
                self.modifer_check = true;
                println!("Modcheck: {}", self.modifer_check);
            };
            if checkbox.secondary_clicked() {
                self.modifer_check = false;
                println!("Modcheck: {}", self.modifer_check);
            }

            let month_activity = self.activity * 30.0;
            let month_wats: f32 = self.watts * month_activity;
            let kw = month_wats / 1000.0;
            let cost = self.e_rates * kw;
            let define_monthly = self.w1 + self.w2 + self.w3 + self.w4;
            let mean = define_monthly / 4.0;
            let weekly = mean * 7.0;
            let month = weekly * 4.0;
            let tax = month * 0.2;
            let mut payout = month - tax;
            let profit = payout - cost;
            self.mod_payout = payout;
            
            let mod_payout = self.mod_payout;
            if self.modifer_check {
                ui.label("Enter the modifier reason");
                let mod_reason = ui.text_edit_singleline(&mut self.mod_reason);
                ui.label("Enter the modifier value");
                let mod_value = ui.add(egui::DragValue::new(&mut self.mod_value));

                if ui.button("Subtract").clicked() {
                    let mod_payout = payout - self.mod_value as f32;
                    println!("Modifier payout:  -{}", self.mod_payout);
                };
                if ui.button("Add").clicked() {
                    let mod_payout = payout + self.mod_value as f32;
                    println!("Modifier payout: +{}", self.mod_payout);
                };
            }

            egui::ScrollArea::vertical().show(ui, |ui| { //Adds a scrollbar to anything nested in here
            ui.separator();
            ui.label(format!("\nAccount Holder: {:?}", self.name));
            ui.label(format!("Wattage: {:?}", self.watts));
            ui.label(format!("Eletricity rates: {:?}", self.e_rates));
            ui.label(format!("Activity: {:?}", self.activity));
            ui.label(format!("\nMonthly: {:?}", month));
            ui.label(format!("First week: {:?}", self.w1));
            ui.label(format!("Second week: {:?}", self.w2));
            ui.label(format!("Third week: {:?}", self.w3));
            ui.label(format!("Fourth week: {:?}", self.w4));
            ui.label(format!("Mean: {:?}\n", mean));
            ui.label(format!("Monthly activity: {:?}", month_activity));
            ui.label(format!("Monthly wattage: {:?}", month_wats));
            ui.label(format!("Monthly kw: {:?}", kw));
            ui.label(format!("Monthly cost: {:?}", cost));
            ui.label(format!("Define monthly payouts: {:?}", define_monthly));
            ui.label(format!("Mean monthly payouts: {:?}", mean));
            ui.label(format!("Weekly payouts: {:?}", weekly));
            ui.label(format!("Monthly payouts: {:?}", month));
            ui.label(format!("Tax: {:?}", tax));
            ui.label(format!("Payout: {:?}", payout));
            ui.label(format!("Profit: {:?}", profit));
            ui.separator();

            if self.modifer_check {
                ui.label(format!("Modifier reason: {:?}", self.mod_reason));
                ui.label(format!("Modifier value: {:?}", self.mod_value));
                ui.label(format!("Modified payout: {:?}", mod_payout));
            }
        }); // End of scrollbar
        });
    }
}

//fn main() {
//    cls();
//
//    println!("-Enter the account ID you wish to calculate for-");
//    let mut id = String::new();
//    std::io::stdin().read_line(&mut id).unwrap();
//    let id = id.trim();
//    print!("");
//
//    let mut w1: f32 = 0.0;
//    let mut input = String::new();
//    println!("-Please enter the first week of average payout-");
//    std::io::stdin()
//        .read_line(&mut input)
//        .expect("Not a valid string");
//    w1 = input.trim().parse().expect("Not a valid number");
//    print!("");
//
//    let mut w2: f32 = 0.0;
//    let mut input = String::new();
//
//    println!("-Please enter the seccond week of average payout-");
//    std::io::stdin()
//        .read_line(&mut input)
//        .expect("Not a valid string");
//    w2 = input.trim().parse().expect("Not a valid number");
//    print!("");
//    let mut w3: f32 = 0.0;
//    let mut input = String::new();
//
//    println!("-Please enter the third week of average payout-");
//    std::io::stdin()
//        .read_line(&mut input)
//        .expect("Not a valid string");
//    w3 = input.trim().parse().expect("Not a valid number");
//    print!("");
//    let mut w4: f32 = 0.0;
//    let mut input = String::new();
//
//    println!("-Please enter the fourth week of average payout-");
//    std::io::stdin()
//        .read_line(&mut input)
//        .expect("Not a valid string");
//    w4 = input.trim().parse().expect("Not a valid number");
//    print!("");
//
//    let mut watts: f32 = 0.0;
//    let mut input = String::new();
//
//    println!("-Please input your current Kw/h usage(watts)-");
//    std::io::stdin()
//        .read_line(&mut input)
//        .expect("Not a valid string");
//    watts = input.trim().parse().expect("Not a valid number");
//    print!("");
//    let mut e_rates: f32 = 0.0;
//    let mut input = String::new();
//
//    println!("-Please enter your Kw/h electric costs-");
//    std::io::stdin()
//        .read_line(&mut input)
//        .expect("Not a valid string");
//    e_rates = input.trim().parse().expect("Not a valid number");
//    print!("");
//    let mut activity: f32 = 0.0;
//    let mut input = String::new();
//
//    println!("-Please enter your expected mining activity per day (In hourly rate IE: 14, 5, 24)-");
//    std::io::stdin()
//        .read_line(&mut input)
//        .expect("Not a valid string");
//    activity = input.trim().parse().expect("Not a valid number");
//
//    let month_activity = activity * 30.0;
//    let month_wats = watts * month_activity;
//    let kw = month_wats / 1000.0;
//    let cost = e_rates * kw;
//    let modifer = 0;
//
//    let define_monthly = w1 + w2 + w3 + w4;
//    let mean = define_monthly / 4.0;
//    let weekly = mean * 7.0;
//    let month = weekly * 4.0;
//    let tax = month * 0.2;
//    let payout = month - tax;
//    let profit = payout - cost;
//    let mut mpayout = payout;
//
//    let mut modifer: f32 = 0.0;
//    let mut mod_reason = String::new();
//    let mut mod_value = 0;
//    let mut add_sub = "";
//    loop {
//        cls();
//        let mut mod_yn = String::new();
//        println!("-Are there any modifiers (Y/N)-");
//        std::io::stdin().read_line(&mut mod_yn).unwrap();
//        let mod_yn = mod_yn.trim();
//
//        if mod_yn == "y" || mod_yn == "Y" {
//            mod_value = 1;
//            cls();
//
//            let mut input = String::new();
//            println!("-Please enter the modifer-");
//            std::io::stdin()
//                .read_line(&mut input)
//                .expect("Not a valid string");
//            modifer = input.trim().parse().expect("Not a valid number");
//            print!("");
//
//            println!("-Whats the modifer reason-");
//            std::io::stdin().read_line(&mut mod_reason).unwrap();
//            mod_reason = mod_reason.trim().to_string();
//            print!("");
//
//            println!("-Type (A/S) for addition and subtraction respectfully-");
//            let mut modtype = String::new();
//            let _b1 = std::io::stdin().read_line(&mut modtype).unwrap();
//            let modtype = modtype.trim();
//
//            if modtype == "s" || modtype == "S" {
//                add_sub = "Subtraction";
//                mpayout = payout - modifer;
//                break;
//            } else if modtype == "a" || modtype == "A" {
//                add_sub = "Addition";
//                mpayout = payout + modifer;
//                break;
//            } else {
//                break;
//            }
//        } else if mod_yn == "n" || mod_yn == "N" {
//            break;
//        } else {
//            println!("Invalid input, please respond with 'Y' or 'N'");
//            get_input();
//        }
//    }
//    cls();
//    println!("-{} has made-", id);
//    println!("${} \n", payout,);
//    print!("");
//    if mod_value == 1 {
//        println!("-Modified Payout-");
//        println!("${} \n", mpayout);
//        println!("-Modifier reason-");
//        println!("{}", mod_reason);
//        println!("-Modifer amount-");
//        println!("${}", modifer);
//        println!("-Modifer Type-");
//        println!("{}", add_sub);
//        print!("");
//    }
//    println!("-Estimated electric bill-");
//    println!("${} \n", cost);
//    println!("-Estimated profit margin-");
//    println!("${} \n", profit);
//
//    let p = payout.to_string();
//    let mut file = std::fs::File::create(id).expect("create failed");
//    file.write_all("\n-".as_bytes()).expect("write failed");
//    file.write_all(id.as_bytes()).expect("write failed");
//    file.write_all(" Has Made-\n$".as_bytes())
//        .expect("write failed");
//    file.write_all(p.as_bytes()).expect("write failed");
//    file.write_all("\n".as_bytes()).expect("write failed");
//    if mod_value == 1 {
//        let mp = mpayout.to_string();
//        let m = modifer.to_string();
//        file.write_all("\n-Modified Payout-\n$".as_bytes())
//            .expect("write failed");
//        file.write_all(mp.as_bytes()).expect("write failed");
//        file.write_all("\n-Modifer Reason-\n".as_bytes())
//            .expect("write failed");
//        file.write_all(mod_reason.as_bytes()).expect("write failed");
//        file.write_all("\n-Modifer Amount-\n$".as_bytes())
//            .expect("write failed");
//        file.write_all(m.as_bytes()).expect("write failed");
//        file.write_all("\n-Modifer Type-\n".as_bytes())
//            .expect("write failed");
//        file.write_all(add_sub.as_bytes()).expect("write failed");
//        file.write_all("\n".as_bytes()).expect("write failed");
//    }
//    let c = cost.to_string();
//    let ep = profit.to_string();
//    file.write_all("\n-Estimated electric bill-\n$".as_bytes())
//        .expect("write failed");
//    file.write_all(c.as_bytes()).expect("write failed");
//    file.write_all("\n-Estimated profit margin-\n$".as_bytes())
//        .expect("write failed");
//    file.write_all(ep.as_bytes()).expect("write failed");
//
//    print!("");
//    println!("Recipt file created.");
//
//    let dir = "Recipts";
//    fs::create_dir_all(dir).unwrap();
//    std::fs::rename(id, format!("{dir}/{id}.txt")).unwrap();
//
//    let mut privkey = String::new();
//    std::io::stdin().read_line(&mut privkey).unwrap();
//    let privkey = privkey.trim();
//    if privkey == "3121" {
//        println!("-Tax Collected-");
//        println!("${}\n", tax);
//        get_input();
//    }
//    credits();
//}
//
