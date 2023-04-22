//
//  ViewController.swift
//  greetings
//
//  Created by user on 2022/1/9.
//

import UIKit

class ViewController: UIViewController {

    override func viewDidLoad() {
        super.viewDidLoad()

        let rustGreetings = RustGreetings()
        //print("\(rustGreetings.sayHello(to: "world"))")

        let label = UILabel(frame: CGRect(x: 0, y: 0, width: 300, height: 21))
        label.center = CGPoint(x: 160, y: 285)
        label.textAlignment = .center
        label.text = rustGreetings.sayHello(to: "from iOS")
        self.view.addSubview(label)
    }
}
