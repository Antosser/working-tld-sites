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
}
