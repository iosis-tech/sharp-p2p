use futures::stream::FuturesUnordered;
use futures::{FutureExt, StreamExt};
use libsecp256k1::{PublicKey, SecretKey};
use rand::thread_rng;
use sharp_p2p_compiler::cairo_compiler::tests::models::fixture as compiler_fixture;
use sharp_p2p_compiler::cairo_compiler::CairoCompiler;
use sharp_p2p_compiler::traits::CompilerController;
use sharp_p2p_runner::cairo_runner::tests::models::fixture as runner_fixture;
use sharp_p2p_runner::cairo_runner::CairoRunner;
use sharp_p2p_runner::traits::RunnerController;

#[tokio::test]
async fn run_single_job() {
    let mut rng = thread_rng();
    let compiler_fixture = compiler_fixture();
    let runner_fixture = runner_fixture();

    let compiler = CairoCompiler::new(libsecp256k1::SecretKey::random(&mut rng));
    let runner = CairoRunner::new(
        runner_fixture.program_path,
        PublicKey::from_secret_key(&SecretKey::random(&mut rng)),
    );

    compiler
        .run(compiler_fixture.program_path, compiler_fixture.program_input_path)
        .unwrap()
        .map(|job| {
            println!("job: {:?}", job);
            runner.run(job.unwrap()).unwrap()
        })
        .flatten()
        .await
        .unwrap();
}

#[tokio::test]
async fn run_multiple_job() {
    let mut rng = thread_rng();
    let compiler_fixture1 = compiler_fixture();
    let compiler_fixture2 = compiler_fixture();
    let runner_fixture1 = runner_fixture();

    let compiler = CairoCompiler::new(libsecp256k1::SecretKey::random(&mut rng));
    let runner = CairoRunner::new(
        runner_fixture1.program_path,
        PublicKey::from_secret_key(&SecretKey::random(&mut rng)),
    );
    let mut futures = FuturesUnordered::new();

    let job_trace1 = compiler
        .run(compiler_fixture1.program_path, compiler_fixture1.program_input_path)
        .unwrap()
        .map(|job| runner.run(job.unwrap()).unwrap())
        .flatten()
        .boxed_local();

    let job_trace2 = compiler
        .run(compiler_fixture2.program_path, compiler_fixture2.program_input_path)
        .unwrap()
        .map(|job| runner.run(job.unwrap()).unwrap())
        .flatten()
        .boxed_local();

    futures.push(job_trace1);
    futures.push(job_trace2);

    while let Some(job_trace) = futures.next().await {
        job_trace.unwrap();
    }
}