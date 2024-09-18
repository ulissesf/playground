#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B<'a> {
    refa: &'a A,
}

impl B<'_>
{
    pub fn new_b<'a>(ra: &'a A) -> B<'a>
    {
        B { refa: ra }
    }
}

#[derive(Debug)]
struct C<'c> {
    name: String,
    vecb: Vec<B<'c>>,
}

impl C<'_>
{
    pub fn new_c<'a,'b>(n: &'a str) -> C<'b>
    {
        C { name: n.to_string().clone(), vecb: Vec::new() }
    }

    pub fn populate_vecb<'a,'b>(&'a mut self, veca: &'b Vec<A>)
        where 'b: 'a
    {
        for _ in 0..4 {
            let newb = B::new_b(&veca[0]);
            self.vecb.push(newb);
        }
    }

    pub fn get_vecc<'a,'b>(veca: &'b Vec<A>) -> Vec<C<'a>>
        where 'b: 'a
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
