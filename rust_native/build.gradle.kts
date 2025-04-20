
task<Exec>("buildRust"){
    commandLine("sh","-c","cargo build")
}

task<Exec>("clean"){
    commandLine("sh","-c","cargo clean")}