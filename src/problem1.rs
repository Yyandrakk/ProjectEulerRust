pub mod problem1{
    pub fn my_solution(){

        let sum = (1..1000).filter(|&x| x%3==0 || x%5==0).fold(0,|x, y| x+y);
        println!("La solucion es {}",sum);
    }
}

