use std::fs;


pub fn get_doc_dir()-> Result<String, String>{
    if let Some(document_dir) = dirs::document_dir() {
        // Define the subfolder name for your application
        let app_subfolder = "Rustic"; // Change to your desired subfolder name
    
        // Create the full path to the subfolder
        let app_data_path = document_dir.join(app_subfolder);
    
        // Create the subfolder if it doesn't exist
        if !app_data_path.exists() {
            if let Err(err) = fs::create_dir_all(&app_data_path) {
                return Err(err.to_string());
            }
        }
        Ok(app_data_path.to_str().unwrap_or_default().to_string())
        
    } else {
        Err("error no doc detected".to_owned())
    }
}


pub fn get_music_dir()-> Result<String, String>{
    if let Some(music_dir) = dirs::audio_dir() {
        // Define the subfolder name for your application
        let app_subfolder = "Rustic"; // Change to your desired subfolder name
    
        // Create the full path to the subfolder
        let app_data_path = music_dir.join(app_subfolder);
    
        // Create the subfolder if it doesn't exist
        if !app_data_path.exists() {
            if let Err(err) = fs::create_dir_all(&app_data_path) {
                return Err(err.to_string());
            }
        }
        Ok(app_data_path.to_str().unwrap_or_default().to_string())
        
    } else {
        Err("error no doc detected".to_owned())
    }
}



