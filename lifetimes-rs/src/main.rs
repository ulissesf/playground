#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B<'b> {
    refa: &'b A,
}

impl B<'_>
{
    pub fn new_b<'b>(ra: &'b A) -> B<'b>
    {
        B { refa: ra }
    }
}

#[derive(Debug)]
struct C<'b> {
    name: String,
    vecb: Vec<B<'b>>,
}

impl<'b> C<'b>
{
    pub fn new_c<'a>(n: &'a str) -> C<'b>
    {
        C { name: n.to_string().clone(), vecb: Vec::new() }
    }

    pub fn populate_vecb<'a>(&'a mut self, veca: &'b Vec<A>)
    {
        for _ in 0..4 {
            let newb = B::new_b(&veca[0]);
            self.vecb.push(newb);
        }
    }

    pub fn get_vecc(veca: &'b Vec<A>) -> Vec<C<'b>>
    {
        let mut vecc: Vec<C> = Vec::new();

        for _ in 0..7 {
            let mut newc = C::new_c(&"bla");
            newc.populate_vecb(veca);
            vecc.push(newc);
        }

        vecc
    }
}

fn main() {
    let mut veca: Vec<A> = Vec::new();
    for _ in 0..5 {
        veca.push(A{});
    }

    let vecc = C::get_vecc(&veca);
    println!("{:?}", vecc);
}
