use clap::Clap;

#[derive(Clap)]
struct Opts {
    #[clap(long, default_value = "1")]
    count: usize,
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();

    //test_complete_file_in_sequence().await;
    //test_complete_file_concurrently().await;
    test_complete_file_concurrently_with_dupes().await;
}

/// Download a complete file concurrently.
async fn test_complete_file_concurrently() {
    println!("Running concurrent range download.");
    let url = "http://codemonkey.net/range/10000.iso";
    let ranges = &[
        "bytes=0-999",
        "bytes=1000-1999",
        "bytes=2000-2999",
        "bytes=3000-3999",
        "bytes=4000-4999",
        "bytes=5000-5999",
        "bytes=6000-6999",
        "bytes=7000-7999",
        "bytes=8000-8999",
        "bytes=9000-9999",
    ];

    let mut tasks = vec![];

    for range in ranges {
        let th = tokio::spawn(async move {
            let client = reqwest::Client::new();
            let response = client
                .get(url)
                .header("Range", *range)
                .send()
                .await
                .unwrap();
            let body = response.bytes().await.unwrap();
            dbg!(body.len());
        });
        tasks.push(th);
    }

    futures::future::join_all(tasks).await;
}

/// Download a complete file concurrently with duplicate requests to fake overlaps.
async fn test_complete_file_concurrently_with_dupes() {
    println!("Running concurrent range download.");
    let url = "http://codemonkey.net/range/10000.iso";
    let ranges = &[
        "bytes=0-999",
        "bytes=1000-1999",
        "bytes=2000-2999",
        "bytes=3000-3999",
        "bytes=4000-4999",
        "bytes=5000-5999",
        "bytes=6000-6999",
        "bytes=7000-7999",
        "bytes=8000-8999",
        "bytes=9000-9999",
    ];

    let mut tasks = vec![];

    for range in ranges {
        let th = tokio::spawn(async move {
            let client = reqwest::Client::new();
            let response = client
                .get(url)
                .header("Range", *range)
                .send()
                .await
                .unwrap();
            let body = response.bytes().await.unwrap();
            dbg!(body.len());
        });
        tasks.push(th);
    }

    for range in ranges {
        let th = tokio::spawn(async move {
            let client = reqwest::Client::new();
            let response = client
                .get(url)
                .header("Range", *range)
                .send()
                .await
                .unwrap();
            let body = response.bytes().await.unwrap();
            dbg!(body.len());
        });
        tasks.push(th);
    }

    for range in ranges {
        let th = tokio::spawn(async move {
            let client = reqwest::Client::new();
            let response = client
                .get(url)
                .header("Range", *range)
                .send()
                .await
                .unwrap();
            let body = response.bytes().await.unwrap();
            dbg!(body.len());
        });
        tasks.push(th);
    }

    futures::future::join_all(tasks).await;
}

/// Download a complete file sequentially.
async fn test_complete_file_in_sequence() {
    let url = "http://codemonkey.net/range/10000.iso";
    let ranges = &[
        "bytes=0-999",
        "bytes=1000-1999",
        "bytes=2000-2999",
        "bytes=3000-3999",
        "bytes=4000-4999",
        "bytes=5000-5999",
        "bytes=6000-6999",
        "bytes=7000-7999",
        "bytes=8000-8999",
        "bytes=9000-9999",
    ];
    let client = reqwest::Client::new();

    for range in ranges {
        let response = client
            .get(url)
            .header("Range", *range)
            .send()
            .await
            .unwrap();
        let body = response.bytes().await.unwrap();
        dbg!(body.len());
    }
}
