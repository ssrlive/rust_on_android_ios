package com.krupitskas.pong;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.widget.TextView;

public class MainActivity extends AppCompatActivity {
    static {
        System.loadLibrary("pong_android");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        RustBindings g = new RustBindings();
        String r = g.sayHello("world");
        ((TextView)findViewById(R.id.helloWorldText)).setText(r);
    }
}
