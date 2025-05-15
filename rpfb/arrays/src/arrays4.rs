fn main() {
   
   let name_array: [&str;6] = ["Theo","Nina","Eva","Otis","Vin","Sidi"];

   let mut count = 0;

   while count < name_array.len() {
       println!("Element at {}: {}", count, name_array[count]);
       count = count + 1;
   }

}
