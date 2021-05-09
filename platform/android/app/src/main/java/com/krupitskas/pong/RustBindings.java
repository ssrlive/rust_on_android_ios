package com.krupitskas.pong;

public class RustBindings {

    private static native String greeting(final String pattern);

    public String sayHello(String to) {
        return greeting(to);
    }
}