package com.krupitskas.pong;

import android.graphics.Bitmap;

public class RustBindings {
    static {
        System.loadLibrary("pong_lib");
    }

    public static native String greeting(final String pattern);

    public static native void renderFractal(Bitmap bitmap);
}