package com.krupitskas.pong;

public class RustBindings {
    static {
        System.loadLibrary("pong_android");
    }

    private static native String greeting(final String pattern);

    public String sayHello(String to) {
        return greeting(to);
    }
}