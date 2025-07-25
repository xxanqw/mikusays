// mikuart now shipped as module for better code organization
// all ASCII art were obtained from https://emojicombos.com/miku-ascii-art

use rand::Rng;

pub fn get_miku_art() -> Vec<&'static str> {
    let mut rng = rand::rng();
    let styles = vec![
        //MARK: small art
        vec![
            "⠀⠀⠀⠀⠀⠀⠀⠀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣷⣀⡤⠤⠤⠤⠤⢤⣄⣀⡀⠀⠀⠀⣀⣀⡀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⢀⣾⣿⣟⡵⠚⠉⠀⠤⠂⠀⠀⠀⠀⠀⠀⠉⠓⠦⣾⣿⣿⣿⡄⠀⠀⠀⠀",
            "⠀⠀⠀⣴⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⣄⠀⠀⠀⠘⢦⡙⢷⣜⣿⣿⣆⠀⠀⠀",
            "⠀⠀⢺⣿⣿⡟⠁⠀⢀⠀⠀⡆⠀⠀⠀⠀⠀⠈⢣⡀⠀⠀⠀⠙⡼⡍⡘⣿⣿⣆⠀⠀",
            "⠀⠀⠀⢹⡟⠀⠀⢀⠇⢀⡼⡇⠀⠀⠀⣤⡀⠀⠀⢱⡀⠀⠀⠀⠸⣿⡴⡘⣿⣿⣆⠀",
            "⠀⠀⠀⣼⡅⠀⠀⣘⢀⡾⢥⣿⣰⠀⠀⡟⢷⡀⢷⣄⢷⡀⠀⠀⠀⢻⡞⣧⣿⡿⠋⠀",
            "⠀⠀⢠⡿⡇⠀⠀⣽⣾⣤⣤⣈⢿⠆⠀⡇⠘⠹⣿⡝⢮⣇⠀⠀⠀⢸⣿⣿⡟⢱⠀⠀",
            "⠀⠀⢸⠀⡏⡇⠀⣿⢱⠿⣿⣻⡝⢿⣄⢳⢐⣶⣾⣷⣾⣿⠀⠀⡇⠘⣿⠉⡄⠸⡀⠀",
            "⠀⠀⡏⢸⣧⣷⠀⣿⠸⢯⣉⡾⠁⠈⠻⣾⡜⣗⠿⣭⡏⣿⠀⢀⣧⡀⣿⡄⡇⠈⡇⠀",
            "⠀⢠⠇⠸⢱⡟⣆⣿⣄⠀⠀⠀⠀⠀⠀⠈⠁⠑⠖⠚⠀⣾⠀⣼⣿⠙⠇⡇⠇⠀⢇⠀",
            "⠀⢸⠀⠀⢸⠀⠘⢾⣿⣗⣦⣄⣀⠰⠤⠄⠀⠀⣀⣠⢴⢧⠾⠿⠃⠀⠀⡇⠀⠀⢸⠀",
            "⠀⡏⢀⠀⢸⠀⠀⠀⠀⠙⠿⣣⣾⣭⢿⣫⡿⠻⡍⠠⠟⠁⠀⠀⠀⠀⠀⡇⢀⠀⢸⠀",
            "⢠⠁⢸⠀⢸⠀⠀⠀⠀⢀⣴⣿⣧⢸⢿⠇⣵⣼⣿⡄⠀⠀⠀⠀⠀⠀⠀⡇⡎⠀⠘⡇",
            "⢸⠀⠸⡄⡇⠀⠀⠀⣠⣾⣿⣿⡏⣸⣸⠈⠀⢹⣿⣿⣆⠀⠀⠀⠀⠀⠀⡿⡇⠀⠀⡇",
            "⡄⠀⠀⣇⢰⠀⠀⠸⢿⣿⣿⢞⠇⣟⢸⠅⡆⠘⣿⣿⣿⣧⠀⠀⠀⠀⠀⣇⡇⠀⠀⢠",
            "⢁⢆⠀⠸⣼⠀⠀⠘⠋⢩⣾⣧⣶⣿⣿⣆⣼⣿⣿⣿⣻⠿⡄⠀⠀⠀⠀⣿⠁⠐⠀⢸",
            "⠘⡌⣆⠀⢻⡆⠀⠀⠀⠘⠫⣟⣿⣿⣿⣿⣿⣻⣯⠟⠉⠋⠁⠀⠀⠀⢠⡏⢀⠷⠀⢸",
            "⠀⠙⢿⣗⢔⢷⡀⠀⠀⠀⠀⢸⣶⣶⣿⢳⣶⣖⡇⠀⠀⠀⠀⠀⠀⠀⡼⢀⢎⡇⢠⡇",
            "⠀⠀⠀⠙⠻⠲⠽⠄⠀⠀⠀⠀⣿⣿⣿⢸⣿⣿⡇⠀⠀⠀⠀⠀⢀⣞⢔⣃⣼⡴⠋⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢾⡻⠿⢨⡿⣿⡇⠀⠀⠀⠀⠀⠉⠉⠉⠉⠁⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        ],
        vec![
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣾⣿⠷⠒⠛⠉⠛⠓⠦⣤⣶⣳⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⣠⠖⢦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣻⠽⠀⢀⣠⡔⣦⣄⠀⠰⣟⢻⡁⠉⢳⡄⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠸⠉⠙⠲⢷⣄⡈⠳⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⡿⠁⣠⠖⠋⠘⣇⡇⠈⠳⡄⠘⢷⣷⠀⠀⢹⡄⠀⠀⠀⠀⠀⠀⠀",
            "⠨⠷⣶⡦⢄⡈⠑⢦⡈⢧⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⡿⢡⡼⣓⠁⠀⠀⢿⣿⠒⠢⢽⡄⠀⠸⣧⠀⠀⢻⡀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠉⠙⠒⠯⢤⣽⡀⠉⠲⣄⠀⠀⠀⠀⠀⠀⠀⡼⢹⠇⣾⠻⣁⡟⠀⠀⠀⠻⢾⣉⡾⣷⠠⢐⣿⡆⠂⠀⢳⡀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠑⠦⡀⠑⢄⠀⠀⢀⣀⡼⢁⣾⣿⡏⠀⠀⢠⠤⣤⣀⡀⠀⠉⠀⢸⣼⣿⣿⣿⣷⠸⡀⠈⢧⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠳⣄⠳⣾⣿⣿⣷⣼⡭⣿⣧⡀⠀⡼⠀⠀⢸⠀⠀⠀⢀⣼⣿⣿⠟⢻⡀⣧⠀⠘⣇⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⣇⠀⠀⢸⠀⣀⣠⣾⣿⡟⠁⠀⠈⡇⢸⡄⠀⢹⡆⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⣿⣿⡿⠉⣛⣿⣯⠿⣻⡿⠛⢿⣿⠓⠀⠀⠀⣇⠈⣇⠀⠈⣿⡀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡼⠉⣽⡹⣏⣟⣻⣿⣷⢴⡿⢩⣿⡞⠹⡄⠀⠀⣹⣇⠀⠀⠀⣟⠀⢿⠀⢃⠸⣧⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⠃⢰⣯⢳⡝⣦⢻⣿⢏⡾⠁⢸⠀⠷⢶⠿⣿⣿⣿⣿⠀⠀⢀⣏⠰⠌⠀⠸⡐⡽⡄⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡞⠀⣾⢧⡻⣜⡧⠟⠛⣾⠁⢀⡿⠒⠎⢦⢹⣿⣿⣿⣿⣧⠀⢰⡇⠨⢀⡐⠀⡇⣝⣳⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠃⢀⡿⣧⣛⣾⢿⡄⢀⣽⣠⣏⠀⠢⠄⡈⢺⣿⣿⣿⣿⣿⡆⣸⠃⠀⡀⢧⠀⢸⢰⠽⡆",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⢠⠊⣹⢧⢯⡽⣮⢿⣿⣿⣿⣿⣧⣀⣤⣴⣿⣿⣯⣿⢿⣿⣿⡿⠀⠁⡀⢸⠆⢸⣹⢚⣷",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣯⡏⠀⣿⣛⡮⣗⠿⣦⣽⠛⠿⣿⣿⣿⣿⣿⢻⣿⣷⣿⣿⣿⣿⢃⡀⠁⡀⢸⢣⠸⡜⣷⢺",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⠀⢸⢸⡿⡼⣭⡻⣵⡿⣯⣙⢚⣧⣴⠒⣋⣡⣴⣟⠳⠴⢿⣿⣾⠀⠠⠀⡼⢣⢸⡕⣻⣿",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡄⢸⡇⠏⣷⡳⡽⢶⡟⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠘⠀⣯⡄⠀⠄⣏⣛⢦⠽⣱⣟",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢷⡘⣿⣄⢹⣳⡽⠋⠀⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠹⣶⠀⢸⢣⡝⢮⡹⣥⡟",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢷⣼⡹⢾⠏⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⠁⠀⠀⠀⠀⠙⢧⣜⡳⣚⢧⢳⡽⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠻⢿⠂⠀⠀⠀⠈⣿⣿⣿⣿⣿⣿⣿⣿⠏⠀⠀⠀⠀⠀⠀⠀⢹⢶⡙⣮⠟⠁⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⠿⢛⡿⠿⠿⠋⠀⠀⠀⠀⠀⠀⠀⠀⢸⡧⠟⠁⠀⠀⠀",
        ],
        vec![
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣀⣀⠀⠀⣀⡠⠴⠒⠚⠉⠉⠓⠒⠦⣄⣶⠒⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⡷⢬⣉⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠠⡌⠻⣧⢻⣧⣤⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣖⠗⡋⢹⠀⠀⢰⡄⠀⠀⢸⣷⡀⠀⣠⠽⣆⢼⣇⢻⣸⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡜⣡⣶⢋⡏⠙⢢⣏⣇⠀⠀⠈⣇⡵⡏⠀⠀⢹⡏⢾⣿⠃⢿⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⢿⢻⣏⣿⡇⡄⣾⠀⠹⡄⠄⠀⡇⠀⠹⣤⠈⠹⣿⣾⢸⠀⢘⣷⣄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣴⣯⣿⣽⣿⣷⢸⡗⠦⣄⡹⣼⣄⣿⣴⠛⠹⡄⡇⣿⣿⠾⠚⢹⢿⢽⣽⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⣿⣞⣾⣿⢿⣯⢻⢻⡴⠞⠁⠈⠻⣿⣌⡉⠓⣿⣰⡿⠀⠀⠀⠸⡜⡾⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⢀⡴⣡⠊⢸⣹⠁⠈⠙⣾⡄⠁⠀⢰⠛⠉⠉⠉⢳⣀⣿⣿⠃⠀⠀⣀⣀⣧⣿⡞⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⣠⠋⡴⠁⠀⠸⢿⣤⣤⣤⣹⣿⣷⣶⣾⣷⣶⣶⣺⣋⣽⣿⣷⠶⠟⠛⠋⢧⠀⠀⠸⡜⣷⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⢀⡜⠁⡰⠁⠀⠀⢠⡿⠀⠀⠀⠉⠉⠉⠙⢻⡟⣹⣿⠃⣿⠋⠁⠀⠀⠀⠀⠀⠸⡄⠀⠀⢣⠹⣧⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⢠⠏⡀⢠⠇⠀⠀⢠⡿⠁⠀⠀⠀⠀⣤⣶⡴⠚⢻⠡⣸⠀⢹⣆⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠸⡄⢻⣇⠀⠀⠀⠀⠀⠀",
            "⠀⠀⢀⡏⣼⠁⢸⠀⠀⠀⣾⠃⠀⠀⠀⠀⠀⢻⣿⣧⣀⣬⠋⠁⠀⣠⣿⣶⣆⠀⠀⠀⠀⠀⡇⠀⠀⠀⡇⠈⣿⡀⠀⠀⠀⠀⠀",
            "⠀⠀⣸⣸⣿⠀⡇⠀⢰⣸⡟⠀⠀⠀⣀⣠⠴⠚⣟⣻⣧⣯⣗⣤⣾⣿⣿⡿⠋⠀⠀⠀⠀⣸⣤⠀⠀⠀⡇⡆⢻⠃⠀⠀⠀⠀⠀",
            "⠀⠀⣿⡿⢸⡀⣇⠀⣸⣿⡁⠀⣾⣻⡁⣀⣤⣶⠟⠋⠉⠛⢿⣋⣻⡏⠉⠀⠀⠀⠀⠀⢰⣿⡇⠀⠀⠀⣷⡇⣸⡄⠀⠀⠀⠀⠀",
            "⠀⠀⠿⠇⠀⢧⢸⠀⣿⡿⠇⠀⠈⠛⠛⠋⠉⠀⠀⠀⠀⠀⡟⠀⣿⠇⠀⠀⠀⠀⠀⢠⣿⣿⡇⠀⠀⣰⡿⣧⣿⠃⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⢿⣄⣹⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⡇⠀⣿⠀⠀⠀⠀⠀⠀⣸⡿⢸⠁⢠⣾⠋⢰⣿⡏⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠉⠛⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⣶⣶⡿⠀⠀⠀⠀⠀⠀⠉⠁⢸⣶⡟⠁⠀⠾⠟⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        ],
        vec![
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡖⣉⡬⣷⣦⠶⠶⠞⠙⡦⠖⠒⠒⠒⢤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡤⢤⣠⠔⢦⡀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⡞⢉⣩⢯⡴⠃⢀⡴⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠓⢦⡀⠀⠀⠀⠀⠀⠀⠀⠻⡀⠀⠁⠀⢀⡇⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡜⡿⠉⠲⣵⠘⡆⠴⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⢦⡀⠀⠀⣀⣀⣀⠀⠙⣆⠀⡠⠎⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⢀⡤⠞⠓⠓⠲⢄⡘⠃⠹⡄⠀⠀⠀⢀⡀⠀⠀⠀⡀⠀⠀⠀⢀⠀⠀⠈⢳⡀⠹⡄⢀⡇⢀⡞⠀⠀⠈⡏⠁⠀⠀⠀⠀",
            "⠀⠀⠀⠀⢀⡴⠋⠀⠀⠀⠀⣀⡤⠽⠂⠀⠧⡄⣀⢀⡏⣇⠀⠀⢀⡿⣄⠀⢀⠈⣇⠀⠀⠀⠙⡆⢁⡟⢁⡞⠀⢀⡀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⣠⠞⠁⠀⠀⠀⠀⠀⠘⢧⡤⠖⢒⠿⡆⠹⡬⠻⣦⡘⢆⠀⡼⠀⣈⡽⠫⣀⠈⡆⠀⠀⠀⠓⡼⢀⡿⠚⠉⠉⢹⠀⠀⠀⠀⠀⠀⠀",
            "⠀⢰⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡖⣱⠁⠱⡞⠱⡤⣄⡉⠀⠹⡇⠀⠁⢀⣠⠎⠙⢷⠀⠀⠀⢠⠃⠀⠀⠀⣠⠤⠚⠃⠀⠀⠀⠀⠀⠀",
            "⠀⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠏⠀⡇⠀⢀⠟⡆⠳⡄⢙⣦⠀⠀⠀⠾⣍⡀⠀⠀⢸⡀⠀⢀⡏⠀⠀⡴⠋⠈⠉⠓⠦⡀⠀⠀⠀⠀⠀",
            "⢸⠁⢀⡴⠂⠀⠀⠀⠀⡀⠀⡏⠀⢰⢁⡔⠋⠀⢳⠀⠙⡍⢰⠒⠚⡇⠀⠀⠙⠂⠀⣼⠃⣠⡜⠀⠀⡞⠁⠀⠀⠀⠀⠀⠈⠳⡄⠀⠀⠀",
            "⣧⢞⡏⠀⠀⠀⠀⠀⡼⣇⠀⡇⠀⢸⠎⠀⡤⠖⠒⡇⠀⢳⠈⠓⠚⠁⠀⠀⠀⢀⡤⠿⠋⣰⠃⠀⣜⠁⠀⠀⠀⠀⠀⠀⠀⠀⠘⣆⠀⠀",
            "⠀⢸⠀⠀⠀⠀⠀⠀⡇⠈⠳⠇⠀⠀⠀⡞⠁⠀⠀⠹⡄⠈⣇⡀⠀⠀⣀⡤⠞⠉⠀⠀⢀⡇⠀⣰⠃⠙⢦⠀⠀⠀⠀⠀⣤⣄⡀⢸⠀⠀",
            "⠀⢸⡀⠀⠀⠀⠀⠀⡇⠀⠀⠀⢀⣔⣲⠟⠲⣄⣀⡠⠟⢻⢁⣏⡟⢩⡁⢀⡟⠦⣀⣀⣾⣄⣰⠃⠀⠀⠀⠱⡄⠀⠀⠀⠀⡇⠹⠃⠀⠀",
            "⠀⠀⠑⠦⣄⡀⠀⠀⣧⠀⠀⢀⠏⣷⠃⠀⠀⠀⠉⠳⣄⡞⢯⡘⡖⣆⠳⣼⠀⠀⠉⠳⣴⠃⠉⠀⠀⠀⠀⢀⡇⠀⠀⠀⠀⢳⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠉⠉⠉⠁⠀⠀⢸⠀⠛⠀⠀⠀⠀⠀⢀⣈⡽⣶⡵⠃⠈⠙⠺⣄⠤⠀⠀⠘⢦⡀⠀⠀⠀⠠⠟⠚⡇⠀⠀⢰⡏⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠓⠦⠤⠤⠒⠖⢊⡝⠀⠀⠀⠀⠀⠀⠀⠀⢛⠒⠦⠖⠋⠉⠀⠀⠀⠀⠀⣠⣞⡥⠤⠤⠞⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠤⡯⠤⠤⠤⠤⠤⠤⠤⠤⠤⢼⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⣇⠀⣄⠀⠀⠀⠀⠀⢀⡞⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠞⠁⠉⠉⠉⠉⢦⡞⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        ],
        vec![
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⢀⡖⠓⠶⢦⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣀⡘⣿⣿⣟⠙⢶⣄⡀⠀⠉⠳⠤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡠⠖⠛⠉⠀⠀⠀⠀⠈⠿⣿⡛⣦⠤⠤⣉⡓⠀⠀⠀⠀⠙⠓⠤⣀⡀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣶⣶⣤⠖⠁⠀⠀⠀⠀⢀⣀⣤⣄⠀⠀⢿⣴⣿⣿⣶⣤⣍⡓⠶⣤⣀⡀⠀⠀⠀⠉⠑⠒⠤⠄⢀⡀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠏⢩⣿⣧⣼⡁⠀⣀⠀⠀⢠⣤⠀⠈⠙⢯⠳⣆⠀⠹⣝⢻⢿⠛⠁⠉⠲⢌⡉⠙⠒⠂⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⠀⣸⣿⠛⠉⠉⠉⢁⠀⠀⠀⠻⠓⢄⠀⠀⠀⠈⠁⠀⠉⢿⣿⣇⠀⠀⠀⠀⠉⠳⢤⡀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⢠⡇⠀⡿⢁⠀⠀⠀⠀⠈⢣⠀⠀⢸⣆⡀⠀⠀⠀⠀⠀⠀⠙⢮⢻⣿⡄⠀⠀⠀⠀⠀⠀⠈⠑⠶⢄⣀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⡜⠀⣼⣡⣫⠏⠀⠀⠀⢀⡏⠳⣄⣎⢇⠙⠛⣦⣤⣀⠀⠀⠀⠈⣧⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠳⠤⣄",
            "⠀⠀⠀⠀⠀⠀⠀⢰⠃⢠⢻⢣⣿⠀⡀⣤⡄⢸⡇⠀⠈⠙⠾⣦⡀⠸⣿⣿⣿⡶⣦⠀⣿⣼⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈",
            "⠀⠀⠀⠀⠀⠀⢰⠋⠀⢸⣿⡏⣿⠀⣧⠹⡇⢸⣷⣶⣶⣦⠀⠀⠁⠀⠈⠉⠉⠀⢻⡆⡿⠳⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⣰⠃⠀⠀⢸⣿⠀⣿⣧⡹⣆⢻⡀⢿⠊⠻⠿⠀⠀⠀⠀⠀⡀⠀⠀⢠⣧⣧⠀⠙⠀⠀⠀⠀⠀⠀⠀⢀⣀⢀⣠⣄⠀⠀⠀",
            "⠀⠀⠀⢀⡜⠁⠀⠀⠀⠈⣇⠸⠁⡇⠳⣝⢿⣷⡌⢿⣄⠀⠀⠸⢶⣶⣿⣿⠀⢠⢻⡿⠷⢽⣦⣄⡀⠀⠀⠀⠀⢀⣿⣾⠟⠋⠉⠉⠲⡀",
            "⠀⠀⣠⠋⠀⠀⠀⠀⠀⠀⠏⠀⣸⠀⠀⠈⠓⢿⡝⣄⢻⣟⢤⡤⢄⣙⡷⢏⡴⠃⢸⣀⠀⠀⠀⠀⠀⠀⠀⠀⢘⣳⣿⣣⣴⠶⣤⣲⣄⠀",
            "⢀⡜⠃⠀⠀⠀⣀⣀⡀⠀⠀⢀⠏⠀⠀⠀⠀⠀⠙⢮⣻⡬⠛⠓⠚⠷⣿⡋⠉⠉⣿⣿⣷⡔⠒⠛⠓⢦⡀⠀⠈⣯⣿⡝⢿⣷⣼⣻⣎⣷",
            "⠎⠀⠀⠀⠀⠺⡉⠉⠙⠒⢤⣜⣀⣀⣠⣤⣤⣤⣤⣴⣿⣶⣄⠀⠀⠀⠈⢻⡒⠀⣿⣣⣘⣿⣆⠀⠀⠀⠹⣄⢸⣿⣿⣷⡈⢻⣿⣷⡿⣅",
            "⠀⠀⠀⠀⠀⠀⡉⠓⣢⢤⣄⠀⠈⠉⠉⠙⢿⣿⣿⣿⣿⣯⣿⣷⠀⠀⠀⠀⢳⢦⠉⠉⠳⣌⠛⣝⠲⢤⡀⠈⣾⣿⣿⣿⣿⣦⡙⣿⣷⣿",
            "⠀⠀⠀⣠⠇⠟⢀⣾⣵⣿⠿⠀⠀⠀⠀⠀⠘⣮⠹⣿⣿⣿⣿⣿⣀⡀⠀⠀⠈⡇⠀⠀⠀⠈⢧⡀⠱⣄⠈⠓⢾⣿⣿⣿⣿⣿⣿⣿⣿⣼",
            "⠀⢀⣴⠏⠆⢀⣾⣿⡿⠃⠀⢈⠉⠓⠒⠒⠉⠈⢳⡙⣿⣿⡟⠋⠉⠻⣧⣀⠀⣿⠀⠀⠀⠀⠀⠱⡄⠈⠳⣄⠀⢘⣿⣿⣿⣿⣿⣟⣧",
            "⢀⣿⠏⠀⣠⡾⠛⠉⣀⣀⠀⠀⠑⢜⠢⢤⣀⠀⢈⣧⢻⡟⢧⠀⠀⠀⠀⠹⣆⣿⣷⣄⠀⠀⠀⠀⠹⣦⠀⠘⢦⠘⣿⣿⣿⣿⣿⣿⣿⢿",
            "⠈⠏⢠⡞⠁⣀⣴⣾⣿⡟⠁⣀⣀⠀⢣⠤⣌⣳⣾⣿⢸⠁⠀⠳⡀⠀⠀⢠⣿⠿⣿⡛⠓⠀⠀⠀⣠⣿⡆⠀⠸⣿⣿⣿⣿⣿⣿⣿⣿⣟",
            "⠀⠀⠘⠲⠚⢉⣼⡿⠋⢀⣴⣿⣿⡇⠀⣧⣄⠈⠙⠻⣿⠀⠀⠀⠙⢶⣶⣿⣿⡄⠈⠛⢦⣄⣀⠘⠛⣻⡇⠀⢠⡏⣿⣿⣿⣿⣿⣿⣿⣿",
            "⠀⠀⠀⠀⣰⡟⠹⠁⣠⣾⣁⠘⣿⡀⠀⣿⣿⣿⣷⣦⣾⠀⠀⠀⠀⠀⠙⣏⠿⠹⢦⡀⠀⠀⢀⣴⠾⠉⡇⠀⠸⡇⠈⠹⣿⣿⣿⣿⣿⣿",
            "⠀⠀⠀⣰⡿⠀⠈⠈⠳⢾⣻⡿⣿⣷⣾⣿⡿⢿⣫⠝⠁⠀⠀⠀⠀⠀⠀⠈⢮⠣⡄⠻⠶⣾⡿⠋⣰⣾⡇⠀⠀⣷⡄⠀⠀⠙⠿⣿⣿⣫",
            "⠀⠀⢠⣿⠇⠀⠀⠀⠀⠀⠀⠉⠛⠛⠛⠛⠋⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢳⣄⠀⠀⠉⢠⣾⣟⣿⠃⠀⢀⣿⠇⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⢈⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢨⣿⠗⠀⣠⣿⡿⢁⡿⠀⠀⢸⣿⠀⠀⠀⠀⠀⠀⠀⠀",
        ],
        //MARK: medium art
        vec![
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣶⣶⣤⡤⠤⠶⠶⠦⠤⣄⣿⢿⣷⢦⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⠟⠋⠻⠄⣀⠀⠀⠀⠴⢮⠑⢦⣿⣎⠻⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⡾⢡⠞⠀⡘⠀⣿⣷⣄⠀⠀⠈⢷⡀⠻⣿⣆⢸⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⡏⢀⠏⡀⢠⢇⠀⢻⠹⣿⣷⠄⣆⠘⣿⣶⣿⡉⡇⢧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⠘⡞⡾⡇⡿⠼⢆⠸⡞⣹⡾⣧⡹⡄⢻⣻⣿⡇⢸⠸⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⢰⣧⠀⠋⣷⢶⣮⠓⣿⡅⢿⣿⣷⡘⠘⣿⡿⠁⠀⢇⢻⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣾⣿⣆⠈⢿⡾⣿⡆⠈⠉⠀⠉⠷⡿⡄⣿⡇⠀⢰⡼⡌⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢈⣿⠈⠢⣸⣿⣆⠁⠀⠔⠂⢀⣼⣼⣿⣿⣧⠀⠸⡙⣵⢸⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⠃⡀⠀⢸⢿⣿⣿⣶⣾⡯⠗⣿⡾⣥⠉⢻⡀⠀⢇⠙⣇⢷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⡃⢰⠁⠀⡟⠈⣻⢯⣽⣿⣧⣴⠟⣤⡘⡇⠘⡇⠀⠘⡄⠸⡘⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⡷⢁⠇⠀⢸⠃⢰⣯⢊⠏⣷⣾⣽⡇⠉⠹⡇⠀⢷⠀⠀⢱⠀⠱⠘⣧⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⠟⠀⡞⠀⠀⡟⣴⣏⣧⠎⢀⠁⢀⣸⣧⣤⣤⣧⠀⢸⠀⠀⠀⢣⠀⠀⠘⣆⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⣼⠋⠀⡘⠀⠀⢸⡿⣿⢿⠏⠀⢸⠉⣳⣴⡟⢻⣿⣿⡆⠘⡇⠀⠀⠈⣄⠀⠀⠘⣆⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⢀⡼⠁⠀⣰⠁⠀⢀⡾⡛⢁⣮⡄⡰⣿⠘⠳⡿⡧⠿⠿⣿⣧⠀⡇⠀⠀⠀⠘⣆⠀⠀⠸⣧⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⢀⡞⠀⠀⣰⠃⠀⢠⢿⡴⠓⡟⠛⠁⠀⣷⡗⢤⡸⡇⢀⣠⣿⣿⣆⢃⠀⠀⠀⠀⠈⢆⠀⠀⠹⡄⠀⠀⠀⠀",
            "⠀⠀⠀⠀⢠⡟⠁⠀⣰⠃⠀⣴⣥⣾⣧⡾⡇⠀⢀⡼⣷⠿⢀⣽⡇⠈⡟⠏⢿⣿⣿⠀⠀⠀⠀⠀⠘⡀⠀⠀⢹⡀⠀⠀⠀",
            "⠀⠀⠀⢀⡾⠀⠀⢠⠃⢀⣾⣿⣿⣿⡿⢡⣿⣠⣾⠉⠻⣷⣿⣿⡇⠌⢹⠸⣘⣿⣿⣷⡄⠀⠀⠀⠀⢱⠀⠀⠈⣷⠀⠀⠀",
            "⠀⠀⠀⡼⠁⠀⠀⡞⡰⠋⠀⠈⣷⣽⡳⢿⣷⣻⣿⠀⣀⣿⡿⣿⡧⠀⠘⡇⢿⢟⣿⣿⣿⡦⠀⠀⠀⠀⢧⠀⣆⠸⡇⠀⠀",
            "⠀⠀⣸⠁⢀⠀⡸⢸⣥⣀⢀⣼⠟⣿⡏⠙⠒⢲⠽⠭⢵⡞⠊⠉⣿⠀⢀⣼⡈⢾⣿⣿⡿⠁⠀⠱⡀⠀⠘⡆⢸⣆⢻⡀⠀",
            "⠀⢠⠇⢀⡎⢠⠃⠀⠈⢙⣿⣿⣿⣿⣧⣀⣀⣼⠀⠀⠀⣧⣤⣶⣿⣐⣿⣿⣷⣾⣿⢿⡄⠀⠀⡇⢳⠀⠀⢣⠀⣏⢎⡇⠀",
            "⠀⠸⢀⣾⠁⡜⠀⠀⠀⠘⠛⡏⠀⢹⢿⡿⣿⡇⠀⠀⠀⢸⡍⣹⢿⣿⡈⠹⠿⠛⠋⠈⡇⠀⡄⠀⠀⠀⠀⠘⠀⠹⡜⣷⠀",
            "⠀⡇⣜⡇⠀⠃⠀⠀⠀⠀⢀⠁⠀⣾⣾⣿⣿⠇⠀⠀⠀⠈⣧⠙⠿⣿⣧⡀⠀⠀⠀⠀⡇⠀⢇⠀⠀⠀⠀⠀⠀⠰⢇⢸⠀",
            "⠀⡗⢹⠀⠀⠀⠀⠀⠀⠀⢸⠀⢠⡏⠀⣏⣿⠀⠀⠀⠀⠀⢹⣤⠀⢻⣿⣷⡀⠀⠀⠀⢳⠀⢸⠀⠀⠀⠀⠀⠀⠀⢸⠘⡇",
            "⢰⡇⣼⠀⠀⠀⠀⠀⠀⠀⣸⠀⡾⡀⢸⣿⣿⡀⠀⠀⠀⠀⠀⣿⠀⠈⢿⣿⣷⡄⠀⠀⢸⠀⠀⡆⠀⠀⠀⠀⠀⠀⣼⡀⡇",
            "⠘⡇⣿⡄⠀⠀⠀⠀⠀⠀⡏⢠⡇⠁⢸⣿⣿⡇⠀⠀⠀⠀⠀⢹⡇⠀⠘⣧⠻⣿⡄⠀⢸⠀⢰⡇⠀⠀⠀⠀⠀⢀⡟⡇⡇",
            "⠀⡇⡇⣧⠀⡄⠀⠀⠀⠀⡇⢸⠱⠀⢸⠐⣿⡇⠀⠀⠀⠀⠀⠈⡏⠀⠀⢹⡀⢹⣷⡀⡟⠀⡞⡇⠀⠀⢀⠆⠀⡼⠀⣇⡇",
            "⠀⢃⡇⠸⡆⢱⠀⠀⠀⠀⡇⣸⠀⠀⣾⡆⢸⣧⠀⠀⠀⠀⠀⠀⢷⡆⠀⠈⣿⠀⣿⣷⡇⣰⠁⡇⠀⣦⠎⠀⣸⠃⢰⣿⠃",
            "⠀⢸⣿⠀⠹⣆⠣⡀⠀⠀⢻⢿⠀⠀⣿⣤⣼⣿⡄⠀⠀⠀⠀⠀⢸⣧⣤⣴⣿⣡⣾⣿⣷⠁⠀⡇⢠⠏⢀⡼⠃⠀⢸⡟⠀",
            "⠀⠀⢻⡆⠀⠈⠻⣷⣄⠀⢸⢼⣾⣿⠿⠿⠛⠻⣿⡄⠀⠀⠀⠀⠘⣿⣿⣿⡿⠿⢿⣿⣿⠀⢰⠇⢀⣴⠟⠁⠀⠀⣾⠃⠀",
            "⠀⠀⠈⠁⠀⠀⠀⠈⠙⠷⣤⣿⣿⣧⣀⣀⠀⠀⢻⣿⠀⠀⠀⠀⢸⠁⣨⡿⠀⢀⣸⣿⠏⠀⣾⡶⠋⠁⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠛⢿⣾⣷⣴⣿⡿⠀⠀⠀⠀⢺⣿⣯⣀⣶⣾⡿⠋⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠉⠀⠀⠀⠀⠀⠀⠙⠛⠛⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        ],
        vec![
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠄⠋⠑⣦⡀⠀⠀⣠⠴⢋⠁⠀⠀⠀⢀⡴⠚⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠑⢦⡀⠀⢀⣠⠤⠒⠉⠉",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠢⠐⠒⠀⠀⠀⠀⠀⠙⣦⠞⢁⠔⠉⠀⠀⠀⢰⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⣞⠉⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⠃⠀⠀⠀⣠⠞⠁⠀⠋⠀⢀⡴⠀⢰⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢢⡀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⡷⢀⡾⢁⡞⠁⠀⣠⠞⠁⠔⠋⠀⠀⡜⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠱⡀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣿⣇⣸⡵⠋⠀⣠⠞⠁⠀⠀⡄⠀⠀⢀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⡄⠀⣀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡼⠋⣨⠏⢀⣤⠔⠁⠀⠀⢀⡼⠁⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠰⣿",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣇⡼⠉⢀⠞⠀⠀⠀⠀⢀⡞⠀⠀⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿",
            "⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⠟⠁⢠⡟⠀⠀⠀⠀⠀⡘⠁⠀⠀⠀⠐⣿⠀⠀⠀⠀⠀⠀⠀⢀⡿⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹",
            "⠀⠀⠀⠀⠀⠀⠀⠀⣼⡟⠀⣠⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣿⡀⠀⠀⠀⠀⠀⠀⢸⠁⢻⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⢀⡟⠀⣰⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠏⢸⡇⠀⠀⠀⠀⠀⠀⢸⠀⠀⢻⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⡼⠀⢠⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣻⠋⠀⠀⡇⠀⠀⠀⠀⠀⠀⣸⠀⢦⠀⠹⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰",
            "⠀⠀⠀⠀⠀⠀⡸⠁⢠⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠷⠶⠦⣤⡇⠀⠀⠀⠀⠀⠀⣿⠀⠈⢣⠀⠘⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸",
            "⠀⠀⠀⠀⠀⣰⠃⢀⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⠋⠀⠀⠀⠀⣇⠀⡇⠀⠀⠀⠀⡟⠀⠀⠀⠑⠄⠈⢷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⢰⠃⢀⣼⠃⢀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⡏⠀⠀⠀⠀⠀⢹⣰⣿⡄⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⣀⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠃⠀⠚⠁⢠⡌⠀⠀⠀⠀⠀⠀⠀⠀⣠⣿⣷⣿⣷⣶⣦⡀⢸⣏⡏⢷⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠁⠀⠙⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⡾⠀⢨⡇⠀⠀⠀⠀⠀⠀⠀⢰⣿⠋⢡⠉⢻⣿⣿⣿⣦⣹⡇⠸⣆⠀⠀⣇⠀⠀⠀⠀⠀⠀⢀⣠⠄⠈⢳⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⢰⠇⣀⣿⡇⣄⠀⠀⠀⠀⠀⢸⣿⡟⠀⣼⣿⣿⠟⢿⣿⣿⣿⠇⠀⢻⣆⠀⢸⠀⠀⠀⠀⠀⠶⣉⣠⣶⣾⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⣿⠀⣿⡿⠁⢿⠀⠀⠀⠀⠀⣼⡟⢷⠀⢻⠻⣇⡀⣸⣿⡿⠁⠀⠀⠀⢻⡀⢸⠀⠀⠀⠀⠀⣶⠿⠿⣿⣿⣿⣿⣿⡿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⡟⠀⣿⠇⠀⢸⡆⠀⠀⠀⠀⢻⣿⠀⠃⠘⢧⣉⣋⣉⡿⠃⠀⠀⠀⠀⠀⠻⣼⡀⠀⠀⠀⠀⠩⣦⣴⡿⠛⠛⣿⣽⡿⠙⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠁⢸⡟⠀⠰⠸⣇⠀⠀⠀⠀⣾⣿⡄⠀⠀⠀⠈⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠙⠧⠀⠀⠀⠀⠀⣿⢿⣷⣤⣠⡿⣻⠇⠀⣼⢿⣦⣴⡆⠀⠀⠀⠀⠀⠰⡄⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠸⠃⠀⠀⡆⢻⠀⠀⠀⢰⡏⢿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⠜⠶⣬⣭⣥⡴⠋⠠⠞⠁⣰⣿⣿⣷⡀⠀⠀⠀⠀⢀⡇⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢱⡸⣇⠀⠀⣸⠁⢸⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⡿⠉⣿⡿⣧⠀⠀⠀⠀⣽⡅⠀⢇⡀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠇⢻⡇⢀⡏⠀⢸⡏⢻⣄⠀⠀⠀⠀⠀⠀⢠⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⡿⠁⠀⢸⣧⣿⡄⠀⠀⠀⡟⡇⠀⣾⠁⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢷⣾⡇⠀⢸⡇⠀⠙⢷⣄⠀⠀⠀⠀⠈⣏⠉⠉⠓⠒⠤⣤⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⠃⠀⠀⣰⣿⣿⣷⡀⠀⢰⠇⡇⢠⡇⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⢀⡀⢀⣸⣟⣿⡀⢸⡇⠀⠀⠀⠉⠳⣦⡀⠀⠀⠈⠳⢤⣤⣤⡴⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⠇⠀⠀⣼⣟⢻⣿⣿⣇⠀⢸⠀⡇⣾⠃⠀⠀",
            "⠀⠀⠀⢀⣤⠖⠋⠉⠉⠉⠉⠉⠉⠁⠈⠙⢷⣸⣷⠀⠀⢀⣠⡶⠋⠉⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣠⣴⡾⠻⣿⣿⣿⡿⠀⢠⠾⠛⠛⠛⣿⣿⣿⡀⡏⠀⣷⠏⠀⠀⠀",
            "⠀⠀⢠⠏⢁⠏⠀⠀⣰⡇⠀⠀⢠⡇⠀⠀⡄⠈⢻⣦⣤⣾⠁⠀⠀⠀⠈⠿⣝⠶⣄⣀⠤⠤⠖⣲⠟⠋⠁⠀⠈⣻⢷⣤⣿⣿⣿⠃⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⠇⠀⠏⠀⠀⠀⠀",
            "⠀⠀⡎⢀⣼⠀⠀⢠⡻⠀⠀⢀⣿⠀⠀⢰⠃⠀⢸⠃⠀⢸⡆⠀⠀⠀⠀⠀⠈⠳⣄⠀⣠⠴⠛⠁⠀⠀⠀⠀⢹⠇⡜⠛⠿⠿⣯⣤⣾⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀",
            "⠀⢸⠃⣼⡇⠀⠀⣾⡇⠀⠀⢸⠃⠀⠀⡎⠀⠀⣎⠀⢠⡟⠃⠀⠀⠀⠀⠀⠀⠀⢿⠟⠁⠀⠀⠀⠀⠀⠀⠀⡜⢹⡇⠀⠀⠀⡾⠃⠀⠈⠉⠙⠻⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⣯⣀⣀⣼⡟⠀⠀⣠⠏⠀⠀⣰⠃⠀⣸⠋⣷⣿⠀⠐⠀⠀⠀⠀⠀⠀⢀⣨⣤⡀⠀⠀⠀⣰⠚⠀⠀⠁⠀⣷⠀⠀⠈⠁⠀⠀⠀⠀⠀⠀⠀⠉⢻⣿⡇⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠸⢤⣠⢾⡁⠀⣠⠴⣏⣄⡀⠀⢀⡿⠃⠀⢀⠀⠀⠀⠀⢀⡾⠋⠉⠉⠛⣆⠀⢠⡏⠀⠀⠀⠀⠀⠸⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣧⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠉⠉⠁⠀⠀⠀⠉⠛⣻⠀⠀⠀⠈⠀⢀⣀⠤⢻⠀⠀⠀⠀⠀⣿⡆⠘⠇⠀⠀⠀⠀⠀⠀⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣆⠀⠀⠀⠀⠀⠀",
            "⢀⠀⢀⠀⣀⣀⣀⣀⣀⣼⣀⡀⠀⢠⣀⣀⡀⣈⡅⠀⠀⠀⠀⠀⠈⠀⠀⠈⢷⡀⠀⠀⣠⠟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⡼⠷⠤⠀⠀⠀⠀",
        ],
        vec![
            "⠀⠀⠀⠀⠀⢀⡞⡼⠋⠀⠀⣤⣦⣿⣿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⢿⡟⢟⢻⣍⠉⠙⣿⣿⣿⣶⣿⣿⣿⣿",
            "⠀⠀⠀⠀⢀⡞⣼⠃⠀⢀⣾⣿⣿⣿⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢤⡙⣷⡄⠈⢣⡻⣿⣿⣿⣿⣿",
            "⠀⠀⠀⢀⡞⣸⠏⠀⢠⣿⣿⣿⣿⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡀⠀⠈⠛⣿⣶⡀⢳⡝⣿⣿⣿⣿",
            "⠀⢠⠆⡾⢰⡏⠀⣰⣿⣿⣿⣿⣿⠁⠀⠀⠀⢀⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣷⣀⠀⠀⠈⢿⣷⣼⣿⡄⣿⣿⣿",
            "⠀⠘⢸⣥⣿⠁⣰⣿⣿⣿⣿⣿⡟⢰⠀⠀⢀⡞⠀⠀⠀⢀⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⢿⣷⣄⣠⣾⡟⠙⠻⣿⠹⣿⣷",
            "⠀⠁⠈⣿⣧⣴⣿⣿⣿⣿⣿⣿⠇⠈⠀⠀⢰⠃⠀⠀⠀⣿⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⣹⣿⣿⣯⣧⡄⠀⠈⢧⠹⣿",
            "⠀⠀⠀⠉⠈⠻⣿⣿⡟⢻⣿⣿⠀⠀⠀⡄⡎⠀⠀⠀⣼⡟⢷⠠⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡯⣾⠿⠁⠙⢿⣿⡇⠀⠀⠸⣷⡈",
            "⠀⠀⠀⠀⠀⢠⣿⣿⡇⠸⣿⡏⠀⠀⠀⣇⠃⠀⢠⣾⣿⠃⣿⠠⡄⠀⠀⠀⠀⠀⠀⠀⠀⣠⠖⢸⠁⡇⠀⠀⠀⠁⠀⡇⠀⠀⠀⣿⠀",
            "⢠⠀⠀⠀⠀⢸⣿⣿⣿⡀⢿⡇⠀⠀⠀⣿⣀⡴⢃⠟⠁⠀⢿⡇⢧⣀⠀⠀⠀⠀⠀⠀⢠⠗⢲⣿⢲⡗⠀⠀⠀⠀⠀⠐⠀⠈⢷⣿⠀",
            "⡈⠀⠀⠀⠀⣼⣿⣿⡿⢃⢸⡇⡇⠀⠀⢿⠛⠠⠋⠀⠀⠀⠘⣿⣌⢿⠀⠀⠀⡆⠀⢀⡏⢀⡞⣹⡏⢳⠀⠀⠀⠀⠀⠘⠀⢀⠸⠟⠂",
            "⣿⠀⠀⠀⠀⣿⣿⣿⣿⢸⡆⠃⣇⠀⠀⠸⡤⠷⠶⢶⣴⣶⣄⠈⠛⣎⣇⠀⠀⡇⢀⣾⢁⠞⠀⡿⠁⢹⠀⠀⠀⠀⠀⠀⠀⢸⣦⡀⠀",
            "⣿⠀⠀⠀⢰⣿⣿⣿⣿⠀⡇⣌⢻⡀⠀⠀⢱⡀⠀⠀⠈⠙⠻⠀⠀⠈⠙⢶⡀⢳⠎⢘⣸⣶⣾⣧⣄⠀⡇⣦⠀⠀⠀⡀⠀⠀⢿⣧⠀",
            "⣿⠀⠀⠀⢸⣿⡿⣿⣿⡇⢱⠘⣤⣱⡀⠀⠈⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠛⠻⣴⣿⣿⣿⣿⡙⢿⣷⣇⢸⠀⠀⠀⡇⠀⠇⢸⣿⡄",
            "⣿⠀⠀⠀⢸⣿⡧⣿⣿⣧⠸⡀⢻⡍⠱⡄⠀⠈⢦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⢏⣙⡿⠋⠀⢈⣽⣿⣿⡆⠀⠀⣿⠀⠀⠀⡇⠇",
            "⢹⡀⠀⠀⢸⣿⡇⢸⡿⣿⡄⡇⠈⢣⠀⠙⡄⠐⡀⠀⠤⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⢠⣾⣟⣿⣿⣹⣄⢠⡿⠀⠀⠀⠀⠀",
            "⢸⡇⠀⠀⣾⠟⠃⢸⡇⢻⣿⡇⠀⢀⡡⠖⣻⣆⠘⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⢁⣾⣿⠿⠋⣹⢿⣄⠀⠀⠀⠀⠀",
            "⢸⡇⠀⠀⡍⠀⠀⢸⡇⠈⢿⡷⠞⠁⢀⡼⠋⣿⣷⢤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣼⡟⢻⣿⡿⠁⣠⣾⡷⠾⢿⡇⠀⠀⠀⠀",
            "⠘⣷⠀⠀⠀⠀⠀⢸⡷⣶⠏⠀⢀⡴⠋⠀⢀⣿⣿⡀⠙⠢⣀⠀⠀⢀⣀⣀⣤⣴⣶⣿⣿⣿⠃⢰⡟⠁⣰⠟⠁⣠⣴⣿⠀⠀⠀⠀⠀",
            "⠀⣿⡀⠀⠀⣠⢞⣥⡾⠁⢀⡴⠋⠀⠀⢀⣾⢡⣿⡷⣤⣀⣈⣉⣉⣠⣿⡿⢋⣭⠥⣶⠞⠁⠠⠫⠤⢼⣁⣠⣾⣟⣿⣿⡆⠀⠀⠀⠀",
            "⠀⣿⡇⠀⢸⡏⢿⡿⠁⠀⠊⠀⠀⠀⠀⡾⢁⣾⠟⠙⠛⠿⠿⠿⢿⣿⢏⡴⠋⢀⣴⠷⠾⠒⡑⠒⠒⠤⣈⣿⣿⣿⣿⣿⠇⠀⠀⠀⠀",
            "⠀⢹⣿⠀⠸⡇⢸⣉⠓⠲⢤⣄⣀⣀⡤⠖⠋⠀⠀⠀⠀⠀⠀⣼⣿⢋⡞⣡⣴⡟⣹⠀⠀⠀⠀⢉⡶⠄⠈⣿⣿⣿⣿⣿⣷⡀⠀⠀⠀",
            "⠀⠸⣿⣆⢀⡏⠒⠬⣍⣓⠲⢬⣿⣦⣄⠀⠀⠀⠀⣀⠀⠀⣸⣿⣇⣾⣿⣿⣿⡇⡿⣄⠀⣠⠔⠉⠀⠀⣰⣿⣿⠋⣩⣿⡿⣷⠀⠀⠀",
            "⠀⠀⣿⣿⣸⣷⡄⠀⠀⢠⣉⠓⢬⡙⢿⣷⣄⠀⠀⠀⠙⢲⣿⣿⣿⣿⣿⣿⣿⢠⡇⠈⡷⢤⣀⡀⢀⣰⡿⣿⣿⣿⠋⠀⡇⢿⠀⠀⠀",
            "⠀⠀⢸⣿⠏⡾⣿⣦⣄⣀⠙⢿⣶⣽⣦⣝⢿⣷⡀⠀⢀⣿⢿⣿⣿⣿⣿⡿⢿⣾⠗⢀⡇⠀⠀⠉⠉⡝⠀⣿⣿⣿⠀⠀⡇⢸⠀⠀⠀",
            "⠀⠀⠈⣿⢰⡿⠋⠙⠻⢿⣿⣶⣿⣿⣿⣿⣦⣹⣷⡄⣸⣿⣿⣿⣿⣿⠋⣰⣿⣿⣇⣸⠀⠀⠀⠀⠀⡇⢸⣿⣿⣿⣆⠀⠃⠈⠀⠀⠀",
        ],
        vec![
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⣤⣤⡤⢤⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⡾⠋⢲⡄⠀⠀⠀⠁⠀⠀⠀⠉⠳⣮⠷⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⣤⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⡿⠋⠀⠀⠀⠙⣆⠀⢀⣠⣴⣶⣿⣿⣿⣿⣷⣿⣿⣿⡗⠲⠶⢤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠹⡄⠀⠀⠀⠀⠀⠀⠀⠀⣸⡿⠁⠀⠀⠀⠀⠀⣸⣿⡿⠟⠛⣩⡴⠋⠁⠈⣻⠛⢧⡉⠛⠷⣄⡀⠉⠛⢦⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⢱⠀⠀⠀⠀⠀⠀⣠⣾⡟⠛⠦⣤⡀⠀⣠⣾⠟⠋⠀⢠⡾⠋⣀⠀⠀⡼⠁⠀⡆⠹⡄⠀⠈⠳⣅⠀⡀⠙⢧⡀⠀⠀⠀⠀⠀⠀⠀⣀⣠⣤⣶⠆⠀⠀⠀⠀",
            "⠀⠀⠘⡄⠀⠀⠀⠀⡼⠋⡏⠀⠀⠀⠀⠉⣵⠟⠁⠀⠀⣴⠏⠀⣰⠁⠀⢰⠃⠀⠀⡇⠀⢻⡄⠀⠀⠙⣦⠈⠳⣄⠹⣄⠀⠀⠀⣠⣴⣾⣿⣿⣿⣤⣄⠀⠀⠀⠀",
            "⠀⠀⠀⣷⠀⠀⠀⢰⠇⢀⣇⠀⠀⠀⢀⡼⠋⠀⣴⠖⠛⣱⠀⣰⠃⠀⡄⢸⠀⢠⠀⡇⠀⠘⣿⡀⠀⠀⠘⣧⠀⠈⢳⣜⣧⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⠄⠀⠀⠀",
            "⠀⠀⠀⢻⡀⠀⠀⢸⠀⣸⠻⣆⢀⣴⡯⠀⢀⡞⠁⠀⣰⠃⣰⠃⠀⣼⠀⡏⠀⣸⠀⡇⠀⠀⣧⠷⡄⠀⠀⡘⣇⠀⢦⣝⣿⣿⣿⣿⣿⠟⠏⠀⠘⢿⣀⠀⠀⠀",
            "⠀⠀⠀⢸⡇⠀⠀⡟⢠⡏⠀⣹⣿⡞⠁⢀⡞⠀⠀⣴⠇⣼⡟⠀⢠⠃⢠⣇⠀⣷⠀⡇⠀⠀⣿⠀⢹⠀⠀⠘⡘⣦⠀⠉⢻⣿⣿⣿⣿⣿⠀⠀⠀⠀⣨⣟⠁⠀⠀",
            "⠀⠀⠀⣼⣿⠀⢰⡇⣼⠁⢸⣿⣿⠃⢀⣾⠀⠀⡼⢹⢱⠃⡇⠀⡼⠀⢸⣿⠀⣿⣄⡇⠀⠀⣿⠀⠈⡇⠀⠀⢣⠙⡆⠀⠘⣿⣿⣿⣿⣷⣷⠒⠚⠋⠁⣹⣷⠀⠀",
            "⠀⠀⠀⣿⠙⡆⢸⣧⡏⠀⣿⡟⡟⠀⡾⡋⠀⡾⠓⢺⡟⢴⣧⠀⡇⠀⠸⣿⣆⢻⠻⣷⠀⠀⣿⠀⢰⢹⡄⠀⠘⡆⢹⡀⢀⣿⡿⣿⣿⣿⣷⡲⢶⣶⣖⠛⠋⠀⠀",
            "⠀⠀⠀⣿⠀⣧⢸⣿⠀⢸⣿⣴⠇⡼⠀⣇⡾⠁⠀⢸⡇⠀⡇⢀⡇⠀⠀⣿⡟⢮⣦⣸⣄⠀⣿⠀⢸⡘⣧⠀⠀⣷⠀⣷⠈⣿⣀⣨⡏⠙⢿⠷⣿⣿⡿⠷⣆⠀⠀",
            "⠀⠀⠀⡿⠀⣿⢸⡏⠀⣼⣿⣿⢀⡇⠀⣿⣷⣿⣶⣾⣗⠀⢹⣼⡇⠀⠀⢸⡇⠈⠻⣝⣿⠻⢾⡄⠈⡇⢻⡀⠀⢻⡄⢹⣾⡏⠙⠿⠟⠻⣿⡷⢽⣿⣦⣀⣼⣷⠄",
            "⠀⠀⢸⡇⠀⠸⡿⠁⢀⣿⠁⡿⣾⡇⢘⣿⡟⣿⣟⣿⡟⢷⡈⣿⣿⡀⠀⠈⡇⠀⠀⠈⠻⣦⣼⣏⠹⡇⠀⣧⠀⢸⣇⠈⣿⠷⣾⣿⣆⣤⣯⡀⣀⣿⣇⡼⠟⠉⠀",
            "⠀⠀⢸⠁⠀⠀⡇⠀⢸⣿⠀⣿⣿⣷⡀⡇⠀⣿⢿⡿⠇⠈⠁⠈⢯⣷⡄⠀⣟⣭⣷⣶⣶⣮⣿⡸⣆⣳⠀⣿⠀⠘⣿⠀⣿⣄⣼⡏⠀⣹⣿⣭⡿⠿⠋⠀⠀⠀⠀",
            "⠀⠀⣿⠀⠀⠀⡇⠀⢸⣿⠀⠸⣿⣯⣷⣳⡀⠘⠛⠋⠀⠀⠀⠀⠀⠀⠙⢦⣇⣾⣿⣿⣿⡎⢿⣗⣹⣷⠀⣿⡇⢠⣿⣤⡇⠠⡿⣿⠋⢹⠂⣿⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⡏⠀⠀⠀⡇⠀⢸⣿⠀⠀⠻⣿⣿⠹⢷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢧⡻⠟⠻⠇⢈⡟⢻⣷⠀⢁⡇⣼⢿⣿⠂⢰⠇⡟⠀⣸⡆⢸⠀⠀⠀⠀⠀⠀⠀",
            "⠀⢰⡇⠀⠀⠀⡇⠀⠸⣿⠀⠀⠀⠉⣿⣆⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠛⠻⠭⠌⠋⢠⡿⡇⢠⣼⣿⣿⣸⡟⠀⣸⠀⠇⣰⠏⣷⣾⠀⠀⠀⠀⠀⠀⠀",
            "⠀⢸⡇⠀⠀⠀⡇⠀⠀⣿⠀⠀⠀⢀⡇⠘⢦⡀⠀⠀⠠⣤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣼⡇⢰⣿⣿⡿⢻⠀⢀⡇⢠⡼⠃⠀⣿⣿⠀⠀⠀⠀⠀⠀⠀",
            "⠀⣸⠁⠀⠀⣸⡷⠿⢦⣽⣀⠀⠀⢸⠁⠀⠀⠹⣦⡀⠀⠀⠉⠉⠉⠉⠁⠀⠀⠀⠀⠀⢀⣠⠾⣿⣿⣠⠟⢡⡟⠀⢸⠀⠘⢀⡾⠁⠀⠀⢿⡏⠀⠀⠀⠀⠀⠀⠀",
            "⣤⡇⠀⠀⠀⣿⠲⣆⠀⠀⠉⢛⣶⣯⣤⣤⣠⣤⣼⢷⣦⡀⠀⠀⠀⠀⠀⢀⣀⣤⡤⠶⠋⠁⠀⣸⠟⠁⠀⠋⠀⠀⢸⣦⣷⠞⡇⠀⠀⠀⣸⡇⠀⠀⠀⠀⠀⠀⠀",
            "⣿⠃⠀⠀⢀⣿⡄⢿⠉⢳⣶⢿⣿⣿⣿⣿⣿⡿⠃⢸⣿⠙⠓⠒⠒⠊⠉⢉⡽⢿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣿⡎⢀⡇⠀⠀⠀⣿⡇⠀⠀⠀⠀⠀⠀⠀",
            "⣿⠀⠀⠀⢸⠙⣧⠘⣿⠭⢥⣸⣿⣿⣿⣿⣿⠃⠀⠀⠻⣧⡀⠀⣀⡤⠞⠋⠀⢸⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⢰⠀⠀⠀⠀⢿⠁⠀⠀⠀⠀⠀⠀⠀",
            "⡏⠀⠀⠀⡿⡄⡸⡖⢿⣷⣶⣾⣿⣿⣿⣿⡇⠀⠀⠀⣀⣈⣻⡻⠝⠒⠁⠀⠀⣸⣿⣿⣷⣦⣄⡀⠀⠀⠀⠀⠀⠀⢸⡟⠀⠈⠀⠀⠀⠀⣾⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠁⠀⠀⣸⠃⠙⣧⠸⡄⠹⣍⠉⣿⠛⡿⢿⣀⡤⣖⣋⣩⡿⢻⠻⡄⠀⠀⠀⢰⣿⣿⣿⣿⡿⢹⡏⠉⠙⠒⠢⣄⣀⣸⠃⠀⠀⠀⠀⠀⠀⢻⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⢠⠏⠀⠀⣹⡀⠙⢦⡉⣿⣿⡾⠁⠀⠀⣀⠝⣿⠋⣀⣈⢧⡹⡖⠀⢀⣿⣿⣿⣿⡿⠃⢰⡷⢤⣀⣀⠀⠀⠙⢿⡆⠀⠀⠀⠀⠀⢰⢸⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⢠⡟⠀⢀⠶⢹⠇⢠⡎⠁⣿⣽⠃⠀⠀⠈⠀⠀⢸⣿⡿⠟⠉⠉⠙⢦⠞⣸⣿⡿⠿⣭⣉⠛⢶⡤⣽⣎⠙⡗⠦⣈⣧⠀⠀⠀⠀⠀⢸⡘⡇⠀⠀⠀⠀⠀⠀⠀",
            "⢀⡞⢀⡴⠃⠀⢸⠀⡞⠀⢸⣟⣿⣄⣀⠀⠀⠀⠀⠘⠛⠀⠀⠀⠀⠀⠀⠀⡟⢷⠀⠀⡘⡫⣳⣄⢹⡄⣸⠛⢻⢦⣼⣿⡀⠀⡇⠀⠀⠈⣷⢻⠀⠀⠀⠀⠀⠀⠀",
            "⣾⠐⠋⠀⠀⠀⣾⢸⠁⠀⢸⣽⠁⠀⠩⠳⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⣵⣷⣄⣠⣭⠀⢈⣿⣾⣇⡏⠉⢻⠀⠈⢿⠀⠀⡇⠀⠀⠀⣿⡞⣇⠀⠀⠀⠀⠀⠀",
            "⠋⠀⠀⠀⠀⢀⡇⠘⠀⠀⣿⡇⠀⣀⠀⠀⠊⢳⡀⠀⠀⣀⣀⣀⣀⡀⢰⣯⣸⡋⠁⠻⠟⠑⢿⡟⢻⡙⠀⠀⢸⠀⠀⠈⡇⠀⢹⠀⠀⠀⢸⣷⢻⡀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⣾⠁⢰⠀⠀⣿⠁⢺⣿⡧⠀⠀⠐⠹⠞⠉⠁⠀⠀⠊⠉⣿⠻⣿⠿⢶⣿⡦⣴⣾⣧⠏⠀⠀⠀⢸⠐⠀⠀⢧⠀⢸⠀⠀⠀⠈⡟⣏⣇⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⢠⣿⣀⡏⠀⠀⣹⡄⠀⠀⠀⠀⠀⠀⠀⠀⢠⣶⣶⠀⠀⢰⢧⣶⣷⣄⣤⣯⠀⣸⢿⡿⠖⣆⠀⠀⢸⠀⣀⡤⣼⠀⢸⡇⠀⠀⠀⢷⢹⣸⡄⠀⠀⠀⠀",
            "⠀⠀⠀⠀⣸⣿⡅⠀⠀⠀⠈⣳⣀⠀⠀⠀⠀⠀⠀⠀⠀⣉⡉⠀⢠⣇⣀⣽⡋⠈⢻⢟⡿⣡⠞⠀⠀⠈⠓⢲⠞⠋⣀⢴⡟⠀⠈⡇⠀⠀⠀⢸⣄⣯⣧⠀⠀⠀⠀",
        ],
        vec![
            "⠀⠀⠀⠀⢰⣿⣿⣿⣿⡿⠁⠠⢀⣿⡙⣎⠁⠀⢀⠀⣯⠰⠀⡀⠰⢹⡋⢍⢿⣿⡹⢿⣏⢷⢹⠨⣁⠋⡙⢻⣿⣏⣏⠳⠄⠀⢹⠰⡁⢏⡿⣧⠀⠀⠀⠀",
            "⠀⠀⠀⠀⣹⣯⣿⣿⣿⠃⡈⡴⢹⣇⠳⠈⢀⠐⠀⢂⡇⢂⠁⢀⠂⡏⠌⢆⠘⣷⢯⡹⣿⠞⡆⠆⠀⠈⠀⠁⠹⣞⡦⡁⢊⠀⠌⡱⠀⡈⢽⣛⡆⠀⠀⠀",
            "⠀⠀⠀⠀⣼⣷⣞⣿⠇⡚⣼⠁⣿⢨⡑⠈⡀⠄⠡⣘⡇⢀⠂⠠⢸⠌⠀⠈⡀⠈⢿⡱⢿⣿⡸⡄⠀⠀⠀⠀⠀⠘⣷⠡⢀⠂⢦⠑⠠⠐⢸⣻⡇⠀⠀⠀",
            "⠀⠀⠀⠀⣿⣿⢾⡟⢬⣱⡏⣴⣏⠖⡠⠁⠄⡈⢰⡱⡇⢀⠂⠡⡍⠀⠀⠀⠴⢄⣈⢿⡜⡿⣕⠲⠀⡀⡠⠔⠒⠉⠸⣗⠄⠂⢱⡘⠠⠁⢂⠿⣽⠀⠀⠀",
            "⠀⠀⠀⠀⣿⣿⣻⢍⣶⡻⢼⣽⣎⠳⣔⡣⡔⢦⣏⡷⡇⠠⠈⣴⠃⠀⠀⠀⠀⠀⠀⠉⢷⡹⢻⡄⢣⠀⠀⢀⠤⣀⣦⣹⣞⡄⠘⡆⠡⠐⢈⠺⣝⠀⠀⠀",
            "⠀⠀⠀⠀⣿⣿⡏⡼⣞⡏⣿⣿⡜⣣⢿⣱⢛⡾⣽⣻⡇⠠⢁⣿⣶⣶⣤⣄⡉⠳⡄⠀⠀⠙⡄⠹⡄⡄⠀⣱⡿⠟⠛⠻⣿⡶⡀⣏⠐⡀⣢⠸⣝⡆⠀⠀",
            "⠀⠀⠀⠀⣿⣿⣱⣻⣽⡇⣿⣷⣙⡞⣧⣏⢯⣿⢷⣻⣧⠐⣸⠛⠉⠉⠉⠛⢿⣦⡀⠀⠀⠀⠀⠀⠙⡴⣰⣿⣶⣷⣦⡀⠘⣿⡄⡧⠐⢠⠙⡖⣭⡇⠀⠀",
            "⠀⠀⠀⢠⣿⢧⢷⣛⡽⡇⣿⡧⡇⢽⣳⣞⣻⣾⢿⣻⣿⠀⣿⣾⣿⣿⣷⡄⠀⠙⠃⠀⠀⠀⠀⠀⠀⠘⢆⣿⣿⣿⣿⠇⠀⣿⣷⡗⠠⠘⠀⠈⠳⣇⠀⠀",
            "⠀⠀⠀⢸⣿⢺⣟⡧⡗⣇⣿⣽⡇⢚⣷⣞⣯⣿⣟⣿⢿⣷⡿⣿⣿⣿⡿⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠉⠛⠠⠀⢰⣿⣿⡕⢀⠃⠀⠀⠀⠙⠀⠀",
            "⠀⠀⠀⢸⣯⣟⣿⣳⠇⡇⣿⣿⡇⢘⣷⣿⣿⣿⣻⣯⢿⢿⡇⠁⠉⠋⠁⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠐⠂⠈⢠⣿⣿⣏⠀⠆⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⣾⣿⡽⣾⣭⡇⣏⣿⣿⣷⠀⣿⣿⣿⣿⣿⣳⢿⠘⡏⠀⠀⠀⠂⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣷⠸⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠈⣾⢯⣷⣻⢖⡇⣾⣽⣿⠙⣧⢜⣿⣿⣿⣿⣯⢿⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⡟⣯⣟⣆⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⣟⡾⣭⣟⡞⡇⣽⣿⣿⠀⠹⡜⣿⣿⣿⣿⣿⣾⠟⠢⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡴⡀⠀⠀⠀⠀⣀⢾⣟⠧⡏⣿⢻⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⣯⢷⣟⢾⡹⡇⣾⢹⣯⠀⠀⠘⣽⣿⣿⣿⣷⡿⢦⡀⠐⣂⡀⠀⠀⠀⠀⠀⠀⠀⠀⠐⠍⠘⠁⠀⠀⠠⢚⠀⢈⡾⣓⡇⢯⡚⡄⠀⠀⠀⠀⠀⠀",
            "⠀⠀⢀⣯⡿⣏⣾⡱⡇⣿⠠⣿⠀⠀⠀⠈⢾⣿⣿⣿⣿⠈⠻⠿⣿⣿⣑⣆⠠⣀⣤⣶⣶⣶⣶⣶⣄⡀⠔⠁⠀⠀⡆⠠⣟⢸⠄⢳⡁⡇⠀⠀⠀⠀⠀⠀",
            "⠀⠀⢸⣷⡿⣏⢶⣹⢣⣿⠀⢿⠀⠀⠀⠀⠀⢻⣿⣿⣿⠀⠀⠀⢀⡝⠙⠻⣟⠿⢿⣿⣯⣿⣷⣿⠏⠁⠄⡀⠀⠀⣇⠀⢻⣏⠂⢸⡁⣇⠀⠀⠀⠀⠀⠀",
            "⠀⠀⣸⣯⢿⡜⣧⢻⠼⣿⠀⠀⠀⠀⠀⠀⠀⠀⣻⣿⣳⡄⠐⠂⢹⡑⢦⡀⡜⡢⣌⠉⠁⣠⢮⢡⢢⡜⢾⢸⠐⠂⠸⠤⠽⣎⡔⢠⢓⢸⠀⠀⠀⠀⠀⠀",
            "⠀⠀⣿⣟⡿⡜⣧⢻⣜⡇⡖⠀⠀⠀⠀⠀⡠⠊⠈⠟⣯⠇⠀⠀⡔⢉⠶⡱⣭⢳⡝⣗⣾⡝⣮⢳⢧⡞⢹⢂⠀⠀⠀⠀⠀⠀⠑⢮⡣⠼⡆⠀⠀⠀⠀⠀",
            "⠀⢠⣿⡞⣽⡝⣮⢹⢜⡆⠁⠀⠀⠀⢠⠊⠀⠀⠀⠀⠹⣇⠀⣰⣿⠻⡘⡵⣩⢷⣫⡽⢧⣻⣜⡯⢏⡔⢌⠎⡆⠀⠀⠀⠀⠀⠀⠀⠙⣆⢧⠀⠀⠀⠀⠀",
            "⠀⠈⣿⡝⣾⡝⣮⢹⣎⢧⠀⠀⢀⠔⠃⠀⠀⠀⠀⠀⠀⠉⢀⣿⣽⣷⡏⢶⡹⣾⠁⠠⠀⠌⣅⠸⢘⡹⣸⠀⢻⡀⠀⠀⠀⠀⠀⠀⠀⠘⠷⠀⠀⠀⠀⠀",
            "⠀⢸⣷⢻⣼⡳⣍⢾⣿⢺⡅⠀⠤⠂⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⢾⣷⣻⣷⣿⣿⣜⣡⢞⣰⠓⠷⡤⣥⠗⢸⠈⢧⠀⠀⠀⠀⠀⠀⠀⠀⠈⡑⡀⠀⠀⠀",
            "⠀⣼⣏⠷⣎⡷⡭⣞⣯⠽⣇⠊⡄⠀⠀⠀⠀⠀⠀⠀⠀⢲⣿⣯⣿⣟⢯⣿⠿⣛⡏⡞⢮⣽⡈⠆⠈⠁⢀⠘⡀⠸⡧⠀⠀⠀⠀⠀⠀⠀⠀⠰⠠⡀⠀⠀",
            "⠀⣿⣎⠿⣜⣳⣓⣯⣟⢦⠇⡐⠀⠀⠀⠀⠀⠀⡐⡑⣆⣿⡿⣽⡿⠮⠇⠋⠱⢌⣿⣹⢳⡞⠐⠀⠀⠀⠂⠌⡀⠀⢿⣰⢯⡄⠀⠀⠀⠀⠀⠀⣽⣓⡀⠀",
            "⢸⣽⡞⣽⢫⡷⡭⣿⡾⡇⣰⠀⢀⠀⡀⢀⢀⡠⣢⠃⣻⢿⡽⠟⠁⠀⠀⠀⠀⠈⣷⣯⢷⣻⠀⠀⠀⠀⠈⡔⠠⠀⠸⡿⡰⣞⣄⠀⠀⡀⠀⠀⢩⢞⠑⡄",
        ],
        vec![
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣠⣤⣤⣤⣀⣀⡀⠀⠀⠀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⢄⢀⠀⡀⣠⣴⣾⣿⠿⠟⠛⠛⠛⠿⠿⠿⠽⠿⠿⠿⠿⠿⢶⣦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⡿⡵⣯⣾⣿⣿⠟⠉⢀⣤⠶⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣿⢫⣮⢿⣿⡿⠛⠁⠠⠔⢋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⠈⠻⣷⣦⠠⣃⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣾⢟⣵⡿⢻⡿⠋⠀⠀⣠⢴⡾⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠈⢻⣷⣏⢫⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⢸⣿⣠⠋⠀⡠⣪⢞⡵⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⣿⣧⡻⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡟⡙⢾⣿⣸⡟⠁⠀⢠⠞⠁⠋⠀⠀⡴⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣷⡹⣷⡄⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠁⡇⢸⣿⠋⠀⢀⡶⠃⠀⠀⢠⠃⠜⠁⠀⣀⠀⠀⠀⠀⢀⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠨⣿⠏⣿⣇⡀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡏⠀⡄⡼⠁⠀⢀⣾⠁⠀⠀⠀⡜⡸⠀⠀⣰⢻⠀⠀⠀⠀⢸⠉⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣆⠀⠀⠀⠀⠀⢿⢸⣿⢻⢱⡄⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⠁⢀⡟⠀⢠⢃⣾⠇⠀⠀⠀⢰⠁⠀⠀⣰⢃⢸⡄⡄⠀⠀⢸⢀⠘⢧⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⡄⢸⠀⠀⠀⢸⣾⠃⠈⠈⡇⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡎⢠⠞⠀⡰⠃⣼⠏⠀⠀⠀⠀⡜⠀⠀⢰⠃⠉⠸⡇⣧⠀⠀⢸⠈⢦⠘⢷⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⢱⡀⠁⠀⠀⢸⡿⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣵⠃⠀⢰⠁⣼⠏⠀⠀⠀⠀⢀⠇⠀⢠⠟⠒⠒⠴⣷⣿⡀⠀⢸⠂⠀⠱⣄⠻⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⢧⠀⠀⠀⠈⠀⠀⠀⠀⠣⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⣠⡾⢋⠀⢀⠇⣸⠏⠀⠀⠀⠀⠀⡸⠄⢀⠎⠀⠀⠀⠀⠸⣇⢣⠀⢸⡄⠀⠀⠈⡴⠞⣆⠀⠀⠀⠀⠀⠀⠀⠀⠘⡆⠀⠀⠀⠀⠀⠀⠀⢰⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⢀⡾⢟⡇⡘⠃⡜⣼⣯⠀⠀⠀⠀⠀⠀⡇⢀⡞⠀⠀⠀⠀⠀⠀⢻⠈⢧⠸⡇⠀⠀⠀⠀⠀⠙⢆⠀⠀⠀⠀⠀⠀⠀⠀⢹⡀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠐⠉⠀⡸⠀⠃⢰⣻⣯⠇⠀⠀⠀⠀⠀⢸⢁⣾⣤⣀⠀⠀⠀⠀⠀⠀⠀⠈⢷⡇⠀⠀⠀⠀⠀⠀⠈⠧⡀⠀⢠⠀⠀⠀⠀⠀⢧⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⢀⣷⢣⠎⠀⠀⠀⠀⠀⠀⢹⡞⠀⠉⠛⠿⣶⣬⡒⠄⠀⠀⠀⠈⠋⠀⠀⠀⣀⡀⠀⣀⣀⣼⣦⡀⢇⠀⠀⠀⠀⠸⣧⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⢀⠀⠀⣼⣯⠏⢀⣀⡀⠀⠀⠀⠀⢸⡇⢰⠿⠿⠿⠟⠛⠛⠃⠀⠀⠀⠀⠀⠀⠀⣯⣴⣾⡿⠛⠛⠉⠉⠳⡞⡆⠀⢀⣀⢰⠻⣆⠀⠀⠀⠀⠈⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⣿⠋⠀⡜⠀⡇⠀⠀⠀⠀⢸⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠙⠛⠿⢷⣶⠀⡼⠀⠹⣀⣺⠈⡏⠇⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⡏⠀⠜⠁⠀⢀⠃⢰⠃⠀⣰⡆⠀⢸⠉⢣⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⢧⣄⣴⠋⢿⡀⢹⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⢠⠇⠀⠀⠀⠀⡞⠀⢸⢀⡞⠁⣸⠀⢸⣄⠀⢣⡀⠀⠀⠀⠀⠀⡤⠤⠤⢄⣀⠀⠀⠀⠀⠀⠀⠀⢀⠞⡟⡎⡧⠨⠀⠀⡇⠈⡇⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⢠⠇⢀⣿⠏⠀⡰⢹⠀⢸⠈⠳⣤⣥⣄⣀⣀⣀⡀⠣⡀⠀⢀⠔⠀⠀⠀⠀⠀⣀⡴⠃⣼⡇⢠⠛⡄⠀⠀⢡⠀⢺⠀⠀⠀⠀⠀⠀⢰⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⡞⠀⠀⠀⠀⡼⠀⢸⠃⠀⢠⠃⠀⡆⢸⠀⠀⠈⠉⢻⣿⣿⣿⣿⡇⠈⠉⠁⠀⠀⠀⣠⠶⠾⠧⠄⠐⣻⢠⠃⠀⠱⡀⠀⠘⡄⠈⣆⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⣰⣣⠤⠎⠉⠽⠁⠀⠘⠀⡲⠏⠀⠀⢱⢸⠀⠀⠀⠀⠀⠘⠀⣷⢄⠑⠦⠤⠒⢂⣩⡮⡞⠀⠀⠀⠀⢠⣧⠃⠀⠀⠀⢣⠠⠀⠃⠀⠘⠍⠙⢦⠤⣕⠾⡄⠀⠀⠀⠀",
            "⠀⠀⠀⠀⢰⢹⡀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠀⣄⡀⠈⢿⠀⠀⠀⣰⢞⠇⠀⠘⠀⠙⣢⣠⠖⠉⠈⠀⠀⠈⢢⣀⠀⠰⠃⠀⠀⢀⡴⡞⠉⠀⠀⠀⠀⠀⠀⠈⠀⢸⠎⡇⠀⠀⠀⠀",
            "⠀⠀⠀⢀⣾⣄⢧⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣝⠢⡌⠀⠀⢸⣿⡆⠀⠀⠀⠀⢀⡨⠶⢀⡀⠀⣠⢦⡀⣸⣿⠀⠀⠀⣠⢞⣩⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠞⡼⢱⠀⠀⠀⠀",
            "⠀⠀⠀⣼⣿⣾⣷⣍⠲⢄⡀⠀⠀⠀⠀⠀⢀⣾⣿⡷⣌⢢⡀⣾⠛⣿⡀⠀⠀⣠⡏⠀⠀⠀⢱⣘⢧⣠⢿⡏⢻⡧⢠⠟⡵⢿⣿⣧⠀⠀⠀⠀⠀⠀⣀⡤⢞⣡⣾⣾⣌⣆⠀⠀⠀",
            "⠀⠀⢰⣿⣿⣿⣿⣿⣿⣶⣬⡑⠲⢤⣀⣠⣾⣿⣿⠃⠈⢢⢻⡇⠀⠙⠷⠞⠋⠉⢷⠀⠀⢠⡋⠛⠻⠻⠟⠁⠈⣻⢇⠞⠁⣹⣿⣿⣷⣄⣀⠤⠖⣋⣥⣶⣿⣿⣿⣿⣿⣼⡄⠀⠀",
            "⠀⢠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣬⣌⣉⠙⠛⠓⠒⠚⣠⠏⠀⠀⠀⠀⠀⠀⢸⠀⠀⢸⡀⠀⠀⠀⠀⠀⠀⢿⡘⠒⠒⠚⠛⠋⣉⣨⣤⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠀⠀",
            "⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣭⣽⠿⣹⠁⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠸⡇⠀⠀⠀⠀⠀⠀⠀⠸⣿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢧⠀",
            "⢾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠋⡴⡏⠀⠀⠀⠀⠀⠀⠀⠀⡟⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠈⣿⡄⠙⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣯⢇",
            "⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠋⢠⢰⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⡇⠀⢠⣀⣀⣀⣀⣦⠀⢹⣧⠀⠈⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣮",
            "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠏⠀⠀⠘⣿⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⡇⠀⠀⠉⠉⠉⠉⠁⠀⠀⢿⣶⠀⠀⠈⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
            "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠁⠀⠀⠀⢰⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⠀⠀⠀⠈⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿",
        ],
        vec![
            "⠀⠀⠀⠀⠀⠀⢀⡤⣢⠟⢁⣴⣾⡿⠋⢉⠱⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠑⠒⠦⢄⣀⣴⠟⢡⣠⣼⣿⡿⢳⣄⡀⠀⠀",
            "⠀⠀⠀⠀⠀⢀⣾⡿⠃⣠⣿⣿⠿⠂⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢲⡿⠋⢰⣾⣿⣿⡟⠀⠀⠈⠙⢆⠀",
            "⠀⠀⠀⠀⠀⡜⠻⣷⣾⣿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣿⣁⣰⢸⣿⢻⠟⢀⠀⠀⠀⠀⠀⠁",
            "⠀⠀⠀⠀⠰⠀⠀⢙⡿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣿⣿⣿⣿⣯⡀⠀⢃⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⢠⠎⠀⠀⠀⠀⠀⠀⣼⠀⢀⠀⠀⠀⠀⠀⢠⣷⡀⠀⠀⠀⠀⡀⠄⠀⠀⠀⠀⢻⣿⣿⣿⣧⠑⠀⣢⡄⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⡰⠃⢀⠄⠀⠀⠀⠀⣼⡿⡆⢸⠀⠀⠀⠀⠀⠈⣿⢷⡄⠀⠀⠀⠱⡀⠰⡀⠀⠀⠈⢿⣿⣿⣿⣧⠀⢸⣧⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠡⢢⠋⠀⠀⠀⠀⣼⡟⠀⣇⢸⡆⠀⠀⠀⡄⠀⢿⠀⢳⡄⠀⠀⠀⢳⠀⢳⠀⠀⠀⠈⣿⣿⣿⣿⣷⣘⡟⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⢀⠇⠀⠀⠀⠀⣸⡿⢤⠤⠸⡸⣷⠀⠀⠀⢱⠀⣾⡤⠤⢿⡤⢀⡀⠀⢧⠘⡆⠀⠀⠀⢸⡟⠻⢿⠟⣿⣷⡄⠀⠀⠀",
            "⠀⠀⠀⠀⠀⡞⠀⠀⠀⠀⢰⡿⢠⠇⠀⠀⢳⣿⢇⠀⠀⠈⡇⣿⡇⠀⠀⠻⣄⠀⠀⠘⡆⡇⠀⠀⠀⠀⣇⢀⡏⠀⣿⡿⣄⠀⠀⠀",
            "⠀⠀⠀⠀⢰⠁⠀⠀⠀⠀⣿⠁⣄⣀⣀⡀⠈⢿⡜⡄⠀⠀⢹⣿⡇⠐⢄⣀⠘⢧⡀⠀⠹⣿⠀⠀⠀⠀⢸⣿⣷⣶⣿⡇⢹⡇⠀⠀",
            "⠀⠀⠀⠀⠾⠀⠀⠀⠀⢸⣧⣾⠟⢉⣽⣿⣦⠈⢷⡘⣆⠀⠸⡟⣷⣶⠟⠛⢻⣷⣦⣀⠀⢻⠀⠀⠀⠀⢸⣏⣩⣼⣿⡇⠈⣷⠀⠀",
            "⠀⠀⠀⠃⠀⠀⠀⠀⠀⣿⡿⠁⠀⣠⣾⣿⣿⠀⠈⢿⠺⡆⠀⣧⢸⠀⠀⢀⣹⣿⣿⣿⣷⣼⣤⠀⠀⠀⢸⣿⣿⣿⣿⠀⠀⣿⠀⠀",
            "⠀⠀⣠⠄⣀⠀⠀⠀⢠⣿⡇⠀⠀⢻⢻⣟⢿⠀⠀⠈⠣⠈⠓⠾⠀⠀⠀⣿⣿⢿⣿⣿⠘⡇⡞⠀⠀⢠⣾⣿⣿⣿⡏⠀⠀⢹⠀⠀",
            "⠀⠀⠛⠀⣿⠀⠀⠀⢸⣿⣿⡀⠀⠈⠃⠐⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣄⣐⣠⠏⢠⣿⠁⠀⠀⢸⣿⣿⣿⣿⠀⠀⠀⢸⠀⠀",
            "⠀⠀⠀⠀⢹⡆⠰⡀⢸⡟⠩⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⠃⠀⠀⠀⢸⣿⣿⣿⠟⠀⠀⠀⠘⠀⠀",
            "⠀⠀⠀⠀⢎⣿⡀⢱⢞⣁⣀⡿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠞⡏⡼⠀⠀⠀⣾⣿⠋⠁⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠈⠿⠻⡇⠀⠀⠒⠢⢵⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣽⠁⠀⠀⢠⡿⢹⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⡀⠀⠀⠀⠀⠀⠀⡟⣦⡀⠀⠀⠀⠈⠓⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣴⢿⡇⠀⠀⡄⣸⣇⣼⣀⣀⣀⠀⠀⠀⠀⠀⠀",
            "⡀⠀⠀⠀⠀⠀⢰⠇⣿⢸⣦⡀⠀⠀⠀⠀⠈⠲⣄⡀⠀⠀⠀⠀⠀⣀⡤⠒⢉⡴⠃⣸⠀⠀⢰⣿⣿⣿⠃⡤⠊⠁⠉⠑⢄⠀⠀⠀",
            "⡇⠀⠀⠀⠀⠀⢸⠀⣿⣾⣿⢿⠲⣄⠀⠀⠀⠀⠘⠟⣦⣤⣴⡒⠉⢀⡠⠖⠉⠀⣠⠃⠀⣠⣿⣿⡿⠁⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⢸⠀⣿⠛⢿⠈⢢⠏⠀⠀⠀⠀⠀⣰⣏⣀⣿⠗⠊⠁⠀⠀⣠⣾⠃⢀⡴⠿⠛⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⢸⢀⠇⠀⠈⢠⠃⠀⠀⠀⠀⠀⢰⠟⠁⠀⢹⢇⠀⣀⠴⠊⡱⠥⠔⠋⠀⠀⢰⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⢸⡟⠀⢀⡴⠁⠀⠀⠀⠀⠀⢠⡟⠀⠀⣰⢿⡘⣾⡅⠀⠀⠀⠀⢀⠄⠀⢠⠏⢀⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⢸⠀⣰⣿⠀⠀⠀⠀⠀⠀⢠⣿⠃⢀⡾⡇⠘⠻⡿⢷⡀⠀⠀⠒⠁⠀⢠⠏⢀⠏⣸⠃⢻⠏⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⣧⣾⣹⣿⠀⠀⠀⠀⠀⢠⠏⢉⠀⡞⣰⡇⠀⣴⣥⠞⢷⠀⠀⠀⠀⣠⠎⠀⠸⣶⠋⣠⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        ],
        vec![
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣤⣄⣤⣤⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣄⣄⣠⣤⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⢿⣿⣿⣿⣿⡢⠀⠀⠀⠀⣀⣄⣤⣶⣶⣿⣿⠶⠶⠷⢶⣾⣿⣷⣦⣶⣄⣄⢀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⢿⣿⣯⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⡿⣽⣿⣿⣿⡽⣟⣿⣷⣤⣶⡿⠿⠛⠋⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠙⠻⠿⣾⣦⣤⣾⣿⡿⣯⣿⣿⣿⣯⣟⣿⣷⣆⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⢠⣿⣿⡿⣽⣿⣿⣿⣿⣿⣽⣾⠿⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⢿⣿⣽⣿⣿⠿⣿⣿⣾⡽⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⣠⣿⣿⡿⣽⣿⡿⠛⢻⣷⡿⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣷⣶⣷⠿⢿⣿⣿⡽⣿⣿⣦⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⣰⣿⣿⡿⣽⣿⣿⢁⣴⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠢⣀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣷⣄⢀⢻⣷⣿⣽⣻⣿⣖⡀⠀⠀⠀⠀",
            "⠀⠀⠀⣼⣿⣿⢿⣽⣿⡿⣡⣿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡰⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢗⢆⠀⠀⠀⠀⠀⠀⠀⠀⠈⢿⣷⡊⢿⣿⣷⣯⢿⣿⣖⡀⠀⠀⠀",
            "⠀⠀⠀⢼⣿⣟⣿⣟⣿⣿⣿⠟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢛⢣⡀⠀⠀⠀⠀⠀⠀⠀⠀⢿⣿⣤⣿⣿⣟⣿⣿⣿⠇⠀⠀⠀",
            "⠀⠀⠀⠀⢹⣿⣷⣿⣽⣿⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡸⡅⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢣⡙⡄⠀⠀⠀⠀⠀⠀⠀⠀⢻⣿⣿⣳⣿⣿⣿⠁⠀⠀⠀⠀",
            "⠀⠀⠀⢀⣾⡿⠈⢹⣿⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠄⠀⠀⣇⠃⠀⠀⠀⠀⠀⠀⣼⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢳⡘⡄⠀⠀⠀⠀⠀⠀⠀⠀⢿⣿⠟⢇⠘⣿⣇⠀⠀⠀⠀",
            "⠀⠀⠀⣼⣿⠃⠀⣾⣿⠁⣰⠁⠀⠀⠀⠀⠀⠀⠀⡾⠀⠀⣰⣿⣆⠀⠀⠀⠀⠀⠀⡏⡆⠀⠀⠀⢳⡀⠀⠀⠀⢀⠘⣆⠹⡀⠀⠀⠀⠀⠀⠀⠀⠘⣿⡏⠼⡀⢹⣷⡆⠀⠀⠀",
            "⠀⠀⢠⣿⡇⠀⢠⣿⣏⣴⡇⠀⠀⠀⠀⠀⠀⢀⣸⡃⠀⣰⡿⠹⣿⡄⠀⠀⠀⠀⠀⣿⣿⡆⠀⠀⠀⢿⣦⡀⠀⠀⣆⢹⢰⣧⡀⠀⠀⠀⠀⠀⠀⠀⢻⣿⢂⡇⠈⣿⣷⠀⠀⠀",
            "⠀⠀⣾⡿⠀⠀⢸⣿⣿⣿⠇⠀⠀⠀⠀⠀⠀⣿⣿⠁⣰⡿⠁⠀⢹⣷⡀⠀⠀⠀⢀⣿⡿⢿⣦⠀⠀⠈⣿⣿⣄⠀⢸⠞⡬⣿⡇⠀⠀⠀⠀⠀⠀⠀⢸⣿⡆⢣⠀⢸⣿⡆⠀⠀",
            "⠀⢠⣿⠇⠀⠀⢸⠿⣿⡿⠀⠀⠀⠀⠀⠀⠀⣿⣗⣰⣿⠁⠀⠀⠀⢻⣷⡀⠀⠀⢠⢿⣷⠈⠻⣷⣄⠀⠘⣿⣿⣧⡀⡇⡒⣿⡇⠀⠀⠀⠀⢠⡄⠀⠸⣿⡇⣹⠀⠀⣿⣧⠀⠀",
            "⠀⣼⣿⠀⠀⠀⢎⢲⣿⠇⠀⣰⠀⠀⠀⠀⠀⣿⣷⣿⠃⠀⠀⠀⠀⠀⠹⣷⡀⠀⠀⣿⣿⠀⠀⠈⠻⢿⣦⣜⣿⣟⢿⣶⡡⣿⡷⠀⠀⠀⠀⢸⣱⠀⠀⣿⣗⢸⠀⠀⢹⣯⡄⠀",
            "⠀⣾⡇⠀⠀⠀⢘⣾⡿⠀⢰⣹⠀⠀⠀⠀⠀⣿⣿⡟⠀⠀⠀⠀⠀⠀⠀⠙⢿⣆⠀⢻⣿⡀⠀⠀⠀⠀⠉⠛⠿⢿⠎⠻⣿⣿⣟⠀⠀⠀⠀⢸⣤⣧⠀⢻⣿⣈⠀⠀⠘⣿⡇⠀",
            "⠈⣿⠃⠀⠀⠀⢸⣿⢃⣤⣿⣿⣇⠀⠀⠀⠀⢿⣿⠁⠀⠀⠀⠀⣀⡀⠀⠀⠈⠻⣷⣬⣿⣇⠀⠀⠀⢀⣀⠀⠀⠀⠀⠀⠈⢻⡏⠀⠀⠀⠀⣿⣿⣿⣶⣼⣿⣦⠀⠀⠀⣿⡷⠀",
            "⢰⣿⠀⠀⠀⠀⢿⣿⣿⣿⣿⡽⣿⣆⠀⠀⠀⢸⣿⠀⠀⠀⠀⢸⣑⣚⡆⠀⠀⠀⠈⠙⠿⣿⠄⠀⢰⡫⢜⡇⠀⠀⠀⠀⠀⣿⡇⠀⠀⠀⣸⣿⣟⣾⣿⣿⠿⡏⠀⠀⠀⢸⣧⠀",
            "⢸⣿⠀⠀⠀⠀⠘⣼⣿⢾⣿⣿⡽⣿⣆⠀⠀⠈⣿⡇⠀⠀⠀⠈⠶⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠳⠽⠃⠀⠀⠀⠀⠀⣿⠇⠀⠀⣰⣿⡿⣾⣿⣽⣿⢲⠁⠀⠀⠀⢸⣿⠀",
            "⢸⣿⠀⠀⠀⠀⠀⠹⣿⣾⠻⣿⣿⡽⣿⣦⡀⠀⢹⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀      ⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⡿⠀⠀⣴⣿⣿⣽⣿⢽⣿⣏⠆⠀⠀⠀⠀⠀⣿⡃",
            "⢸⣿⠀⠀⠀⢦⡀⠀⢻⣿⡄⠙⢻⣿⣽⣿⣿⣦⡈⢿⣇⠀⠀⠀⠀⠀⠀⠀       ⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⠇⣠⣾⣿⣿⣿⠋⠁⣼⣿⠋⠀⣠⡏⠀⠀⠀⣿⡆",
            "⠸⣿⠀⠀⠀⠀⠙⢗⠤⣻⣯⡄⠀⠙⢛⣿⣿⣿⣿⣾⣿⣷⣶⣤⣤⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⣠⣤⣶⣾⣿⣿⣿⡟⠋⠀⠉⢁⣠⣾⡿⣁⠤⡺⠎⠀⠀⠀⠀⣿⡇",
            "⠀⣿⡄⠀⠀⠀⠀⠀⠉⠒⠽⠿⣿⣴⣴⣿⣿⣽⣿⣽⣻⢏⡈⠉⠙⠛⠿⠻⠻⠿⠿⠿⠿⠿⠿⠿⠻⠿⠟⠛⠋⠉⠛⠟⣋⣥⣤⣶⣾⣿⠿⠟⠛⠑⠒⠋⠀⠀⠀⠀⠀⢀⣿⠀",
            "⠀⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠉⣿⣿⣾⣿⣿⣿⣿⣿⣾⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣴⣿⡟⠛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⠀",
            "⠀⣻⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣉⠙⠛⠿⣿⣤⣀⣈⣿⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣐⣿⠟⠉⠀⠀⢀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⡇⠀",
            "⠀⣿⣿⠀⢀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠳⡢⢌⠹⣿⡿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⢋⠴⡲⠝⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠄⢸⡿⡇⠀",
            "⠀⠘⣿⣧⠈⠳⡤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠳⣑⣹⣿⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⡏⠖⠉⠀⣀⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⡴⠋⣠⣿⡟⠀⠀",
            "⠀⠀⠐⢿⣷⡀⠈⠓⠽⢒⣒⣐⣂⣒⠒⠒⠐⠢⠌⣓⣢⣄⡈⢺⣿⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣡⣴⣮⣝⣐⠓⠚⠨⠽⠂⣖⣂⣖⡒⠭⠓⠉⣀⣴⣿⠉⠀⠀⠀",
            "⠀⠀⠀⠈⠻⢿⣷⣄⡀⠀⠀⠀⠀⢀⣀⣤⣴⣾⣿⠿⠿⠿⠿⣿⣿⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠿⠟⠛⠛⠛⠻⢿⣷⣦⣤⣀⣀⣀⣀⣀⣠⣴⣾⡿⠋⠁⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠙⠻⠿⣿⡿⢿⣿⠿⠿⠋⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠛⠻⠿⠻⠛⠿⠛⠋⠁⠀⠀⠀⠀⠀⠀⠀",
        ],
        vec![
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣠⢤⣤⠤⠀⢤⣠⠤⠒⠒⠒⡒⠛⢛⠛⠛⠒⠢⠤⣀⡀⠀⠀⠀⠀⠀⣀⣤⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠴⠒⠉⢀⡴⠋⠀⠀⠀⠀⠀⠄⣀⠀⠀⠈⠉⠒⢽⡢⣄⠀⢤⡀⠉⠲⣄⠰⣾⣟⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⢀⡤⠊⠁⠀⠀⣴⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠢⣄⠀⠀⠀⠉⠪⠳⡄⠉⠢⡀⠈⢷⣿⣿⢾⣿⣿⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⡠⠋⠀⠀⢀⡴⡞⠁⠀⠀⠀⠀⢸⠁⠀⠀⠀⠀⠀⠀⠈⠃⠀⡄⠀⠀⠐⢜⢆⠀⠑⡄⠀⣿⣿⣟⣿⣿⣿⠛⠉⠛⠛⠳⣦⡀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⢀⠞⠁⠀⢀⣴⠏⢰⠃⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⡆⠀⠀⠈⢞⣆⠀⠈⢆⠸⣿⣿⢾⣿⣿⡂⠀⠀⠀⠀⠀⠻⣆⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⢀⠟⠀⢀⣠⣿⠃⢀⡇⠀⠀⠀⠀⠀⠀⣼⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⠀⠀⠀⠸⡼⡄⠀⠘⣇⢿⣿⣟⣿⣿⣧⠀⠀⠀⠀⠀⠀⠸⡄⠀⠀⠀⠀",
            "⠀⠀⠀⢠⠏⠀⢠⠣⣰⠃⢀⠞⡇⠀⠀⠀⠀⠀⠀⡧⠘⢦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣇⠀⠀⠀⢹⢳⠀⠀⢹⡞⣿⣿⣞⣿⣿⣆⠀⠄⠀⠀⠀⠀⠙⡄⠀⠀⠀",
            "⠀⠀⢀⠏⠀⣠⠣⢱⠃⠀⠈⢸⡇⠀⠀⠀⠀⠀⠀⢹⠀⠀⠹⣤⠀⠐⢤⠀⠀⠀⠀⠀⣿⡀⠀⠀⠀⡟⡇⠀⠀⣿⣸⣿⣿⡽⣿⣿⣄⠀⠀⠀⠀⠀⠀⢹⡄⠀⠀",
            "⠀⠀⡞⠀⣰⠃⢆⡏⠀⠐⢀⡟⢱⡀⠠⡀⠀⠀⠀⢸⡆⠀⠀⠈⠳⣄⠀⠑⡄⠀⠀⠀⢸⡇⠀⠀⠀⢹⢳⠀⡀⢻⣶⢻⣿⣿⢿⣿⣿⣄⠀⠀⠀⠀⠀⠀⢣⠀⠀",
            "⠀⢸⠃⢰⠇⢸⢸⠃⠀⣠⡾⠀⠀⢧⠀⢱⠀⠀⠀⠀⡇⠀⠀⠀⠀⠈⠳⢄⡈⠂⠀⠀⣸⡃⠀⠀⠀⢸⢸⠀⢣⣼⣿⡏⣿⣿⡿⠿⠟⢻⡄⠀⠀⠀⠀⠀⠘⡄⠀",
            "⠀⡏⢀⣿⠀⡏⣿⠀⢀⣿⣃⣀⣀⣀⣳⡳⣀⠀⠀⠀⢻⠀⠀⠒⠒⠶⠖⠒⠓⢦⣀⠀⢱⠃⠀⠀⠀⠠⡼⠀⢸⣿⣿⣏⢾⠁⠀⠀⠀⠸⣧⠀⠀⠀⠀⠀⠀⢡⠀",
            "⢰⠇⡾⣿⠀⠃⣿⠀⢸⡏⠉⠈⢉⣩⣩⣟⣿⣧⡀⠃⠘⡇⠀⠀⠀⠠⣲⣮⣤⣤⣽⣿⡚⠀⠀⠀⢀⠘⡇⠀⣿⣿⣿⣯⢸⠀⠀⠀⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀⡄",
            "⢸⢰⠇⣿⠀⠀⢹⡀⢸⡇⣠⣶⣿⣿⣿⣷⣽⠻⣗⢆⢠⢻⡀⠀⢠⣾⠿⣿⣿⣿⣿⠿⣇⠀⠀⠠⠀⣾⠃⢰⣿⢻⣿⣿⣿⣶⣤⠄⠀⠀⢿⡀⠀⠀⠀⠀⠀⠀⢃",
            "⣼⡎⠀⣿⠀⠀⣎⣇⢼⡿⣿⠉⣯⣽⣿⣿⡞⠁⠈⠻⣝⢯⣧⡀⠀⠸⣿⢿⣿⣿⣿⠀⢘⣇⠀⠀⢰⡏⠀⣾⡟⢸⣿⣿⡿⡿⢿⠀⠀⠀⢸⡆⠀⠀⠀⠀⠀⠀⠸",
            "⢻⠃⠀⢹⠀⠀⠸⣞⢿⣿⡙⠄⠻⠿⠿⠿⠇⠀⠀⠀⠀⠉⠹⣦⠀⠀⠈⠉⠉⠁⠀⠀⠈⢸⡄⠐⡞⠀⣼⣿⠁⣸⣿⣿⣿⣿⡏⠀⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀",
            "⠈⠀⠀⠘⡇⠠⠀⣿⠈⣿⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⡿⠁⣼⣿⠃⠀⣿⡿⣽⣿⣿⠇⠀⠀⠀⠘⣿⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⢿⠀⠀⣾⣇⢻⣿⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⣃⣾⣿⣿⠀⢰⣿⡿⣽⣿⣿⠀⠀⠀⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⢸⡆⠀⣿⣿⣌⣿⣷⡀⠀⠀⠀⠀⠀⠀⠀⢀⢀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣼⠟⢡⡟⣿⡿⢀⣿⣿⣿⣿⣿⡏⠀⠀⠀⠀⠀⢻⡀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⢸⣇⠌⠿⣿⣷⣞⢻⣧⡀⠀⠀⠀⠀⠿⠗⠶⠒⠛⠃⠀⠀⠀⠀⠀⠀⢀⡰⣻⡟⢠⣾⣿⡿⡷⠿⠿⢿⣿⡿⠿⠃⠀⠀⠀⠀⠀⢸⡁⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⣼⢻⣌⠀⠛⣿⣿⣶⢽⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⡿⣾⠿⣠⣿⠋⠁⠘⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⡇⠀⢿⠀⡃⢽⣿⣿⣦⣽⣿⣿⡖⢤⡀⠀⠀⠀⠀⠀⠀⣠⠤⠖⠋⠁⢰⣯⣿⣿⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⡇⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⡇⠀⠀⢰⠁⠚⣿⣿⣿⠛⣿⣿⡇⠀⠈⠑⢢⣤⣤⡖⠋⠀⠀⠀⠀⣤⣾⣿⢻⡟⣿⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⢸⠇⠀⠀⡘⢌⡱⢽⣞⡿⠀⣻⢿⡇⠀⠀⢀⣼⣥⡀⠀⠀⣠⡤⠒⠋⠉⠉⣿⠀⠈⠈⠙⠻⢦⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⣼⠀⠀⠀⡘⠤⡹⢸⣯⡇⠀⣿⢸⡇⠀⠀⢼⠁⡀⣩⣷⣯⡁⠀⠀⠀⠀⢄⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⡟⠀⠀⠀⢘⠠⣝⠢⢿⡇⠀⣟⢸⡗⠀⠀⡞⠐⣾⠟⠅⠈⢷⣄⠀⠀⠀⠀⠈⢦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀",
            "⠀⠀⢀⠇⠀⠀⠐⠨⢐⢣⡑⢾⡇⠀⡿⢸⣏⢀⡼⢠⣾⣿⠁⠀⢀⣠⣿⣆⠀⠀⢀⡈⠁⢙⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀⠂⠀⠀⠀⠀",
            "⠀⠀⢸⠀⠀⠀⠀⡍⢢⢃⡜⣹⡇⠀⡏⣼⠟⠉⣰⣿⡿⣿⣤⣤⣔⡿⠀⠹⣧⠐⠧⠋⣠⣾⠟⠛⠷⠤⠤⠒⠒⠒⠤⣄⡀⠀⠀⠀⠀⠀⠀⠀⠸⡇⠀⠀⠀⠀⠀",
            "⠀⠀⡞⠀⠀⠀⠠⣐⣢⡧⠼⠿⠧⠴⢿⠃⣠⣾⠟⠋⢀⡟⠀⠀⣸⠃⠀⠀⠙⢷⣆⣾⡟⣻⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠲⣄⠀⠀⠀⠀⠀⠀⡇⠀⠂⠀⠀⠀",
            "⠀⢠⡇⠀⠀⣠⠞⠉⠀⠀⠀⢀⣠⢢⣷⠾⠋⠁⠀⢀⡞⠀⠀⠀⡏⠀⠀⠀⠀⠀⠉⠉⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢦⠀⠀⠀⠀⠀⣿⠀⠀⡀⠀⠀",
            "⠀⢸⠀⢠⠞⠁⠀⠀⠀⠀⢀⣾⢃⠎⠀⡀⡀⢀⢠⠞⠀⠀⠀⢰⠇⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣇⠀⠀⠀⠀⢿⠀⠀⠀⠀⠀",
        ],
        //MARK: large art
        vec![
            "⠀⠠⢄⢠⢲⣒⠧⡐⢄⢢⠰⢠⠄⡀⠰⢢⠔⡢⠔⡢⢄⠀⠆⡴⡰⢠⠤⡀⠐⠹⢶⣒⢖⢢⢖⡰⢂⠦⡤⡀⠠⡀⠀⠲⢒⡔⣢⠔⡰⢠⠄⢢⡐⢠⢂⡔⢠⠄⡄⠂⡄⠠⢀⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⢈⠱⡌⢢⢯⡝⢢⡉⢦⡑⢎⡡⢎⡐⠈⣇⢮⢱⡹⣰⣉⠦⠘⢰⢻⣦⠳⣱⠀⢀⠈⢻⣿⣔⡧⣽⢩⣎⡵⣩⡤⠐⢆⠀⠈⠑⠃⠞⢱⠣⡝⡴⣈⠇⢦⡘⢆⠎⡔⡡⢂⠅⠢⢌⠠⢁⠈⠄⠂⠁⠠⠀⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⢠⠣⡜⢣⡛⡌⠧⣜⡡⢚⠥⣚⠤⣊⠄⢺⣬⠳⡼⣱⢎⡼⡀⠀⠑⣾⣧⡳⣍⠀⠣⢄⠘⢿⣾⣵⣻⣜⣷⣣⢿⡵⡈⢧⡀⠃⠜⣠⢂⢄⡉⠐⠹⣌⠧⣘⡍⡜⢢⡱⠌⣌⠑⡠⠒⡈⠄⢂⠡⠈⠄⡐⠀⡈⠀⠁⠀⠀⠀⠀⠀⠀⠀",
            "⢄⡳⡜⢧⡽⣖⡳⣬⢳⡝⣮⣜⡱⢆⠢⢹⣎⡿⡵⢯⣞⡵⡃⢈⣆⠐⢿⣽⡚⣥⠘⣦⡁⠈⢻⣯⣿⢾⣭⣟⣯⢿⣵⡈⢷⡄⢡⠂⡄⠈⠘⠕⣆⣀⠘⠐⠌⡐⠃⠤⠑⡠⠊⠄⡑⠠⠈⠄⠂⢁⠠⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀",
            "⣌⡳⣝⢧⣿⣻⣷⣭⢗⣻⢶⣭⢳⣍⢎⠰⣿⣻⡟⣿⢯⣷⡁⢠⢿⣧⡈⢳⣿⡳⠆⢹⣿⣅⡂⢹⣿⢿⣾⣟⣾⡿⣾⣷⠘⣿⡄⠳⣌⢆⠀⠀⠀⠈⠳⣦⡀⠀⠈⠀⠀⠀⠐⠀⠀⠀⠁⠀⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⢢⣟⣼⢻⣷⣿⣷⡿⣿⣽⣳⣯⢷⣎⡎⡄⣿⣷⣿⢿⣻⢾⠁⣸⠘⣿⣷⡄⠻⣧⡟⠀⣿⣿⣦⡀⠙⣿⣿⡾⣿⣽⣿⣽⣆⢸⣿⡄⠹⣞⡕⡀⠀⠀⠀⠀⠙⢦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⣹⢾⡽⣻⣿⣾⡿⣿⣷⣻⣷⢯⣿⢶⡹⠄⣿⣿⢾⣿⢿⣻⠀⣷⡂⣿⣿⣿⠄⠹⣿⣃⠈⣉⣉⣉⣀⠙⣷⣿⢿⣽⣾⣟⣿⡀⢿⣷⡀⠛⣿⣮⣆⠀⠀⠀⠀⠀⠙⢦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⢫⣿⣽⣻⣷⡿⣿⣿⡿⣽⡿⣿⣾⢯⡟⡆⣹⣿⡿⣯⣿⡏⢘⡮⠓⠉⣁⣤⣴⣆⠹⣷⠈⣿⣿⣿⣿⡄⠙⣿⣿⢯⣿⣾⢿⣇⢸⣿⣷⠀⠱⢾⣻⣧⡀⠀⠀⠀⠀⠀⠳⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⢸⣷⡿⣽⣾⢿⣿⣷⣿⢿⣻⣿⢾⡿⣝⠦⢹⣿⣟⣿⣷⠁⢁⣤⣶⣿⣿⣿⣿⣿⣧⠸⡆⢻⣿⣿⣿⣯⣃⠸⣿⡿⣯⣿⡿⣿⠀⣿⣿⡆⠀⠈⠳⣟⣷⡄⠀⠀⠀⠀⠀⠈⢣⡀⠀⠀⠀⢀⠀⠀⠀⠀⡈⠐⠀⠂⠀⠀⠀⠀⠀⠀⠀",
            "⡇⢿⣟⣿⣽⡿⣷⣿⣯⣿⣿⣽⣿⣻⡽⢎⢸⣿⣿⣽⡞⢰⣟⣿⣿⣿⣿⣿⣿⣿⣿⣧⠀⠘⣿⠻⣿⡟⢁⣆⠹⣿⣟⣷⣿⢿⡇⣿⢿⣿⠀⠀⠀⠙⢯⣿⣆⠀⠀⠂⠀⠀⠀⠱⡀⠀⠀⠀⠀⠀⠀⠀⠄⡀⢀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⣿⡸⣿⣏⣿⣹⣿⣷⣿⢿⣾⡿⣾⣿⣹⠇⣸⣿⣷⣿⠁⣾⣿⣿⣿⣿⣿⡿⣁⣿⣿⡿⠇⠀⠀⠀⠀⠀⠉⠏⠀⢹⣿⣏⣿⣿⡇⢹⣿⣿⡇⠀⠀⠀⠀⢿⣹⣇⠀⠀⠀⠆⠀⠀⢱⠀⠀⠀⠀⢀⠀⠰⠆⡀⠀⠀⠀⠀⠰⠀⠀⠀⠀",
            "⢿⣧⢻⣿⣽⣿⣳⣿⣯⣿⣯⣿⣟⣷⣯⠃⣿⣿⣽⡏⣰⣿⣿⣿⣿⡿⢋⣴⡿⠛⠁⣠⣤⠤⠶⠾⠶⣶⣦⣄⠈⠈⣿⣯⣿⣾⡇⢸⣿⣟⣧⠘⣁⠀⠀⠀⠙⣿⣆⠀⠄⠀⠀⠄⠀⢡⠀⠀⠴⠀⠠⠘⡐⡆⠘⠄⡀⠀⠄⠐⠀⠀⠀",
            "⡞⣿⡎⢿⣾⣻⣽⣷⡿⣽⣾⣯⣿⣻⡼⠃⣿⣟⡾⢠⣿⣿⣿⣿⠟⣱⡿⠋⠀⣠⡾⠋⠀⠀⢀⠀⠀⠀⠉⠻⣷⣄⠸⣿⡷⣿⣇⢸⣿⡿⣿⡄⢯⢄⣀⠀⠀⠈⢻⣦⠀⠀⠁⠀⠀⠀⢂⠀⠂⠀⠀⠄⡑⡀⠀⠁⠒⠀⠊⢐⠀⠀⠀",
            "⣷⡘⣿⡜⣿⣻⣽⣾⢿⣟⣷⣿⣳⡿⣍⢣⣿⡿⢀⣿⣿⣿⣿⣿⣾⡟⠱⠀⣴⡽⠁⠀⠀⠆⣐⣬⣐⠀⠀⠀⠙⣿⡆⢻⣟⣿⡇⢸⣿⣿⢿⡇⠈⠀⣿⣿⡄⠀⠀⠙⣷⡀⠈⠀⠀⠀⠈⢀⣀⠀⠀⢠⠐⠃⠚⠀⠀⠀⠔⢈⠀⠀⠒",
            "⠿⠇⠘⣿⡘⢿⣯⡿⣟⣯⣿⢾⡿⣽⢣⢸⣿⢁⣾⣿⣿⣿⣿⣿⣿⣧⠆⠀⣿⣿⣾⣷⣆⢈⡿⣞⣟⣷⡄⠀⠀⠘⣷⠸⣿⣯⡇⢸⣿⣯⣿⡇⢸⠀⢹⣿⣿⡀⠀⠀⠈⢻⣄⠀⠂⠈⠀⠘⢽⠀⠀⠠⢉⢦⠰⠀⡀⠄⣀⠨⠀⠀⠀",
            "⠀⣴⣧⠘⣿⡌⢿⣟⣿⢯⣿⣟⡿⣝⠆⡿⠃⣼⣿⣿⣿⣿⣿⣿⣿⣿⣸⠈⣿⣿⣿⣿⡿⠀⠹⣯⣛⣯⡆⢸⣯⣄⠙⠀⣿⣟⡇⣾⣿⣽⡿⡇⠘⠁⠀⠻⣿⡇⠀⠀⠄⠀⠹⣆⠀⠀⠀⠀⠈⠜⠁⠀⠨⠠⢉⠔⡁⠊⠄⡘⠀⠀⠈",
            "⠀⢻⣿⣧⡘⣧⠆⢻⣯⣿⣻⣾⢿⡱⢲⢃⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡄⣿⣿⣷⡀⠀⠀⣶⣌⡙⠓⢂⣼⣿⠟⣀⡅⢻⣿⠀⣿⣽⣾⣿⠃⠀⣀⣀⣰⣿⡇⢰⠀⠀⠀⡀⠈⢧⡀⠀⠀⠀⠀⠀⠀⠀⠁⢂⠐⠠⢁⠂⠄⠐⠀⠀",
            "⡩⠀⢿⣿⣷⡈⢟⡄⢹⡿⣽⣛⣮⠃⢡⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣄⠸⣿⣿⣿⣿⣿⡿⢋⣼⣿⡇⢸⡇⢰⣿⣯⣷⣿⠀⠀⢿⣿⣿⣿⠆⠈⢱⠀⠀⢀⠀⠈⠳⡀⠀⠀⠃⠀⠐⠱⠚⠲⠞⠳⠦⠈⠀⠀⠀⠤",
            "⡱⢁⠘⣿⣿⣿⡄⠙⠄⡉⢻⡿⣅⠀⢺⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣟⠻⢿⣷⣌⠻⠿⠟⣫⣴⣿⣿⣿⡇⢸⠃⠈⣿⣽⣷⡇⠀⠀⢸⣿⡾⠟⠀⠀⠈⢆⠀⠀⠀⠄⠀⠱⡄⠀⠠⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⣱⠩⡀⠸⣿⡍⠀⣀⠀⠀⠈⢻⠖⠀⠀⠘⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣿⣿⣶⣶⣾⣿⣿⣿⣿⣿⣿⠇⡛⡰⢈⣿⣻⣾⠁⠀⠀⠉⠁⠀⠀⠀⠀⠀⠈⠆⠀⠀⠀⠀⠀⠘⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⣇⠣⠜⡀⢠⡅⠀⣿⣷⡄⢀⢀⢇⣦⠀⠀⠀⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠂⢁⡏⢸⣟⡞⡟⠀⠀⠀⠀⠀⠀⢀⣀⣤⣶⣾⣿⣿⣿⣶⣤⡀⠀⠈⢦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠸⣹⡐⠡⠀⢠⠀⣿⣿⠿⠆⠀⢞⡦⢯⠐⣦⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣸⡇⣸⢯⡿⠁⠀⠀⠀⢀⣠⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⠀⠀⢣⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠑⢌⣃⠂⠀⠣⣹⣿⣦⠀⠀⣦⡈⠃⠀⡁⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣇⣰⣿⠃⣼⢯⠓⠀⠀⠀⠀⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡀⠀⠡⠀⠀⠀⠠⡄⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⢍⡀⠀⠙⢟⣿⣷⣄⠻⣿⡿⠃⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⠀⣯⡜⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠀⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠄⠀⠀⠡⣤⣬⣥⣤⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠠⢘⠶⠁⠀⠀⠀⠀⠀⢹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠋⡠⠁⠬⠁⠀⠀⠀⠀⠀⠀⠸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠇⠀⠀⠀⠀⠀⠀⠀⠁⢂⠐⡀⠄",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⠁⡴⠋⠀⠁⠀⠀⠀⠀⢰⡇⠀⠀⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠆",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡁⠉⢙⣿⣿⣿⣿⣿⣿⣿⣿⠟⠠⠮⠁⠀⠀⠀⠀⠀⠀⠀⣾⣷⠀⠀⠘⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⢂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⢀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣯⣿⣿⣿⣿⣿⣿⠟⠀⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⣿⡇⠀⠀⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠏⣠⣾⠀⢂⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⢂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠛⠿⠿⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⣿⣿⣿⣿⠀⠀⠀⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠋⣰⣿⣿⠄⠡⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠐⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠲⠖⠢⠀⢌⡉⠙⠛⠛⠛⠛⠛⣁⠤⠀⠀⠀⢀⠀⠀⠀⠀⠀⠀⢀⣼⣿⣿⣿⣿⣿⣇⠀⠀⠸⣿⣿⣿⣿⣿⣿⣿⡿⠁⢸⡿⣿⣿⠀⠈⠡⠀⠀⠰⢄⠢⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⢸⡏⢹⣷⠀⢀⠀⠀⢎⣱⢹⡸⢈⡹⡎⣀⠀⠀⢀⠀⠀⠀⠀⢀⣿⡿⣿⣏⣿⣹⣏⣿⠀⠀⠀⢿⣷⣿⣹⣿⣹⡿⠁⣶⠈⢸⢷⡏⢀⠀⠁⠀⠀⢾⣆⢉⡆⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⠀⠀⠀⠀⠀⠀⠸⣤⡑⢨⢇⠲⡩⢆⡣⢄⡀⢂⡀⣠⣼⢿⡷⡿⣯⣟⣷⣻⢯⡿⡇⠀⠀⠸⣧⢿⣽⣾⠏⢁⣾⣻⠀⢈⠻⠀⣞⣧⢦⡄⢸⠢⠁⠀⠈⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⢀⠡⢈⠡⠒⠀⠀⠐⢻⠳⢮⡶⣜⢦⣣⡤⣤⣤⠟⣗⣏⡾⣵⣻⣞⣞⣳⢯⣟⢳⣇⠀⠄⠀⢷⣋⣷⣋⢤⡟⣾⢝⠂⠠⠀⢘⡳⣬⠞⣅⠘⠂⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠁⠂⠄⢃⠢⢑⢂⡀⠀⠈⣟⡱⣚⢬⠗⣣⠝⣦⢫⠞⡭⣞⡱⢧⡳⣍⢾⡱⣞⢎⣧⢳⠀⠀⠀⢈⠵⣣⢟⢮⢽⡸⠋⠀⢰⣙⠀⢹⣊⠽⣜⠲⡄⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⢁⠘⡨⠄⡊⠔⡂⡔⠀⠀⢎⠱⣘⠪⡹⢡⠛⠴⠋⡞⣱⢣⡝⢬⡓⡭⢎⡵⣌⢏⠶⣩⠇⢈⢠⠈⡞⡵⢎⡳⢎⡑⠁⠀⡸⣜⠀⢠⠓⠞⡬⡱⡈⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⠀⢂⠡⢂⠥⡉⢔⡈⠆⢀⠀⠈⠄⠣⢡⢃⠹⡈⠣⠜⢂⢣⠚⡥⡱⡙⡜⢦⡙⢎⡱⢃⠆⠈⠀⠀⡜⡱⢎⡕⠊⢀⠌⠀⠰⡘⡄⠀⢐⠎⢥⠓⡡⠂⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠰⢁⠂⠀⠀⠀⠀⠠⢀⠀⠁⠂⠜⢠⢃⡌⠀⢂⠄⠀⠀⠀⠊⠐⠨⠁⡜⠈⠢⢁⠢⠡⢱⠘⠤⣉⠦⠱⣉⠆⠀⠀⠀⡸⢑⢎⠈⠠⠌⠀⠀⢒⠱⡘⠄⠈⡜⢢⠉⠔⠁⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⠀⠀⠀⠀⠀⠐⠀⡀⢂⠐⠠⡀⠄⡀⠈⠂⠈⠘⡄⢣⢁⡀⠠⠐⠂⠀⠀⠁⠂⠀⠃⠂⠁⠂⠐⠄⠃⠐⠂⠀⠀⠀⠡⠋⠀⠌⠂⠀⠀⡈⠆⢂⠱⠀⡈⠔⢂⠉⡄⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠀⠈⠄⠐⠈⠄⡁⢂⠐⢀⠈⠐⠤⢈⠱⠀⡔⠠⡀⠄⡀⣀⠂⠡⠌⢂⠡⠒⠰⡀⠄⡀⢀⠠⢀⢂⠉⠀⠀⢀⠂⡘⠠⡁⠀⡐⠈⠔⡈⠰⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠂⠁⠠⠈⠐⠠⠀⠄⡀⠂⡀⠄⡀⢀⠀⠁⠐⠁⠐⠈⠐⡀⠌⠡⠘⠠⢁⠊⠡⠐⡡⠈⠄⢂⠁⠂⠀⠀⠀⠂⠐⠁⠂⠀⠐⢀⠁⠂⠐⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⡈⢀⠡⠀⠀⢁⠀⠂⠐⠀⠂⠁⢀⠂⠐⢤⠄⡀⠈⠠⠁⠂⠄⡈⠄⠁⡀⠁⠀⠂⠀⠀⠀⠀⠀⠄⠀⠀⡀⠐⠈⠀⠠⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        ],
        vec![
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣴⢿⠶⢦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⡴⠛⠁⠀⢸⣶⠀⣿⣂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⠖⠋⠁⠀⠀⠀⠀⠀⣘⡷⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⠞⠋⠀⠀⠀⠀⠀⠀⣀⡴⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⠖⠋⠀⠀⠀⠀⠀⠀⣠⡴⠛⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡀⠀⠀⠀⠀⠀⠀⠀⢀⣠⠖⠋⠀⠀⠀⠀⠀⠀⣤⣴⣛⣁⣀⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣧⣄⠀⠀⠀⠀⢀⡴⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠈⠉⠉⠉⠉⠉⠉⠙⠛⠓⠒⠶⠦⢤⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢳⡈⠙⠲⣤⡞⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⢦⣹⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢷⣴⠞⠁⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⣴⡴⣴⠴⣴⢤⣴⢤⣤⣤⣤⣤⣤⣄⣀⣀⣀⣹⣼⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡴⠋⠁⠀⠀⠀⠀⠀⠀⢀⣤⠖⠛⠛⠚⠒⠒⠦⠶⠤⠤⠴⠤⠴⠒⠒⠒⠲⠤⢤⣄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡴⠋⠀⠀⠀⠀⠀⠀⢀⣠⠴⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠑⠢⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡾⠉⠀⠀⠀⠀⠀⠀⢀⣶⣿⣷⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠷⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡴⠋⠀⠀⠀⠀⠀⠀⣠⠶⠋⠁⠀⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠳⣌⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡴⠋⠀⠀⠀⠀⠀⣀⣴⠚⠉⠀⠀⠀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠀⠀⠀⠀⠈⢳⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡴⠋⠀⠀⠀⠀⢀⣠⠾⣿⣾⡇⠀⠀⢀⡼⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣼⡶⠀⠀⠀⠀⠀⠀⢀⡀⠀⠀⠀⠀⠀⠀⠀⠙⣆⠀⠀⠀⠀⠀⠹⣄⠀⢀⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣰⠏⠀⠀⠀⠀⣠⣴⣿⢷⣿⠟⠃⠀⠀⢀⣾⠁⠀⠀⠀⠀⠀⠀⠀⠀⣠⠞⣹⠃⠀⠀⠀⠀⠀⠀⠀⣾⡇⠀⠀⠀⠀⠀⠀⠀⠀⠘⣆⠀⠀⠀⠀⠀⠸⣄⡞⢁⣬⠙⢳⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⡟⠛⣲⡞⠁⠀⠀⢀⣠⡾⠛⠉⣠⡟⠃⠀⠀⠀⢠⡞⠁⠀⠘⢦⡀⠀⠀⠀⢀⡼⠁⢰⠇⠀⠀⠀⠀⠀⠀⠀⣸⠛⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣆⠀⠀⠀⠀⢀⡟⢀⣾⠏⣠⠻⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⡟⠀⢹⣾⠃⠀⠀⣀⣴⡟⠛⢶⣶⡾⠃⠀⠀⠀⠀⢀⡞⠀⠀⠀⠀⠈⢳⡄⠀⣰⠏⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀⢀⡏⠀⢹⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⣻⡄⠀⠀⠀⡞⢀⣾⡟⢠⠃⢘⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡏⠉⣷⠀⠀⣙⣿⠶⠾⢿⣵⡙⣧⢰⡟⠁⠀⠀⠀⠀⠀⡼⠁⠀⠀⠀⠀⠀⠀⢹⣞⡃⠀⠀⠀⡿⠀⠀⠀⠀⠀⠀⠀⣾⠁⠀⠀⢷⡀⠀⠀⠀⣀⣤⠾⠂⠀⠚⣧⠀⠀⡼⢁⣾⡟⣰⠓⠒⠒⠿⢶⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣸⣇⠀⠈⠷⣤⡟⠁⠀⠀⠀⠉⠐⣿⡞⠀⠀⠀⣠⡦⠀⢰⠇⠀⠀⠀⠀⠀⠀⢠⡞⠙⢧⡀⠀⢰⡇⠀⠀⠀⠀⠀⠀⢠⡏⠀⠀⠀⠘⣧⣠⠴⠚⠉⠀⠀⠀⠀⠀⣿⠀⣰⠃⣾⡟⣲⠃⠀⠀⠀⠀⣾⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⡏⠙⣦⡀⢰⡟⠀⠀⢀⡄⠀⠀⢸⡿⠀⢀⣰⡎⢹⠁⠀⢾⠀⠀⠀⠐⠻⣤⣠⠟⠀⠀⠈⠻⠄⠺⣇⠀⠀⠀⠀⠀⠀⣾⠁⢀⣠⡴⠛⢻⡄⠀⠀⠀⠀⠀⠀⠀⠀⣿⣧⡇⣾⣿⣿⠃⠀⠀⠀⠀⠀⣾⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⡿⣄⠀⠙⢻⠀⠀⢀⣾⠁⠀⠀⣼⠇⣠⣿⠁⠀⡟⠀⢀⡟⠀⠀⠀⠀⠀⢠⠟⠳⢦⣀⠀⠀⠀⠀⡿⠀⠀⠀⠀⠀⠲⣿⠴⠋⠀⠀⠀⠀⣷⠀⡄⠀⠀⠀⠀⠀⢀⣿⣯⣇⢾⢇⣿⠄⠀⠀⠀⠀⠀⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢘⣧⡈⠙⠒⠾⣦⣲⡾⠁⠀⠀⢀⣿⣴⢛⡟⠀⢸⠃⠀⢐⡗⠀⠀⠀⠀⢠⠟⠀⠀⠀⠈⠓⠦⣄⣀⣿⣀⠀⠀⠀⠀⢠⡏⠀⠀⠀⢀⣀⡤⢾⡟⠃⠀⠀⠀⠀⠀⢨⡇⢸⣼⣿⣿⡟⠀⠀⠀⠀⠀⢰⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡾⠊⠻⠦⣤⣴⠟⠂⠀⠀⠀⠀⢘⣿⢁⣰⠃⠀⢸⠀⠀⢸⠄⠀⠀⠀⢀⡞⠀⠀⠀⣀⣤⠴⠚⠋⠀⠹⡇⠀⠀⠀⠀⣼⣤⡤⠖⠛⠉⠀⠀⠀⣇⠀⠀⠀⠀⠀⠀⣸⣷⢼⠘⣿⣿⡄⠀⠀⠀⠀⠀⠰⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⢀⣼⠁⠀⠀⢰⡞⢹⣦⡀⠀⠀⣠⡀⢸⠏⣲⡏⠀⠀⣻⠀⠀⣸⡅⠀⠀⢠⣾⣠⠴⠖⠋⠁⠀⠀⠀⠀⠀⠀⠻⡅⠀⠀⢀⡏⠉⠓⠦⣄⡀⠀⠀⠀⣿⣄⠀⠀⠀⠀⠀⣿⡋⠻⣦⣟⣻⠁⠀⠀⠀⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⣸⠃⠀⠀⢰⠟⠀⢸⠂⡏⠉⠙⠻⢤⡟⣲⡟⠀⠀⠀⣧⠀⣼⠛⡇⠀⠀⡾⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠻⡄⠀⣼⠃⠀⠀⠀⠀⠉⠓⢦⣀⣺⡆⠀⠀⠀⠀⠀⣾⣥⡀⣸⡿⡟⠀⠀⠀⠀⠀⠀⢸⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⢠⡏⠀⠀⣰⢟⠀⠀⣸⠀⡇⠀⠀⢠⡞⢒⡿⠁⠀⠀⠀⣻⣴⠓⠚⣧⠀⣸⡇⠀⠀⠀⠀⠀⠀⠀⣼⠶⢤⣄⣀⡀⣀⠹⣄⡏⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⣷⠤⠀⠀⠀⠀⣧⣾⡟⡟⠠⡇⠀⠀⠀⠀⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⢀⣾⠀⠀⣰⠏⠂⠀⢰⣿⠀⣧⣠⠶⠋⢀⣼⠁⠀⠀⠀⠀⠘⠏⠀⠀⢿⡀⡿⢷⠀⠀⠀⠀⠀⠀⣸⠳⠀⠀⠀⠀⠉⠉⠉⠉⠉⠉⠉⠳⡄⠀⠀⠀⠀⠀⠀⣿⡂⠀⠀⠀⣸⣿⡿⢠⠃⠐⡇⠀⠀⠀⠀⠀⠀⢸⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠸⢷⣄⣰⠋⠀⠀⠀⢸⣿⠀⠈⠁⢀⣰⠞⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣇⠘⢷⣄⠀⠀⠀⢰⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⡆⠀⠀⠀⠀⢸⢟⡇⠀⠀⢀⣿⣿⠁⡎⠀⢠⡇⠀⠀⠀⠀⠀⠀⢸⡂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠀⠀⠀⠀⠘⠻⡦⢶⣾⣏⡁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢧⠀⠀⡟⠳⢤⣀⢸⡃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣷⠀⠀⠀⠀⣾⠀⣧⠀⠀⣸⣿⣟⣺⠃⠀⢸⡇⠀⠀⠀⠀⠀⠀⢸⡅⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⢙⡟⠒⠒⠢⠤⢤⣄⣀⣀⠀⠀⠀⠈⢧⡀⡇⠀⠀⣽⠛⠿⢤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣆⣀⣠⣴⡏⠀⣿⠀⢀⡟⠚⠛⠁⠀⠀⢼⡇⠀⠀⠀⠀⠀⠀⢸⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠞⡇⠀⠀⠀⠀⠀⠀⠀⢈⡟⠑⠒⠤⣄⣿⣧⢀⣴⡋⠀⠀⠀⣨⣿⡏⠛⣟⠒⣿⠚⢺⡿⠶⣾⡛⠛⠉⠉⠀⡾⢀⣾⢻⡀⡿⠁⠀⠀⠀⠀⠀⠘⡇⠀⠀⠀⠀⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠏⢠⡟⠀⠀⠀⠀⠀⠀⠀⣸⠁⠀⠀⠀⠀⠙⠻⠛⠁⠉⠙⠒⢦⡟⠀⠻⣤⣿⣿⣅⢲⡿⠉⣄⡏⠙⣆⠀⠀⣾⠟⠋⠀⢾⡿⠁⠀⠀⠀⠀⠀⠀⢸⡧⠀⠀⠀⠀⠀⠀⠰⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡴⠁⠀⡾⠀⠀⠀⠀⠀⠀⠀⢠⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⠀⠀⢀⡟⢶⡏⠙⠁⠀⣾⣣⡀⠘⣧⣴⣃⠀⠀⠀⠙⠁⠀⠀⠀⠀⠀⠀⠀⠰⣿⠀⠀⠀⠀⠀⠀⠐⣿⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠊⠀⠀⡼⠁⠀⠀⠀⠀⠀⠀⢀⡞⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⠀⠀⣸⠁⣸⡇⠀⠀⠀⠻⣿⢷⣀⣸⡄⢹⡍⠙⠋⠛⠲⡖⢦⡀⠀⠀⠀⠀⠀⢸⡄⠀⠀⠀⠀⠀⠀⢻⠛⣤⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠴⠋⠀⠀⢀⡴⠁⠀⠀⠀⠀⠀⠀⠀⡼⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣤⣴⡖⣿⡇⠀⢠⡏⠀⠻⡃⠀⠀⠀⣶⣏⣨⣿⣿⣹⠟⠀⠀⠀⠀⠀⣧⠀⣷⠀⠀⠀⠀⠀⠀⣇⠀⠀⠀⠀⠀⠀⠘⡆⠈⠧⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⢀⡀⠀⠀⠀⠀⠀⣀⡶⠏⠀⠀⠀⠀⢰⡾⠀⠀⠀⠀⠀⠀⠀⠀⣸⠁⠀⠀⠀⠀⠀⠀⠀⠀⢸⡿⢿⣶⣸⡷⠈⡇⠀⣾⠀⠀⢸⡇⠀⠀⢾⡏⢿⣿⣿⣇⠀⠀⠀⠀⠀⠀⠀⡿⠀⡿⢿⡀⠀⠀⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀⣿⡀⠀⠉⢆⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠈⢿⡁⠋⠚⠋⠉⠁⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⣰⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⣿⣿⡿⠋⠀⠀⠙⣦⡏⠀⠀⢸⡇⠀⣴⡾⠀⢺⡿⢿⣭⣧⡀⠀⠀⠀⢀⡴⠃⣼⢃⡿⠁⠀⠀⠀⠀⠺⡅⠀⠀⠀⠀⠀⠀⠈⠁⠀⠀⠀⠉⠲⢄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠉⠓⠒⠢⠆⢶⡦⠶⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠻⣄⢣⣷⡄⠀⠀⠀⡾⠀⠀⠀⢸⡟⠛⠉⠀⠀⠈⢳⣼⠀⡿⢿⡖⠒⠚⠉⢀⣴⠷⠚⠁⠀⠀⠀⠀⠀⠀⢿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠓⠒⠦⠤⠤⢦⣴⡶⠞⠃⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⡰⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡾⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⡾⢧⢻⣆⣀⣼⠇⠀⠀⠀⢸⣇⠀⠀⠀⢀⣠⢾⡟⣿⠃⠀⠙⠒⠒⠚⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣦⣤⣴⠒⠋⠁⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⢀⡤⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠞⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠳⣤⣿⢯⡁⢻⠀⠀⠀⠀⣸⢿⠦⣄⣰⣏⣠⣟⡴⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠉⠓⠲⠤⠤⢄⣀⣀⡀⠀",
            "⠀⠀⠀⠀⣠⠔⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡼⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠿⣤⣉⠚⣆⠀⠀⣠⣿⣼⣤⠜⢛⣹⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣤⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡽⠃",
            "⠐⢶⡚⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠖⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡷⢌⣉⢻⣷⣴⠿⠻⣿⣖⢻⡏⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠞⠀⠀",
            "⠀⠀⠉⠓⠦⢄⣀⡀⠀⠀⠀⠀⠀⠀⣀⣠⠴⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣧⠀⠀⠈⡇⠉⠀⢠⡏⠀⢹⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠳⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡤⠚⠁⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠉⠉⠃⠙⠋⠛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⡆⠀⠀⡇⠀⠀⠸⣧⣴⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠢⢄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣠⠴⠚⠁⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢧⠀⠀⣿⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠉⠛⠒⠒⠃⠛⠉⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣷⣙⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        ],
    ];

    let selected_style = rng.random_range(0..styles.len());
    styles[selected_style].clone()
}
