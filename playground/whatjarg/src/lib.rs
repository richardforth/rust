pub fn get_jarg(sc: &String) -> String {
     let normalized_sc = sc.to_lowercase();
      match normalized_sc.as_str() {
          "abc" => {
              format!("\n\tAbstract Base Class (Python).\n\tABC Programming language, precusror to Python.\n")
          }
          "soflam" => {
              format!("\n\tSpecial Operations Forces Laser Aquisition Marker\n")
          }
          _ => {
              format!("\nShortcode {} not recognised.", normalized_sc.as_str())
         }
     }
}

