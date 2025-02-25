package org.example

class App {
	external fun sayHello();

	companion object {
		init {
			System.loadLibrary("arducat");
		}
	}
}

fun main() {
	println("Привет всем. Сегодня мы попытаемся подружить C++ и Kotlin");
	App().sayHello();
}



