pub fn get_jarg(sc: &String) -> String {
     let normalized_sc = sc.to_lowercase();
      match normalized_sc.as_str() {
          "soflam" => {
              format!("Special Operations Forces Laser Aquisition Marker\n")
          }
          _ => {
              format!("\nShortcode {} not recognised.", normalized_sc.as_str())
         }
     }
}

