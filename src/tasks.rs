use std::comm::{channel, Sender, Receiver};

fn spawn_500() {
  for num in range(0u, 500) {
    spawn(proc() {
      println!("{:u}", num);
    });
  }
}

fn channels() {
  let (chan, port) = channel();

  spawn(proc() {
    chan.send(10u);
  });

  println!("{:u}", port.recv());
}

fn add_one(sender: Sender<uint>, receiver: Receiver<uint>) {
  let mut value: uint;
  loop {
    value = receiver.recv();
    sender.send(value + 1);
    if value == 3 { break; }
  }
}

fn main() {
  let (from_parent_sender, from_parent_receiver) = channel();
  let (from_child_sender, from_child_receiver) = channel();

  spawn(proc() {
    add_one(from_child_sender, from_parent_receiver);
  });

  for num in range(0u, 4) {
    from_parent_sender.send(num);
    println!("{:u}", from_child_receiver.recv());
  }
}
