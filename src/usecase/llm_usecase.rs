use dotenv::dotenv;
use std::env;
use std::io::Write;
use llm::Model;

pub struct LlmUsecase {
    
}

impl LlmUsecase {
    pub async fn generate(&self, text: String) -> String {
        dotenv().ok();

        let llm_path = &env::var("LLM_PATH").expect("LLM_PATH must be set");

        // load a GGML model from disk
        let llama = llm::load::<llm::models::NeoX>(
            // path to GGML file
            std::path::Path::new(llm_path),
            // llm::ModelParameters
            Default::default(),
            // load progress callback
            llm::load_progress_callback_stdout
        )
        .unwrap_or_else(|err| panic!("Failed to load model: {err}"));
        println!("ok reading!");

        let mut session = llama.start_session(Default::default());
        println!("start session");

        let res = session.infer::<std::convert::Infallible>(
            // model to use for text generation
            &llama,
            // randomness provider
            &mut rand::thread_rng(),
            // the prompt to use for text generation, as well as other
            // inference parameters
            &llm::InferenceRequest {
                prompt: "Rust",
                maximum_token_count: Some(50), 
                ..Default::default()
            },
            // llm::OutputRequest
            &mut Default::default(),
            // output callback
            |t| {
                print!("{t}");
                std::io::stdout().flush().unwrap();
        
                Ok(())
            }
        );
        println!("returned res");

        match res {
            Ok(result) => println!("\n\nInference stats:\n{result}"),
            Err(err) => println!("\n{err}"),
        }

        llm_path.to_owned()
    }
}

