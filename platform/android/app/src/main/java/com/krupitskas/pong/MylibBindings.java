package com.krupitskas.pong;

public class MylibBindings {
    static {
        System.loadLibrary("mylib");
    }

    public static native int add(int a, int b);
}
