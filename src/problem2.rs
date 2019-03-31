pub mod problem2 {
    pub fn my_solution(){
       // let f = fibonacci_stop(1,2,0);
        let mut x = 1;
        let mut y = 2;
        let mut accum = 0;
        let f  = loop {
            if x > 4000000{
                break accum;
            }

            if x % 2 == 0 {
                accum += x;
            }
            let aux = y;
            y = x + y;
            x = aux;
        };


        println!("La solucion es {}",f);
    }

}