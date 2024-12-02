use ic_cdk::api::trap;
use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
struct Thread {
    id: u64,
    title: String,
    content: String,
    author: Principal,
    comments: Vec<Comment>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
struct Comment {
    content: String,
    author: Principal,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
struct Ad {
    id: u64,
    title: String,
    description: String,
    category: String,
    price: u64,
    seller: Principal,
}

struct KaskusReptil {
    threads: HashMap<u64, Thread>,
    ads: HashMap<u64, Ad>,
    next_thread_id: u64,
    next_ad_id: u64,
}

impl KaskusReptil {
    fn new() -> Self {
        Self {
            threads: HashMap::new(),
            ads: HashMap::new(),
            next_thread_id: 1,
            next_ad_id: 1,
        }
    }

    // Forum functionalities
    fn create_thread(&mut self, title: String, content: String, author: Principal) -> u64 {
        let thread = Thread {
            id: self.next_thread_id,
            title,
            content,
            author,
            comments: vec![],
        };
        self.threads.insert(self.next_thread_id, thread);
        self.next_thread_id += 1;
        self.next_thread_id - 1
    }

    fn add_comment(&mut self, thread_id: u64, content: String, author: Principal) -> Result<(), String> {
        if let Some(thread) = self.threads.get_mut(&thread_id) {
            thread.comments.push(Comment { content, author });
            Ok(())
        } else {
            Err("Thread tidak ditemukan".to_string())
        }
    }

    fn list_threads(&self) -> Vec<Thread> {
        self.threads.values().cloned().collect()
    }

    // Marketplace functionalities
    fn create_ad(&mut self, title: String, description: String, category: String, price: u64, seller: Principal) -> u64 {
        let ad = Ad {
            id: self.next_ad_id,
            title,
            description,
            category,
            price,
            seller,
        };
        self.ads.insert(self.next_ad_id, ad);
        self.next_ad_id += 1;
        self.next_ad_id - 1
    }

    fn list_ads(&self, category: Option<String>) -> Vec<Ad> {
        self.ads
            .values()
            .filter(|ad| {
                if let Some(cat) = &category {
                    &ad.category == cat
                } else {
                    true
                }
            })
            .cloned()
            .collect()
    }
}

static mut KASKUS_REPTIL: Option<KaskusReptil> = None;

#[ic_cdk::init]
fn init() {
    unsafe {
        KASKUS_REPTIL = Some(KaskusReptil::new());
    }
}

#[ic_cdk::update]
fn create_thread(title: String, content: String) -> u64 {
    let caller = ic_cdk::caller();
    let platform = unsafe { KASKUS_REPTIL.as_mut().unwrap() };
    platform.create_thread(title, content, caller)
}

#[ic_cdk::update]
fn add_comment(thread_id: u64, content: String) -> Result<(), String> {
    let caller = ic_cdk::caller();
    let platform = unsafe { KASKUS_REPTIL.as_mut().unwrap() };
    platform.add_comment(thread_id, content, caller)
}

#[ic_cdk::query]
fn list_threads() -> Vec<Thread> {
    let platform = unsafe { KASKUS_REPTIL.as_ref().unwrap() };
    platform.list_threads()
}

#[ic_cdk::update]
fn create_ad(title: String, description: String, category: String, price: u64) -> u64 {
    let caller = ic_cdk::caller();
    let platform = unsafe { KASKUS_REPTIL.as_mut().unwrap() };
    platform.create_ad(title, description, category, price, caller)
}

#[ic_cdk::query]
fn list_ads(category: Option<String>) -> Vec<Ad> {
    let platform = unsafe { KASKUS_REPTIL.as_ref().unwrap() };
    platform.list_ads(category)
}
