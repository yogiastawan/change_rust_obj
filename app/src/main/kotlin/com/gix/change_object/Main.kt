package com.gix.change_object
fun main() {
    var app = App()

    val nativeThread = Thread { app.mainNative() }
    nativeThread.start()

    println(app.greeting)

    // Thread.sleep(1) //wait for address initialized

    println("Get address: " + app.getAddress())
    println("Get address: " + app.getAddress())

    println("double the grade")
    app.doubleGrade(app.getAddress())

    println("waiting to off..")

    app.turnOff(app.getAddress())
}
