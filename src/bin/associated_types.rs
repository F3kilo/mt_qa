fn main() {
    // execute_generic(Request); // Error!
    execute_generic::<Request, u32>(Request);
    execute_associated(Request);
}

trait AssociatedFuture {
    type Output;

    fn poll(&self) -> Self::Output;
}

trait GenericFuture<T> {
    fn poll(&self) -> T;
}

struct Request;

impl GenericFuture<u32> for Request {
    fn poll(&self) -> u32 {
        todo!()
    }
}

impl GenericFuture<String> for Request {
    fn poll(&self) -> String {
        todo!()
    }
}

impl AssociatedFuture for Request {
    type Output = u32;

    fn poll(&self) -> Self::Output {
        todo!()
    }
}

// impl AssociatedFuture for Request {
//     type Output = String;

//     fn poll(&self) -> Self::Output {
//         todo!()
//     }
// }

fn execute_generic<F: GenericFuture<T>, T>(fut: F) -> T {
    fut.poll()
}

fn execute_associated<F:AssociatedFuture>(req: F) -> F::Output {
    req.poll()
}