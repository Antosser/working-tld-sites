use hickory_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tap::Tap;

fn main() {
    let tlds = include_str!("../tlds-alpha-by-domain.txt")
        .lines()
        .collect_vec();

    let working_sites = tlds
        .par_iter()
        .filter(|tld| {
            ureq::get(&format!("http://{}", tld))
                .call()
                .is_ok()
                .tap(|ok| println!("{} - {}", tld, ok))
        })
        .collect::<Vec<_>>();
    println!("{:?}", working_sites);

    // None

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let with_ip = tlds
        .par_iter()
        .filter_map(|tld| -> Option<(&str, String)> {
            Some((
                tld,
                resolver
                    .lookup_ip(*tld)
                    .tap(|v| {
                        if v.is_err() {
                            println!("{} - None", tld)
                        }
                    })
                    .ok()?
                    .iter()
                    .next()
                    .tap(|v| println!("{} - {:?}", tld, v))?
                    .to_string(),
            ))
        })
        .collect::<Vec<_>>();
    println!("{:?}", with_ip);

    // [
    //     ("AI", "209.59.119.34"),
    //     ("ARAB", "127.0.53.53"),
    //     ("CM", "195.24.205.60"),
    //     ("MUSIC", "127.0.53.53"),
    //     ("TK", "217.119.57.22"),
    //     ("UZ", "91.212.89.8"),
    //     ("VA", "2a01:b8:0:1:212:77:0:2"),
    //     ("WS", "64.70.19.33"),
    //     ("XN--L1ACC", "180.149.98.234"),
    //     ("XN--MXTQ1M", "127.0.53.53"),
    //     ("XN--NGBRX", "127.0.53.53"),
    // ]
}
