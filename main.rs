fn high_and_low(numbers: &str) -> String 
{

  let integers: Vec<i32> = numbers.split_whitespace()
                           .filter_map(|word| word.parse::<i32>().ok())
                           .collect();
                           
   format!("{} {}", integers.clone().iter().max().unwrap(), integers.clone().iter().min().unwrap())                    

}


fn main ()
{
    println!("{:?}", high_and_low("1 9 3 4 -5"));
}
