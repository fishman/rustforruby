use task::spawn;
use pipes::stream;
use pipes::Port;
use pipes::Chan;

fn main() {
    let (port, chan): (Port<int>, Chan<int>) = stream();

    do spawn |move chan| {
        chan.send(10);
    }

    io::println(int::str(port.recv()));
}
