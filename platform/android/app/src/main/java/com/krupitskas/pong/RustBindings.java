package com.krupitskas.pong;

import android.graphics.Bitmap;

public class RustBindings {
    static {
        System.loadLibrary("pong_lib");
    }

    private static native String greeting(final String pattern);

    public String sayHello(String to) {
        return greeting(to);
    }

    public static native void renderFractal(Bitmap bitmap);
}